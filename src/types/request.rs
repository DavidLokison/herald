use serde::{Serialize, Deserialize};
use time::Date;

use super::intermediate::{self, IntoIntermediate};
use crate::impl_intermediate;

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceCheck {
    pub birthday: Date,
    pub team: bool,
}

impl_intermediate!(PriceCheck => &);

#[derive(Deserialize, Debug)]
pub struct NewRegistration<'r> {
    pub name: NewName<'r>,
    pub address: NewAddress<'r>,
    pub phone: &'r str,
    pub email: &'r str,
    pub comment: &'r str,
    pub emergency: EmergencyContact<'r>,
    pub persons: Vec<NewPerson<'r>>,
    pub items: Vec<NewItem<'r>>,
}

impl<'r> IntoIntermediate<'r> for NewRegistration<'r> {
    type Intermediate = intermediate::NewRegistration<'r>;
    fn into_intermediate(&'r self) -> Self::Intermediate {
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
pub struct NewPerson<'r> {
    pub name: NewName<'r>,
    pub address: NewAddress<'r>,
    pub birthday: Date,
    pub comment: &'r str,
    pub flags: NewPersonFlags,
}

impl<'r> IntoIntermediate<'r> for NewPerson<'r> {
    type Intermediate = intermediate::NewPerson<'r>;
    fn into_intermediate(&'r self) -> Self::Intermediate {
        Self::Intermediate {
            name: self.name.into_intermediate(),
            birthday: self.birthday.into_intermediate(),
            address: self.address.into_intermediate(),
            comment: self.comment.into_intermediate(),
            flags: self.flags.into_intermediate(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewItem<'r> {
    pub article_id: &'r str,
    pub comment: &'r str,
}

impl_intermediate!(NewItem<'r> => &);

#[derive(Deserialize, Debug)]
pub struct NewName<'r> {
    pub title: &'r str,
    pub firstname: &'r str,
    pub lastname: &'r str,
}

impl_intermediate!(NewName<'r> => "{}\t{}\t{}", title, firstname, lastname);

#[derive(Deserialize, Debug)]
pub struct NewAddress<'r> {
    pub street: &'r str,
    pub zip: &'r str,
    pub city: &'r str,
}

impl_intermediate!(NewAddress<'r> => "{}\n{} {}", street, zip, city);

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPersonFlags {
    pub vegetarian: bool,
    pub team: bool,
}

impl_intermediate!(NewPersonFlags => &);

#[derive(Serialize, Deserialize, Debug)]
pub struct EmergencyContact<'r> {
    pub name: &'r str,
    pub phone: &'r str,
}

impl_intermediate!(EmergencyContact<'r> => &);
