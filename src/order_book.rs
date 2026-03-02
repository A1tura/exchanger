use std::collections::HashMap;

use crate::order::{Order, Side};

pub struct OrderBook {
    pub bids: HashMap<f32, Vec<Order>>,
    pub asks: HashMap<f32, Vec<Order>>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            bids: HashMap::new(),
            asks: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        match order.side {
            Side::Bid => {
                todo!("Add bid");
            },
            Side::Ask => {
                todo!("Add ask");
            }
        }
    }
}
