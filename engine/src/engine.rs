use std::collections::HashMap;

use order_book::{
    order::{Order, OrderReq},
    order_book::{OrderBook, OrderBookErrors},
};

use crate::events::{EngineError, EngineEvent, Event};

pub struct Engine {
    books: HashMap<String, OrderBook>,
}

impl Engine {
    pub fn new() -> Self {
        return Engine {
            books: HashMap::new(),
        };
    }

    pub fn new_book(&mut self, symbol: String) {
        self.books.insert(symbol, OrderBook::new());
    }

    fn get_book(&mut self, symbol: &String) -> Result<&mut OrderBook, EngineError> {
        match self.books.get_mut(symbol) {
            Some(book) => return Ok(book),
            None => return Err(EngineError::InvalidBook),
        }
    }

    pub fn handle_event(&mut self, event: Event) -> Result<Vec<EngineEvent>, EngineError> {
        match event {
            Event::NewOrder { symbol, order_req } => self.new_order(symbol, order_req),
            Event::CancelOrder { symbol, order_id } => self.cancel_order(symbol, order_id),
            Event::GetSnapshot { symbol, depth } => self.get_snapshot(symbol, depth),
        }
    }

    fn new_order(
        &mut self,
        symbol: String,
        order_req: OrderReq,
    ) -> Result<Vec<EngineEvent>, EngineError> {
        let mut events: Vec<EngineEvent> = Vec::new();

        let ob = self.get_book(&symbol)?;
        let (order_id, trades) = ob.submit_order(&order_req);

        events.push(EngineEvent::OrderAccepted { order_id });

        if let Some(trades) = trades {
            for trade in trades.iter() {
                events.push(EngineEvent::Trade {
                    maker_order_id: trade.maker,
                    taker_order_id: trade.taker,
                    price: trade.price.clone(),
                    quantity: trade.quantity,
                });

                let maker = ob.get_order(&trade.maker);
                let taker = ob.get_order(&trade.taker);

                match maker {
                    Ok(order) => {
                        events.push(EngineEvent::OrderPartiallyFilled {
                            order_id: order.id,
                            remaining: trade.quantity - order.order.quantity,
                        });
                    }
                    Err(_) => {
                        events.push(EngineEvent::OrderFilled {
                            order_id: trade.maker,
                        });
                    }
                }

                match taker {
                    Ok(order) => {
                        events.push(EngineEvent::OrderPartiallyFilled {
                            order_id: order.id,
                            remaining: trade.quantity - order.order.quantity,
                        });
                    }
                    Err(_) => {
                        events.push(EngineEvent::OrderFilled { order_id: trade.taker });
                    }
                }
            }
        }

        return Ok(events);
    }

    fn get_snapshot(
        &mut self,
        symbol: String,
        depth: Option<usize>,
    ) -> Result<Vec<EngineEvent>, EngineError> {
        let mut events: Vec<EngineEvent> = Vec::new();
        let ob = self.get_book(&symbol)?;

        let snapshot = ob.snapshot(depth);
        events.push(EngineEvent::BookSnapshot { snapshot });

        return Ok(events);
    }

    fn cancel_order(
        &mut self,
        symbol: String,
        order_id: u32,
    ) -> Result<Vec<EngineEvent>, EngineError> {
        let mut events: Vec<EngineEvent> = Vec::new();
        let ob = self.get_book(&symbol)?;

        match ob.cancel_order(&order_id) {
            Ok(_) => events.push(EngineEvent::OrderCancelled { order_id }),
            Err(err) => return Err(EngineError::OrderBookError(err)),
        };

        return Ok(events);
    }
}
