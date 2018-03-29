#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

extern crate serde;
extern crate serde_json;

use rocket_contrib::{Json};

// X-GitHub-Event

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

struct Event(String);

impl<'a, 'r> FromRequest<'a, 'r> for Event {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Event, ()> {
        let keys: Vec<_> = request.headers().get("X-GitHub-Event").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let key = keys[0];

        return Outcome::Success(Event(key.to_string()));
    }
}

#[post("/github", format = "application/json", data = "<message>")]
fn hello(event: Event, message: Json<serde_json::Value>) -> String {
    println!("EVENT: {}", event.0);
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