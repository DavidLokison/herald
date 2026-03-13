use serde::{Serialize, Deserialize};
use time::Date;
use uuid::Uuid;

pub(crate) mod intermediate;

#[derive(Serialize, Deserialize)]
pub struct PriceCheck {
    pub birthday: Date,
    pub team: bool,
}

#[derive(Serialize, Debug)]
pub struct Price {
    pub value: u32,
    pub decimals: u8,
    pub currency: String,
}

#[derive(Serialize, Debug)]
pub struct UpstreamHealth {
    pub ping: f32,
}

#[derive(Serialize, Debug)]
pub struct Event {
    pub id: Uuid,
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
