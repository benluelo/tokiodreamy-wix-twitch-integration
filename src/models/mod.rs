use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

use crate::models::wix::{NewOrder, OrderNumber};

pub mod wix;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/generated/")]
pub struct Breaks {
    ordered_breaks: Vec<OrderWithOrder>,
}

impl Breaks {
    pub fn move_up(&mut self, idx: usize) {
        self.ordered_breaks.swap(idx, idx - 1)
    }

    pub fn move_down(&mut self, idx: usize) {
        self.ordered_breaks.swap(idx, idx + 1)
    }

    pub fn new_order(&mut self, order: OrderWithOrder) {
        self.ordered_breaks.push(order);
    }

    pub fn complete(&mut self, idx: usize) -> OrderWithOrder {
        self.ordered_breaks.remove(idx)
    }

    pub fn remove(&mut self, id: OrderNumber) {
        self.ordered_breaks.retain(|brk| brk.order_id == id)
    }

    pub fn empty() -> Breaks {
        Self::from_iter([])
    }

    pub fn iter(&self) -> BreaksIter {
        BreaksIter {
            iter: self.ordered_breaks.iter(),
        }
    }

    pub fn idx_is_last(&self, idx: usize) -> bool {
        idx == self.ordered_breaks.len() - 1
    }

    pub fn idx_is_first(&self, idx: usize) -> bool {
        idx == 0
    }

    pub fn is_empty(&self) -> bool {
        self.ordered_breaks.is_empty()
    }
}

impl FromIterator<OrderWithOrder> for Breaks {
    fn from_iter<T: IntoIterator<Item = OrderWithOrder>>(iter: T) -> Self {
        Self {
            ordered_breaks: Vec::from_iter(iter),
        }
    }
}

pub struct BreaksIter<'a> {
    iter: core::slice::Iter<'a, OrderWithOrder>,
}

impl<'a> Iterator for BreaksIter<'a> {
    type Item = &'a OrderWithOrder;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum StreamCaptureWindowMessage {
    BreakCompleted(usize),
    MoveUp(usize),
    MoveDown(usize),
    NewOrder(OrderWithOrder),
    InitialOrders(Vec<OrderWithOrder>),
}

#[cfg(test)]
mod test_widget {
    use super::*;

    #[test]
    fn test_serde() {
        serde_json::from_str::<StreamCaptureWindowMessage>(
            &serde_json::to_string(&StreamCaptureWindowMessage::BreakCompleted(1)).unwrap(),
        )
        .unwrap();
    }
}

impl StreamCaptureWindowMessage {
    pub fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

/// Messages sent from the client to the main server.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ClientMsg {
    BreakCompleted { order_number: OrderNumber },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ServerMsg {
    NewOrder { new_order: OrderWithJson },
    BreakCompletedSuccess { order_number: OrderNumber },
    BreakCompletedError { order_number: OrderNumber },
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

impl From<OrderWithJson> for OrderWithOrder {
    fn from(o: OrderWithJson) -> Self {
        OrderWithOrder {
            twitch_username: o.twitch_username,
            order_id: o.order_id,
            order: serde_json::from_value(o.json).unwrap(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/generated/")]
pub struct OrderWithOrder {
    pub twitch_username: String,
    pub order_id: OrderNumber,
    pub order: NewOrder,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/generated/")]
pub enum SseEvent {
    BreaksUpdated(Breaks),
}
