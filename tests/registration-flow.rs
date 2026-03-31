mod common;

use crate::common::TestSuite;

#[test]
fn get_open_events_succeeds() {
    let suite = TestSuite::spawn();
    suite.sql_server.create_open_event();
    let response = reqwest::blocking::get(format!("http://localhost:{}/events/open", suite.port()))
        .expect("Rocket should be responsive by now");
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}
