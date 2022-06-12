use aws_sdk_s3::{
  error::PutObjectError, model::ObjectCannedAcl, output::PutObjectOutput, types::ByteStream,
  types::SdkError, Client,
};
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct ObjectPut {
  pub path: String,
  pub key: String,
  pub acl: String,
}

impl ObjectPut {
  pub async fn put(
    &self,
    client: &Client,
    bucket_name: &String,
  ) -> (
    &String,
    Result<Result<PutObjectOutput, SdkError<PutObjectError>>, String>,
  ) {
    let path = Path::new(&self.path);
    if !path.exists() {
      return (
        &self.path,
        Err(format!("Path {} doesn't exist", path.display())),
      );
    }

    // get body
    let body = ByteStream::from_path(path).await;
    let body = match body {
      Ok(body) => body,
      Err(e) => {
        return (&self.path, Err(e.to_string()));
      }
    };

    let content_type = mime_guess::from_path(path)
      .first_or_octet_stream()
      .to_string();
    let acl = match self.acl.as_str() {
      "AuthenticatedRead" => ObjectCannedAcl::AuthenticatedRead,
      "AwsExecRead" => ObjectCannedAcl::AwsExecRead,
      "BucketOwnerFullControl" => ObjectCannedAcl::BucketOwnerFullControl,
      "BucketOwnerRead" => ObjectCannedAcl::BucketOwnerRead,
      "Private" => ObjectCannedAcl::Private,
      "PublicRead" => ObjectCannedAcl::PublicRead,
      "PublicReadWrite" => ObjectCannedAcl::PublicReadWrite,
      _ => ObjectCannedAcl::PublicRead,
    };
    // build request
    let req = client
      .put_object()
      .body(body)
      .content_type(content_type)
      .key(self.key.to_owned())
      .acl(acl)
      .bucket(bucket_name);

    // send
    (&self.path, Ok(req.send().await))
  }
}
