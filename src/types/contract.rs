use serde::{Serialize, Deserialize};
use sqlx::{
    Database,
    Executor,
    Error,
    MySql,
};
use sqlx::types::Json;
use uuid::Uuid;
use time::Date;

use super::Name;

#[non_exhaustive]
#[derive(Serialize)]
pub enum Contract {
    #[serde(untagged)]
    OrderConfirmation(OrderConfirmation),
    #[serde(untagged)]
    PaymentInformation(PaymentInformation),
    #[serde(untagged)]
    PaymentConfirmation(PaymentConfirmation),
}

impl Contract {
    pub async fn fetch<'e, E>(contract: &str, e: E, id: Uuid) -> Result<Self, Error>
    where
        E: Executor<'e, Database = MySql> + 'e
    {
        Ok(match contract {
            "order_confirmation" => Self::OrderConfirmation(OrderConfirmation::fetch(e, id).await?),
            "payment_information" => Self::PaymentInformation(PaymentInformation::fetch(e, id).await?),
            "payment_confirmation" => Self::PaymentConfirmation(PaymentConfirmation::fetch(e, id).await?),
            _ => todo!(),
        })
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
    #[serde(serialize_with = "Name::serialize", flatten)]
    pub name: String,
    pub email: String,
}

impl<'de> Deserialize<'de> for Contact {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Input {
            name: String,
            email: String,
        }

        let input = Input::deserialize(deserializer)?;
        Ok(Contact {
            name: input.name,
            email: input.email,
        })
    }
}

