use reqwest;
//use rocket::futures::TryFutureExt;
//use rocket::serde::json::Json;
use rocket::Config;
use local_ip_address::local_ip;
use local_ip_address::list_afinet_netifas;
use rocket::serde::{json::Json, Serialize};
use tokio::time;
use std::{fs, io::Write, path::Path, time::Duration};
use gethostname::gethostname;
use std::env;

use crate::file_cache::{Cache, FileInfo};

//const FILES_URL: &str = "http://192.168.1.86:8000/files";
//const SERVER_URL: &str = "http://192.168.1.86:8000";
//const FILES_URL: &str = "http://ctrl_plane:8000/files";
//const SERVER_URL: &str = "http://ctrl_plane:8000";
pub const FOLDER_NAME: &str = "./local_cache";


fn ctrl_plane_url() -> String {
    let host_addr = env::var("CTRL_PLANE_ADDR").expect("Missing CTRL_PLANE_ADDR env variable");
    let host_port = env::var("CTRL_PLANE_PORT").expect("Missing CTRL_PLANE_PORT env variable");
    format!("{}:{}", host_addr, host_port)
}


pub async fn subscribe(mut cache: Cache) {

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

#[derive(Debug, Serialize, Clone)]
struct Host {
    ip: String,
    name: String,
    port: u16
}

pub async fn register_host() -> Result<Config, Box<dyn std::error::Error>> {
    /*let network_interfaces = list_afinet_netifas();

    if let Ok(network_interfaces) = network_interfaces {
        for (name, ip) in network_interfaces.iter() {
            println!("{}:\t{:?}", name, ip);
        }
    } else {
        println!("Error getting network interfaces: {:?}", network_interfaces);
    }*/

    let my_local_ip = local_ip().unwrap();
    //println!("Hostname: {:?}", gethostname());
    let hostname = gethostname().to_str().unwrap().to_string();
    let host = Host{ip: my_local_ip.to_string(), name: hostname, port: 0};

    let client = reqwest::Client::new();
    let url = format!("http://{}/register", ctrl_plane_url());
    let res = client.post(url)
    .json(&host)
    .send()
    .await?;

    println!("{}", res.status().as_str());

    let port = res.json::<u16>().await?;

    let config = Config {
        address: my_local_ip,
        port: port,
        ..Config::default()
    };
    
    println!("{}", port);
    Ok(config)
}
