
use serde::{Serialize, Deserialize};
use std::env;


pub fn ctrl_plane_url() -> String {
    let host_addr = env::var("CTRL_PLANE_ADDR").expect("Missing CTRL_PLANE_ADDR env variable");
    let host_port = env::var("CTRL_PLANE_PORT").expect("Missing CTRL_PLANE_PORT env variable");
    format!("{}:{}", host_addr, host_port)
}