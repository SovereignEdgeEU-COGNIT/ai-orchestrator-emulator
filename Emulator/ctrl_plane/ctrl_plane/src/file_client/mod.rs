mod file_cache;
pub mod file_listener;

use reqwest;
use tokio::time;
use std::{fs, io::Write, path::Path, time::Duration};
use std::env;
use serde::{Deserialize, Serialize};

use file_cache::Cache;
use file_listener::FileListener;

use crate::common::ctrl_plane_url;

pub const FOLDER_NAME: &str = "./local_cache";


#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Hash)]
pub struct FileInfo {
    filename: String,
    hash: String,
}


impl FileInfo {

    pub fn new(filename: String, hash: String) -> FileInfo {
        FileInfo{filename, hash}
    }

    pub fn get_filename(&self) -> &String {
        return &self.filename;
    }
}

pub struct FileClient {
    cache: Cache
}

impl FileClient {

    pub fn new() -> FileClient {
        let _ = std::fs::create_dir(FOLDER_NAME);
        FileClient{cache: Cache::new()}
    }

    pub fn register_listener(&mut self, handler: FileListener) {
        self.cache.register_listener(handler);
    }

    pub async fn subscribe_files(&mut self) {

        loop {
    
            if let Err(err) = self.check_files().await {
                println!("{:?}", err)
            }
            
            time::sleep(Duration::from_secs(5)).await;
        }
    }    

    async fn check_files(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("http://{}/files", ctrl_plane_url());
        let response = reqwest::get(url).await?;
        let files = response.json::<Vec<FileInfo>>().await?;

        for file_info in files {
            
            if self.cache.in_cache(&file_info) {
                continue;
            }

            if let Err(e) = self.download_file(file_info).await {
                println!("{:?}", e)
            }
        }
        Ok(())
    }

    async fn download_file(&mut self, file_info: FileInfo ) -> Result<(), Box<dyn std::error::Error>> {

        let filename = file_info.get_filename();
        let url = format!("http://{}/files", ctrl_plane_url());
        let url = format!("{}/{}", url, filename);
        println!("{:?}", file_info);

        let file_data = reqwest::get(&url).await?.bytes().await?;
        let mut file = fs::File::create(Path::new(FOLDER_NAME).join(filename))?;
        file.write_all(&file_data)?;

        self.cache.process_file(file_info);
        Ok(())
    }
}


pub fn listen_on_thread(listeners: Vec<FileListener>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut file_client = FileClient::new();
        for listener in listeners {
            file_client.register_listener(listener);
        }
        tokio::runtime::Runtime::new().unwrap().block_on(file_client.subscribe_files());
    })
}