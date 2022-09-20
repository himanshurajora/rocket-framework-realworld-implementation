use rocket::response::Redirect;

#[macro_use]
extern crate rocket;

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
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello_world, go_to_google])
        .mount("/api", routes![auth_me])
}
