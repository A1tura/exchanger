use std::{
    collections::{BTreeMap, HashMap},
    fmt::{self, Display},
};

use crate::order::{Order, OrderReq, Price, Side, Type};
use crate::order_book::OrderBookErrors;

#[derive(Debug)]
pub struct Level {
    pub price: f64,
    pub total_quantity: u32,
}

#[derive(Debug)]
pub struct Snapshot {
    pub bids: Vec<Level>,
    pub asks: Vec<Level>,
}

#[derive(Debug)]
pub struct OrderBook {
    pub bids: BTreeMap<Price, Vec<u32>>,
    pub asks: BTreeMap<Price, Vec<u32>>,

    pub orders: HashMap<u32, Order>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),

            orders: HashMap::new(),
        }
    }

    pub fn snapshot(&self, depth: Option<usize>) -> Snapshot {
        let mut snapshot = Snapshot {
            bids: Vec::new(),
            asks: Vec::new(),
        };

        let d = match depth {
            Some(d) => d,
            None => std::cmp::max(self.bids.len(), self.asks.len()),
        };

        let bids_depth = std::cmp::min(self.bids.len(), d);
        let asks_depth = std::cmp::min(self.asks.len(), d);

        for (i, price) in self.bids.keys().enumerate() {
            if i > bids_depth {
                break;
            }

            let mut level = Level {
                price: price.as_float(),
                total_quantity: 0,
            };

            let orders = self.bids.get(&price).unwrap();
            for order_id in orders.iter() {
                for order in self.orders.get(order_id) {
                    level.total_quantity += order.order.quantity;
                }
            }

            snapshot.bids.push(level);
        }

        for (i, price) in self.asks.keys().enumerate() {
            if i > bids_depth {
                break;
            }

            let mut level = Level {
                price: price.as_float(),
                total_quantity: 0,
            };

            let orders = self.asks.get(&price).unwrap();
            for order_id in orders.iter() {
                for order in self.orders.get(order_id) {
                    level.total_quantity += order.order.quantity;
                }
            }

            snapshot.asks.push(level);
        }

        return snapshot;
    }

    pub fn submit_order(&mut self, orderReq: &OrderReq) -> u32 {
        let mut order = Order::new(orderReq.clone());
        match orderReq.order_type {
            Type::Limit => self.execute_limit(&mut order),
            Type::Market => self.match_order(&mut order),
        }

        return order.id;
    }

    pub fn cancel_order(&mut self, order_id: &u32) -> Result<(), OrderBookErrors> {
        match self.orders.get_mut(order_id) {
            Some(order) => {
                match order.order.side {
                    Side::Bid => {
                        if self.bids.get(&order.order.price).unwrap().len() == 1 {
                            self.bids.remove(&order.order.price);
                            return Ok(());
                        }

                        let book = self.bids.get_mut(&order.order.price).unwrap();

                        for i in 0..book.len() {
                            if book[i] == *order_id {
                                book.remove(i);
                            }
                        }
                    }
                    Side::Ask => {
                        if self.asks.get(&order.order.price).unwrap().len() == 1 {
                            self.asks.remove(&order.order.price);
                            return Ok(());
                        }

                        let book = self.asks.get_mut(&order.order.price).unwrap();
                        for i in 0..book.len() {
                            if book[i] == *order_id {
                                book.remove(i);
                            }
                        }
                    }
                }

                self.orders.remove(order_id);
            }
            None => return Err(OrderBookErrors::OrderNotExist),
        };
        return Ok(());
    }

    fn execute_limit(&mut self, order: &mut Order) {
        if (self.asks.len() >= 1 && order.order.side == Side::Bid)
            || (self.bids.len() >= 1 && order.order.side == Side::Ask)
        {
            self.match_order(order);
        }

        if order.order.quantity == 0 {
            return;
        }

        self.orders.insert(order.id, order.clone());
        match &order.order.side {
            Side::Bid => {
                if self.bids.contains_key(&order.order.price) {
                    self.bids
                        .get_mut(&order.order.price)
                        .unwrap()
                        .push(order.id);
                } else {
                    let mut orders: Vec<u32> = Vec::new();
                    orders.push(order.id);
                    self.bids.insert(order.order.price.clone(), orders);
                }
            }
            Side::Ask => {
                if self.asks.contains_key(&order.order.price) {
                    self.asks
                        .get_mut(&order.order.price)
                        .unwrap()
                        .push(order.id);
                } else {
                    let mut orders: Vec<u32> = Vec::new();
                    orders.push(order.id);
                    self.asks.insert(order.order.price.clone(), orders);
                }
            }
        }
    }

    fn match_order(&mut self, order: &mut Order) {
        match &order.order.side {
            Side::Bid => {
                let best_ask_id = self.get_best_ask();
                let mut best_ask = self.orders.get_mut(&best_ask_id).unwrap();
                if (order.order.order_type == Type::Limit
                    && best_ask.order.price <= order.order.price)
                    || order.order.order_type == Type::Market
                {
                    if best_ask.order.quantity > 0 {
                        if best_ask.order.quantity > order.order.quantity {
                            best_ask.order.quantity -= order.order.quantity;
                            order.order.quantity = 0;
                        } else {
                            order.order.quantity -= best_ask.order.quantity;
                            self.asks.pop_first();
                            self.remove_order(&order);
                        }
                    }
                }
            }
            Side::Ask => {
                while order.order.quantity != 0 && !self.bids.is_empty() {
                    let best_bid_id = self.get_best_bid();
                    let mut best_bid = self.orders.get_mut(&best_bid_id).unwrap();
                    if (order.order.order_type == Type::Limit
                        && best_bid.order.price >= order.order.price)
                        || order.order.order_type == Type::Market
                    {
                        if best_bid.order.quantity > 0 {
                            if best_bid.order.quantity > order.order.quantity {
                                best_bid.order.quantity -= order.order.quantity;
                                order.order.quantity = 0;
                            } else {
                                order.order.quantity -= best_bid.order.quantity;
                                self.bids.pop_last();
                                self.remove_order(&order);
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn remove_order(&mut self, order: &Order) {
        self.orders.remove(&order.id);
    }

    pub fn get_best_bid(&self) -> u32 {
        if self.bids.len() >= 1 {
            return *self
                .bids
                .iter()
                .next_back()
                .unwrap()
                .1
                .iter()
                .next()
                .unwrap();
        }
        return 0;
    }

    pub fn get_best_ask(&self) -> u32 {
        if self.asks.len() >= 1 {
            return *self.asks.iter().next().unwrap().1.iter().next().unwrap();
        }
        return 0;
    }
}

impl Display for OrderBook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "Bids: \n");
        for (price, orders) in &self.bids {
            let _ = write!(f, "\t{}:\n", price.as_float());
            for order in orders {
                let order = self.orders.get(order).unwrap();
                let _ = write!(
                    f,
                    "\t\tid: {}; quantity: {}\n",
                    order.id, order.order.quantity
                );
            }
        }

        let _ = write!(f, "Asks: \n");
        for (price, orders) in &self.asks {
            let _ = write!(f, "\t{}:\n", price.as_float());
            for order in orders {
                let order = self.orders.get(order).unwrap();
                let _ = write!(
                    f,
                    "\t\tid: {}; quantity: {}\n",
                    order.id, order.order.quantity
                );
            }
        }

        return Ok(());
    }
}
