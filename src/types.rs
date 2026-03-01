use serde::Serialize;
use time::Date;

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
    // TODO: make the price an own structure with an u32, an u8 for decimals and a &str (String?)
    // for the currency
    pub price: u32,
}
