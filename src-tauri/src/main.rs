#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::str::FromStr;

use aws_sdk_s3::{
  Client,
  Credentials,
  Config,
  Endpoint,
  Region
};
use http::Uri;

fn new_aws_client(
  access_key_id: String,
  secret_access_key: String,
  endpoint: String,
  region: String,
  is_path_style: bool,
) -> Client {
  let cred = Credentials::new(access_key_id, secret_access_key, None, None, "");
  let conf = {
    if is_path_style {
      Config::builder()
        .endpoint_resolver(Endpoint::immutable(Uri::from_str(&endpoint).unwrap()))
    } else {
      Config::builder()
    }
  }
    .credentials_provider(cred)
    .region(Region::new(region))
    .build();
  let client = Client::from_conf(conf);

  client
}

#[tauri::command]
async fn new_s3(
  name: String,
  access_key_id: String,
  secret_access_key: String,
  endpoint: String,
  region: String,
  is_path_style: bool,
) {
  let client = new_aws_client(access_key_id, secret_access_key, endpoint, region, is_path_style);
  let req = client
    .list_objects_v2()
    .prefix("")
    .bucket(name);
  let res = req.send().await;
  match res {
    Ok(results) => {
      for objs in results.contents {
        objs.iter().for_each(|obj| {
          if let Some(key) = &obj.key {
            println!("{}", key);
          }
        });
      }
    }
    Err(e) => {
      println!("Error: {}", e);
    }
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![new_s3])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
