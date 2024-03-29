use axum::{
    extract::State,
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
};
use futures::{Stream, StreamExt};
use tokio::sync::watch;
use tokio_stream::wrappers::WatchStream;

use crate::models::{Breaks, SseEvent};

pub(crate) async fn get(
    State(receiver): State<watch::Receiver<Breaks>>,
    // TODO: Better error type
) -> Sse<impl Stream<Item = Result<Event, String>>> {
    Sse::new(WatchStream::new(receiver).map(|breaks| {
        dbg!(&breaks);
        Event::default()
            .json_data(SseEvent::BreaksUpdated(breaks))
            .map_err(|err| err.to_string())
    }))
    .keep_alive(KeepAlive::default())
}
