#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;
use rocket::serde::{json::Json, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use sha2::{Sha256, Digest};


#[get("/")]
fn index() -> &'static str {
    "Hello, this is the server!"
}


#[get("/run/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("files").join(file)).await.ok()
}


fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![index])
        .mount("/todo", routes![new, toggle, delete])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}


// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .mount("/", routes![index, list_files, files])
// }

