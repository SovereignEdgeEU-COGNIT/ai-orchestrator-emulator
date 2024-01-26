use rocket::{State, get, post};
use std::sync::{Arc, Mutex};
use rocket::serde::json::Json;
use rocket::{Rocket, Build, routes};

use crate::registry_client::{NodeInfo, NodeType, HostInfo, SRInfo};

use super::faas_server::{self, FaaSServer};

pub struct Registry {
    hosts: Mutex<Vec<HostInfo>>,
    srs: Mutex<Vec<SRInfo>>,
    faas_server: Arc<Mutex<FaaSServer>>,
    //flavor_map: Mutex<Vec<Vec<String>>>,
}

impl Registry {
    pub fn new(faas_server: Arc<Mutex<FaaSServer>>) -> Registry {
        Registry {
            hosts: Mutex::new(Vec::new()),
            srs: Mutex::new(Vec::new()),
            faas_server: faas_server,
            //flavor_map: Mutex::new(Vec::new()),
        }
    }

    fn register_host(&self, host: HostInfo) {
        let mut hosts = self.hosts.lock().unwrap();

        match hosts.iter_mut().find(|existing_host | existing_host.get_name() == host.get_name()) {
            Some(existing_host) => {
                *existing_host = host;
            },
            None => {
                hosts.push(host);
                //flavor_map.push(Vec::new());
            }
        }

        hosts.iter().for_each(|x| println!("{:?}", x));
    }

    fn register_sr(&self, sr: SRInfo) {
        self.faas_server.lock().unwrap().initiate_faas(sr.clone());
        let mut srs = self.srs.lock().unwrap();

        match srs.iter_mut().find(|existing_sr | existing_sr.get_name() == sr.get_name()) {
            Some(existing_sr) => {
                *existing_sr = sr;
            },
            None => {
                srs.push(sr);
            }
        }

        srs.iter().for_each(|x| println!("{:?}", x));
    }

    fn get_hosts(&self) -> Vec<HostInfo> {
        let hosts = self.hosts.lock().unwrap();
        hosts.iter().cloned().collect()
    }

    fn get_srs(&self) -> Vec<SRInfo> {
        let sr = self.srs.lock().unwrap();
        sr.iter().cloned().collect()
    }
}

/**
 * Registers a new host.
 *
 * # Arguments
 * * `node_ip` - The IP address of the new host.
 * * `hosts_shared` - Shared state of hosts.
 *
 * # Returns
 * The port number assigned to the new host.
 */
// curl 194.28.122.122:8000/register/host -X POST -H "Content-Type: application/json" -d '{"ip": "
#[post("/register", format = "json", data = "<node>")]
fn register_node(node: Json<NodeType>, registry: &State<Registry>) {
    
    match node.0 {
        NodeType::Host(host) => {
            registry.register_host(host);
        },
        NodeType::SR(sr) => {
            registry.register_sr(sr);
        }
    }
}

//curl 194.28.122.122:8000/list?node_type=host
#[get("/list?<node_type>")]
fn list(node_type: String, registry: &State<Registry>) -> Json<Vec<NodeType>> {

    match node_type.as_str() {
        "host" => {
            Json(registry.get_hosts().iter().map(|x| NodeType::Host(x.clone())).collect())
        },
        "sr" => {
            Json(registry.get_srs().iter().map(|x| NodeType::SR(x.clone())).collect())
        },
        _ => {
            Json(Vec::new())
        }
    }
}

/* #[get("/hosts/flavor")]
fn get_host_flavors(hosts_shared: &State<Mutex<Vec<Host>>>, flavor_map_shared: &State<Mutex<Vec<Vec<String>>>>) -> Json<Vec<FlavorMapping>> {
    let mut hosts = hosts_shared.lock().unwrap();
    let mut flavor_map = flavor_map_shared.lock().unwrap();
    let flavor_mapping = hosts.iter()
        .zip(flavor_map.iter())
        .map(|(h, f)| FlavorMapping{host: h.clone(), flavors: f.clone()})
        .collect();
    Json(flavor_mapping)
} */

pub fn initiate(rocket: Rocket<Build>, faas_server: Arc<Mutex<FaaSServer>>) -> Rocket<Build>{
    rocket.mount("/", routes![register_node, list])
        .manage(Registry::new(faas_server))
}