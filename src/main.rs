use rocket::{get, post, launch, routes};
use rocket::serde::json::Json;
use uuid::Uuid;

mod core;
mod data;
mod types;
use crate::core::{Connection, Response};
use crate::types::*;
use crate::types::request::*;

#[get("/health")]
async fn check_health(mut db: Connection) -> Response<UpstreamHealth> {
    use std::time::{Instant, Duration};
    struct TestStatus {
        test_name: String,
        message: String,
    }
    let tic = Instant::now();
    let tests: Vec<TestStatus> = sqlx::query_as!(
            TestStatus,
            "SELECT test_name, message FROM dolt_test_run('health') WHERE status <> 'PASS'",
        )
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

expose_endpoint!(#[get("/events/open")] get_open_events -> Vec<Event>);
expose_endpoint!(#[get("/events/types")] get_event_types -> Vec<String>);
expose_endpoint!(#[get("/events/types/<event_type_slug>/items")] get_bookable_items -> Vec<Article>, event_type_slug: &str);
expose_endpoint!(#[post("/events/<event_id>/persons_price_check", format = "json", data = "<persons>")] check_persons_price -> Vec<Article>, event_id: Uuid, persons: Json<Vec<PriceCheckPersonData>>);

#[launch]
fn rocket() -> _ {
    core::build()
        .mount("/", routes![
            check_health,
            get_open_events,
            get_event_types,
            get_bookable_items,
            check_persons_price,
        ])
}
