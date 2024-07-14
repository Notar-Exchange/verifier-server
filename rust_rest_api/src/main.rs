#[macro_use] extern crate rocket;

use rocket::data::{Limits, ToByteUnit};
use rocket::figment::Figment;

use serde::Deserialize;

extern crate tlsn_verifier;
use tlsn_verifier::*;

use serde_json::Value;
use std::{str};

mod verify;
mod request_opt; 

use rocket::form::Form;

#[derive(FromForm)]
struct VerifyProofFormData {
    proof: String,
}

#[post("/", data = "<form_data>")]
async fn verify_proof(form_data: Form<VerifyProofFormData>) -> String {
    let data = form_data.into_inner();
    let pem = str::from_utf8(include_bytes!("../notary.pem")).unwrap();
    let proof = data.proof.as_str();
    let result = verify::verify(proof, pem).await.expect("result");
    format!("{}", result)
}

#[get("/")]
async fn test() -> String {
    let pem = str::from_utf8(include_bytes!("../notary.pem")).unwrap();
    let proof = str::from_utf8(include_bytes!(
        "../proof-good-example.json"
    ))
    .unwrap();

    let result = verify::verify(proof, pem).await.expect("result");

    format!("I am the tester man {}", result)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/test", routes![test]);
    rocket::build().mount("/verify", routes![verify_proof])
}