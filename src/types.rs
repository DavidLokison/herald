use sqlx::{
    MySql,
    Type,
    mysql::MySqlTypeInfo,
};
use serde::Serialize;
use time::Date;

#[derive(Serialize, Debug)]
pub struct Price {
    pub value: u32,
    pub decimals: u8,
    pub currency: String,
}

impl Type<MySql> for Price {
    fn type_info() -> MySqlTypeInfo {
        u32::type_info()
    }
}

impl From<u32> for Price {
    fn from(value: u32) -> Self {
        Self {
            value: value,
            decimals: 2,
            currency: "EUR".to_string(),
        }
    }
}
#[derive(Serialize, Debug)]
pub struct UpstreamHealth {
    pub ping: f32,
}

#[derive(Serialize, Debug)]
pub struct Event {
    pub id: String,
    pub r#type: String,
    pub title: String,
    pub begin: Date,
    pub end: Date,
    pub description: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct Article {
    pub id: String,
    pub description: String,
    pub price: Price,
}
