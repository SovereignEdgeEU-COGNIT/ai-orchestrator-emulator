use reqwest;
use rocket::serde::json::Json;
use tokio::time;
use std::{fs, io::Write, path::Path, time::Duration};

use crate::file_cache::{Cache, FileInfo};

const FILES_URL: &str = "http://localhost:8000/files";
const SERVER_URL: &str = "http://localhost:8000";
pub const FOLDER_NAME: &str = "./local_cache";

pub async fn subscribe(mut cache: Cache) {

    loop {

        if let Err(err) = check_files(&mut cache).await{
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


pub async fn register_host() -> Result<(), Box<dyn std::error::Error>> {
    let testval:i32 = 5;

    let client = reqwest::Client::new();
    let url = format!("{}/register", SERVER_URL);
    let res = client.post(url)
    .json(&testval)
    .send()
    .await?;

    println!("{}", res.status().as_str());
    Ok(())
}
