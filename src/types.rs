use std::ops::Deref;
use sqlx::{Executor, MySql};
use uuid::Uuid;

pub(crate) mod intermediate;
pub mod request;
pub mod response;
pub mod contract;

use crate::Error;

pub trait TryQuery<T>: Sized {
    fn try_query<'e, E>(e: E, t: T) -> impl Future<Output = Result<Self, Error>> + Send
    where
        E: 'e + Executor<'e, Database = MySql>;
}

pub struct EventId<'s>(&'s Uuid);

impl<'s> Deref for EventId<'s> {
    type Target = &'s Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> TryQuery<&'a Uuid> for EventId<'a> {
    async fn try_query<'e, E>(e: E, event_id: &'a Uuid) -> Result<Self, Error>
    where
        E: 'e + Executor<'e, Database = MySql>
    {
        if let None = sqlx::query_file_scalar!("sql/events/exists.sql", event_id).fetch_optional(e).await? {
            Err(Error::NotFound(event_id.as_hyphenated().to_string()))
        } else {
            Ok(Self(event_id))
        }
    }
}

pub struct EventTypeSlug<'s>(&'s str);

impl<'s> Deref for EventTypeSlug<'s> {
    type Target = &'s str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> TryQuery<&'a str> for EventTypeSlug<'a> {
    async fn try_query<'e, E>(e: E, event_type_slug: &'a str) -> Result<Self, Error>
    where
        E: 'e + Executor<'e, Database = MySql>
    {
        if let None = sqlx::query_file_scalar!("sql/event_types/exists.sql", event_type_slug).fetch_optional(e).await? {
            Err(Error::NotFound(event_type_slug.to_string()))
        } else {
            Ok(Self(event_type_slug))
        }
    }
}
