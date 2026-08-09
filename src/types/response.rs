use serde::Serialize;
use time::Date;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct Price {
    pub description: String,
    pub value: i32,
    pub decimals: u8,
    pub currency: String,
}

#[derive(Serialize, Debug)]
pub struct UpstreamHealth {
    pub ping: f32,
    pub tests: Vec<Test>,
}

#[derive(Serialize, Debug)]
pub struct Event {
    pub id: Uuid,
    pub r#type: String,
    pub title: String,
    pub start: Date,
    pub end: Date,
    pub description: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct Article {
    pub id: String,
    #[serde(flatten)]
    pub price: Price,
}

#[derive(Serialize, Debug)]
pub struct Test {
    pub name: String,
    pub status: TestStatus,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub enum TestStatus {
    PASS,
    FAIL,
}

impl From<String> for TestStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "PASS" => Self::PASS,
            "FAIL" => Self::FAIL,
            _ => unreachable!(),
        }
    }
}
