use rocket::{get, launch, routes};
use rocket::http::Status;
use serde::Serialize;
use time::Date;

mod core;
use crate::core::{Connection, Response};

#[derive(Serialize, Debug)]
struct UpstreamHealth {
    ping: f32,
}

#[derive(Serialize, Debug)]
struct Event {
    id: String,
    r#type: String,
    title: String,
    begin: Date,
    end: Date,
    description: Option<String>,
}

#[derive(Serialize, Debug)]
struct Article {
    id: String,
    description: String,
    // TODO: make the price an own structure with an u32, an u8 for decimals and a &str (String?)
    // for the currency
    price: u32,
}

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
    let events: Vec<Event> = sqlx::query_as!(
            Event,
            "SELECT event_id AS id, event_type_slug AS type, title, begin, end AS `end!`, description FROM api_events WHERE deadline >= CURRENT_DATE",
        )
        .fetch_all(&mut **db).await?;
    Ok((Status::Ok, events).into())
}

#[get("/events/types")]
async fn get_event_types(mut db: Connection) -> Response<Vec<String>> {
    let types: Vec<String> = sqlx::query_scalar!("SELECT event_type_slug FROM event_types")
        .fetch_all(&mut **db).await?;
    Ok(types.into())
}

#[get("/events/types/<event_type_slug>/items")]
async fn get_bookable_items(mut db: Connection, event_type_slug: &str) -> Response<Vec<Article>> {
    // TODO: return a 404 if the event type slug doesn't exist
    let items: Vec<Article> = sqlx::query_as!(
            Article,
            "SELECT article_id as id, description, price FROM api_item_articles WHERE event_type_slug IS NULL OR event_type_slug = ?",
            event_type_slug,
        )
        .fetch_all(&mut **db).await?;
    Ok(items.into())
}

#[launch]
fn rocket() -> _ {
    core::build()
        .mount("/", routes![
            check_health,
            get_open_events,
            get_event_types,
            get_bookable_items,
        ])
}
