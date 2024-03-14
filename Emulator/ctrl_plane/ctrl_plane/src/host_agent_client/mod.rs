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
    flavor: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartContainerResponse {
    pub container_name: String,
    pub container_port: u16,
}


fn host_agent_url(host: HostInfo) -> String {
    format!("http://{}:{}/start-container", host.get_ip(), host.get_port())
}

pub async fn start_sr(host: HostInfo, cpu: f32, memory: i32, flavor: &String) -> Result<StartContainerResponse, Box<dyn std::error::Error>> {
    let url = host_agent_url(host);
    let client = reqwest::Client::new();

    let container_req = StartContainerRequest {
        cpu: cpu,
        memory: memory,
        flavor: flavor.clone(),
    };
    let res = client.post(url)
    .json(&container_req)
    .send()
    .await?;

    let res_val = res.json::<StartContainerResponse>().await?;

    //println!("{}", res.status().as_str());

    Ok(res_val)
}