use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ObjectPut {
  pub path: String,
  pub key: String,
  pub acl: String,
}
