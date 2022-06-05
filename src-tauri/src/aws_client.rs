use crate::internal_error::InternalError;
use aws_sdk_s3::{Client, Config, Credentials, Endpoint, Region};
use http::Uri;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::{collections::HashMap, str::FromStr};

pub struct AwsClient {
  s3_client: Option<Client>,
  access_key_id: String,
  bucket_name: String,
  secret_access_key: String,
  endpoint: String,
  region: String,
  is_path_style: bool,
}

// constructors, getters, and setters
impl AwsClient {
  pub fn new(
    access_key_id: String,
    bucket_name: String,
    secret_access_key: String,
    endpoint: String,
    region: String,
    is_path_style: bool,
  ) -> Self {
    let cred = Credentials::new(
      access_key_id.clone(),
      secret_access_key.clone(),
      None,
      None,
      "",
    );
    let conf = {
      if is_path_style {
        Config::builder().endpoint_resolver(Endpoint::immutable(Uri::from_str(&endpoint).unwrap()))
      } else {
        Config::builder()
      }
    }
    .credentials_provider(cred)
    .region(Region::new(region.clone()))
    .build();
    let client = Client::from_conf(conf);

    Self {
      s3_client: Some(client),
      access_key_id,
      bucket_name,
      secret_access_key,
      endpoint,
      region,
      is_path_style,
    }
  }
}

impl Serialize for AwsClient {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("AwsClient", 5)?;
    state.serialize_field("access_key_id", &self.access_key_id)?;
    state.serialize_field("bucket_name", &self.bucket_name)?;
    state.serialize_field("secret_access_key", &self.secret_access_key)?;
    state.serialize_field("endpoint", &self.endpoint)?;
    state.serialize_field("region", &self.region)?;
    state.serialize_field("is_path_style", &self.is_path_style)?;
    state.end()
  }
}

// S3 commands
impl AwsClient {
  /// Lists objects inside a folder (prefix)
  /// returns a HashMap<String, Vec<String>> where the keys are: "folders" and "files"
  ///
  /// # Arguments
  ///
  /// - `prefix` - folder name (e.g.: "", "foo/bar")
  pub async fn list_objects_in_folder(
    &self,
    prefix: &str,
  ) -> Result<HashMap<String, Vec<String>>, InternalError> {
    // get S3 client
    let client = match self.s3_client.as_ref() {
      Some(client) => client,
      None => return Err(InternalError::ClientUninitialized),
    };
    // build request
    let req = client
      .list_objects_v2()
      .prefix(prefix)
      .delimiter("/")
      .bucket(&self.bucket_name);
    // send and parse response
    let res = req.send().await;
    let res = match res {
      Ok(res) => res,
      Err(e) => return Err(InternalError::ListObjectsError(e.to_string())),
    };
    // insert key results to a vector
    let mut keys: HashMap<String, Vec<String>> = HashMap::new();
    keys.insert("files".into(), vec![]);
    keys.insert("folders".into(), vec![]);
    res.contents().unwrap_or_default().iter().for_each(|key| {
      if let Some(k) = key.key() {
        keys
          .get_mut("files")
          .unwrap()
          .push(k.split('/').last().unwrap().to_string())
      }
    });
    res
      .common_prefixes()
      .unwrap_or_default()
      .iter()
      .for_each(|prefix| {
        if let Some(k) = &prefix.prefix {
          keys
            .get_mut("folders")
            .unwrap()
            .push(k.split('/').nth_back(1).unwrap().to_owned())
        }
      });
    Ok(keys)
  }

  /// HEAD an object with certain `key`
  /// returns a HashMap<String, String> where the keys are "content_type" and "key" (as in object
  /// key). If content_type
  ///
  /// # Arguments
  ///
  /// - `key` - object/file key/name
  pub async fn head_object(&self, key: &str) -> Result<HashMap<String, String>, InternalError> {
    let guess_type = || {
      mime_guess::from_path(key.split('/').nth_back(0).unwrap())
        .first_or_octet_stream()
        .to_string()
    };

    // get S3 client
    let client = match self.s3_client.as_ref() {
      Some(client) => client,
      None => return Err(InternalError::ClientUninitialized),
    };
    // build request
    let req = client.head_object().key(key).bucket(&self.bucket_name);
    // send and parse response
    let res = req.send().await;
    let res = match res {
      Ok(res) => res,
      Err(e) => return Err(InternalError::HeadObjectError(e.to_string())),
    };
    let content_type = match res.content_type() {
      Some(ct) => Some(ct.to_string()),
      _ => None,
    };
    let content_type = match content_type {
      Some(ct) => {
        if ct == "application/octet-stream".to_string() {
          guess_type()
        } else {
          ct
        }
      }
      _ => guess_type(),
    };
    let mut retval = HashMap::<String, String>::new();
    retval.insert("key".into(), key.into());
    retval.insert("content_type".into(), content_type);
    Ok(retval)
  }
}
