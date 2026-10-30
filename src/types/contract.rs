use serde::Serialize;
use sqlx::{
    Database,
    Executor,
    Error,
    MySql,
};
use uuid::Uuid;

use super::Name;

#[non_exhaustive]
pub enum Contract {
    OrderConfirmation(OrderConfirmation),
}

impl Contract {
    pub async fn fetch<'e, E>(contract: &str, e: E, id: Uuid) -> Result<Self, Error>
    where
        E: Executor<'e, Database = MySql> + 'e
    {
        Ok(match contract {
            "order_confirmation" => Self::OrderConfirmation(OrderConfirmation::fetch(e, id).await?),
            _ => todo!(),
        })
    }

    pub fn into_inner(self) -> impl Serialize {
        match self {
            Self::OrderConfirmation(i) => i,
        }
    }
}

trait FromRegistration<DB>: Sized
where
    DB: Database,
{
    fn fetch<'e, E>(e: E, id: Uuid) -> impl Future<Output = Result<Self, Error>>
    where
        E: Executor<'e, Database = DB> + 'e;
}

#[derive(Serialize)]
pub struct Contact {
    #[serde(with = "Name", flatten)]
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct ClubInformation {
    pub short: String,
    pub long: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct OrderConfirmation {
    pub event_title: String,
    pub recipient: Contact,
    pub club: ClubInformation,
    pub autoapproved: bool,
}

impl FromRegistration<MySql> for OrderConfirmation {
    async fn fetch<'e, E>(e: E, id: Uuid) -> Result<Self, Error>
    where
        E: Executor<'e, Database = MySql> + 'e,
    {
        struct Intermediate {
            title: String,
            registration_name: String,
            registration_email: String,
            club_name_full: String,
            club_name_short: String,
            club_email: String,
            autoapproved: bool,
        }
        let data = sqlx::query_file_as!(Intermediate, "sql/contracts/order_confirmation.sql", id).fetch_one(e).await?;
        Ok(Self{
            event_title: data.title,
            recipient: Contact{
                name: data.registration_name,
                email: data.registration_email,
            },
            club: ClubInformation{
                short: data.club_name_short,
                long: data.club_name_full,
                email: data.club_email,
            },
            autoapproved: data.autoapproved,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Contact, ClubInformation, OrderConfirmation};

    #[test]
    fn order_confirmation_serializes_recipient_as_object() {
        let obj = OrderConfirmation{
            event_title: String::from("Unittesting is great"),
            recipient: Contact{
                name: String::from("Mister\tTesto\tTestoson"),
                email: String::from("user@example.com"),
            },
            club: ClubInformation{
                short: String::from("Unittest Group"),
                long: String::from("Unittest Group"),
                email: String::from("mail@example.com"),
            },
            autoapproved: false,
        };
        let val = rocket::serde::json::to_value(&obj).expect("Serialize failed");
        let val = val.as_object().unwrap();
        assert!(val.get("recipient").is_some_and(|r| r.is_object()));
        let val = val.get("recipient").unwrap().as_object().unwrap();
        assert!(val.get("title").is_some_and(|s| s.is_string()));
    }
}
