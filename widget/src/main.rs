mod app;
mod server;

use iced::{pure::Application, window, Settings};

use crate::app::StreamWindow;

pub fn main() -> iced::Result {
    StreamWindow::run(Settings {
        window: window::Settings {
            transparent: true,
            ..Default::default()
        },
        ..Default::default()
    })
}
