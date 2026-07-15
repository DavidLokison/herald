use serde::Serialize;

#[non_exhaustive]
pub enum Contract {
    OrderConfirmation(OrderConfirmation),
}

impl Contract {
    pub fn into_inner(self) -> impl Serialize {
        match self {
            Self::OrderConfirmation(i) => i,
        }
    }
}

#[derive(Serialize)]
pub struct OrderConfirmation {
    pub event_title: String,
    pub registration_name: String,
    pub club_sender: String,
    pub club_name: String,
    pub approved: bool,
}
