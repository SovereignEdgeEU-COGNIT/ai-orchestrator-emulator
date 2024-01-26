use tokio_tungstenite::tungstenite::connect;

use crate::{common::ctrl_plane_url, registry_client::{HostInfo, SRInfo}};


#[derive(Clone)]
pub struct JobDescription {
    pub name: String,
    pub repeat_rate: u64,
    pub args: String,
    pub repeat_times: u32
}

pub struct Job {
    pub host: SRInfo,
    pub job_description: JobDescription
}

pub async fn subscribe_jobs(job_types: Vec<JobDescription>, callback: fn(Job)) {
    // Replace with your server URL

   let url = format!("ws://{}/websocket", ctrl_plane_url());
   let (mut ws_stream, _) = connect(url).expect("Failed to connect");
   println!("WebSocket client connected");

   //let (_, read) = ws_stream.split();
   loop {
       let message = ws_stream.read_message().expect("Error receiving the message");
       //let message = message;
       println!("Recieved: {}", message);
       let sr_info = serde_json::from_str::<SRInfo>(&message.to_string()).expect("Incorrect parsing of FlavorMapping");

       match job_types.iter().find(|job_desc| job_desc.name == *sr_info.get_flavor()) {
            Some(job_desc) => {
                let job = Job{host: sr_info.clone(), job_description: job_desc.clone()};
                callback(job);
            },
            None => {
                println!("No job found for flavor: {}", sr_info.get_flavor());
            }
       }
       //"asd".to_string().
   
   }
}
