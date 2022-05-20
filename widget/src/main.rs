mod app;
mod server;

use iced::{
    pure::Application,
    window::{self, Icon},
    Settings,
};
use models::{ICON, ICON_HEIGHT, ICON_WIDTH};

use crate::app::Widget;

pub fn main() -> iced::Result {
    Widget::run(Settings {
        window: window::Settings {
            transparent: true,
            icon: Some(Icon::from_rgba(ICON.to_vec(), ICON_HEIGHT, ICON_WIDTH).unwrap()),
            ..Default::default()
        },
        ..Default::default()
    })
}
