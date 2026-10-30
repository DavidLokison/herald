use std::ops::Deref;
use serde::{Serialize, Deserialize};
use sqlx::{Executor, MySql};
use uuid::Uuid;

pub(crate) mod intermediate;
pub mod request;
pub mod response;
pub mod contract;

use crate::Error;
use intermediate::{impl_intermediate, IntoIntermediate};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Name<'r> {
    pub title: &'r str,
    pub firstname: &'r str,
    pub lastname: &'r str,
}

impl_intermediate!(Name<'r> => "{}\t{}\t{}", title, firstname, lastname);

impl<'r> From<&'r str> for Name<'r> {
    fn from(s: &'r str) -> Self {
        const ERROR: &'static str = "Error parsing name info";
        let mut data = s.split("\t");
        Name{
            title: data.next().expect(ERROR),
            firstname: data.next().expect(ERROR),
            lastname: data.next().expect(ERROR),
        }
    }
}

impl<'r> Name<'r> {
    pub fn serialize<S, T>(t: &'r T, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        Self: From<&'r T>,
        T: ?Sized,
    {
        Self::from(t).serialize(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address<'r> {
    pub street: &'r str,
    pub zip: &'r str,
    pub city: &'r str,
}

impl_intermediate!(Address<'r> => "{}\n{} {}", street, zip, city);

impl<'r> From<&'r str> for Address<'r> {
    fn from(s: &'r str) -> Self {
        const ERROR: &'static str = "Error parsing address info";
        let mut data = s.split("\n");
        let street = data.next().expect(ERROR);
        let zipcity = data.next().expect(ERROR);
        let (zip, city) = zipcity.split_once(" ").expect(ERROR);
        Address{
            street: street,
            zip: zip,
            city: city,
        }
    }
}

impl<'r> Address<'r> {
    pub fn serialize<S, T>(t: &'r T, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        Self: From<&'r T>,
        T: ?Sized,
    {
        Self::from(t).serialize(s)
    }
}

#[cfg(test)]
mod tests {
    use super::{Name, Address};

    #[test]
    fn create_name_struct() {
        let name = Name::from("Mister\tTesto\tTestoson");
        assert_eq!(name.title, "Mister");
        assert_eq!(name.firstname, "Testo");
        assert_eq!(name.lastname, "Testoson");
    }

    #[test]
    fn create_address_struct() {
        let adr = Address::from("Testroad 1\n01234 Sometown");
        assert_eq!(adr.street, "Testroad 1");
        assert_eq!(adr.zip, "01234");
        assert_eq!(adr.city, "Sometown");
    }
}
