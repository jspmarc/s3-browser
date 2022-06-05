use crate::file_node::FileNode;
use crate::internal_error::InternalError;
use crate::CurrentClient;
use std::collections::HashMap;

#[tauri::command]
pub async fn list_objects(
  folder: &str,
  current_client: tauri::State<'_, CurrentClient>,
) -> Result<HashMap<String, Vec<FileNode>>, InternalError> {
  if let Some(client) = current_client.0.lock().await.as_ref() {
    client.list_objects_in_folder(folder).await
  } else {
    Err(InternalError::ClientUninitialized)
  }
}

#[tauri::command]
pub async fn head_object(
  file: &str,
  current_client: tauri::State<'_, CurrentClient>,
) -> Result<HashMap<String, String>, InternalError> {
  let result = match current_client.0.lock().await.as_ref() {
    Some(client) => client.head_object(file).await,
    _ => return Err(InternalError::ClientUninitialized),
  };
  let result = match result {
    Ok(result) => result,
    Err(e) => return Err(e),
  };

  Ok(result)
}
