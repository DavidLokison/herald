use sqlx::MySqlConnection;
use uuid::Uuid;

use crate::{Result, Error};
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

pub async fn event_exists(e: &mut MySqlConnection, event_id: &Uuid) -> Result<()> {
    if let None = sqlx::query_file_scalar!("sql/events/exists.sql", event_id).fetch_optional(e).await? {
        Err(Error::NotFound(event_id.as_hyphenated().to_string()))
    } else {
        Ok(())
    }
}

pub async fn event_type_exists(e: &mut MySqlConnection, event_type_slug: &str) -> Result<()> {
    if let None = sqlx::query_file_scalar!("sql/event_types/exists.sql", event_type_slug).fetch_optional(e).await? {
        Err(Error::NotFound(event_type_slug.to_string()))
    } else {
        Ok(())
    }
}

pub async fn get_event_types(e: &mut MySqlConnection) -> Result<Vec<String>> {
    let types = sqlx::query_file_scalar!("sql/event_types/list.sql").fetch_all(e).await?;
    Ok(types)
}

pub async fn get_bookable_items(e: &mut MySqlConnection, event_type_slug: &str) -> Result<Vec<Article>> {
    event_type_exists(e, event_type_slug).await?;
    let articles = sqlx::query_file_as!(intermediate::Article, "sql/events/items.sql", event_type_slug).fetch_all(e).await?;
    // TODO: attach actual price info
    let articles = articles.into_iter().map(|a| a.with_price_info(2, "EUR".to_string())).collect();
    Ok(articles)
}

pub async fn get_open_events(e: &mut MySqlConnection) -> Result<Vec<Event>> {
    sqlx::query_file_as!(Event, "sql/events/open.sql").fetch_all(e).await
        .map_err(Into::into)
}

pub async fn check_persons_price(e: &mut MySqlConnection, event_id: &Uuid, persons: &Vec<PriceCheck>) -> Result<Vec<Article>> {
    event_exists(e, event_id).await?;
    let articles = sqlx::query_file_as!(intermediate::Article, "sql/events/registrations/00_checkup.sql", event_id, persons.as_json()).fetch_all(e).await?;
    // TODO: attach actual price info
    let articles = articles.into_iter().map(|a| a.with_price_info(2, "EUR".to_string())).collect();
    Ok(articles)
}

pub async fn create_registration(e: &mut MySqlConnection, event_id: &Uuid, registration: &NewRegistration<'_>) -> Result<()> {
    event_exists(e, event_id).await?;
    sqlx::query_file!("sql/events/registrations/01_begin.sql", event_id, registration.as_json()).execute(&mut *e).await?;
    sqlx::query_file!("sql/events/registrations/02_persons.sql", registration.persons.as_json()).execute(&mut *e).await?;
    sqlx::query_file!("sql/events/registrations/03_items.sql", registration.items.as_json()).execute(&mut *e).await?;
    sqlx::query_file!("sql/events/registrations/04_finish.sql").execute(e).await?;
    Ok(())
}
