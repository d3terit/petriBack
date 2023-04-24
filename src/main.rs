#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

mod routes;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() ->  _ {
    let routes = routes::routes();
    rocket::build().attach(CORS).mount("/", routes)
}