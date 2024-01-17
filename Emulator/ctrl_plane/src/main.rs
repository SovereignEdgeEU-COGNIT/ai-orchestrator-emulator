#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;
use rocket::serde::{json::Json, Serialize, Deserialize};
use rocket::State;
use std::iter::Zip;
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::Mutex;
use sha2::{Sha256, Digest};

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

#[derive(Serialize, Deserialize)]
struct FlavorMapping {
    host: Host,
    flavors: Vec<String>
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
fn start_job(job_info: Json<FlavorMapping>, hosts_shared: &State<Mutex<Vec<Host>>>, flavor_map_shared: &State<Mutex<Vec<Vec<String>>>>) {
    let mut hosts = hosts_shared.lock().unwrap();
    let mut flavor_map = flavor_map_shared.lock().unwrap();
    //flavor_map.push(Vec::new());
    hosts.iter_mut()
        .zip(flavor_map.iter_mut())
        .filter(|(host, flavors)| host.name == job_info.host.name)
        .for_each(|(host, flavors)| flavors.append(&mut job_info.flavors.clone()))
}

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

    let flavor_map = Vec::<Vec::<String>>::new();
    let flavor_map_shared = Mutex::new(flavor_map);
    rocket::build()
        .mount("/", routes![index, list_files, files, register_host, get_hosts, get_host_flavors])
        .manage(hosts_shared)
        .manage(flavor_map_shared)
}
