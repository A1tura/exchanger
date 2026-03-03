use rand::random;

use crate::order::Price;

#[derive(Clone, PartialEq, Eq)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    Limit,
    Market
}

#[derive(Clone)]
pub struct Order {
    pub id: u32,

    pub order_type: Type,
    pub side: Side,

    pub price: Price,

    pub quantity: u32,
}

impl Order {
    pub fn new(order_type: Type, side: Side, price: f64, quantity: u32) -> Order {
        Order {
            id: random::<u32>(),
            order_type,
            side,
            price: Price::from(price),
            quantity
        }
    }
}
