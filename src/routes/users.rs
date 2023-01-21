use hyper::{Client, Request, Uri};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    name: String,
    age: i32,
}

#[get("/users")]
pub fn get_all_users() -> Json<Vec<User>> {
    let mut users = vec![];

    // github users api url
    let url = "https://api.github.com/users";

    let user = User {
        name: String::from("Himanshu Jangid"),
        age: 20,
    };

    let mut i = 0;
    loop {
        if i > 9 {
            break;
        }
        users.push(user.clone());
        i += 1;
    }

    return Json(users);
}
