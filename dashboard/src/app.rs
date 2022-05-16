use crossbeam_channel::Sender;
use serde::{Deserialize, Serialize};
// use futures::executor::block_on;
use iced::pure::{
    button, column, container, horizontal_space, row, scrollable, text, widget::svg::Svg,
    Application, Element,
};
use iced::pure::{horizontal_rule, tooltip};
use iced::rule::FillMode;
use iced::{
    button, container, executor, rule, Background, Color, Command, Length, Padding, Vector,
};
use iced_native::subscription::Subscription;
use models::wix::OrderLineItem;
use models::{Breaks, OrderWithOrder, SseEvent};
use tokio::runtime::Handle;
use tokio::sync::watch;

use crate::initialize_widget_server;
use crate::server::{self, connect, UserMessage};

enum AppState {
    Disconnected,
    Connected(futures::channel::mpsc::Sender<UserMessage>),
}

impl Default for AppState {
    fn default() -> Self {
        AppState::Disconnected
    }
}

#[derive(Debug, Clone)]
pub enum InnerAppMessage {
    MessageSent,
    Connected,
    Disconnected,
    EventSourceEvent(crate::server::Event),
    BreakCompleted(usize),
    MoveUp(usize),
    MoveDown(usize),
}

pub(crate) struct Dashboard {
    breaks: Breaks,
    state: AppState,
    widget_notification_sender: watch::Sender<Breaks>,
}

impl Application for Dashboard {
    type Message = InnerAppMessage;
    type Flags = ();
    type Executor = executor::Default;

