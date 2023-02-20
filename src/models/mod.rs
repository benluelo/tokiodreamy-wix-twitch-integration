use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

use crate::models::wix::{NewOrder, OrderNumber};

pub mod wix;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/generated/")]
pub struct Breaks {
    ordered_breaks: Vec<OrderWithOrder>,
}

impl Breaks {
    pub fn initialize() -> Breaks {
        Self {
            ..Default::default()
        }
    }

    /// Creates a new [`Breaks`] from the provided [`Vec`]. Assumes that the vec is sorted.
    pub fn from_ordered(ordered_breaks: Vec<OrderWithOrder>) -> Breaks {
        Self { ordered_breaks }
    }

    pub fn move_up(&mut self, idx: usize) {
        self.ordered_breaks.swap(idx, idx - 1)
    }

    pub fn move_down(&mut self, idx: usize) {
        self.ordered_breaks.swap(idx, idx + 1)
    }

    pub fn new_order(&mut self, order: OrderWithOrder) {
        self.ordered_breaks.push(order);
    }

    pub fn remove_by_id(&mut self, id: OrderNumber) {
        self.ordered_breaks.retain(|brk| brk.order_id != id)
    }

    pub fn get_mut_by_id(&mut self, id: OrderNumber) -> Option<&mut OrderWithOrder> {
        self.ordered_breaks
            .iter_mut()
            .find(|brk| brk.order_id == id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Order {
    pub twitch_username: String,
    pub order_id: OrderNumber,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrderWithJson {
    pub twitch_username: String,
    pub order_id: OrderNumber,
    pub json: Value,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/generated/")]
pub struct OrderWithOrder {
    pub twitch_username: Option<String>,
    pub order_id: OrderNumber,
    pub order: NewOrder,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/generated/")]
pub enum SseEvent {
    BreaksUpdated(Breaks),
}
