
use rocket::futures::SinkExt;

use rocket::{State, get, post};
use std::iter::Zip;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use rocket::routes;



use tokio::runtime::Runtime;

use self::faas_server::FaaSServer;

//use crate::common::{Host, FileInfo};

mod file_server;
mod faas_server;
mod registry_server;
/* 
#[derive(Serialize, Deserialize)]
struct FlavorMapping {
    host: Host,
    flavors: Vec<String>
}
 */

#[get("/")]
fn index() -> &'static str {
    "Hello, this is the server!"
}

#[rocket::main]
pub async fn rocket() {
    
    /* let nodes = Vec::<NodeInfo>::new();
    let hosts_shared = Mutex::new(hosts);

    let connections: Mutex<Option<WebSocketConnection>> = Mutex::new(None);

    let flavor_map = Vec::<Vec::<String>>::new();
    let flavor_map_shared = Mutex::new(flavor_map); */
    let faas_server = Arc::new(FaaSServer::new());
    let mut rocket = rocket::build();
    rocket = rocket.mount("/", routes![index]);
    rocket = registry_server::initiate(rocket, faas_server.clone());
    rocket = faas_server::initiate(rocket, faas_server.clone());
    rocket = file_server::initiate(rocket);
    rocket.launch().await;
        /*.mount("/", routes![index, list_files, files, register_host, get_hosts, get_host_flavors, websocket, start_job])
        .manage(hosts_shared)
        .manage(flavor_map_shared)
        .manage(connections) */
}
