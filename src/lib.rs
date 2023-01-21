#![feature(proc_macro_hygiene, decl_macro)]

use rocket::Rocket;
#[macro_use]
extern crate rocket;

mod routes;

#[get("/")]
fn hello() -> &'static str {
    "Welcome to rocket server"
}

pub fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![hello, routes::users::get_all_users])
}
