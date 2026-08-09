use serde::Serialize;
use time::Date;

use super::request;
use super::response;

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
pub(crate) use impl_intermediate;

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
    pub food_options: u8,
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



pub struct PricePreview {
    pub description: String,
    pub price: i32,
}

impl PricePreview {
    pub fn with_price_info(self, decimals: u8, currency: String) -> response::Price {
        response::Price {
            description: self.description,
            value: self.price,
            decimals: decimals,
            currency: currency,
        }
    }
}

pub struct Article {
    pub id: String,
    pub description: String,
    pub price: i32,
}

impl Article {
    pub fn with_price_info(self, decimals: u8, currency: String) -> response::Article {
        response::Article {
            id: self.id,
            price: response::Price {
                description: self.description,
                value: self.price,
                decimals: decimals,
                currency: currency,
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::IntoIntermediate;

    #[test]
    fn intermediate_borrow_date() {
        use time::macros::date;
        let date = date!(2022-04-19);
        let date_int = date.into_intermediate();
        assert!(std::ptr::eq(&date, date_int));
    }

    #[test]
    fn intermediate_borrow_str() {
        let s = "Hello World";
        let s_int = s.into_intermediate();
        assert!(std::ptr::eq(s, s_int));
    }
}
