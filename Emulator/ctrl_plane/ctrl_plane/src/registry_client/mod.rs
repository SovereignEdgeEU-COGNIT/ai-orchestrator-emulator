
//use local_ip_address::local_ip;
//use local_ip_address::list_afinet_netifas;
use gethostname::gethostname;
use rocket::{http::uri::Host, serde::json};
use std::env;
use serde::{Serialize, Deserialize};

use crate::common::ctrl_plane_url;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HostInfo {
    ip: String,
    name: String,
    port: u16,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientInfo {
    flavor: String,
    execution_time: u32,
    request_rate: u32,
}

impl ClientInfo {

    fn empty() -> ClientInfo {
        ClientInfo{flavor: "".to_string(), execution_time: 0, request_rate: 0}
    }

    pub fn get_flavor(&self) -> &String {
        &self.flavor
    }

    pub fn get_execution_time(&self) -> u32 {
        self.execution_time
    }

    pub fn get_request_rate(&self) -> u32 {
        self.request_rate
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SRInfo {
    ip: String,
    name: String,
    port: u16,
    client_info: ClientInfo,
    host_info: HostInfo,
}


//interface for both SRInfo and HostInfo
pub trait NodeInfo {
    fn get_ip(&self) -> &String;
    fn get_name(&self) -> &String;
    fn get_port(&self) -> u16;
    async fn register(&self) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NodeType {
    Host(HostInfo),
    SR(SRInfo),
}

impl HostInfo {
    
    pub fn new() -> HostInfo {
        let ip = std::env::var("HOST_IP").unwrap();
        let port_str = std::env::var("HOST_PORT").unwrap();
        let port: u16 = port_str.parse::<u16>().unwrap();
        let name = gethostname().to_str().unwrap().to_string();
        HostInfo{ip, name, port}
    }

    pub fn empty() -> HostInfo {
        HostInfo{ip: "".to_string(), name: "".to_string(), port: 0}
    }
}

impl SRInfo {
        
    pub fn empty() -> SRInfo {
        let ip = std::env::var("HOST_IP").unwrap();
        let port_str = std::env::var("HOST_PORT").unwrap();
        let port: u16 = port_str.parse::<u16>().unwrap();
        let name = gethostname().to_str().unwrap().to_string();
        SRInfo{ip, name, port, client_info: ClientInfo::empty(), host_info: HostInfo::empty()}
    }

    pub fn new(ip: String, port: u16, name: String, client_info: ClientInfo, host_info: HostInfo) -> SRInfo {
        SRInfo{ip, port, name, client_info, host_info}
    }

    pub fn set_client_info(&mut self, client_info: ClientInfo) {
        self.client_info = client_info;
    }

    pub fn set_host_info(&mut self, host_info: HostInfo) {
        self.host_info = host_info;
    }

    pub fn get_client_info(&self) -> &ClientInfo {
        &self.client_info
    }

    pub fn get_ip(&self) -> &String {
        &self.ip
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_host_info(&self) -> &HostInfo {
        &self.host_info
    }
}

impl NodeInfo for HostInfo {
    fn get_ip(&self) -> &String { return &self.ip; }
    fn get_name(&self) -> &String { return &self.name; }
    fn get_port(&self) -> u16 { return self.port; }
    async fn register(&self) -> Result<(), Box<dyn std::error::Error>> {
        register_node(NodeType::Host(self.clone())).await
    }
}

impl NodeInfo for SRInfo {
    fn get_ip(&self) -> &String { return &self.ip; }
    fn get_name(&self) -> &String { return &self.name; }
    fn get_port(&self) -> u16 { return self.port; }
    async fn register(&self) -> Result<(), Box<dyn std::error::Error>> {
        register_node(NodeType::SR(self.clone())).await
    }
}

pub async fn register_node(node: NodeType) -> Result<(), Box<dyn std::error::Error>> {
    /*let network_interfaces = list_afinet_netifas();

    if let Ok(network_interfaces) = network_interfaces {
        for (name, ip) in network_interfaces.iter() {
            println!("{}:\t{:?}", name, ip);
        }
    } else {
        println!("Error getting network interfaces: {:?}", network_interfaces);
    }*/

    let client = reqwest::Client::new();
    let url = format!("http://{}/register", ctrl_plane_url());
    println!("Registering node at {}", url);
    println!("Node: {:?}", json::to_string(&node).unwrap());
    let res = client.post(url)
    .json(&node)
    .send()
    .await?;

    println!("{}", res.status().as_str());

    Ok(())
}
