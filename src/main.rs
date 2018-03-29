#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    rocket::ignite().mount("/hello", routes![hello]).launch();
}


// AJM impl notes:
// * one config file with ???
// * N config files with one file per repo, each containing
//   * github repo url
//   * gitlab repo url
//   * Secret
// * Need a guide with setup steps for github + gitlab