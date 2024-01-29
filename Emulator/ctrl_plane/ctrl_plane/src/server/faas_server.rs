use rocket::futures::SinkExt;
use rocket_ws::{WebSocket, Channel};
use std::collections::{HashMap, LinkedList};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use rocket::serde::json::Json;
use rocket::{State, get, post, Rocket, Build, routes};
use serde::{Serialize, Deserialize};

use crate::registry_client::{HostInfo, NodeInfo, SRInfo};
use crate::host_agent_client::{self};

use super::registry_server;


// Define a struct to hold WebSocket connections
struct WebSocketConnection {
    tx: mpsc::Sender<SRInfo>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SREnv {
    cpu: f32,
    mem: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FaaSReq {
    host_info: HostInfo,
    flavor: String,
    sr_env: SREnv,
}

pub struct FaaSServer {
    client_emulator: Mutex<Option<WebSocketConnection>>,
    uninitiated_faas: Mutex<HashMap<String, FaaSReq>>,
    //host_agent_client: Mutex<HostAgentClient>,
}

impl FaaSServer {

    pub fn new(/* host_agent_client_mutex: Mutex<HostAgentClient> */) -> FaaSServer {
        FaaSServer {
            client_emulator: Mutex::new(None),
            uninitiated_faas: Mutex::new(HashMap::new()),
            //host_agent_client: host_agent_client_mutex,
        }
    }

    fn set_client(&self, ws: WebSocket) -> Channel<'static> {
        let (new_client_emulator, channel) = WebSocketConnection::new(ws);
        *self.client_emulator.lock().unwrap() = Some(new_client_emulator);

        channel
    }

    fn send(&self, sr_info: SRInfo) {
        if let Some(cem) = &*self.client_emulator.lock().unwrap() {
            cem.send(sr_info);
        }
    }

    fn add_uninitated_faas(&self, sr_name: String, faas_req: FaaSReq) {
        let mut uninitiated_faas = self.uninitiated_faas.lock().unwrap();
        uninitiated_faas.insert(sr_name.clone(), faas_req);
    }

    pub fn initiate_faas(&self, mut sr_info: SRInfo) -> SRInfo {
        let mut uninitiated_faas = self.uninitiated_faas.lock().unwrap();
        let faas_req = uninitiated_faas.remove(sr_info.get_name()).unwrap(); // WARNING: This might cause panic if SR wasn't initiated by this server or if race condition occurs
        sr_info.set_flavor(faas_req.flavor);
        sr_info.set_host_info(faas_req.host_info);
        self.send(sr_info.clone());
        sr_info
    }
}

impl WebSocketConnection {

    fn new(ws: WebSocket) -> (WebSocketConnection, Channel<'static>) {
        let (tx, rx): (Sender<SRInfo>, Receiver<SRInfo>) = mpsc::channel();

        let channel = ws.channel(move |mut ws_stream| Box::pin(async move {
        
            tokio::spawn(async move {
                loop {
                    if let Ok(sr_info) = rx.recv() {
                        let sr_info_str = serde_json::to_string(&sr_info).unwrap();
                        println!("sending {}", sr_info_str);
                        //ws_stream.send(rocket_ws::Message::Text(sr_info_str));
                        ws_stream.send(rocket_ws::Message::Text(sr_info_str)).await.expect("client conn failed");
                    }
                }
            });
            Ok(())
        }));

        (WebSocketConnection{tx: tx}, channel)
    }

    fn send(&self, sr_info: SRInfo) {
        let _ = self.tx.send(sr_info);
    }
}


#[get("/websocket")]
fn websocket(ws: WebSocket, server: &State<Arc<Mutex<FaaSServer>>>) -> Channel<'static> {
    server.lock().unwrap().set_client(ws)
}

//curl 194.28.122.122:8000/start -X POST -H "Content-Type: application/json" -d '{"host_info": {"ip":"194.28.122.122","name":"Cognit-test","port":8001}, "flavor": "flavor1", "sr_env": {"cpu": 1.0, "mem": 1024}}'
#[post("/start", format = "json", data = "<faas_req>")]
async fn start_function(
        faas_req: Json<FaaSReq>, 
        server: &State<Arc<Mutex<FaaSServer>>>,
    ) {

    let sr_env = faas_req.0.sr_env.clone();
    let host_info = faas_req.0.host_info.clone();
    let sr_name = host_agent_client::start_sr(host_info, sr_env.cpu, sr_env.mem).await.unwrap();
    server.lock().unwrap().add_uninitated_faas(sr_name, faas_req.0);
    //let sr_info: SRInfo = registry_server::subscribe_sr(sr_name.clone()).await;
    /* let mut hosts = hosts_shared.lock().unwrap();
    let mut flavor_map = flavor_map_shared.lock().unwrap();
    //flavor_map.push(Vec::new());
    hosts.iter_mut()
        .zip(flavor_map.iter_mut())
        .filter(|(host, flavors)| host.name == job_info.host.name)
        //.for_each(|(host, flavors)| flavors.append(&mut job_info.flavors.clone()));
        .for_each(|(host, flavors)| *flavors = job_info.flavors.clone());
    let cem = client_emulator.lock().unwrap();
     */
    /* let sr_info = SRInfo::new(
        "asdf".to_string(), // TODO: get ip
        8080, // TODO: get port
        sr_name,
        faas_req.0.flavor,
    );
    server.send(sr_info); */
}


pub fn initiate(rocket: Rocket<Build>, faas_server: Arc<Mutex<FaaSServer>>) -> Rocket<Build>{
    rocket.mount("/", routes![websocket, start_function])
        .manage(faas_server)
}