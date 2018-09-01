#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rand;
extern crate reqwest;
extern crate xml;

#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

mod bgg;

use rand::Rng;
use std::path::{Path,PathBuf};
use rocket::response::NamedFile;
use rocket_contrib::{Json};

// /api resolves to the API

#[get("/")]
fn api() -> Json<bgg::BoardGame> {
    let random_num = rand::thread_rng().gen_range(1, bgg::BGG_NUM_GAMES);
    Json(bgg::get_game(random_num))
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
