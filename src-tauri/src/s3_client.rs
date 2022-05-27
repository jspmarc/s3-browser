#[derive(Debug)]
pub struct S3Client {
  bucket_url: String,
  access_key_id: String,
  secret_access_key: String,
}

impl S3Client {
  pub fn new(bucket_url: &str, access_key_id: &str, secret_access_key: &str) -> Self {
    Self {
      bucket_url: bucket_url.to_string(),
      access_key_id: access_key_id.to_string(),
      secret_access_key: secret_access_key.to_string(),
    }
  }

  pub fn print(&self) {
    println!("Bucket URL: {}\nAccess key ID: {}\nSecret access key: {}",
      self.bucket_url,
      self.access_key_id,
      self.secret_access_key
    ).into()
  }
}
