use std::process::Command;

use crate::file_cache::{FileInfo, FileHandler};

const TC_FILE_NAME: &str = "tc_data.txt";

pub fn new_tc_handler() -> FileHandler {
    FileHandler::new(update_tc, is_tc_file)
}

fn update_tc(file_info: &FileInfo) {

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };

    let hello = output.stdout;
    println!("{:?}", hello);
}

fn is_tc_file(file_info: &FileInfo) -> bool {
    file_info.get_filename().eq(TC_FILE_NAME)
}