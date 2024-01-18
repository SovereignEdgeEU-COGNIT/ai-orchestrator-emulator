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
        Command::new("./i_hate_my_life.sh").arg(flags).spawn();
    }

    /* let output_parsed = match str::from_utf8(&output.stdout) {
        Ok(v) => v.to_owned(),
        Err(e) => format!("Invalid UTF-8 sequence: {}", e),
    };

    Json(output_parsed) */
    Json("".to_string())
}



/* 
fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![index])
        .mount("/todo", routes![new, toggle, delete])
        .attach(Template::fairing())
} */

pub async fn serve(host_config: Config) -> Result<(), Box<dyn std::error::Error>> {
    rocket::custom(host_config)
        .mount("/", routes![index, run_dummy_app])
        .ignite().await?
        .launch().await?;

    Ok(())
}

