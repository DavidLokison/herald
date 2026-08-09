use sqlx::MySqlConnection;
use time::Date;

use crate::Result;
use crate::types::*;
use crate::types::request::*;
use crate::types::response::*;
use crate::types::intermediate::{self, IntoIntermediate};

pub async fn run_tests(e: &mut MySqlConnection, filter: &str) -> Result<UpstreamHealth> {
    use std::time::{Instant, Duration};
    let tic = Instant::now();
    let tests = sqlx::query_file_as!(Test, "sql/health.sql", filter).fetch_all(e).await?;
    let ping = tic.elapsed().div_duration_f32(Duration::from_millis(1));
    Ok(UpstreamHealth {
        ping: ping,
        tests: tests,
    })
}

pub async fn run_tests_health(e: &mut MySqlConnection) -> Result<UpstreamHealth> {
    run_tests(e, "health").await
}

pub async fn get_event_types(e: &mut MySqlConnection) -> Result<Vec<String>> {
    let types = sqlx::query_file_scalar!("sql/event_types/list.sql").fetch_all(e).await?;
    Ok(types)
}

pub async fn get_bookable_items(e: &mut MySqlConnection, event_type_slug: &EventTypeSlug<'_>) -> Result<Vec<Article>> {
    let articles = sqlx::query_file_as!(intermediate::Article, "sql/events/items.sql", **event_type_slug).fetch_all(e).await?;
    // TODO: attach actual price info
    let articles = articles.into_iter().map(|a| a.with_price_info(2, "EUR".to_string())).collect();
    Ok(articles)
}

pub async fn get_open_events(e: &mut MySqlConnection) -> Result<Vec<Event>> {
    sqlx::query_file_as!(Event, "sql/events/open.sql").fetch_all(e).await
        .map_err(Into::into)
}

pub async fn get_registration_preview(e: &mut MySqlConnection, event_id: &EventId<'_>, birthdays: &Vec<Date>) -> Result<Vec<Price>> {
    let articles = sqlx::query_file_as!(intermediate::PricePreview, "sql/events/registrations/00_checkup.sql", **event_id, birthdays.as_json()).fetch_all(e).await?;
    // TODO: attach actual price info
    let articles = articles.into_iter().map(|a| a.with_price_info(2, "EUR".to_string())).collect();
    Ok(articles)
}

pub async fn create_registration(e: &mut MySqlConnection, event_id: &EventId<'_>, registration: &NewRegistration<'_>, manual: bool) -> Result<()> {
    sqlx::query_file!("sql/events/registrations/01_begin.sql", **event_id, registration.as_json()).execute(&mut *e).await?;
    sqlx::query_file!("sql/events/registrations/02_persons.sql", registration.persons.as_json()).execute(&mut *e).await?;
    sqlx::query_file!("sql/events/registrations/03_items.sql", registration.items.as_json()).execute(&mut *e).await?;
    sqlx::query_file!("sql/events/registrations/04_finish.sql", if manual { "awaiting_approval" } else { "automatic_approval" }).execute(e).await?;
    Ok(())
}
