use rocket::{get, post, launch, routes};
use rocket::http::Status;
use rocket::serde::json::Json;
use uuid::Uuid;

mod core;
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
    let events: Vec<Event> = sqlx::query_as!(
            Event,
            "SELECT event_id AS `id: _`, event_type_slug AS type, title, begin, end AS `end!`, description FROM api_events WHERE deadline >= CURRENT_DATE",
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
    sqlx::query_scalar!("SELECT 1 FROM event_types WHERE event_type_slug = ?", event_type_slug)
        .fetch_optional(&mut **db).await?
        .ok_or_else(|| (Status::NotFound, event_type_slug))?;
    let items: Vec<Article> = sqlx::query_as!(
            Article,
            "SELECT article_id as id, description, price FROM api_item_articles WHERE event_type_slug IS NULL OR event_type_slug = ?",
            event_type_slug,
        )
        .fetch_all(&mut **db).await?;
    Ok(items.into())
}

#[post("/events/<event_id>/persons_price_check", format = "json", data = "<persons>")]
async fn check_persons_price(mut db: Connection, event_id: Uuid, persons: Json<Vec<PriceCheckPersonData>>) -> Response<Vec<Article>> {
    sqlx::query_scalar!("SELECT 1 FROM events WHERE event_id = ?", event_id)
        .fetch_optional(&mut **db).await?
        .ok_or_else(|| (Status::NotFound, event_id.as_hyphenated().to_string()))?;
    let table_def = persons.0.iter().map(|_| "SELECT ?, ?, ?").collect::<Vec<_>>().join(" UNION ALL ");
    let query_str = format!(concat!(
            "WITH\n",
            " map AS (SELECT * FROM util_event_article_policy_map WHERE event_id = ?),\n",
            " data (ord, birthday, team) AS ({})\n",
            "SELECT HEX(article_id) AS id, a.description, price\n",
            "FROM data\n",
            " INNER JOIN LATERAL (\n",
            "  SELECT article_id FROM map WHERE (\n",
            "   policy_flags IS NULL OR policy_flags = team\n",
            "  ) AND (\n",
            "   policy_birthday IS NULL OR policy_birthday >= birthday\n",
            "  ) ORDER BY policy_flags DESC, policy_age DESC LIMIT 1\n",
            " ) m\n",
            " INNER JOIN articles a USING (article_id)\n",
            "ORDER BY ord"), table_def);
    let mut query = sqlx::query_as(query_str.as_str()).bind(event_id);
    for (index, person) in persons.0.iter().enumerate() {
        query = query.bind(index as u32).bind(person.birthday).bind(person.team);
    }
    query.fetch_all(&mut **db).await.map_err(Into::into).map(Into::into)
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
