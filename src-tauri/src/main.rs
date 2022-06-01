#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod aws_client;
mod internal_error;
use std::sync::Mutex;

use aws_client::AwsClient;
use internal_error::InternalError;

struct CurrentClient(Mutex<Option<AwsClient>>);

#[tauri::command]
async fn init_app(
  name: String,
  access_key_id: String,
  secret_access_key: String,
  endpoint: String,
  region: String,
  is_path_style: bool,
  current_client: tauri::State<'_, CurrentClient>,
) -> Result<(), InternalError> {
  let client = AwsClient::new(
    access_key_id,
    name,
    secret_access_key,
    endpoint,
    region,
    is_path_style,
  );
  *current_client.0.lock().unwrap() = Some(client);
  Ok(())
}

fn main() {
  tauri::Builder::default()
    .manage(CurrentClient(Mutex::new(None)))
    .invoke_handler(tauri::generate_handler![init_app])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
