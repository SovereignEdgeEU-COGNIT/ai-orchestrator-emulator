#[macro_use] extern crate rocket;


//mod ctrl_plane_client;
//mod file_cache;
mod tc_handler;
mod client_server;


//use ctrl_plane_client::FOLDER_NAME;
use ctrl_plane::registry_client::{self, NodeInfo};
use ctrl_plane::file_client;


#[tokio::main]
async fn main() {

    let sr_info = registry_client::SRInfo::empty();
    let reg_res = sr_info.register().await;

    let listeners = vec![tc_handler::new_tc_handler()];
    

    match reg_res {
        Ok(_) => {
            file_client::listen_on_thread(listeners);
            
            println!("{:?}", sr_info);
            let client_server = client_server::serve(sr_info);
            let _ = client_server.await;
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


