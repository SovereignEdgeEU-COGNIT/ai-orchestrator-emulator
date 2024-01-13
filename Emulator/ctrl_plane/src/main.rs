#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;
use rocket::serde::{json::Json, Serialize};
use rocket::State;
use std::collections::LinkedList;
//use serde::Serialize;
//use serde_json::json;
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::{Arc, Mutex};
use sha2::{Sha256, Digest};

#[derive(Serialize)]
struct FileInfo {
    filename: String,
    hash: String,
}

struct Host {
    ip: String,
    port: u32
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
#[post("/register", format = "json", data = "<node_ip>")]
fn register_host(node_ip: Json<String>, hosts_shared: &State<Mutex<Vec<Host>>>) -> Json<u32> {
    let mut hosts = hosts_shared.lock().unwrap();
    let last_alloc_port = hosts.last().unwrap_or(&Host{ip: String::new(), port:1233}).port;
    let host = Host{ip: node_ip.0, port: last_alloc_port + 1};
    hosts.push(host);
    Json(last_alloc_port + 1)
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
    let hosts_arc = Mutex::new(hosts);
    rocket::build()
        .mount("/", routes![index, list_files, files, register_host])
        .manage(hosts_arc)
}
