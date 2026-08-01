mod common;

use std::assert_matches;
use rocket::serde::json::{Value, from_str};
use uuid::Uuid;
use crate::common::TestSuite;

#[test]
fn get_open_events_succeeds() {
    let suite = TestSuite::spawn();
    suite.sql_server.create_open_event();
    let response = reqwest::blocking::get(suite.path("/events/open")).unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let response_text = response.text().unwrap();
    println!("{}", &response_text);
    let response_payload: Value = from_str(&response_text).unwrap();
    let response_data = response_payload.get("data").unwrap();
    assert!(response_data.is_array());
    let response_data = response_data.as_array().unwrap();
    assert_eq!(response_data.len(), 1);
    let event_data = &response_data[0];
    assert!(event_data.is_object());
    let event_data = event_data.as_object().unwrap();
    assert!(event_data.contains_key("id"));
    let event_id = event_data.get("id").unwrap();
    assert!(event_id.is_string());
    let event_id = event_id.as_str().unwrap();
    assert_matches!(Uuid::try_parse(event_id), Ok(_));
}

#[test]
fn get_open_events_empty_array() {
    let suite = TestSuite::spawn();
    let response = reqwest::blocking::get(suite.path("/events/open")).unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let response_text = response.text().unwrap();
    println!("{}", &response_text);
    let response_payload: Value = from_str(&response_text).unwrap();
    let response_data = response_payload.get("data").unwrap();
    assert!(response_data.is_array());
    let response_data = response_data.as_array().unwrap();
    assert_eq!(response_data.len(), 0);
}

#[test]
fn registration_preview_succeeds() {
    let suite = TestSuite::spawn();
    suite.sql_server.create_open_event();
    let response = reqwest::blocking::get(suite.path("/events/open")).unwrap();
    let response_text = response.text().unwrap();
    println!("{}", &response_text);
    let response_payload: Value = from_str(&response_text).unwrap();
    let event_id = response_payload.get("data").unwrap()
        .as_array().unwrap()[0]
        .as_object().unwrap()
        .get("id").unwrap()
        .as_str().unwrap();
    let preview_query = format!("/events/{}/registrations/preview?[].birthday=1974-05-19&[].birthday=1975-08-31", event_id);
    let response = reqwest::blocking::get(suite.path(&preview_query)).unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let response_text = response.text().unwrap();
    println!("{}", &response_text);
    let response_payload: Value = from_str(&response_text).unwrap();
    let response_data = response_payload.get("data").unwrap();
    assert!(response_data.is_array());
    let response_data = response_data.as_array().unwrap();
    assert_eq!(response_data.len(), 2);
    for article_data in response_data {
        assert!(article_data.is_object());
        assert!(article_data.as_object().unwrap().contains_key("price"));
    }
}
