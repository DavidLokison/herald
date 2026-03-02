use serde::Deserialize;
use time::Date;

#[derive(Deserialize)]
pub struct PriceCheckPersonData {
    pub birthday: Date,
    pub team: bool,
}
