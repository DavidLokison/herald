use serde::Serialize;
use time::Date;

#[macro_export]
macro_rules! impl_intermediate {
    ($T:ty) => {
        impl IntoIntermediate for $T {
            type Intermediate = Self;
            fn into_intermediate(&self) -> Self::Intermediate {
                (*self).clone()
            }
            fn as_json(&self) -> String {
                rocket::serde::json::to_string(self).unwrap()
            }
        }
    };
    ($T:ty => $str:literal, $($args:ident),*) => {
        impl IntoIntermediate for $T {
            type Intermediate = String;
            fn into_intermediate(&self) -> Self::Intermediate {
                format!($str, $(self.$args),*)
            }
        }
    };
}

pub trait IntoIntermediate {
    type Intermediate: Serialize;
    fn into_intermediate(&self) -> Self::Intermediate;
    fn as_json(&self) -> String {
        rocket::serde::json::to_string(&self.into_intermediate()).unwrap()
    }
}

impl_intermediate!(String);
impl_intermediate!(Date);
impl_intermediate!(bool);

impl<T> IntoIntermediate for Vec<T> where T: IntoIntermediate {
    type Intermediate = Vec<<T as IntoIntermediate>::Intermediate>;
    fn into_intermediate(&self) -> Self::Intermediate {
        self.iter().map(<T as IntoIntermediate>::into_intermediate).collect()
    }
}

pub struct Article {
    pub id: String,
    pub description: String,
    pub price: u32,
}

impl Article {
    pub fn with_price_info(self, decimals: u8, currency: String) -> super::Article {
        super::Article {
            id: self.id,
            description: self.description,
            price: super::Price {
                value: self.price,
                decimals: decimals,
                currency: currency,
            }
        }
    }
}
