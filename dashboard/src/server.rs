use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::StreamExt;
use iced_futures::futures;
use iced_native::subscription::{self, Subscription};
use models::OrderWithOrder;
use models::{wix::OrderNumber, SseEvent};
use reqwest::Client;
use reqwest_eventsource::{Event as EsEvent, EventSource};

pub fn connect() -> Subscription<Event> {
    struct Connect;

    subscription::unfold(
        std::any::TypeId::of::<Connect>(),
        SubscriptionState::NotConnected,
        |state| async move {
            match state {
                SubscriptionState::NotConnected => {
                    let event_source = EventSource::get("http://localhost:3000/sse");

                    let (sender, receiver) = channel(100);

                    let client = Client::new();

                    let all_orders = client
                        .get("http://localhost:3000/all_orders")
                        .send()
                        .await
                        .unwrap()
                        .json::<Vec<_>>()
                        .await
                        .unwrap();

                    dbg!(&all_orders.len());

                    println!("connected");

                    (
                        Some(Event::Connected {
                            user_message_channel: sender,
                            all_orders,
                        }),
                        SubscriptionState::Connected {
                            event_source,
                            user_input: receiver,
                            client,
                        },
                    )
                }
                SubscriptionState::Connected {
                    mut event_source,
                    mut user_input,
                    client,
                } => {
                    let mut fused_event_source = event_source.by_ref().fuse();

                    futures::select! {
                        received = fused_event_source.select_next_some() => {
                            handle_received(received, event_source, user_input, client)
                        }

                        message = user_input.select_next_some() => {
                            handle_user_message(message, event_source, user_input, client).await
                        }
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
    user_input: Receiver<UserMessage>,
    client: Client,
) -> (Option<Event>, SubscriptionState) {
    match received_event {
        Ok(EsEvent::Message(message)) => {
            let event = serde_json::from_str::<SseEvent>(&message.data).unwrap();
            (
                Some(Event::MessageReceivedFromEventSource(event)),
                SubscriptionState::Connected {
                    event_source,
                    user_input,
                    client,
                },
            )
        }
        Ok(_) => (
            None,
            SubscriptionState::Connected {
                event_source,
                user_input,
                client,
            },
        ),
        Err(_) => (
            Some(Event::EventStreamDisconnected),
            SubscriptionState::NotConnected,
        ),
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UserMessage {
    BreakCompleted(OrderNumber),
}

async fn handle_user_message(
    message: UserMessage,

    // to pass through in the state
    event_source: EventSource,
    user_input: Receiver<UserMessage>,
    client: Client,
) -> (Option<Event>, SubscriptionState) {
    match message {
        UserMessage::BreakCompleted(order_number) => {
            client
                .get(format!(
                    "http://localhost:3000/order_completed/{}",
                    order_number
                ))
                .send()
                .await
                .unwrap();
            (
                None,
                SubscriptionState::Connected {
                    event_source,
                    user_input,
                    client,
                },
            )
        }
    }
}

enum SubscriptionState {
    NotConnected,
    Connected {
        event_source: EventSource,
        user_input: Receiver<UserMessage>,
        client: Client,
    },
}

#[derive(Debug, Clone)]
pub enum Event {
    /// EventStream setup was successful. Contains a channel to the subscription to allow sending messages to the
    Connected {
        user_message_channel: Sender<UserMessage>,
        all_orders: Vec<OrderWithOrder>,
    },
    EventStreamDisconnected,
    MessageReceivedFromEventSource(SseEvent),
}
