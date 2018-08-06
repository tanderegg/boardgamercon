#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::path::{Path,PathBuf};
use rocket::response::NamedFile;


// /api resolves to the API

#[get("/")]
fn api() -> &'static str {
    "Hello, from the API!"
}

// /assets resolves to the assets folder

#[get("/<path..>")]
fn assets(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/assets").join(path)).ok()
}

// Most paths resolve to the SPA at index.html

#[get("/")]
fn spa_empty() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).ok()
}

#[get("/<_path..>", rank = 2)]
fn spa_path(_path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![spa_path, spa_empty])
        .mount("/api", routes![api])
        .mount("/assets", routes![assets])
        .launch();
}
