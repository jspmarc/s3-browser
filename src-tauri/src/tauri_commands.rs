use crate::internal_error::InternalError;
use crate::CurrentClient;
use std::collections::HashMap;

#[tauri::command]
pub async fn list_objects(
  folder: &str,
  current_client: tauri::State<'_, CurrentClient>,
) -> Result<HashMap<String, Vec<String>>, InternalError> {
  if let Some(client) = current_client.0.lock().await.as_ref() {
    client.list_objects_in_folder(folder).await
  } else {
    Err(InternalError::ClientUninitialized)
  }
}
