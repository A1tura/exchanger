const SCALE: i32 = 8;

#[derive(Hash, Eq, PartialEq, Clone, Ord, PartialOrd, Debug)]
pub struct Price {
    pub price: u64,
}

impl Price {
    pub fn from(price: f64) -> Price {
        let price = (price * 10.0_f64.powf(SCALE as f64)) as u64;
        return Price { price };
    }

    pub fn as_float(&self) -> f64 {
        self.price as f64 / 10_f64.powf(SCALE as f64)
    }
}
