use tokio::{time};
use futures::{future};
use futures::stream::StreamExt;
use serde::{Serialize, Deserialize};
use tokio_tungstenite::tungstenite::connect;
use std::{sync::{Mutex, Arc}, time::Duration, io::Write, future::Future, fs, path::Path};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use std::env;

use crate::host_client;


use reqwest;

//const HOSTS_URL: &str = "http://ctrl_plane:8000/hosts";
//const FILES_URL: &str = "http://ctrl_plane:8000/files";
//const WS_URL: &str = "ws://ctrl_plane:8000/websocket";
// const HOSTS_URL: &str = "http://192.168.1.86:8000/hosts";
// const FILES_URL: &str = "http://192.168.1.86:8000/files";
// const WS_URL: &str = "ws://192.168.1.86:8000/websocket";
pub const FOLDER_NAME: &str = "./local_cache";
use crate::file_cache::{Cache, FileInfo};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Host {
    ip: String,
    name: String,
    port: u16
}

impl Host {
    pub fn get_ip(&self) -> &String {&self.ip}
    pub fn get_port(&self) -> u16 {self.port}
}

fn ctrl_plane_url() -> String {
    let host_addr = env::var("CTRL_PLANE_ADDR").expect("Missing CTRL_PLANE_ADDR env variable");
    let host_port = env::var("CTRL_PLANE_PORT").expect("Missing CTRL_PLANE_PORT env variable");
    format!("{}:{}", host_addr, host_port)
}

pub async fn subscribe_hosts(hosts_shared: Arc<Mutex<Vec<Host>>>) {
    loop {

        if let Err(err) = get_hosts(&hosts_shared).await {
            println!("{:?}", err)
        }
        
        time::sleep(Duration::from_secs(5)).await;
    }
}

async fn get_hosts(hosts_shared: &Mutex<Vec<Host>>) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://{}/hosts", ctrl_plane_url());
    let response = reqwest::get(url).await?;
    let hosts_new = response.json::<Vec<Host>>().await?;
    let mut hosts_curr = hosts_shared.lock().unwrap();

    if hosts_curr.len() != hosts_new.len() {
        *hosts_curr = hosts_new;

        hosts_curr.iter().for_each(|host| println!("{:?}", host))
    }

    Ok(())
}

#[derive(Clone)]
pub struct JobDescription {
    pub name: String,
    pub repeat_rate: u64,
    pub args: String,
}

pub struct Job {
    pub host: Host,
    pub job_description: JobDescription
}

#[derive(Serialize, Deserialize)]
struct FlavorMapping {
    host: Host,
    flavors: Vec<String>
}

pub async fn subscribe_jobs() {
     // Replace with your server URL

    let url = format!("ws://{}/websocket", ctrl_plane_url());
    let (mut ws_stream, _) = connect(url).expect("Failed to connect");
    println!("WebSocket client connected");

    let cpu_job = JobDescription{name: "cpu".to_string(), repeat_rate: 25, args: " --class cpu --all 1 -t 5s".to_string()};
    let disk_job = JobDescription{name: "disk".to_string(), repeat_rate: 25, args: " --class io --all 1 -t 5s".to_string()};

    let job_descriptions = vec![cpu_job, disk_job];

    //let (_, read) = ws_stream.split();
    loop {
        let message = ws_stream.read_message().expect("Error receiving the message");
        //let message = message;
        println!("Recieved: {}", message);
        let job_info = serde_json::from_str::<FlavorMapping>(&message.to_string()).expect("Incorrect parsing of FlavorMapping");

        let job_desc = job_descriptions.iter().find(|job_desc| job_desc.name == *job_info.flavors.first().unwrap()).expect("Invalid job sent");
        let job = Job{host: job_info.host.clone(), job_description: job_desc.clone()};
        host_client::emulate_client(job);
        //"asd".to_string().
    
    }
}

pub async fn subscribe_files(mut cache: Cache) {

    loop {

        if let Err(err) = check_files(&mut cache).await {
            println!("{:?}", err)
        }
        
        time::sleep(Duration::from_secs(5)).await;
    }
}

async fn check_files(cache: &mut Cache) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://{}/files", ctrl_plane_url());
    let response = reqwest::get(url).await?;
    let files = response.json::<Vec<FileInfo>>().await?;

    for file_info in files {
        
        if cache.in_cache(&file_info) {
            continue;
        }

        if let Err(e) = download_file(cache, file_info).await {
            println!("{:?}", e)
        }
    }
    Ok(())
}

async fn download_file(cache: &mut Cache, file_info: FileInfo ) -> Result<(), Box<dyn std::error::Error>> {
    let filename = file_info.get_filename();
    let url = format!("http://{}/files", ctrl_plane_url());
    let url = format!("{}/{}", url, filename);
    println!("{:?}", file_info);

    let file_data = reqwest::get(&url).await?.bytes().await?;
    let mut file = fs::File::create(Path::new(FOLDER_NAME).join(filename))?;
    file.write_all(&file_data)?;

    cache.process_file(file_info);
    Ok(())
}
