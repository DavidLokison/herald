use rocket::{get, launch, routes};
use rocket::http::Status;
use serde::Serialize;

mod core;
use crate::core::{Connection, Response};

#[derive(Serialize, Debug)]
struct UpstreamHealth {
    ping: f32,
}

#[get("/health")]
async fn check_health(mut db: Connection) -> Response<UpstreamHealth> {
    use std::time::{Instant, Duration};
    let tic = Instant::now();
    let tests: Vec<(String, String)> = sqlx::query_as("SELECT test_name, message FROM dolt_test_run('health') WHERE status <> 'PASS'")
        .fetch_all(&mut **db).await?;
    let ping = tic.elapsed();
    if tests.is_empty() {
        Ok(UpstreamHealth {
            ping: ping.div_duration_f32(Duration::from_millis(1)),
        }.into())
    } else {
        todo!()
    }
}

#[launch]
fn rocket() -> _ {
    core::build()
        .mount("/", routes![check_health])
}
