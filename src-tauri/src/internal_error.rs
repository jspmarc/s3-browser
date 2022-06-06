use std::collections::HashMap;

use serde::ser::{Serialize, Serializer};

/// remember to update serializer when this struct is updated
pub enum InternalError {
  ClientUninitialized,
  ListObjectsError(String),
  HeadObjectError(String),
  /// hashmap where key name of all files and value is error string
  PutObjectsSomeFailed(HashMap<String, String>),
  // Other(String),
}

impl Serialize for InternalError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      InternalError::ClientUninitialized => {
        serializer.serialize_unit_variant("InternalError", 0, "ClientUninitialized")
      }
      InternalError::ListObjectsError(s) => {
        serializer.serialize_newtype_variant("InternalError", 1, "ListObjectsError", &s)
      }
      InternalError::HeadObjectError(s) => {
        serializer.serialize_newtype_variant("InternalError", 2, "GetObjectError", &s)
      }
      InternalError::PutObjectsSomeFailed(s) => {
        serializer.serialize_newtype_variant("InternalError", 3, "PutObjectsSomeFailed", &s)
      } // InternalError::Other(s) => {
        //   serializer.serialize_newtype_variant("InternalError", 3, "Other", &s)
        // }
    }
  }
}
