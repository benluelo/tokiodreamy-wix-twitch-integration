use std::net::SocketAddr;

use axum::{
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
    routing::get,
    Router,
};
use futures::StreamExt;
use iced::{
    pure::Application,
    window::{self, Icon},
    Settings,
};
use models::{Breaks, ICON, ICON_HEIGHT, ICON_WIDTH};
use tokio::sync::watch;

use crate::app::Dashboard;
use tokio_stream::wrappers::WatchStream;

mod app;
mod server;

const DEFAULT_FONT: &[u8] = include_bytes!("../../assets/NanumGothic-Regular.ttf");

pub fn main() -> iced::Result {
    // tracing_subscriber::fmt::init();
    dotenv::dotenv().unwrap();

    Dashboard::run(Settings {
        window: window::Settings {
            icon: Some(Icon::from_rgba(ICON.to_vec(), ICON_HEIGHT, ICON_WIDTH).unwrap()),
            ..Default::default()
        },
        default_font: Some(DEFAULT_FONT),
        ..Default::default()
    })
}

async fn initialize_widget_server(channel: watch::Receiver<Breaks>) {
    let app = Router::new().route(
        "/sse",
        get(|| async move {
            dbg!();
            let stream = WatchStream::new(channel)
                .map(|breaks| Event::default().json_data(breaks).unwrap())
                .map(Result::<_, String>::Ok);
            Sse::new(stream).keep_alive(KeepAlive::default())
        }),
    );

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
