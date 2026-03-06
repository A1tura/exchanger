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
                symbol: "INTC".to_string(),
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
                symbol: "INTC".to_string(),
                order_req: OrderReq::new(
                    client_id,
                    order_book::order::Type::Market,
                    side,
                    500.00,
                    order.quantity,
                ),
            };
        }
        Message::CancelOrder(order_info) => {
            // TODO: fix hardcored symbol
            return Event::CancelOrder {
                symbol: "INTC".to_string(),
                order_id: order_info.order_id,
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
            client_id: _,
            order_id,
        } => {
            engine_messages::OrderAccepted {
                order_id: *order_id,
            }
            .encode(&mut buf);
            return (buf, MessageType::OrderAccepted);
        }
        EngineEvent::OrderCancelled {
            client_id: _,
            order_id,
        } => {
            engine_messages::OrderCanceled {
                order_id: *order_id,
            }
            .encode(&mut buf);
            return (buf, MessageType::OrderCanceled);
        }
        EngineEvent::OrderPartiallyFilled {
            client_id: _,
            order_id,
            remaining,
        } => {
            engine_messages::OrderPartiallyFilled {
                order_id: *order_id,
                remaining: *remaining,
            }
            .encode(&mut buf);
            return (buf, MessageType::OrderPartiallyFilled);
        }
        EngineEvent::OrderFilled {
            client_id: _,
            order_id,
        } => {
            engine_messages::OrderFilled {
                order_id: *order_id,
            }
            .encode(&mut buf);

            return (buf, MessageType::OrderFilled);
        }
        EngineEvent::Trade {
            maker_client_id,
            maker_order_id,
            taker_client_id,
            taker_order_id,
            price,
            quantity,
        } => {
            engine_messages::Trade {
                maker_client_id: *maker_client_id,
                maker_order_id: *maker_order_id,
                taker_client_id: *taker_client_id,
                taker_order_id: *taker_order_id,
                price: price.as_float(),
                quantity: *quantity,
            }
            .encode(&mut buf);

            return (buf, MessageType::Trade);
        }
        _ => todo!(),
    }
}