    fn new(_: Self::Flags) -> (Self, Command<InnerAppMessage>) {
        let (widget_notification_sender, widget_notification_receiver) =
            watch::channel(Breaks::empty());
        let handle = Handle::current();
        handle.spawn(initialize_widget_server(widget_notification_receiver));
        (
            Self {
                breaks: Breaks::empty(),
                state: Default::default(),
                widget_notification_sender,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Dashboard")
    }

    fn update(&mut self, message: InnerAppMessage) -> Command<InnerAppMessage> {
        match message {
            InnerAppMessage::MessageSent => Command::none(),
            InnerAppMessage::Connected => Command::none(),
            InnerAppMessage::Disconnected => Command::none(),
            InnerAppMessage::BreakCompleted(idx) => {
                self.breaks.complete(idx);
                self.widget_notification_sender
                    .send(self.breaks.clone())
                    .unwrap();

                Command::none()
            }
            InnerAppMessage::EventSourceEvent(event) => match event {
                server::Event::Connected {
                    user_message_channel: sender,
                    all_orders,
                } => {
                    self.state = AppState::Connected(sender);
                    self.breaks = Breaks::from_iter(all_orders);

                    self.widget_notification_sender
                        .send(self.breaks.clone())
                        .unwrap();

                    Command::none()
                }
                server::Event::EventStreamDisconnected => {
                    self.state = AppState::Disconnected;

                    Command::none()
                }
                server::Event::MessageReceivedFromEventSource(event_source_message) => {
                    match event_source_message {
                        SseEvent::NewOrder(order) => {
                            self.breaks.new_order(order.clone());

                            self.widget_notification_sender
                                .send(self.breaks.clone())
                                .unwrap();

                            Command::none()
                        }
                    }
                }
            },
            InnerAppMessage::MoveUp(idx) => {
                self.breaks.move_up(idx);
                self.widget_notification_sender
                    .send(self.breaks.clone())
                    .unwrap();

                Command::none()
            }
            InnerAppMessage::MoveDown(idx) => {
                self.breaks.move_down(idx);
                self.widget_notification_sender
                    .send(self.breaks.clone())
                    .unwrap();

                Command::none()
            }
        }
    }

    fn background_color(&self) -> Color {
        PRIMARY_DARK_COLOR
    }

    fn subscription(&self) -> Subscription<InnerAppMessage> {
        connect().map(InnerAppMessage::EventSourceEvent)
    }

    fn view(&self) -> Element<InnerAppMessage> {
        // dbg!(self.break_order.len());
        scrollable(
            container(
                self.breaks
                    .iter()
                    .enumerate()
                    // .rev()
                    .fold(
                        column()
                            .spacing(5)
                            .width(Length::Fill)
                            .height(Length::Shrink),
                        |col, (idx, order)| {
                            col.push(
                                container(
                                    column()
                                        .spacing(5)
                                        .push(
                                            // username and order number
                                            row()
                                                .push(text(&order.twitch_username).color(PRIMARY_TEXT_COLOR))
                                                .push(horizontal_space(Length::Fill))
                                                .push(text(order.order_id.to_string()).color(PRIMARY_TEXT_COLOR)),
                                        )
                                        .push(
                                            // items
                                            container(build_line_items(&order.order.line_items, idx))
                                                .width(Length::Fill)
                                                .height(Length::Shrink)
                                                .style(ContainerStyle::transparent().bordered(true).line_color(PRIMARY_LIGHT_COLOR)),
                                        )
                                        .push(
                                            row()
                                                .spacing(5)
                                                .push(horizontal_space(Length::Fill))
                                                .push({
                                                    let mut btn = button(Svg::from_path("/home/benluelo/personal-projects/tokio-wix-backend/dashboard/assets/caret-up-fill.svg"))
                                                        .style(ButtonStyle::primary_light());
                                                    if !self.breaks.idx_is_first(idx) {
                                                        btn = btn.on_press(InnerAppMessage::MoveUp(idx))
                                                    }
                                                    btn
                                                })
                                                .push({
                                                    let mut btn = button(Svg::from_path("/home/benluelo/personal-projects/tokio-wix-backend/dashboard/assets/caret-down-fill.svg"))
                                                        .style(ButtonStyle::primary_light());
                                                    if !self.breaks.idx_is_last(idx) {
                                                        btn = btn.on_press(InnerAppMessage::MoveDown(idx))
                                                    }
                                                    btn
                                                }
                                                    )
                                                .push(
                                                    button(text("Completed"))
                                                        .style(ButtonStyle::secondary()).on_press(InnerAppMessage::BreakCompleted(idx)),
                                                ),
                                        ),
                                )
                                .width(Length::Fill)
                                .height(Length::Shrink)
                                .padding(Padding {
                                    top: 5,
                                    right: 5,
                                    bottom: 5,
                                    left: 5,
                                })
                                .style(ContainerStyle::primary().bordered(true).line_color(PRIMARY_LIGHT_COLOR)),
                            )
                            // .push(horizontal_rule(2).style(RuleStyle::primary_light()))
                        },
                    ),
            )
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding(Padding {
                top: 5,
                right: 15,
                bottom: 5,
                left: 5,
            })
            .style(ContainerStyle::primary_dark().bordered(false)),
        )
        .into()
    }
}

fn build_line_items(items: &Vec<OrderLineItem>, idx: usize) -> Element<'static, InnerAppMessage> {
    let col = column()
        .spacing(5)
        .width(Length::Fill)
        .height(Length::Shrink);

    items
        .iter()
        .enumerate()
        .fold(col, |mut items_col, (inner_idx, item)| {
            if inner_idx != 0 {
                items_col = items_col.push(horizontal_rule(2).style(RuleStyle::primary_dark()))
            }
            items_col.push(if idx == 0 {
                dbg!(idx);
                Element::<InnerAppMessage>::new({
                    let row: iced_pure::widget::Row<InnerAppMessage, iced::Renderer> = row()
                        .push(text(item.quantity.to_string()))
                        .push(text("x "))
                        .push(text(&item.name))
                        .push(horizontal_space(iced::Length::Units(5)))
                        .push(
                            item.options.iter().fold(
                                column()
                                    .spacing(5)
                                    .width(Length::Fill)
                                    .height(Length::Shrink),
                                |options_col, option| {
                                    options_col.push(text(format!(
                                        "{}: {}",
                                        option.option, option.selection
                                    )))
                                },
                            ),
                        );
                    row
                })
            } else {
                Element::<InnerAppMessage>::from(
                    tooltip(
                        // item name and quantity
                        row()
                            .push(text(item.quantity.to_string()))
                            .push(text("x "))
                            .push(text(&item.name)),
                        item.options
                            .iter()
                            .map(|option| format!("{}: {}", option.option, option.selection))
                            .collect::<Vec<_>>()
                            .join("\n"),
                        iced::tooltip::Position::FollowCursor,
                    )
                    .style(ContainerStyle::secondary_light()),
                )
            })
        })
        .into()
}

struct ButtonStyle {
    bg_color: Color,
    line_color: Color,
    bordered: bool,
}

impl ButtonStyle {
    fn border_width(&self) -> f32 {
        if self.bordered {
            LINE_WIDTH
        } else {
            0.0
        }
    }
}

impl AppStyle for ButtonStyle {
    fn with_main_color(bg_color: Color) -> Self {
        Self {
            bg_color,
            line_color: Color::TRANSPARENT,
            bordered: false,
        }
    }
}

struct ContainerStyle {
    bg_color: Color,
    line_color: Color,
    bordered: bool,
}

impl ContainerStyle {
    fn line_color(mut self, color: Color) -> Self {
        self.line_color = color;
        self
    }

