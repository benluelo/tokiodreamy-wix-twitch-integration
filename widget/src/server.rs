use std::time::Duration;

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

                    (None, SubscriptionState::Connected { event_source })
                }

                SubscriptionState::Connected { mut event_source } => {
                    dbg!("state: connected");
                    match event_source.next().await {
                        Some(Ok(EsEvent::Message(message))) => {
                            // dbg!(&message);
                            let msg = serde_json::from_str(&message.data).unwrap();
                            println!("received msg: {:?}", &msg);
                            (Some(msg), SubscriptionState::Connected { event_source })
                        }
                        Some(Ok(_)) => (None, SubscriptionState::Connected { event_source }),
                        Some(Err(_)) => (
                            None,
                            SubscriptionState::Disconnected {
                                retry_in: Duration::from_secs(1),
                            },
                        ),
                        None => (None, SubscriptionState::Connected { event_source }),
                    }
                }
                SubscriptionState::Disconnected { retry_in } => {
                    tokio::time::sleep(retry_in).await;
                    (None, SubscriptionState::NotConnected)
                }
            }
        },
    )
}

enum SubscriptionState {
    NotConnected,
    Connected { event_source: EventSource },
    Disconnected { retry_in: Duration },
}
