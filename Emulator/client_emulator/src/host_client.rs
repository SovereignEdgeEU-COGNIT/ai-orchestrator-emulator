use tokio::time;
use futures::future;
use std::{sync::{Mutex, Arc}, time::Duration};
use std::io;
use std::process::{Command, Child};

//use crate::ctrl_plane_client::Host;
use ctrl_plane::faas_client::{Job, JobDescription};

const APP_PATH: &str = "/run_dummy";

async fn exec_app(host_url: String, cmd_args: &String) -> Result<(), Box<dyn std::error::Error>>  {

    let client = reqwest::Client::new();
    let res = client.post(host_url)
    .json(cmd_args)
    .send()
    .await?;

    println!("{:?}", res.json::<String>().await?);

    Ok(())
}


pub fn emulate_client(job_info: Job) {

    tokio::spawn(async move {
        for i in 0..job_info.job_description.repeat_times {
            send_client_requests(&job_info).await;
    
            time::sleep(Duration::from_secs(job_info.job_description.repeat_rate)).await;
        }
    });
    
}

async fn send_client_requests(job_info: &Job) {
    //let mut set = tokio::task::JoinSet::new();

    //let cmd_args = Arc::new(vec!["/C", "echo hello"]);
    let host = &job_info.host;

    // Extract the host url and hold the lock minimal time
    let host_url = format!("http://{}:{}{}", host.get_ip(), host.get_port(), APP_PATH);

    exec_app(host_url, &job_info.job_description.args).await;

    println!("{:?}", job_info)
    /* let mut futures = vec![];

    for host_url in host_urls {
        let shared_vec_clone = Arc::clone(&cmd_args);
        
        let future = tokio::spawn(async move {
            exec_app(host_url, shared_vec_clone).await;
        });
        futures.push(future);
    } */

    //let _results = future::join_all(futures).await;

    
}
/* 
pub fn test_new(hosts_shared: Arc<Mutex<Vec<Host>>>) -> io::Result<()> {
    let classid = 0x10001;
    create_cgroup("process1", classid)?;
    let cmd_args = Arc::new(vec!["/C", "echo hello"]);

    let hosts = hosts_shared.lock().unwrap();
    hosts.iter().for_each(|host| {
        let host_url = format!("http://{}:{}{}", host.get_ip(), host.get_port(), APP_PATH);

        launch_dummy_caller_process_in_cgroup(host_url, classid, Arc::clone(&cmd_args), "process1".to_string());
    });

    Ok(())
} */

fn create_cgroup(name: &str, classid: u32) -> io::Result<()> {
    let mkdir_result = Command::new("mkdir")
        .arg("-p")
        .arg(format!("/sys/fs/cgroup/net_cls/{}", name))
        .status()?;

    if !mkdir_result.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to create cgroup directory"));
    }

    let echo_result = Command::new("bash")
        .arg("-c")
        .arg(format!("echo {:x} > /sys/fs/cgroup/net_cls/{}/net_cls.classid", classid, name))
        .status()?;

    if !echo_result.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to write to net_cls.classid"));
    }

    Ok(())
}


fn launch_dummy_caller_process_in_cgroup(host_url: String, classid: u32, cmd_args: Arc<Vec<&str>>, cgroup_name: String) -> io::Result<Child> {
    let cmd_args_json = serde_json::to_string(&*cmd_args).expect("Failed to serialize cmd_args");

    // Convert classid to a string (hexadecimal format)
    let classid_str = format!("{:x}", classid);

    let mut command = Command::new("./wrapper_dummy_caller_script.sh");
        command.arg(&cgroup_name);
        command.arg(&classid_str);
        command.arg(host_url);
        command.arg(cmd_args_json);
        //.spawn()
        //.expect("Failed to execute script");

    let child = command.spawn()?;

    // Remember to kill the process
    println!("{}", child.id());

    Ok(child)
}

fn launch_arbitrary_process_in_cgroup(executable: &str, cgroup_name: &str, args: &[&str]) -> io::Result<Child> {
    let mut command = Command::new("./wrapper_arbitrary_script.sh");
    command.arg(cgroup_name).arg(executable);
    command.args(args);

    let child = command.spawn()?;

    Ok(child)
}


/* fn main() -> io::Result<()> {
    // Create cgroups
    create_cgroup("process1", 0x10001)?;
    create_cgroup("process2", 0x10002)?;

    // Launch processes in cgroups
    launch_process_in_cgroup("mkdir", "process1",  &["/tmp/sibo2"])?;
    //launch_process_in_cgroup("/path/to/process2_executable", "process2")?;

    Ok(())
} */
