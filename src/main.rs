#![feature(proc_macro_hygiene, decl_macro)]
#![feature(custom_attribute)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;

use rocket::http::{Cookie, Cookies};
use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize)]
struct RegisterPar<'a> {
    email: &'a str,
    password: &'a str,
}

#[post("/register", data = "<data>")]
fn register(data: Json<RegisterPar>) -> String {
    "hey ther".to_string()
}

#[get("/")]
fn index(mut cookies: Cookies) -> String {
    let cookie = Cookie::build("session_token", "123180i9epajkldskj")
        .domain("www.rust-lang.org")
        .path("/")
        .secure(false)
        .http_only(true)
        .finish();

    cookies.add(cookie);
    "Here is a cookie for you!".to_string()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![register, index])
        .launch();
}
