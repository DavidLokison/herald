use sqlx::MySqlConnection;
use uuid::Uuid;

use crate::{Result, Error};
use crate::types::*;
use crate::types::intermediate::{self, IntoIntermediate};

pub async fn event_exists(e: &mut MySqlConnection, event_id: &Uuid) -> Result<()> {
    if let None = sqlx::query_file_scalar!("sql/exists/event.sql", event_id).fetch_optional(e).await? {
        Err(Error::NotFound(event_id.as_hyphenated().to_string()))
    } else {
        Ok(())
    }
}

pub async fn event_type_exists(e: &mut MySqlConnection, event_type_slug: &str) -> Result<()> {
    if let None = sqlx::query_file_scalar!("sql/exists/event_type.sql", event_type_slug).fetch_optional(e).await? {
        Err(Error::NotFound(event_type_slug.to_string()))
    } else {
        Ok(())
    }
}

pub async fn get_event_types(e: &mut MySqlConnection) -> Result<Vec<String>> {
    let types = sqlx::query_file_scalar!("sql/list/event_types.sql").fetch_all(e).await?;
    Ok(types)
}

pub async fn get_bookable_items(e: &mut MySqlConnection, event_type_slug: &str) -> Result<Vec<Article>> {
    event_type_exists(e, event_type_slug).await?;
    let articles = sqlx::query_file_as!(intermediate::Article, "sql/bookable_items.sql", event_type_slug).fetch_all(e).await?;
    // TODO: attach actual price info
    let articles = articles.into_iter().map(|a| a.with_price_info(2, "EUR".to_string())).collect();
    Ok(articles)
}

pub async fn get_open_events(e: &mut MySqlConnection) -> Result<Vec<Event>> {
    sqlx::query_file_as!(Event, "sql/open_events.sql").fetch_all(e).await
        .map_err(Into::into)
}

pub async fn check_persons_price(e: &mut MySqlConnection, event_id: &Uuid, persons: &Vec<PriceCheck>) -> Result<Vec<Article>> {
    event_exists(e, event_id).await?;
    let articles = sqlx::query_file_as!(intermediate::Article, "sql/persons_price_check.sql", event_id, persons.as_json()).fetch_all(e).await?;
    // TODO: attach actual price info
    let articles = articles.into_iter().map(|a| a.with_price_info(2, "EUR".to_string())).collect();
    Ok(articles)
}

pub async fn create_registration(e: &mut MySqlConnection, event_id: &Uuid, registration: &NewRegistration) -> Result<()> {
    event_exists(e, event_id).await?;
    sqlx::query_file!("sql/events/registrations/01_begin.sql", event_id, registration.as_json()).execute(&mut *e).await?;
    sqlx::query_file!("sql/events/registrations/02_persons.sql", registration.persons.as_json()).execute(&mut *e).await?;
    sqlx::query_file!("sql/events/registrations/03_items.sql", registration.items.as_json()).execute(&mut *e).await?;
    sqlx::query_file!("sql/events/registrations/04_finish.sql").execute(e).await?;
    Ok(())
}
