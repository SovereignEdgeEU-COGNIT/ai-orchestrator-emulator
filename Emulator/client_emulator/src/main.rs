mod host_client;

use std::{sync::{Mutex, Arc}, error::Error};
use tokio::time;
use std::{time::Duration};
use ctrl_plane::file_client;
use ctrl_plane::faas_client::{self, JobDescription};
use crate::host_client::emulate_client;

pub const REPEAT_TIMES: i32 = 50000;

#[tokio::main]
async fn main() {
    
    let mut file_cache = file_client::FileClient::new();

    let cpu_job = JobDescription{name: "cpu".to_string(), repeat_rate: 40, repeat_times: 50000, args: " --all=1 --class=cpu  -t 20s".to_string()};
    let disk_job = JobDescription{name: "io".to_string(), repeat_rate: 40, repeat_times: 50000, args: " --all=1 --class=io -t 20s".to_string()};
    let mem_job = JobDescription{name: "mem".to_string(), repeat_rate: 40, repeat_times: 50000, args: " --all=1 --class=memory -t 20s".to_string()};
    let net_job = JobDescription{name: "net".to_string(), repeat_rate: 40, repeat_times: 50000, args: " --all=1 --class=network -t 20s".to_string()};
    
    let job_descriptions = vec![cpu_job, disk_job, mem_job, net_job];
    
    //let hosts = Arc::new(Mutex::new(Vec::new()));

    //let host_subscriber = ctrl_plane_client::subscribe_hosts(Arc::clone(&hosts));
    //let _ = tokio::spawn(host_subscriber);

    /* let file_subscriber = ctrl_plane_client::subscribe_files(file_cache);
    let _ = tokio::spawn(file_subscriber); */

    //host_client::emulate_client(Arc::clone(&hosts)).await;
    //time::sleep(Duration::from_secs(1)).await;
    //host_client::test_new(hosts);
    //ctrl_plane_client::subscribe_jobs().await;

    //faas_client::subscribe_jobs with job_descriptions and host_client::emulate_client() function 
    faas_client::subscribe_jobs(emulate_client).await;

    //time::sleep(Duration::from_secs(5)).await;
}    

