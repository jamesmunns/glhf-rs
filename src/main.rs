#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

extern crate serde;
extern crate serde_json;

use rocket_contrib::{Json};

#[post("/github", format = "application/json", data = "<message>")]
fn hello(message: Json<serde_json::Value>) -> String {
    println!("{:#?}", message);
    "butts".into()
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}


// AJM impl notes:
// * one config file with ???
// * N config files with one file per repo, each containing
//   * github repo url
//   * gitlab repo url
//   * Secret
// * Need a guide with setup steps for github + gitlab