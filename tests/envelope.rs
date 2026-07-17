mod common;

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
    let response = reqwest::blocking::get(suite.path("/events/open")).unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert_eq!(response.headers().get(reqwest::header::CONTENT_TYPE).unwrap(), "application/json");
    let text = response.text().unwrap();
    let payload: Value = from_str(&text).unwrap();
    assert!(payload.is_object());
    assert!(payload.as_object().unwrap().contains_key("data"));
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
