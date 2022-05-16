use futures::StreamExt;
use iced_futures::futures;
use iced_native::subscription::{self, Subscription};
use models::Breaks;
use reqwest_eventsource::{Event as EsEvent, EventSource};

pub fn connect() -> Subscription<Breaks> {
    struct Connect;

    subscription::unfold(
        std::any::TypeId::of::<Connect>(),
        SubscriptionState::NotConnected,
        |state| async move {
            match state {
                SubscriptionState::NotConnected => {
                    let event_source = EventSource::get("http://localhost:3001/sse");

                    println!("connected to event source");

                    // let client = Client::new();

                    // match client
                    //     .get("http://localhost:3001/all_orders")
                    //     .send()
                    //     .await
                    //     .unwrap()
                    //     .json::<Vec<_>>()
                    //     .await
                    // {
                    //     Ok(all_orders) => (
                    //         Some(StreamCaptureWindowMessage::InitialOrders(all_orders)),
                    //         SubscriptionState::Connected { event_source },
                    //     ),
                    //     Err(_) => (None, SubscriptionState::NotConnected),
                    // }
                    (None, SubscriptionState::Connected { event_source })
                }

                SubscriptionState::Connected { mut event_source } => {
                    dbg!("state: connected");
                    match event_source.next().await {
                        Some(received) => handle_received(received, event_source),
                        None => (None, SubscriptionState::Connected { event_source }),
                    }
                }
            }
        },
    )
}

fn handle_received(
    received_event: Result<EsEvent, reqwest_eventsource::Error>,

    // to pass through in the state
    event_source: EventSource,
) -> (Option<Breaks>, SubscriptionState) {
    match received_event {
        Ok(EsEvent::Message(message)) => {
            // dbg!(&message);
            let msg = serde_json::from_str(&message.data).unwrap();
            println!("received msg: {:?}", &msg);
            (Some(msg), SubscriptionState::Connected { event_source })
        }
        Ok(_) => (None, SubscriptionState::Connected { event_source }),
        Err(_) => (None, SubscriptionState::NotConnected),
    }
}

enum SubscriptionState {
    NotConnected,
    Connected { event_source: EventSource },
}
