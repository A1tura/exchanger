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

        match engine
            .handle_event(Event::NewOrder {
                symbol: "SYMBL".to_string(),
                order_req: OrderReq::new(Type::Limit, Side::Ask, 10.00, 100),
            })
            .unwrap()
        {
            EngineEvent::OrderAccepted { order_id } => order_id,
            _ => return,
        };

        let snapshot = match engine
            .handle_event(Event::GetSnapshot {
                symbol: "SYMBL".to_string(),
                depth: None,
            })
            .unwrap()
        {
            EngineEvent::BookSnapshot { snapshot } => snapshot,
            _ => return,
        };

        assert!(snapshot.bids.is_empty());
        assert_eq!(snapshot.asks.len(), 1);

        assert_eq!(snapshot.asks[0].total_quantity, 100);
    }
}

