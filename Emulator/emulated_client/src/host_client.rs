use tokio::time;
use futures::future;
use std::{sync::{Mutex, Arc}, time::Duration};

use crate::ctrl_plane_client::Host;

const APP_PATH: &str = "/run_dummy";

async fn exec_app(host_url: String, cmd_args: Arc<Vec<&str>>) -> Result<(), Box<dyn std::error::Error>>  {

    let client = reqwest::Client::new();
    let res = client.post(host_url)
    .json(&*cmd_args)
    .send()
    .await?;

    println!("{:?}", res.json::<String>().await?);

    Ok(())
}

pub async fn emulate_client(hosts_shared: Arc<Mutex<Vec<Host>>>) {
    loop {
        send_client_requests(Arc::clone(&hosts_shared)).await;
        
        time::sleep(Duration::from_secs(5)).await;
    }
}

async fn send_client_requests(hosts_shared: Arc<Mutex<Vec<Host>>>) {
    //let mut set = tokio::task::JoinSet::new();

    let cmd_args = Arc::new(vec!["/C", "echo hello"]);

    // Extract the host url and hold the lock minimal time
    let host_urls: Vec<String> = {
        let hosts = hosts_shared.lock().unwrap();
        hosts.iter().map(|host| {
            format!("http://{}:{}{}", host.get_ip(), host.get_port(), APP_PATH)
        }).collect()
    };


    let mut futures = vec![];

    for host_url in host_urls {
        let shared_vec_clone = Arc::clone(&cmd_args);
        
        let future = tokio::spawn(async move {
            exec_app(host_url, shared_vec_clone).await;
        });
        futures.push(future);
    }

    let _results = future::join_all(futures).await;

    
}