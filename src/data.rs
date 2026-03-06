use sqlx::MySqlConnection;
use uuid::Uuid;
use rocket::http::Status;

use crate::core::HeraldResponseErr;
use crate::types::*;

pub async fn event_exists(e: &mut MySqlConnection, event_id: &Uuid) -> Result<(), HeraldResponseErr> {
    sqlx::query_file_scalar!("sql/exists/event.sql", event_id).fetch_optional(e).await?
        .map(|_| ())
        .ok_or_else(|| (Status::NotFound, event_id.as_hyphenated().to_string()).into())
}

pub async fn event_type_exists(e: &mut MySqlConnection, event_type_slug: &str) -> Result<(), HeraldResponseErr> {
    sqlx::query_file_scalar!("sql/exists/event_type.sql", event_type_slug).fetch_optional(e).await?
        .map(|_| ())
        .ok_or_else(|| (Status::NotFound, event_type_slug).into())
}

pub async fn get_event_types(e: &mut MySqlConnection) -> Result<Vec<String>, HeraldResponseErr> {
    sqlx::query_file_scalar!("sql/list/event_types.sql").fetch_all(e).await
        .map_err(Into::into)
}

pub async fn get_bookable_items(e: &mut MySqlConnection, event_type_slug: &str) -> Result<Vec<Article>, HeraldResponseErr> {
    event_type_exists(e, event_type_slug).await?;
    sqlx::query_file_as!(Article, "sql/bookable_items.sql", event_type_slug).fetch_all(e).await
        .map_err(Into::into)
}

pub async fn get_open_events(e: &mut MySqlConnection) -> Result<Vec<Event>, HeraldResponseErr> {
    sqlx::query_file_as!(Event, "sql/open_events.sql").fetch_all(e).await
        .map_err(Into::into)
}

