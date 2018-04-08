#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate error_chain;

extern crate serde;
extern crate serde_json;

#[macro_use] extern crate log;
extern crate env_logger;

use rocket_contrib::{Json};

// X-GitHub-Event

use rocket::{Outcome, State};
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use events::github::GitHubEvent;
use std::thread;

struct GitHubEventJson(String);

mod events;
mod errors;
use errors::*;

impl<'a, 'r> FromRequest<'a, 'r> for GitHubEventJson {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<GitHubEventJson, ()> {
        let keys: Vec<_> = request.headers().get("X-GitHub-Event").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let key = keys[0];

        return Outcome::Success(GitHubEventJson(key.to_string()));
    }
}

type EvtTxer = Arc<Mutex<Sender<GitHubEvent>>>;

// TODO: Don't parse to Value first, instead do something that tries to
//   automatically figure out which type it is in maybe tagged enum?
#[post("/github", format = "application/json", data = "<message>")]
fn hello(event: GitHubEventJson, message: Json<serde_json::Value>, evt_tx: State<EvtTxer>) -> Result<String> {
    trace!("{:?}", message.0);
    match event.0.as_ref() {
        "push" => {
            use events::github::push::Push;
            let y: serde_json::Value = (*message).clone();
            let x = serde_json::from_value::<Push>(y).chain_err(|| "Failed to parse push")?;
            debug!("Recieved event: {:?}", x);
            evt_tx
                .lock()
                .map_err(|_e| Error::from("Failed to lock mutex"))?
                .send(GitHubEvent::Push(x))
                .chain_err(|| "Failed to push event internally")?;
        }
        _ => {}
    };

    Ok("thanks!".into())
}

fn main() {
    env_logger::init();

    let (es_tx, es_rx): (Sender<GitHubEvent>, Receiver<GitHubEvent>) = channel();


    let web = thread::spawn(move || {
        rocket::ignite()
            .mount("/", routes![hello])
            .manage(Arc::new(Mutex::new(es_tx)))
            .launch();
    });

    while let Ok(evt) = es_rx.recv() {
        match evt {
            GitHubEvent::Push(p) => {
                info!("Received Push Event for {}:{}", p.repository.name, p.push_ref)
            }
        }
    }

    let _ = web.join();
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