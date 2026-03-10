use std::collections::HashMap;

use bytes::{BufMut, BytesMut};
use order_book::{
    order::{OrderReq, Price, Side},
    order_book::OrderBook,
};

use crate::events::{EngineError, EngineEvent, Event};

pub struct Engine {
    books: HashMap<u32, OrderBook>,
    symbols: HashMap<u32, String>,
}

fn ticker_to_bytes(ticker: &str) -> [u8; 16] {
    let mut buf = [0u8; 16];

    for (i, byte) in ticker.as_bytes().iter().enumerate().take(8) {
        buf[i] = *byte;
    }

    buf
}

impl Engine {
    pub fn new() -> Self {
        return Engine {
            books: HashMap::new(),
            symbols: HashMap::new(),
        };
    }

    pub fn new_book(&mut self, ticker: &str) -> EngineEvent {
        let symbol_id = (self.symbols.len() + 1) as u32;
        self.symbols.insert(symbol_id, ticker.to_string());
        self.books.insert(symbol_id, OrderBook::new());

        return EngineEvent::NewSymbol { ticker: ticker_to_bytes(ticker), symbol_id }
    }

    pub fn get_symbols(&self) -> HashMap<u32, String> {
        return self.symbols.clone();
    }

    fn get_book(&mut self, symbol_id: u32) -> Result<&mut OrderBook, EngineError> {
        match self.books.get_mut(&symbol_id) {
            Some(book) => return Ok(book),
            None => return Err(EngineError::InvalidBook),
        }
    }

    pub fn handle_event(&mut self, event: Event) -> Result<Vec<EngineEvent>, EngineError> {
        match event {
            Event::NewOrder {
                symbol_id,
                order_req,
            } => self.new_order(symbol_id, order_req),
            Event::CancelOrder {
                symbol_id,
                order_id,
                client_id,
            } => self.cancel_order(symbol_id, order_id, client_id),
            Event::GetSnapshot {
                symbol_id,
                depth,
                client_id,
            } => self.get_snapshot(symbol_id, depth, client_id),
        }
    }

    fn new_order(
        &mut self,
        symbol_id: u32,
        order_req: OrderReq,
    ) -> Result<Vec<EngineEvent>, EngineError> {
        let mut events: Vec<EngineEvent> = Vec::new();

        let ob = self.get_book(symbol_id)?;
        let (order_id, trades) = ob.submit_order(&order_req);

        events.push(EngineEvent::OrderAccepted {
            symbol_id,
            client_id: order_req.client_id,
            order_id,
        });

        let create_price_level = |side: Side, price: Price, quantity: u32| -> EngineEvent {
            return EngineEvent::PriceLevel {
                symbol_id,
                side: side as u8,
                price: price.as_float(),
                quantity,
            };
        };

        if let Some(trades) = trades {
            for trade in trades.iter() {
                events.push(EngineEvent::Trade {
                    symbol_id,
                    maker_client_id: trade.maker_client_id,
                    maker_order_id: trade.maker,
                    taker_client_id: trade.taker_client_id,
                    taker_order_id: trade.taker,
                    price: trade.price.clone(),
                    quantity: trade.quantity,
                });

                let maker = ob.get_order(&trade.maker);
                let taker = ob.get_order(&trade.taker);

                match maker {
                    Ok(order) => {
                        events.push(EngineEvent::OrderPartiallyFilled {
                            symbol_id,
                            client_id: order.order.client_id,
                            order_id: order.id,
                            remaining: trade.quantity - order.order.quantity,
                        });
                    }
                    Err(_) => {
                        events.push(EngineEvent::OrderFilled {
                            symbol_id,
                            client_id: trade.maker_client_id,
                            order_id: trade.maker,
                        });
                    }
                }

                match taker {
                    Ok(order) => {
                        events.push(EngineEvent::OrderPartiallyFilled {
                            symbol_id,
                            client_id: order.order.client_id,
                            order_id: order.id,
                            remaining: trade.quantity - order.order.quantity,
                        });
                    }
                    Err(_) => {
                        events.push(EngineEvent::OrderFilled {
                            symbol_id,
                            client_id: order_req.client_id,
                            order_id: trade.taker,
                        });
                    }
                }
            }

            let price_level_ask = ob.get_price_level(&Side::Ask, &order_req.price);
            let price_level_bid = ob.get_price_level(&Side::Bid, &order_req.price);
            if let Some(price_level_ask) = price_level_ask {
                events.push(create_price_level(
                    Side::Ask,
                    order_req.price.clone(),
                    price_level_ask.total_quantity,
                ));
            } else {
                events.push(create_price_level(Side::Ask, order_req.price.clone(), 0));
            }

            if let Some(price_level_bid) = price_level_bid {
                events.push(create_price_level(
                    Side::Bid,
                    order_req.price.clone(),
                    price_level_bid.total_quantity,
                ));
            } else {
                events.push(create_price_level(Side::Bid, order_req.price.clone(), 0));
            }
        } else {
            let price_level = ob.get_price_level(&order_req.side, &order_req.price);
            if let Some(price_level) = price_level {
                events.push(create_price_level(order_req.side, order_req.price.clone(), price_level.total_quantity));
            }
        }

        return Ok(events);
    }

    fn get_snapshot(
        &mut self,
        symbol_id: u32,
        depth: Option<usize>,
        client_id: u32,
    ) -> Result<Vec<EngineEvent>, EngineError> {
        let mut events: Vec<EngineEvent> = Vec::new();
        let ob = self.get_book(symbol_id)?;

        let snapshot = ob.snapshot(depth);
        events.push(EngineEvent::BookSnapshot {
            symbol_id,
            client_id,
            snapshot,
        });

        return Ok(events);
    }

    fn cancel_order(
        &mut self,
        symbol_id: u32,
        order_id: u32,
        client_id: u32,
    ) -> Result<Vec<EngineEvent>, EngineError> {
        let mut events: Vec<EngineEvent> = Vec::new();
        let ob = self.get_book(symbol_id)?;
        let order = ob.get_order(&order_id).unwrap();

        match ob.cancel_order(&order_id) {
            Ok(_) => events.push(EngineEvent::OrderCancelled {
                symbol_id,
                client_id,
                order_id,
            }),
            Err(err) => return Err(EngineError::OrderBookError(err)),
        };

        let price_level = ob.get_price_level(&order.order.side, &order.order.price);
        if let Some(level) = price_level {
            let price_level_event = EngineEvent::PriceLevel {
                symbol_id,
                side: order.order.side as u8,
                price: order.order.price.as_float(),
                quantity: level.total_quantity,
            };
            events.push(price_level_event);
        }

        return Ok(events);
    }
}
