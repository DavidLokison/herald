use serde::{Serialize, Deserialize};
use time::Date;

#[derive(Serialize, Deserialize)]
pub struct PriceCheckPersonData {
    pub birthday: Date,
    pub team: bool,
}
