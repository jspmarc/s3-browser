#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod s3_client;
use s3_client::S3Client;

#[tauri::command]
fn hello(user_name: String) -> String {
  let s = format!("Hello, {}!", user_name);
  println!("{}", s);
  s.into()
}

#[tauri::command]
fn new_s3(bucket_url: String, access_key_id: String, secret_access_key: String) {
  println!("Creating new S3 client");
  let s3 = S3Client::new(&bucket_url, &access_key_id, &secret_access_key);
  println!("Created an S3 client");
  s3.print();
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![hello, new_s3])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
