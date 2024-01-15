#[macro_use] extern crate rocket;


mod ctrl_plane_client;
mod file_cache;
mod tc_handler;
mod client_server;

use ctrl_plane_client::FOLDER_NAME;


#[tokio::main]
async fn main() {

    let host_config = ctrl_plane_client::register_host().await;

    let _ = std::fs::create_dir(FOLDER_NAME);
    let mut file_cache = file_cache::Cache::new();
    file_cache.register_listener(tc_handler::new_tc_handler());
    

    match host_config {
        Ok(host_config) => {
            let ctrl_plane_client = ctrl_plane_client::subscribe(file_cache);
            let _ = tokio::spawn(ctrl_plane_client);
            let client_server = client_server::serve(host_config);
            client_server.await;
            ()
        }
        Err(e) => {
            eprintln!("Failed to register host: {}", e);
            std::process::exit(1);
        }
    }

    //Register listeners/event consumers
    

    //Listen for events
    //ctrl_plane_client::subscribe(cache).await;
}


