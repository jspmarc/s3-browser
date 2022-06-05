use serde::ser::{Serialize, SerializeStruct, Serializer};

const FOLDER_MIME: &'static str = "inode/directory";

pub struct FileNode {
  last_modified: Option<i64>,
  is_folder: bool,
  name: String,
  s3_key: String,
  size: i64,
}

impl FileNode {
  pub fn new(
    last_modified: Option<i64>,
    is_folder: bool,
    name: String,
    s3_key: String,
    size: i64,
  ) -> Self {
    Self {
      last_modified,
      is_folder,
      name,
      s3_key,
      size,
    }
  }
}

impl Serialize for FileNode {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("FileNode", 5)?;
    state.serialize_field("last_modified", &self.last_modified)?;
    state.serialize_field("is_folder", &self.is_folder)?;
    state.serialize_field("name", &self.name)?;
    state.serialize_field("s3_key", &self.s3_key)?;
    state.serialize_field("size", &self.size)?;
    state.end()
  }
}
