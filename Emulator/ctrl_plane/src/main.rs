#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;
use rocket::futures::SinkExt;
use rocket::serde::{json::Json, Serialize, Deserialize};
use rocket::State;
use std::iter::Zip;
use std::path::{Path, PathBuf};
use std::{fs, thread};
use std::sync::Mutex;
use sha2::{Sha256, Digest};
use rocket_ws::{WebSocket, Channel};
use std::sync::mpsc::{self, Receiver, Sender};
use tokio::runtime::Runtime;

#[derive(Serialize)]
struct FileInfo {
    filename: String,
    hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Host {
    ip: String,
    name: String,
    port: u16
}

#[derive(Serialize, Deserialize)]
struct FlavorMapping {
    host: Host,
    flavors: Vec<String>
}


#[get("/")]
fn index() -> &'static str {
    "Hello, this is the server!"
}

#[get("/files/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("files").join(file)).await.ok()
}

/**
 * Lists all files in the "files" directory.
 *
 * # Returns
 * A JSON array of FileInfo objects, each containing a filename and its SHA-256 hash.
 */
#[get("/files")]
fn list_files() -> Json<Vec<FileInfo>> {
    let mut files_info = Vec::new();
    let path = Path::new("files");
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let filename = path.file_name().unwrap().to_str().unwrap().to_owned();
                let hash = hash_file(&path);
                files_info.push(FileInfo { filename, hash });
            }
        }
    }
    Json(files_info)
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
#[post("/register", format = "json", data = "<node_info>")]
fn register_host(node_info: Json<Host>, hosts_shared: &State<Mutex<Vec<Host>>>, flavor_map_shared: &State<Mutex<Vec<Vec<String>>>>) -> Json<u16> {
    let mut hosts = hosts_shared.lock().unwrap();
    let mut flavor_map = flavor_map_shared.lock().unwrap();
    let last_alloc_port = hosts.last().unwrap_or(&Host{ip: String::new(), name:String::new(), port:1233}).port;
    let mut host = node_info.0.clone();
    host.port = last_alloc_port + 1;
    hosts.push(host);
    flavor_map.push(Vec::new());
    hosts.iter().for_each(|x| println!("{:?}", x));
    Json(last_alloc_port + 1)
}

#[get("/hosts")]
fn get_hosts(hosts_shared: &State<Mutex<Vec<Host>>>) -> Json<Vec<Host>> {
    let hosts = hosts_shared.lock().unwrap();
    Json(hosts.iter().cloned().collect())
}

#[get("/hosts/flavor")]
fn get_host_flavors(hosts_shared: &State<Mutex<Vec<Host>>>, flavor_map_shared: &State<Mutex<Vec<Vec<String>>>>) -> Json<Vec<FlavorMapping>> {
    let mut hosts = hosts_shared.lock().unwrap();
    let mut flavor_map = flavor_map_shared.lock().unwrap();
    let flavorMappings = hosts.iter()
        .zip(flavor_map.iter())
        .map(|(h, f)| FlavorMapping{host: h.clone(), flavors: f.clone()})
        .collect();
    Json(flavorMappings)
}

#[post("/start", format = "json", data = "<job_info>")]
fn start_job(
        job_info: Json<FlavorMapping>, 
        hosts_shared: &State<Mutex<Vec<Host>>>, 
        flavor_map_shared: &State<Mutex<Vec<Vec<String>>>>,
        client_emulator: &State<Mutex<Option<WebSocketConnection>>>
    ) {
    let mut hosts = hosts_shared.lock().unwrap();
    let mut flavor_map = flavor_map_shared.lock().unwrap();
    //flavor_map.push(Vec::new());
    hosts.iter_mut()
        .zip(flavor_map.iter_mut())
        .filter(|(host, flavors)| host.name == job_info.host.name)
        .for_each(|(host, flavors)| flavors.append(&mut job_info.flavors.clone()));
    let cem = client_emulator.lock().unwrap();
    
    if let Some(cem) = &*cem {
        cem.send(job_info.0);
    }
}

/**
 * A websocket where an client emulator connects.
 * When a new start_job() is received it forwards the host-flavor info to the client
 * that will start to emulate clients sending that flavor of requests to the host.
 */

//use rocket::{get, launch, post, routes};


// ... [rest of your existing code, like struct definitions] ...

// Define a struct to hold WebSocket connections
struct WebSocketConnection {
    tx: mpsc::Sender<FlavorMapping>
}

impl WebSocketConnection {

    fn new(ws: WebSocket) -> (WebSocketConnection, Channel<'static>) {
        let (tx, rx): (Sender<FlavorMapping>, Receiver<FlavorMapping>) = mpsc::channel();

        let channel = ws.channel(move |mut ws_stream| Box::pin(async move {
        
            tokio::spawn(async move {
                loop {
                    if let Ok(job_info) = rx.recv() {
                        let job_info_str = serde_json::to_string(&job_info).unwrap();
                        println!("sending {}", job_info_str);
                        ws_stream.send(rocket_ws::Message::Text(job_info_str)).await.expect("client conn failed");
                    }
                }
            });
            Ok(())
            //let message = format!("Hello, {}!", name);
            //let _ = ws_stream.send(message.into()).await;
        }));

        (WebSocketConnection{tx: tx}, channel)
    }

    fn send(&self, job_info: FlavorMapping) {
        let _ = self.tx.send(job_info);
    }
}


#[get("/websocket")]
fn websocket(ws: WebSocket, client_emulator: &State<Mutex<Option<WebSocketConnection>>>) -> Channel<'static> {
    let mut client_emulator = client_emulator.lock().unwrap();
    let (new_client_emulator, channel) = WebSocketConnection::new(ws);
    *client_emulator = Some(new_client_emulator);

    channel
}


/* #[post("/start", format = "json", data = "<job_info>")]
async fn start_job(job_info: Json<FlavorMapping>, connections: &State<Connections>) {
    let job_info_str = serde_json::to_string(&*job_info).unwrap();
    connections.ws_set.broadcast_text(&job_info_str).await;
} */

/**
 * Function to calculate the SHA-256 hash of a file.
 *
 * # Arguments
 * * `path` - A Path reference to the file.
 *
 * # Returns
 * A String of the file's SHA-256 hash in hexadecimal.
 */
fn hash_file(path: &Path) -> String {
    let mut hasher = Sha256::new();
    let data = fs::read(path).unwrap();
    hasher.update(&data);
    let result = hasher.finalize();
    format!("{:x}", result)
}

#[launch]
fn rocket() -> _ {
    
    let hosts = Vec::<Host>::new();
    let hosts_shared = Mutex::new(hosts);

    let connections: Mutex<Option<WebSocketConnection>> = Mutex::new(None);

    let flavor_map = Vec::<Vec::<String>>::new();
    let flavor_map_shared = Mutex::new(flavor_map);
    rocket::build()
        .mount("/", routes![index, list_files, files, register_host, get_hosts, get_host_flavors, websocket, start_job])
        .manage(hosts_shared)
        .manage(flavor_map_shared)
        .manage(connections)
}
