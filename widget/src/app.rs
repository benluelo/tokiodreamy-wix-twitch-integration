use iced::{
    button, container, executor,
    pure::{
        column, container, horizontal_rule, horizontal_space, row, scrollable, text, Application,
        Element,
    },
    rule::{self, FillMode},
    Background, Color, Command, Vector,
};
use iced_native::subscription::Subscription;
use models::Breaks;

use crate::server::connect;

pub(crate) struct Widget {
    breaks: Breaks,
}

#[derive(Debug, Clone)]
pub enum Message {
    NewState(Breaks),
}

impl Application for Widget {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        println!("intialized stream window");
        (
            Self {
                breaks: Breaks::empty(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Widget")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::NewState(breaks) => {
                self.breaks = breaks;
            }
        }

        Command::none()
    }

    fn background_color(&self) -> Color {
        Color::TRANSPARENT
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        connect().map(Message::NewState)
    }

    fn view(&self) -> Element<Self::Message> {
        let breaks =
            self.breaks
                .iter()
                .enumerate()
                .fold(column().spacing(5), |mut col, (idx, order)| {
                    let text_size_mutliplier = if idx != 0 { 3 } else { 3 };
                    if idx != 0 {
                        col = col.push(horizontal_rule(1))
                    } else {
                        col = col.push(
                            text("Now opening")
                                .color(Color::WHITE)
                                .size(24 * text_size_mutliplier),
                        )
                    }
                    col.push(
                        text(&order.twitch_username)
                            .color(Color::WHITE)
                            .size(32 * text_size_mutliplier),
                    )
                    .push(
                        row()
                            .push(horizontal_space(iced::Length::FillPortion(1)))
                            .push(
                                order
                                    .order
                                    .line_items
                                    .iter()
                                    .fold(column().spacing(5), |col, item| {
                                        col.push(
                                            row().push(container(
                                                text(if item.quantity > 1 {
                                                    format!(
                                                        "{}x {}",
                                                        &item.quantity.to_string(),
                                                        &item.name
                                                    )
                                                } else {
                                                    item.name.to_string()
                                                })
                                                .color(Color::WHITE)
                                                .size(24 * text_size_mutliplier),
                                            )),
                                        )
                                    })
                                    .width(iced::Length::FillPortion(9)),
                            ),
                    )
                });

        scrollable(container(breaks))
            .scrollbar_margin(0)
            .scrollbar_width(0)
            .scroller_width(0)
            .into()
    }
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
