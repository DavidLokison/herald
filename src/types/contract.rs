use serde::Serialize;
use sqlx::{
    Database,
    Executor,
    Error,
    MySql,
};
use uuid::Uuid;

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
pub struct OrderConfirmation {
    pub event_title: String,
    pub registration_name: String,
    pub club_sender: String,
    pub club_name: String,
    pub approved: bool,
}

impl FromRegistration<MySql> for OrderConfirmation {
    fn fetch<'e, E>(e: E, id: Uuid) -> impl Future<Output = Result<Self, Error>>
    where
        E: Executor<'e, Database = MySql> + 'e,
    {
        sqlx::query_file_as!(OrderConfirmation, "sql/contracts/order_confirmation.sql", id).fetch_one(e)
    }
}
