#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate error_chain;

extern crate serde;
extern crate serde_json;

use rocket_contrib::{Json};

// X-GitHub-Event

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

struct Event(String);

mod events;
mod errors;
use errors::*;

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
fn hello(event: Event, message: Json<serde_json::Value>) -> Result<String> {
    match event.0.as_ref() {
        "issue_comment" => {
            let y: serde_json::Value = (*message).clone();
            let x = serde_json::from_value::<events::github::IssueComment>(y).chain_err(|| "Failed to parse issue_comment")?;
            println!("{:#?}", x)
        }
        "push" => {
            let y: serde_json::Value = (*message).clone();
            let x = serde_json::from_value::<events::github::Push>(y).chain_err(|| "Failed to parse push")?;
            println!("{:#?}", x)
        }
        _ => {}
    };

    Ok("butts".into())
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
// * on push: sync to gitlab
// * on issue_comment: check for `glhf run <job>`