    fn border_width(&self) -> f32 {
        if self.bordered {
            LINE_WIDTH
        } else {
            0.0
        }
    }

    fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }
}

impl AppStyle for ContainerStyle {
    fn with_main_color(bg_color: Color) -> Self {
        Self {
            bg_color,
            line_color: Color::TRANSPARENT,
            bordered: false,
        }
    }
}

struct RuleStyle {
    color: Color,
}

impl AppStyle for RuleStyle {
    fn with_main_color(color: Color) -> Self {
        Self { color }
    }
}

trait AppStyle: Sized {
    fn with_main_color(bg_color: Color) -> Self;

    fn transparent() -> Self {
        Self::with_main_color(Color::TRANSPARENT)
    }
    fn primary() -> Self {
        Self::with_main_color(PRIMARY_COLOR)
    }
    fn primary_light() -> Self {
        Self::with_main_color(PRIMARY_LIGHT_COLOR)
    }
    fn primary_dark() -> Self {
        Self::with_main_color(PRIMARY_DARK_COLOR)
    }
    fn secondary() -> Self {
        Self::with_main_color(SECONDARY_COLOR)
    }
    fn secondary_light() -> Self {
        Self::with_main_color(SECONDARY_LIGHT_COLOR)
    }
    fn secondary_dark() -> Self {
        Self::with_main_color(SECONDARY_DARK_COLOR)
    }
    fn primary_text() -> Self {
        Self::with_main_color(PRIMARY_TEXT_COLOR)
    }
    fn secondary_text() -> Self {
        Self::with_main_color(SECONDARY_TEXT_COLOR)
    }
}

const LINE_WIDTH: f32 = 1.0;

const PRIMARY_COLOR: Color = Color::from_rgb(
    0x42 as f32 / 255.0,
    0x42 as f32 / 255.0,
    0x42 as f32 / 255.0,
);
const PRIMARY_LIGHT_COLOR: Color = Color::from_rgb(
    0x6d as f32 / 255.0,
    0x6d as f32 / 255.0,
    0x6d as f32 / 255.0,
);
const PRIMARY_DARK_COLOR: Color = Color::from_rgb(
    0x30 as f32 / 255.0,
    0x30 as f32 / 255.0,
    0x30 as f32 / 255.0,
);
const SECONDARY_COLOR: Color = Color::from_rgb(
    0x02 as f32 / 255.0,
    0xa5 as f32 / 255.0,
    0xe0 as f32 / 255.0,
);
const SECONDARY_LIGHT_COLOR: Color = Color::from_rgb(
    0x36 as f32 / 255.0,
    0xd6 as f32 / 255.0,
    0xff as f32 / 255.0,
);
const SECONDARY_DARK_COLOR: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x76 as f32 / 255.0,
    0xae as f32 / 255.0,
);
const PRIMARY_TEXT_COLOR: Color = Color::from_rgb(
    0xff as f32 / 255.0,
    0xff as f32 / 255.0,
    0xff as f32 / 255.0,
);
const SECONDARY_TEXT_COLOR: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x00 as f32 / 255.0,
    0x00 as f32 / 255.0,
);

const TOKIO_BLUE: Color = Color {
    r: 2.0 / 255.0,
    g: 165.0 / 255.0,
    b: 224.0 / 255.0,
    a: 1.0,
};

impl container::StyleSheet for ContainerStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::WHITE),
            background: Some(Background::Color(self.bg_color)),
            border_radius: 5.0,
            border_color: if self.bordered {
                self.line_color
            } else {
                Color::TRANSPARENT
            },
            border_width: self.border_width(),
        }
    }
}

impl rule::StyleSheet for RuleStyle {
    fn style(&self) -> rule::Style {
        rule::Style {
            color: self.color,
            width: 1,
            radius: 0.0,
            fill_mode: FillMode::Full,
        }
    }
}

impl button::StyleSheet for ButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::new(1.0, 1.0),
            background: Some(Background::Color(self.bg_color)),
            border_radius: 5.0,
            border_width: self.border_width(),
            border_color: if self.bordered {
                self.line_color
            } else {
                Color::TRANSPARENT
            },
            text_color: Color::WHITE,
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        button::Style {
            shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
            ..active
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::default(),
            ..self.active()
        }
    }

    fn disabled(&self) -> button::Style {
        let active = self.active();

        button::Style {
            shadow_offset: Vector::default(),
            background: active.background.map(|background| match background {
                Background::Color(color) => Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
            }),
            text_color: Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}
