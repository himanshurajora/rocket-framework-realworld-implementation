#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::response::Redirect;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: i32,
}

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/hello")]
fn hello_world() -> &'static str {
    "Hello World, By me"
}

#[get("/me/<name>")]
fn auth_me(name: String) -> String {
    if name == "Himanshu" {
        return String::from("Hello") + &name;
    }
    return String::from("You Are Not Authorized");
}

#[get("/google")]
fn go_to_google() -> Redirect {
    Redirect::to("https://google.com")
}

#[get("/json")]
fn json_response() -> Json<User> {
    let user = User {
        name: String::from("Himanshu"),
        age: 20,
    };
    return Json(user);
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![index, json_response, go_to_google, hello_world, auth_me],
        )
        .launch();
}
