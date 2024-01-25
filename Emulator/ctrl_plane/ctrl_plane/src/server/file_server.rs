
use rocket::{fs::NamedFile, Rocket};
use rocket::{get, routes, Build};
use std::path::{Path, PathBuf};
use rocket::serde::json::Json;
use crate::file_client::FileInfo;
use std::fs;
use sha2::{Sha256, Digest};

#[get("/files/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("files").join(file)).await.ok()
}

/**
 * Lists all files in the "files" directory.
 *
 * # Returns
 * A JSON array of FileInfo objects, each containing a filename and its SHA-256 hash.
 */
#[get("/files")]
fn list_files() -> Json<Vec<FileInfo>> {
    let mut files_info = Vec::new();
    let path = Path::new("files");
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let filename = path.file_name().unwrap().to_str().unwrap().to_owned();
                let hash = hash_file(&path);
                files_info.push(FileInfo::new(filename, hash));
            }
        }
    }
    Json(files_info)
}

/**
 * Function to calculate the SHA-256 hash of a file.
 *
 * # Arguments
 * * `path` - A Path reference to the file.
 *
 * # Returns
 * A String of the file's SHA-256 hash in hexadecimal.
 */
fn hash_file(path: &Path) -> String {
    let mut hasher = Sha256::new();
    let data = fs::read(path).unwrap();
    hasher.update(&data);
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn initiate(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/", routes![list_files, files])
}