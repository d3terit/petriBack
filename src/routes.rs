use rocket::Route;

use std::fs::File;
use rocket::response::content::RawJson;
use std::io::Read;
use rocket::{get, routes};


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/get")]
fn get() -> RawJson<String> {
    let mut file = File::open(format!("static/models/example.json")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap(); 
    RawJson(contents)
}

pub fn routes() -> Vec<Route> {
    routes![
        index,
        get
    ]
}