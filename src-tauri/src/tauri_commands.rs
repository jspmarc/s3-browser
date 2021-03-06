use crate::{
  file_node::FileNode, internal_error::InternalError, object_put::ObjectPut, CurrentClient,
};
use std::collections::HashMap;

#[tauri::command]
pub async fn list_objects(
  key: &str,
  current_client: tauri::State<'_, CurrentClient>,
) -> Result<HashMap<String, Vec<FileNode>>, InternalError> {
  if let Some(client) = current_client.0.lock().await.as_ref() {
    client.list_objects_in_folder(key).await
  } else {
    Err(InternalError::ClientUninitialized)
  }
}

#[tauri::command]
pub async fn head_object(
  key: &str,
  current_client: tauri::State<'_, CurrentClient>,
) -> Result<HashMap<String, String>, InternalError> {
  let result = match current_client.0.lock().await.as_ref() {
    Some(client) => client.head_object(key).await,
    _ => return Err(InternalError::ClientUninitialized),
  };
  let result = match result {
    Ok(result) => result,
    Err(e) => return Err(e),
  };

  Ok(result)
}

#[tauri::command]
pub async fn delete_object(
  key: &str,
  current_client: tauri::State<'_, CurrentClient>,
) -> Result<(), InternalError> {
  if let Some(client) = current_client.0.lock().await.as_ref() {
    client.delete_object(key).await
  } else {
    Err(InternalError::ClientUninitialized)
  }
}

#[tauri::command]
pub async fn put_multiple_objects(
  objects: Vec<ObjectPut>,
  current_client: tauri::State<'_, CurrentClient>,
) -> Result<(), InternalError> {
  if let Some(client) = current_client.0.lock().await.as_ref() {
    client.put_multiple_objects(&objects).await
  } else {
    Err(InternalError::ClientUninitialized)
  }
}
