use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
/// remember to update serializer when this struct is updated
pub enum InternalError {
  ClientUninitialized,
  ListObjectsError(String),
  HeadObjectError(String),
  /// hashmap where key name of all files and value is error string
  PutObjectsSomeFailed(HashMap<String, String>),
  // Other(String),
}
