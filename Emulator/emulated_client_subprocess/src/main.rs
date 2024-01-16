use std::fs::{self, File};
use std::io::{self, Write};
use std::process::{Command, Child};

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

/* fn launch_process_in_cgroup(executable: &str, cgroup_name: &str) -> io::Result<()> {
    let child = Command::new("mkdir")
        .arg("/tmp/sibo")
        .spawn()?
        .id();

    let tasks_path = format!("/sys/fs/cgroup/net_cls/{}/tasks", cgroup_name);
    let mut tasks_file = File::open(tasks_path)?;
    writeln!(tasks_file, "{}", child)?;

    Ok(())
} */

fn launch_dummy_caller_process_in_cgroup(host_ip: String, host_port: String, cmd_args: Arc<Vec<&str>>, cgroup_name: String) -> io::Result<Child> {
    let cmd_args_json = serde_json::to_string(&*cmd_args).expect("Failed to serialize cmd_args");

    let mut command = Command::new("./wrapper_dummy_caller_script.sh")
        .arg(&cgroup_name)
        .arg(host_ip)
        .arg(host_port)
        .arg(cmd_args_json);
        //.spawn()
        //.expect("Failed to execute script");

    let child = command.spawn()?;

    Ok(child)
}

fn launch_arbitrary_process_in_cgroup(executable: &str, cgroup_name: &str, args: &[&str]) -> io::Result<Child> {
    let mut command = Command::new("./wrapper_arbitrary_script.sh");
    command.arg(cgroup_name).arg(executable);
    command.args(args);

    let child = command.spawn()?;

    Ok(child)
}

/* fn launch_process_in_cgroup(executable: &str, cgroup_name: &str) -> io::Result<Child> {
    // Spawn the process
    let child = Command::new("mkdir")
        .arg("/tmp/sibo")
        .spawn()?;

    // Get the process ID
    let pid = child.id();

    // Write the process ID to the cgroup's tasks file
    let echo_result = Command::new("bash")
        .arg("-c")
        .arg(format!("echo {} > /sys/fs/cgroup/net_cls/{}/tasks", pid, cgroup_name))
        .status()?;

    if !echo_result.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to write PID to cgroup tasks file"));
    }

    Ok(child)
} */

fn main() -> io::Result<()> {
    // Create cgroups
    create_cgroup("process1", 0x10001)?;
    create_cgroup("process2", 0x10002)?;

    // Launch processes in cgroups
    launch_process_in_cgroup("mkdir", "process1",  &["/tmp/sibo2"])?;
    //launch_process_in_cgroup("/path/to/process2_executable", "process2")?;

    Ok(())
}
