use sqlx::MySqlConnection;
use uuid::Uuid;
use rocket::http::Status;

use crate::core::Result;
use crate::types::*;
use crate::types::intermediate;
use crate::types::intermediate::IntoIntermediate;

#[macro_export]
macro_rules! expose_endpoint {
    ($(#[$meta:meta])* $name:ident -> $T:ty) => {
        $(#[$meta])*
        async fn $name(mut db: rocket_db_pools::Connection<crate::core::Herald>) -> Result<crate::core::HeraldResponseOk<$T>, crate::core::HeraldResponseErr> {
            crate::data::$name(&mut db).await.map(Into::into)
        }
    };

    ($(#[$meta:meta])* $name:ident -> $T:ty, $($arg:ident : $A:ty),*) => {
        $(#[$meta])*
        async fn $name(mut db: rocket_db_pools::Connection<crate::core::Herald>, $($arg: $A),*) -> Result<crate::core::HeraldResponseOk<$T>, crate::core::HeraldResponseErr> {
            crate::data::$name(&mut db, $(&$arg),*).await.map(Into::into)
        }
    };
}

pub async fn event_exists(e: &mut MySqlConnection, event_id: &Uuid) -> Result<()> {
    sqlx::query_file_scalar!("sql/exists/event.sql", event_id).fetch_optional(e).await?
        .map(|_| ())
        .ok_or_else(|| (Status::NotFound, event_id.as_hyphenated().to_string()).into())
}

pub async fn event_type_exists(e: &mut MySqlConnection, event_type_slug: &str) -> Result<()> {
    sqlx::query_file_scalar!("sql/exists/event_type.sql", event_type_slug).fetch_optional(e).await?
        .map(|_| ())
        .ok_or_else(|| (Status::NotFound, event_type_slug).into())
}

pub async fn get_event_types(e: &mut MySqlConnection) -> Result<Vec<String>> {
    sqlx::query_file_scalar!("sql/list/event_types.sql").fetch_all(e).await
        .map_err(Into::into)
}

pub async fn get_bookable_items(e: &mut MySqlConnection, event_type_slug: &str) -> Result<Vec<Article>> {
    event_type_exists(e, event_type_slug).await?;
    sqlx::query_file_as!(intermediate::Article, "sql/bookable_items.sql", event_type_slug).fetch_all(e).await
        // TODO: attach actual price info
        .map(|v| v.into_iter().map(|a| a.with_price_info(2, "EUR".to_string())).collect())
        .map_err(Into::into)
}

pub async fn get_open_events(e: &mut MySqlConnection) -> Result<Vec<Event>> {
    sqlx::query_file_as!(Event, "sql/open_events.sql").fetch_all(e).await
        .map_err(Into::into)
}

pub async fn check_persons_price(e: &mut MySqlConnection, event_id: &Uuid, persons: &Vec<PriceCheck>) -> Result<Vec<Article>> {
    event_exists(e, event_id).await?;
    sqlx::query_file_as!(intermediate::Article, "sql/persons_price_check.sql", event_id, persons.as_json()).fetch_all(e).await
        // TODO: attach actual price info
        .map(|v| v.into_iter().map(|a| a.with_price_info(2, "EUR".to_string())).collect())
        .map_err(Into::into)
}
