#![feature(proc_macro_hygiene, decl_macro)]

use hello_rocket;

fn main() {
    hello_rocket::rocket().launch();
}
