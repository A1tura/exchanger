use std::any::TypeId;

use bytes::BytesMut;
use engine::events::{EngineEvent, Event};
use order_book::order::{OrderReq, Price, Side};
use protocol::messages::engine_messages;
use protocol::traits::Encode;
use protocol::{Message, MessageType};

pub fn message_as_event(client_id: u32, message: &Message) -> Event {
    match message {
        Message::CreateLimitOrder(order) => {
            let side = if order.side == 1 {
                Side::Ask
            } else {
                Side::Bid
            };
            // TODO: fix hardcored symbol
            return Event::NewOrder {
                symbol_id: order.symbol,
                order_req: OrderReq::new(
                    client_id,
                    order_book::order::Type::Limit,
                    side,
                    order.price,
                    order.quantity,
                ),
            };
        }
        Message::CreateMarketOrder(order) => {
            let side = if order.side == 1 {
                Side::Ask
            } else {
                Side::Bid
            };
            // TODO: fix hardcored symbol
            return Event::NewOrder {
                symbol_id: order.symbol,
                order_req: OrderReq::new(
                    client_id,
                    order_book::order::Type::Market,
                    side,
                    500.00,
                    order.quantity,
                ),
            };
        }
        Message::CancelOrder(order) => {
            // TODO: fix hardcored symbol
            return Event::CancelOrder {
                symbol_id: order.symbol,
                order_id: order.order_id,
                client_id: client_id,
            };
        }
        _ => todo!(),
    }
}

pub fn engine_event_as_bytes(engine_event: &EngineEvent) -> (BytesMut, MessageType) {
    let mut buf = BytesMut::new();
    match engine_event {
        EngineEvent::OrderAccepted {
            symbol_id,
            client_id: _,
            order_id,
        } => {
            engine_messages::OrderAccepted {
                symbol_id: *symbol_id,
                order_id: *order_id,
            }
            .encode(&mut buf);
            return (buf, MessageType::OrderAccepted);
        }
        EngineEvent::OrderCancelled {
            symbol_id,
            client_id: _,
            order_id,
        } => {
            engine_messages::OrderCanceled {
                symbol_id: *symbol_id,
                order_id: *order_id,
            }
            .encode(&mut buf);
            return (buf, MessageType::OrderCanceled);
        }
        EngineEvent::OrderPartiallyFilled {
            symbol_id,
            client_id: _,
            order_id,
            remaining,
        } => {
            engine_messages::OrderPartiallyFilled {
                symbol_id: *symbol_id,
                order_id: *order_id,
                remaining: *remaining,
            }
            .encode(&mut buf);
            return (buf, MessageType::OrderPartiallyFilled);
        }
        EngineEvent::OrderFilled {
            symbol_id,
            client_id: _,
            order_id,
        } => {
            engine_messages::OrderFilled {
                symbol_id: *symbol_id,
                order_id: *order_id,
            }
            .encode(&mut buf);

            return (buf, MessageType::OrderFilled);
        }
        EngineEvent::Trade {
            symbol_id,
            maker_client_id: _,
            maker_order_id,
            taker_client_id: _,
            taker_order_id,
            price,
            quantity,
        } => {
            engine_messages::Trade {
                symbol_id: *symbol_id,
                maker_order_id: *maker_order_id,
                taker_order_id: *taker_order_id,
                price: price.as_float(),
                quantity: *quantity,
            }
            .encode(&mut buf);

            return (buf, MessageType::Trade);
        }
        EngineEvent::PriceLevel {
            symbol_id,
            side,
            price,
            quantity,
        } => {
            engine_messages::PriceLevel {
                symbol_id: *symbol_id,
                side: *side,
                price: *price,
                quantity: *quantity,
            }
            .encode(&mut buf);
            return (buf, MessageType::PriceLevel);
        }
        EngineEvent::NewSymbol {
            ticker,
            symbol_id,
        } => {
            engine_messages::NewSymbol {
                ticker: *ticker,
                symbol_id: *symbol_id,
            }
            .encode(&mut buf);
            return (buf, MessageType::NewSymbol);
        }
        _ => todo!(),
    }
}
