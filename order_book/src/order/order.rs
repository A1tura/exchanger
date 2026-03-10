use crate::order::Price;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Side {
    Bid = 1,
    Ask = 2,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Type {
    Limit,
    Market
}

#[derive(Clone, Debug)]
pub struct OrderReq {
    pub client_id: u32,
    pub order_type: Type,
    pub side: Side,

    pub price: Price,

    pub quantity: u32,
}

#[derive(Clone, Debug)]
pub struct Order {
    pub id: u32,
    pub order: OrderReq,
}

impl OrderReq {
    pub fn new(client_id: u32,order_type: Type, side: Side, price: f64, quantity: u32) -> OrderReq {
        OrderReq {
            client_id,
            order_type,
            side,
            price: Price::from(price),
            quantity
        }
    }
}

impl Order {
    pub fn new(order_req: OrderReq) -> Order {
        Order { id: rand::random::<u32>(), order: order_req }
    }
}
