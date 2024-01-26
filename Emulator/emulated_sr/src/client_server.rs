use ctrl_plane::registry_client::SRInfo;
use reqwest::Error;
use rocket::{Config, Rocket, Build};
use rocket::serde::{json::Json, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;


#[get("/")]
fn index() -> &'static str {
    "Hello, this is the server!"
}

#[post("/run_dummy", format = "json", data = "<cmd_args>")]
fn run_dummy_app(cmd_args: Json<String>) -> Json<String> {
    
    let flags = cmd_args.0;
    let error_msg = format!("failed to execute process using {:?}", &flags);
    println!("Got {}, {}, {}", flags, flags.contains("\n"), flags.contains("\r"));
        
    /* let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(vec!["/C", &format!("echo {}", flags.trim())])
            .output()
            .expect(&error_msg) //
    } else {
        Command::new("sh")
            //.args(vec![flags.trim()])
            //.args(vec!["--class cpu","--all 1","-t 20s"])
            .args(vec!["-c 'stress-ng --class cpu --all 1 -t 5s'"])
            .output()
            .expect(&error_msg)
    }; */
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(vec!["/C", &format!("echo {}", flags.trim())])
            .output()
            .expect(&error_msg); //
    } else {
        Command::new("./emulated_sr/start_function.sh").arg(flags).spawn();
    }

    /* let output_parsed = match str::from_utf8(&output.stdout) {
        Ok(v) => v.to_owned(),
        Err(e) => format!("Invalid UTF-8 sequence: {}", e),
    };

    Json(output_parsed) */
    Json("".to_string())
}


pub async fn serve(sr_info: SRInfo) -> Result<(), Box<dyn std::error::Error>> {

    let ipv4 = "0.0.0.0".parse::<std::net::Ipv4Addr>().unwrap();
    let ip = std::net::IpAddr::V4(ipv4);

    let config = Config {
        address: ip,
        port: sr_info.get_port(),
        ..Config::default()
    };
    rocket::custom(config)
        .mount("/", routes![index, run_dummy_app])
        .ignite().await?
        .launch().await?;

    Ok(())
}

