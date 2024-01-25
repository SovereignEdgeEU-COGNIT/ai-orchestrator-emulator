use std::sync::Mutex;

use serde::{Serialize, Deserialize};

use crate::registry_client::{HostInfo, NodeInfo, SRInfo};

/* 
pub struct HostAgentClient {
    hosts: Arc<Mutex<Vec<HostInfo>>>,
}
 */
#[derive(Serialize, Deserialize, Debug)]
struct StartContainerRequest {
    cpu: f32,
    memory: i32,
}

fn host_agent_url(host: HostInfo) -> String {
    format!("http://{}:{}/start-container", host.get_ip(), host.get_port())
}

pub async fn start_sr(host: HostInfo, cpu: f32, memory: i32) -> Result<String, Box<dyn std::error::Error>> {
    let url = host_agent_url(host);
    let client = reqwest::Client::new();

    let container_req = StartContainerRequest {
        cpu: cpu,
        memory: memory,
    };
    let res = client.post(url)
    .json(&container_req)
    .send()
    .await?;

    let res_val = res.json::<String>().await?;

    //println!("{}", res.status().as_str());

    Ok(res_val)
}