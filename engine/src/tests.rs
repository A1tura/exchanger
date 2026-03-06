#[cfg(test)]
mod tests {
    use order_book::order::{OrderReq, Side, Type};

    use crate::{
        engine::Engine,
        events::{EngineEvent, Event},
    };

    #[test]
    fn engine_generates_trade_events() {
        let mut engine = Engine::new();
        engine.new_book("SYMBL".to_string());

        let ask_events = engine
            .handle_event(Event::NewOrder {
                symbol: "SYMBL".to_string(),
                order_req: OrderReq::new(1, Type::Limit, Side::Ask, 10.00, 100),
            })
            .unwrap();

        let bid_events = engine
            .handle_event(Event::NewOrder {
                symbol: "SYMBL".to_string(),
                order_req: OrderReq::new(2, Type::Limit, Side::Bid, 10.00, 100),
            })
            .unwrap();

        let maker_id = match ask_events[0] {
            EngineEvent::OrderAccepted { order_id, .. } => order_id,
            _ => panic!("Expected OrderAccepted"),
        };

        let taker_id = match bid_events[0] {
            EngineEvent::OrderAccepted { order_id, .. } => order_id,
            _ => panic!("Expected OrderAccepted"),
        };

        match &bid_events[1] {
            EngineEvent::Trade { maker_order_id, taker_order_id, price, quantity, maker_client_id, taker_client_id }  => {
                assert_eq!(*maker_client_id, 1);
                assert_eq!(*taker_client_id, 2);

                assert_eq!(*maker_order_id, maker_id);
                assert_eq!(*taker_order_id, taker_id);
                assert_eq!(price.as_float(), 10.00);
                assert_eq!(*quantity, 100)
            },
            _ => panic!("Expected Trade"),
        }

        match &bid_events[2] {
            EngineEvent::OrderFilled { client_id, order_id } => {
                assert_eq!(*client_id, 1);
                assert_eq!(*order_id, maker_id);
            },
            _ => panic!("Expected OrderFilled"),
        }


        match &bid_events[3] {
            EngineEvent::OrderFilled { client_id, order_id } => {
                assert_eq!(*client_id, 2);
                assert_eq!(*order_id, taker_id);
            },
            _ => panic!("Expected OrderFilled"),
        }
    }
}
