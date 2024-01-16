use tokio::{time};
use futures::future;
use serde::{Serialize, Deserialize};
use std::{sync::{Mutex, Arc}, time::Duration, io::Write, future::Future, fs, path::Path};

use reqwest;

const HOSTS_URL: &str = "http://192.168.1.156:8000/hosts";
const FILES_URL: &str = "http://192.168.1.156:8000/files";
pub const FOLDER_NAME: &str = "./local_cache";
use crate::file_cache::{Cache, FileInfo};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Host {
    ip: String,
    port: u16
}

impl Host {
    pub fn get_ip(&self) -> &String {&self.ip}
    pub fn get_port(&self) -> u16 {self.port}
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
    let response = reqwest::get(HOSTS_URL).await?;
    let hosts_new = response.json::<Vec<Host>>().await?;
    let mut hosts_curr = hosts_shared.lock().unwrap();

    if hosts_curr.len() != hosts_new.len() {
        *hosts_curr = hosts_new;

        hosts_curr.iter().for_each(|host| println!("{:?}", host))
    }

    Ok(())
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
    let response = reqwest::get(FILES_URL).await?;
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
    let url = format!("{}/{}", FILES_URL, filename);
    println!("{:?}", file_info);

    let file_data = reqwest::get(&url).await?.bytes().await?;
    let mut file = fs::File::create(Path::new(FOLDER_NAME).join(filename))?;
    file.write_all(&file_data)?;

    cache.process_file(file_info);
    Ok(())
}
