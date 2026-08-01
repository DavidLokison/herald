mod common;

use std::assert_matches;
use rocket::serde::json::{from_str, Value};
use serde::Deserialize;
use crate::common::TestSuite;

#[allow(unused)]
#[derive(Deserialize)]
struct Error {
    error: String,
    message: String,
}

#[test]
fn success_endpoint_has_data() {
    let suite = TestSuite::spawn();
    let response = reqwest::blocking::get(suite.path("/events/open"))
        .expect("Rocket should be responsive by now");
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert_eq!(response.headers().get(reqwest::header::CONTENT_TYPE).unwrap(), "application/json");
    let response_text = response.text();
    assert_matches!(response_text, Ok(_));
    let response_text = response_text.unwrap();
    let response_payload: Result<Value, _> = from_str(&response_text);
    assert_matches!(response_payload, Ok(_));
    let response_payload = response_payload.unwrap();
    assert!(response_payload.is_object());
    assert!(response_payload.as_object().unwrap().contains_key("data"));
}

#[test]
fn missing_endpoint_returns_404() {
    let suite = TestSuite::spawn();
    let response = reqwest::blocking::get(suite.path("/evnets/oops/typo")).unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
    assert_eq!(response.headers().get(reqwest::header::CONTENT_TYPE).unwrap(), "application/json");
    let text = response.text().unwrap();
    let error: Error = from_str(&text).unwrap();
    assert_eq!(error.error, "ENDPOINT_NOT_FOUND");
}
