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
