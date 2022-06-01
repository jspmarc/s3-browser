#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod aws_client;
use aws_client::AwsClient;
mod file_node;

#[tauri::command]
async fn new_s3(
  name: String,
  access_key_id: String,
  secret_access_key: String,
  endpoint: String,
  region: String,
  is_path_style: bool,
) -> Result<Vec<String>, String> {
  let client = AwsClient::new(
    access_key_id,
    name,
    secret_access_key,
    endpoint,
    region,
    is_path_style,
  );

  let files = client.list_objects_in_folder("/").await;

  files
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![new_s3])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
