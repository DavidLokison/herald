mod common;

use crate::common::TestSuite;

#[test]
fn get_open_events_succeeds() {
    let suite = TestSuite::spawn();
    suite.sql_server.create_open_event();
    let response = reqwest::blocking::get(suite.path("/events/open"))
        .expect("Rocket should be responsive by now");
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}
