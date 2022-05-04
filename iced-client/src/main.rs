use futures::StreamExt;
use models::{OrderWithJson, OrderWithOrder};
use reqwest_eventsource::{Event, EventSource};

#[tokio::main]
async fn main() {
    let mut es = EventSource::get("http://localhost:3000/sse");
    while let Some(event) = es.next().await {
        match event {
            Ok(Event::Open) => println!("Connection Open!"),
            Ok(Event::Message(message)) => {
                println!(
                    "Message: {:#?}",
                    serde_json::from_str::<OrderWithOrder>(&message.data).unwrap()
                )
            }
            Err(err) => {
                println!("Error: {}", err);
                es.close();
            }
        }
    }
}

// mod ws_server;

// use std::collections::HashMap;

// use iced::alignment::{self, Alignment};
// use iced::button::{self, Button};
// use iced::executor;
// use iced::scrollable::{self, Scrollable};
// use iced::text_input::{self, TextInput};
// use iced::{
//     Application, Color, Column, Command, Container, Element, Length, Row, Settings, Subscription,
//     Text,
// };

// pub fn main() -> iced::Result {
//     WebSocket::run(Settings::default())
// }

// #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
// pub struct OrderId {}

// #[derive(Clone, Debug, Default, PartialEq, Eq)]
// pub struct OrderInfo {
//     name: String,
// }

// #[derive(Default)]
// struct WebSocket {
//     breaks: HashMap<OrderId, OrderInfo>,
//     message_log: scrollable::State,
//     new_message: String,
//     new_message_state: text_input::State,
//     new_message_button: button::State,
//     state: State,
// }

// #[derive(Debug, Clone)]
// enum Message {
//     NewMessageChanged(String),
//     Send(ws_server::Message),
//     Echo(ws_server::Event),
//     Server,
// }

// impl Application for WebSocket {
//     type Message = Message;
//     type Flags = ();
//     type Executor = executor::Default;

//     fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
//         (Self::default(), Command::none())
//     }

//     fn title(&self) -> String {
//         String::from("WebSocket - Iced")
//     }

//     fn update(&mut self, message: Message) -> Command<Message> {
//         match message {
//             Message::NewMessageChanged(new_message) => {
//                 self.new_message = new_message;
//             }
//             Message::Send(message) => match &mut self.state {
//                 State::Connected(connection) => {
//                     self.new_message.clear();

//                     connection.send(message);
//                 }
//                 State::Disconnected => {}
//             },
//             Message::Echo(event) => match event {
//                 ws_server::Event::Connected(connection) => {
//                     self.state = State::Connected(connection);

//                     // adds "connected" "disconnected" etc messages to the message log
//                     // self.breaks.push(ws_server::Message::connected());
//                 }
//                 ws_server::Event::Disconnected => {
//                     self.state = State::Disconnected;

//                     // adds "connected" "disconnected" etc messages to the message log
//                     // self.breaks.push(ws_server::Message::disconnected());
//                 }
//                 ws_server::Event::MessageReceived(message) => {
//                     // adds "connected" "disconnected" etc messages to the message log
//                     // self.breaks.push(message);
//                     self.message_log.snap_to(1.0);
//                 }
//             },
//             Message::Server => {}
//         }

//         Command::none()
//     }

//     fn subscription(&self) -> Subscription<Message> {
//         ws_server::connect().map(Message::Echo)
//     }

//     fn view(&mut self) -> Element<Message> {
//         let breaks = self
//             .breaks
//             .iter()
//             .fold(
//                 Scrollable::new(&mut self.message_log),
//                 |scrollable, (id, info)| scrollable.push(Text::new(info.name.clone())),
//             )
//             .width(Length::Fill)
//             .height(Length::Fill)
//             .spacing(10)
//             .into();

//         // Column::with_children(vec![breaks])
//         //     .width(Length::Fill)
//         //     .height(Length::Fill)
//         //     .padding(20)
//         //     .spacing(10)
//         //     .into()

//         breaks
//     }
// }

// enum State {
//     Disconnected,
//     Connected(ws_server::Connection),
// }

// impl Default for State {
//     fn default() -> Self {
//         Self::Disconnected
//     }
// }
