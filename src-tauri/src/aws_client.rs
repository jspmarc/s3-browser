use std::str::FromStr;
use aws_sdk_s3::{
  Client,
  Credentials,
  Config,
  Endpoint,
  Region,
};
use http::Uri;
use serde::ser::{Serialize, Serializer, SerializeStruct};

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
    let cred = Credentials::new(access_key_id.clone(), secret_access_key.clone(), None, None, "");
    let conf = {
      if is_path_style {
        Config::builder()
          .endpoint_resolver(Endpoint::immutable(Uri::from_str(&endpoint).unwrap()))
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
  // TODO: implement correct error type
  /// Lists objects inside a folder (prefix), "/" is equal to ""
  pub async fn list_objects_in_folder(&self, prefix: &str) -> Result<Vec<String>, String> {
    // make "/" to ""
    let prefix: &str = {
      if prefix == "/" {
        ""
      } else {
        prefix
      }
    };

    // get S3 client
    let client = match self.s3_client.as_ref() {
      Some(client) => client,
      None => return Err("S3 client is not initialized".to_string()),
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
      Err(e) => return Err(e.to_string()),
    };
    // insert key results to a vector
    let mut keys: Vec<String> = vec![];
    res.contents().unwrap_or_default().iter().for_each(|key| {
      if let Some(k) = key.key() {
        keys.push(k.to_owned());
      }
    });
    res.common_prefixes().unwrap_or_default().iter().for_each(|prefix| {
      if let Some(k) = &prefix.prefix {
        keys.push(k.clone());
      }
    });
    Ok(keys)
  }
}
