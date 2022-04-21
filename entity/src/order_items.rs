use uuid::Uuid;

pub struct OrderItem {
    pub order_id: Uuid,
    pub item_name: String,
    pub quantity: u32,
    pub id: u32,
}