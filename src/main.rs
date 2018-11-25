#![feature(proc_macro_hygiene, decl_macro)]
#![feature(custom_attribute)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize)]
struct RegisterPar<'a> {
    email: &'a str,
    password: &'a str,
}

#[post("/register", data = "<data>")]
fn register(data: Json<RegisterPar>) -> String {}

fn main() {
    rocket::ignite().mount("/", routes![register]).launch();
}
