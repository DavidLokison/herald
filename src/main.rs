use rocket::{get, launch, routes};
use rocket::http::Status;
use serde::Serialize;
use sqlx::FromRow;
use time::Date;

mod core;
use crate::core::{Connection, Response};

#[derive(Serialize, Debug)]
struct UpstreamHealth {
    ping: f32,
}

#[derive(Serialize, FromRow, Debug)]
struct Event {
    #[sqlx(rename = "event_id")]
    id: String,
    title: String,
    begin: Date,
    end: Date,
    description: String,
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

#[get("/events/open")]
async fn get_open_events(mut db: Connection) -> Response<Vec<Event>> {
    let events: Vec<Event> = sqlx::query_as("SELECT event_id, title, begin, end, description FROM api_events WHERE deadline >= CURRENT_DATE")
        .fetch_all(&mut **db).await?;
    Ok((Status::Ok, events).into())
}

#[launch]
fn rocket() -> _ {
    core::build()
        .mount("/", routes![
            check_health,
            get_open_events,
        ])
}