impl From<Json<Self>> for Contact {
    fn from(t: Json<Self>) -> Self {
        t.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClubInformation {
    pub short: String,
    pub long: String,
    pub email: String,
}

impl From<Json<Self>> for ClubInformation {
    fn from(t: Json<Self>) -> Self {
        t.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClubBankInformation {
    pub name: String,
    pub iban: String,
    pub bic: String,
}

impl From<Json<Self>> for ClubBankInformation {
    fn from(t: Json<Self>) -> Self {
        t.0
    }
}

#[derive(Serialize)]
#[serde(transparent)]
pub struct InvoiceId(String);

impl InvoiceId {
    fn to_base32(bytes: &[u8]) -> String {
        const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
        let mut out = String::new();
        let mut value = 0u8;
        let mut bits = 0;

        for &byte in bytes {
            for shift in (0..8).rev() {
                value = (value << 1) | ((byte >> shift) & 1);
                bits += 1;

                if bits == 5 {
                    out.push(ALPHABET[value as usize] as char);
                    value = 0;
                    bits = 0;
                }
            }
        }

        if bits > 0 {
            value <<= 5 - bits; // zero-pad the final 5-bit group
            out.push(ALPHABET[value as usize] as char);
        }

        out
    }
}

impl<'r> From<&'r [u8]> for InvoiceId {
    fn from(t: &'r [u8]) -> Self {
        Self(Self::to_base32(t))
    }
}

#[derive(Deserialize)]
pub struct Price {
    pub value: i32,
    pub decimals: u8,
    pub currency: String,
}

impl Serialize for Price {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        let sign = if self.value < 0 { "-" } else { "" };
        let mut digits = self.value.unsigned_abs().to_string();
        if self.decimals > 0 {
            let decimals = self.decimals as usize;
            if digits.len() > decimals {
                let (left, right) = digits.as_str().split_at(digits.len() - decimals);
                digits = format!("{}.{}", left, right);
            } else {
                let padding = "0".repeat(decimals - digits.len());
                digits = format!("0.{}{}", padding, digits);
            }
        }
        s.serialize_str(&format!("{} {}{}", self.currency, sign, digits))
    }
}

impl From<Json<Self>> for Price {
    fn from(t: Json<Self>) -> Self {
        t.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct Article {
    pub id: String,
    pub description: String,
    pub count: u8,
    pub price: Price,
}

struct Articles(Vec<Article>);

impl<'r, DB: Database> sqlx::Decode<'r, DB> for Articles
where
    Json<Vec<Article>>: sqlx::Decode<'r, DB>
{
    fn decode(value: <DB as sqlx::database::HasValueRef<'r>>::ValueRef)
        -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>>
    {
        let value = <Json<Vec<Article>> as sqlx::Decode<DB>>::decode(value)?;
        Ok(Self(value.0))
    }
}

impl From<Articles> for Vec<Article> {
    fn from(t: Articles) -> Self {
        t.0
    }
}

#[derive(Serialize)]
pub struct OrderConfirmation {
    pub event_title: String,
    pub recipient: Contact,
    pub club: ClubInformation,
    pub autoapproved: bool,
}

impl FromRegistration<MySql> for OrderConfirmation {
    fn fetch<'e, E>(e: E, id: Uuid) -> impl Future<Output = Result<Self, Error>>
    where
        E: Executor<'e, Database = MySql> + 'e,
    {
        sqlx::query_file_as!(Self, "sql/contracts/order_confirmation.sql", id).fetch_one(e)
    }
}

#[derive(Serialize)]
pub struct PaymentInformation {
    pub event_title: String,
    pub recipient: Contact,
    pub club: ClubInformation,
    pub invoice_id: InvoiceId,
    pub articles: Vec<Article>,
    pub total: Price,
    pub target: Date,
    pub bank: ClubBankInformation,
}

impl FromRegistration<MySql> for PaymentInformation {
    fn fetch<'e, E>(e: E, id: Uuid) -> impl Future<Output = Result<Self, Error>>
    where
        E: Executor<'e, Database = MySql> + 'e,
    {
        sqlx::query_file_as!(Self, "sql/contracts/payment_information.sql", id).fetch_one(e)
    }
}

#[derive(Serialize)]
pub struct PaymentConfirmation {
    pub event_title: String,
    pub recipient: Contact,
    pub club: ClubInformation,
    pub total: Price,
    pub paid: Price,
}

impl FromRegistration<MySql> for PaymentConfirmation {
    fn fetch<'e, E>(e: E, id: Uuid) -> impl Future<Output = Result<Self, Error>>
    where
        E: Executor<'e, Database = MySql> + 'e,
    {
        sqlx::query_file_as!(Self, "sql/contracts/payment_confirmation.sql", id).fetch_one(e)
    }
}

#[cfg(test)]
mod tests {
    use super::{Contact, ClubInformation, OrderConfirmation};
    use super::Price;
    use super::InvoiceId;

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

    #[test]
    fn format_price_with_decimals() {
        let obj = Price{
            value: 12345,
            decimals: 2,
            currency: String::from("USD"),
        };
        let val = rocket::serde::json::to_value(&obj).expect("Serialize failed");
        assert!(val.is_string());
        let val = val.as_str().unwrap();
        assert_eq!(val, "USD 123.45");
    }

    #[test]
    fn format_price_without_decimals() {
        let obj = Price{
            value: 12345,
            decimals: 0,
            currency: String::from("JPY"),
        };
        let val = rocket::serde::json::to_value(&obj).expect("Serialize failed");
        assert!(val.is_string());
        let val = val.as_str().unwrap();
        assert_eq!(val, "JPY 12345");
    }

    #[test]
    fn format_negative_price() {
        let obj = Price{
            value: -12345,
            decimals: 2,
            currency: String::from("USD"),
        };
        let val = rocket::serde::json::to_value(&obj).expect("Serialize failed");
        assert!(val.is_string());
        let val = val.as_str().unwrap();
        assert_eq!(val, "USD -123.45");
    }

    #[test]
    fn base32_conversion() {
        let vals: [u8; 5] = [0xab, 0x8c, 0x3f, 0x32, 0x28];
        let s = InvoiceId::to_base32(&vals);
        assert_eq!(s, "VOGD6MRI");
    }
}
