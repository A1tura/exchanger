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

    pub fn handle_event(&mut self, event: Event) -> Result<EngineEvent, EngineError> {
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
    ) -> Result<EngineEvent, EngineError> {
        let ob = self.get_book(&symbol)?;
        let order_id = ob.submit_order(&order_req);
        return Ok(EngineEvent::OrderAccepted { order_id });
    }

    fn get_snapshot(
        &mut self,
        symbol: String,
        depth: Option<usize>,
    ) -> Result<EngineEvent, EngineError> {
        let ob = self.get_book(&symbol)?;

        let snapshot = ob.snapshot(depth);

        return Ok(EngineEvent::BookSnapshot { snapshot });
    }

    fn cancel_order(&mut self, symbol: String, order_id: u32) -> Result<EngineEvent, EngineError> {
        let ob = self.get_book(&symbol)?;

        match ob.cancel_order(&order_id) {
            Ok(_) => return Ok(EngineEvent::OrderCancelled { order_id }),
            Err(err) => return Err(EngineError::OrderBookError(err)),
        }
    }
}
