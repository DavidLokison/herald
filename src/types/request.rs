use serde::{Serialize, Deserialize};
use time::Date;

use super::intermediate::{self, IntoIntermediate};
use crate::impl_intermediate;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PriceCheck {
    pub birthday: Date,
    pub team: bool,
}
impl_intermediate!(PriceCheck);

#[derive(Deserialize, Debug)]
pub struct NewRegistration {
    pub name: NewName,
    pub address: NewAddress,
    pub phone: String,
    pub email: String,
    pub comment: String,
    pub emergency: EmergencyContact,
    pub persons: Vec<NewPerson>,
    pub items: Vec<NewItem>,
}

impl IntoIntermediate for NewRegistration {
    type Intermediate = intermediate::NewRegistration;
    fn into_intermediate(&self) -> Self::Intermediate {
        Self::Intermediate {
            name: self.name.into_intermediate(),
            address: self.address.into_intermediate(),
            phone: self.phone.into_intermediate(),
            email: self.email.into_intermediate(),
            comment: self.comment.into_intermediate(),
            emergency: self.emergency.into_intermediate(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct NewPerson {
    pub name: NewName,
    pub address: NewAddress,
    pub birthday: Date,
    pub comment: String,
    pub flags: NewPersonFlags,
}

impl IntoIntermediate for NewPerson {
    type Intermediate = intermediate::NewPerson;
    fn into_intermediate(&self) -> Self::Intermediate {
        Self::Intermediate {
            name: self.name.into_intermediate(),
            birthday: self.birthday.into_intermediate(),
            address: self.address.into_intermediate(),
            comment: self.comment.into_intermediate(),
            flags: self.flags.into_intermediate(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewItem {
    pub article_id: String,
    pub comment: String,
}

impl_intermediate!(NewItem);

#[derive(Deserialize, Debug)]
pub struct NewName {
    pub title: String,
    pub firstname: String,
    pub lastname: String,
}

impl_intermediate!(NewName => "{}\t{}\t{}", title, firstname, lastname);

#[derive(Deserialize, Debug)]
pub struct NewAddress {
    pub street: String,
    pub zip: String,
    pub city: String,
}

impl_intermediate!(NewAddress => "{}\n{} {}", street, zip, city);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewPersonFlags {
    pub vegetarian: bool,
    pub team: bool,
}

impl_intermediate!(NewPersonFlags);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmergencyContact {
    pub name: String,
    pub phone: String,
}

impl_intermediate!(EmergencyContact);
