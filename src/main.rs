#[macro_use] extern crate rocket;

mod routes;

#[launch]
fn rocket() ->  _ {
    let routes = routes::routes();
    rocket::build().mount("/", routes)
}