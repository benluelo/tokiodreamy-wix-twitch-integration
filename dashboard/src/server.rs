use std::sync::Arc;

use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::channel::oneshot;
use futures::StreamExt;
use iced_futures::futures;
use iced_native::subscription::{self, Subscription};
use models::OrderWithOrder;
use models::{wix::OrderNumber, SseEvent};
use reqwest::header::AUTHORIZATION;
use reqwest::{Client, Method, RequestBuilder, StatusCode};
use reqwest_eventsource::{Event as EsEvent, EventSource};

pub(crate) fn build_auth_header(auth_key: &str) -> String {
    base64::encode(format!("{}:{}", "tokiodreamy", auth_key))
}

use crate::app::DisconnectReason;

pub fn connect() -> Subscription<Event> {
    struct Connect;

    subscription::unfold(
        std::any::TypeId::of::<Connect>(),
        SubscriptionState::Init,
        |state| async move {
            match state {
                SubscriptionState::NotConnected { auth_key } => {
                    let client = Client::new();

                    let all_orders = {
                        match client
                            .get("http://18.212.208.3:3000/all_orders")
                            .header(AUTHORIZATION, build_auth_header(&auth_key))
                            .send()
                            .await
                        {
                            Ok(response) => {
                                if response.status().is_success() {
                                    match response.json::<Vec<_>>().await {
                                        Ok(all_orders) => all_orders,
                                        Err(why) => {
                                            panic!("{why}")
                                        }
                                    }
                                } else {
                                    match response.status() {
                                        StatusCode::UNAUTHORIZED => {
                                            dbg!("unauthorized");
                                            return create_password_channel();
                                        }
                                        status_code => {
                                            panic!("Unexpected status code: {}", status_code)
                                        }
                                    }
                                }
                            }
                            Err(why) => {
                                panic!("{why}")
                            }
                        }
                    };

                    let event_source = EventSource::new(
                        client
                            .get("http://18.212.208.3:3000/sse")
                            .header(AUTHORIZATION, build_auth_header(&auth_key)),
                    )
                    .unwrap();

                    let (sender, receiver) = channel(100);

                    dbg!(&all_orders.len());

                    println!("connected");

                    (
                        Some(Event::Connected {
                            user_message_channel: sender,
                            all_orders,
                        }),
                        SubscriptionState::Connected {
                            auth_key,
                            event_source,
                            user_input: receiver,
                            client,
                        },
                    )
                }
                SubscriptionState::Connected {
                    auth_key,
                    mut event_source,
                    mut user_input,
                    client,
                } => {
                    let mut fused_event_source = event_source.by_ref().fuse();

                    futures::select! {
                        received = fused_event_source.select_next_some() => {
                            handle_received(received, event_source, user_input, client, auth_key)
                        }

                        message = user_input.select_next_some() => {
                            handle_user_message(message, event_source, user_input, client, auth_key).await
                        }
                    }
                }
                SubscriptionState::NoPassword { mut channel } => {
                    let auth_key = channel.next().await.unwrap();
                    dbg!(&auth_key);
                    (None, SubscriptionState::NotConnected { auth_key })
                }
                SubscriptionState::Init => {
                    // TODO: Attempt to read auth key from file
                    create_password_channel()
                }
            }
        },
    )
}

fn create_password_channel() -> (Option<Event>, SubscriptionState) {
    let (sender, receiver) = channel(1);
    (
        Some(Event::AwaitingPassword {
            channel: (sender),
            last_password_was_bad: true,
        }),
        SubscriptionState::NoPassword { channel: receiver },
    )
}

fn handle_received(
    received_event: Result<EsEvent, reqwest_eventsource::Error>,

    // to pass through in the state
    event_source: EventSource,
    user_input: Receiver<UserMessage>,
    client: Client,
    auth_key: String,
) -> (Option<Event>, SubscriptionState) {
    match received_event {
        Ok(EsEvent::Message(message)) => {
            let event = serde_json::from_str::<SseEvent>(&message.data).unwrap();
            (
                Some(Event::MessageReceivedFromEventSource(event)),
                SubscriptionState::Connected {
                    auth_key,
                    event_source,
                    user_input,
                    client,
                },
            )
        }
        Ok(_) => (
            None,
            SubscriptionState::Connected {
                auth_key,
                event_source,
                user_input,
                client,
            },
        ),
        Err(_) => (
            Some(Event::EventStreamDisconnected(DisconnectReason::Other)),
            SubscriptionState::NotConnected { auth_key },
        ),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UserMessage {
    BreakCompleted(OrderNumber),
}

async fn handle_user_message(
    message: UserMessage,

    // to pass through in the state
    event_source: EventSource,
    user_input: Receiver<UserMessage>,
    client: Client,
    auth_key: String,
) -> (Option<Event>, SubscriptionState) {
    match message {
        UserMessage::BreakCompleted(order_number) => {
            client
                .get(format!(
                    "http://18.212.208.3:3000/order_completed/{}",
                    order_number
                ))
                .header(AUTHORIZATION, build_auth_header(&auth_key))
                .send()
                .await
                .unwrap();
            (
                None,
                SubscriptionState::Connected {
                    auth_key,
                    event_source,
                    user_input,
                    client,
                },
            )
        }
    }
}

enum SubscriptionState {
    Init,
    NoPassword {
        channel: Receiver<String>,
    },
    NotConnected {
        auth_key: String,
    },
    Connected {
        auth_key: String,
        event_source: EventSource,
        user_input: Receiver<UserMessage>,
        client: Client,
    },
}

#[derive(Debug, Clone)]
pub enum Event {
    AwaitingPassword {
        channel: Sender<String>,
        last_password_was_bad: bool,
    },
    /// EventStream setup was successful. Contains a channel to the subscription to allow sending messages to the
    Connected {
        user_message_channel: Sender<UserMessage>,
        all_orders: Vec<OrderWithOrder>,
    },
    EventStreamDisconnected(DisconnectReason),
    MessageReceivedFromEventSource(SseEvent),
}
