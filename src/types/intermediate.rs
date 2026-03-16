use serde::Serialize;
use time::Date;

use super::request;
use super::response;

#[macro_export]
macro_rules! impl_intermediate {
    ($T:ident) => {
        impl IntoIntermediate<'_> for $T {
            type Intermediate = Self;
            #[inline]
            fn into_intermediate(&self) -> Self::Intermediate {
                (*self).clone()
            }
            fn as_json(&self) -> String {
                rocket::serde::json::to_string(self).unwrap()
            }
        }
    };
    (&$T:ident) => {
        impl<'s> IntoIntermediate<'s> for &$T {
            type Intermediate = &'s $T;
            #[inline]
            fn into_intermediate(&'s self) -> Self::Intermediate {
                self
            }
        }
    };
    ($T:ident => &) => {
        impl<'s> IntoIntermediate<'s> for $T {
            type Intermediate = &'s Self;
            #[inline]
            fn into_intermediate(&'s self) -> Self::Intermediate {
                self
            }
        }
    };
    ($T:ident<$r:lifetime> => &) => {
        impl<$r> IntoIntermediate<$r> for $T<$r> {
            type Intermediate = &$r Self;
            #[inline]
            fn into_intermediate(&$r self) -> Self::Intermediate {
                self
            }
        }
    };
    ($T:ident$(<$r:lifetime>)? => $str:literal, $($args:ident),*) => {
        impl$(<$r>)? IntoIntermediate<'_> for $T$(<$r>)? {
            type Intermediate = String;
            #[inline]
            fn into_intermediate(&self) -> Self::Intermediate {
                format!($str, $(self.$args),*)
            }
        }
    };
}

pub trait IntoIntermediate<'s> {
    type Intermediate: Serialize;
    fn into_intermediate(&'s self) -> Self::Intermediate;
    fn as_json(&'s self) -> String {
        rocket::serde::json::to_string(&self.into_intermediate()).unwrap()
    }
}

impl_intermediate!(bool);
impl_intermediate!(Date => &);
impl_intermediate!(&str);

impl<'s, T> IntoIntermediate<'s> for Vec<T> where T: IntoIntermediate<'s> {
    type Intermediate = Vec<<T as IntoIntermediate<'s>>::Intermediate>;
    fn into_intermediate(&'s self) -> Self::Intermediate {
        self.iter().map(<T as IntoIntermediate>::into_intermediate).collect()
    }
}

#[derive(Serialize)]
pub struct NewPerson<'r> {
    pub name: String,
    pub birthday: &'r Date,
    pub address: String,
    pub comment: &'r str,
    pub flags: &'r request::NewPersonFlags,
}

#[derive(Serialize)]
pub struct NewRegistration<'r> {
    pub name: String,
    pub address: String,
    pub phone: &'r str,
    pub email: &'r str,
    pub comment: &'r str,
    pub emergency: &'r request::EmergencyContact<'r>,
}



pub struct Article {
    pub id: String,
    pub description: String,
    pub price: u32,
}

impl Article {
    pub fn with_price_info(self, decimals: u8, currency: String) -> response::Article {
        response::Article {
            id: self.id,
            description: self.description,
            price: response::Price {
                value: self.price,
                decimals: decimals,
                currency: currency,
            }
        }
    }
}
