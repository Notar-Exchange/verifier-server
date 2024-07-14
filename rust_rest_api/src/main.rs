#[macro_use] extern crate rocket;

extern crate tlsn_verifier;
use tlsn_verifier::*;

use serde_json::Value;
use std::{str};

mod verify;
mod request_opt; 

#[get("/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[post("/", data = "<proof>")]
fn verify_proof(proof: String) -> String {
    format!("Data: {}", proof)
}

#[get("/")]
async fn test() -> String {
    let pem = str::from_utf8(include_bytes!("../notary.pem")).unwrap();
    let proof = str::from_utf8(include_bytes!(
        "../proof.json"
    ))
    .unwrap();

    let result = verify::verify(proof, pem).await.expect("result");

    format!("I am the tester man {}", result)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![hello]);
    rocket::build().mount("/test", routes![test]);
    rocket::build().mount("/verify", routes![verify_proof])
}