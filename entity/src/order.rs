use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Order {
    pub twitch_username: String,
    pub order_id: Uuid,
    pub date_created: NaiveDateTime,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrderWithJson {
    pub twitch_username: String,
    pub order_id: Uuid,
    pub date_created: NaiveDateTime,
    pub json: serde_json::Value,
}
