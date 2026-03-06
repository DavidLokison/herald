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

#[get("/events/open")]
async fn get_open_events(mut db: Connection) -> Response<Vec<Event>> {
    data::get_open_events(&mut **db).await.map(Into::into)
}

#[get("/events/types")]
async fn get_event_types(mut db: Connection) -> Response<Vec<String>> {
    data::get_event_types(&mut **db).await.map(Into::into)
}

#[get("/events/types/<event_type_slug>/items")]
async fn get_bookable_items(mut db: Connection, event_type_slug: &str) -> Response<Vec<Article>> {
    data::get_bookable_items(&mut **db, event_type_slug).await.map(Into::into)
}

#[post("/events/<event_id>/persons_price_check", format = "json", data = "<persons>")]
async fn check_persons_price(mut db: Connection, event_id: Uuid, persons: Json<Vec<PriceCheckPersonData>>) -> Response<Vec<Article>> {
    data::check_persons_price(&mut db, &event_id, &persons).await.map(Into::into)
}

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
