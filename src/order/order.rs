use rand::random;

pub enum Side {
    Bid,
    Ask,
}

pub struct Order {
    pub id: u32,
    pub side: Side,
    pub price: f32,
    pub quantity: u32,
}

impl Order {
    pub fn new(side: Side, price: f32, quantity: u32) -> Order {
        Order {
            id: random::<u32>(),
            side,
            price,
            quantity
        }
    }
}
