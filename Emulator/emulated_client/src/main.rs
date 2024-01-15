mod ctrl_plane_client;
mod host_client;
mod file_cache;

use ctrl_plane_client::FOLDER_NAME;
use std::{sync::{Mutex, Arc}, error::Error};

#[tokio::main]
async fn main() {
    let _ = std::fs::create_dir(FOLDER_NAME);
    let mut file_cache = file_cache::Cache::new();
    
    let hosts = Arc::new(Mutex::new(Vec::new()));

    let host_subscriber = ctrl_plane_client::subscribe_hosts(Arc::clone(&hosts));
    let _ = tokio::spawn(host_subscriber);

    let file_subscriber = ctrl_plane_client::subscribe_files(file_cache);
    let _ = tokio::spawn(file_subscriber);

    host_client::emulate_client(Arc::clone(&hosts)).await;
}    