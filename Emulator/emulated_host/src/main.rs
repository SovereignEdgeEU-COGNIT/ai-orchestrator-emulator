


mod ctrl_plane_client;
mod file_cache;
mod tc_handler;

use ctrl_plane_client::FOLDER_NAME;

#[tokio::main]
async fn main() {
    let _ = std::fs::create_dir(FOLDER_NAME);
    let mut cache = file_cache::Cache::new();

    ctrl_plane_client::register_host().await;

    //Register listeners/event consumers
    cache.register_listener(tc_handler::new_tc_handler());

    //Listen for events
    //ctrl_plane_client::subscribe(cache).await;
}

