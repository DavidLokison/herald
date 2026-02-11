use rocket::{get, launch, routes};
use rocket::http::Status;
use serde::Serialize;

mod core;
use crate::core::{Connection, HeraldResponse};

#[derive(Serialize, Debug)]
struct UpstreamHealth {
    ping: f32,
}

#[get("/health")]
async fn check_health(mut db: Connection) -> HeraldResponse<UpstreamHealth> {
    use std::time::{Instant, Duration};
    let tic = Instant::now();
    let tests = sqlx::query_as("SELECT test_name, message FROM dolt_test_run('health') WHERE status <> 'PASS'")
        .fetch_all(&mut **db).await
        .map_err(|e| HeraldResponse::err(Status::InternalServerError, format!("Unknown Error: {}", e.to_string())));
    let ping = tic.elapsed();
    if tests.is_err() {
        return tests.unwrap_err();
    }
    let tests: Vec<(String, String)> = tests.unwrap();
    if tests.is_empty() {
        HeraldResponse::ok(Status::Ok, UpstreamHealth {
            ping: ping.div_duration_f32(Duration::from_millis(1)),
        })
    } else {
        todo!()
    }
}

#[launch]
fn rocket() -> _ {
    core::build()
        .mount("/", routes![check_health])
}
