use serde::{Deserialize, Serialize};

use crate::wix::{NewOrder, OrderNumber};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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

    pub fn complete(&mut self, idx: usize) {
        let _ = self.ordered_breaks.remove(idx);
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

pub mod wix {
    use std::fmt::Display;

    use serde::{Deserialize, Serialize};

    #[derive(
        Debug, Deserialize, Serialize, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, sqlx::Type,
    )]
    #[serde(transparent)]
    #[repr(transparent)]
    #[sqlx(transparent)]
    pub struct OrderNumber(i32);

    impl Display for OrderNumber {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct NewOrder {
        #[serde(rename = "buyerNote")]
        pub buyer_note: Option<String>,

        /// The number of the order, unique and incrementing.
        #[serde(rename = "number")]
        pub order_number: OrderNumber,

        /// The items in this order.
        #[serde(rename = "lineItems")]
        pub line_items: Vec<OrderLineItem>,

        /// Custom field for the order.
        ///
        /// Should be `twitch_username`.
        #[serde(rename = "customField")]
        pub custom_field: Option<CustomField>,
    }

    impl NewOrder {
        pub fn twitch_username(&self) -> Result<String, TwitchUsernameError> {
            self.custom_field
                .as_ref()
                .map(|cf| {
                    cf.title
                        .eq("twitch username")
                        .then(|| cf.value.clone())
                        .ok_or(TwitchUsernameError::IncorrectTitleForCustomField(
                            cf.value.clone(),
                        ))
                })
                .ok_or(TwitchUsernameError::CustomFieldNotPresent)?
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub enum TwitchUsernameError {
        IncorrectTitleForCustomField(String),
        CustomFieldNotPresent,
    }

    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct OrderLineItem {
        /// REVIEW: Necessary?
        #[serde(rename = "index")]
        pub index: Option<u64>,

        #[serde(rename = "quantity")]
        pub quantity: u64,

        /// The name of the item.
        #[serde(rename = "name")]
        pub name: String,

        /// The different options for the item, such as which cards to keep from the breaks.
        #[serde(rename = "options")]
        pub options: Vec<OrderLineItemOption>,

        #[serde(rename = "customTextFields")]
        pub custom_text_fields: Option<Vec<CustomTextField>>,

        #[serde(rename = "mediaItem")]
        pub media_item: OrderMediaItem,

        #[serde(rename = "notes")]
        pub notes: Option<String>,
    }

    /// I'm not sure what this is
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct CustomTextField {
        #[serde(rename = "title")]
        pub title: String,

        #[serde(rename = "value")]
        pub value: String,
    }

    /// Example: https://www.tokiodreamy.com/product-page/boltund-v-collection
    ///
    /// ```json
    /// {
    ///     "option": "Promo Card",
    ///     "selection": "Yes"
    /// }
    /// ```
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct OrderLineItemOption {
        #[serde(rename = "option")]
        pub option: String,

        #[serde(rename = "selection")]
        pub selection: String,
    }

    /// https://static.wixstatic.com/media/{id} for the image file
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct OrderMediaItem {
        #[serde(rename = "altText")]
        pub alt_text: Option<String>,

        #[serde(rename = "id")]
        pub id: String,

        #[serde(rename = "src")]
        pub src: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct CustomField {
        #[serde(rename = "value")]
        pub value: String,

        #[serde(rename = "title")]
        pub title: String,
    }

    #[test]
    fn test_serde() {
        const JSON: &str = r#"
{
  "_id": "d5d43d01-d9a4-4cc2-b257-61184b881447",
  "_updatedDate": "2020-05-27T12:20:37.994Z",
  "buyerLanguage": "en",
  "cartId": "055e1808-b236-48dc-94d5-45288e06ef9c",
  "channelInfo": {
    "type": "WEB"
  },
  "enteredBy": {
    "id": "f6c2c0f9-4e9f-a58d-a02d-9af2497294d9",
    "identityType": "MEMBER"
  },
  "billingInfo": {
    "address": {
      "formatted": "My company name\n235 W 23rd St\nNew York, New York 10011\nUnited States\n+15555555555",
      "city": "New York",
      "country": "USA",
      "addressLine": "235 W 23rd St",
      "postalCode": "10011",
      "subdivision": "NY"
    },
    "firstName": "Jane",
    "lastName": "Doe",
    "email": "janedoe@gmail.com",
    "phone": "+15555555555",
    "company": "My company name",
    "paidDate": "2020-05-27T12:20:37.994Z",
    "paymentMethod": "VISA",
    "externalTransactionId": "7c03ca74-eaf5-4541-8678-9b857634fdcb",
    "paymentGatewayTransactionId": "29A06193U6234935D",
    "paymentProviderTransactionId": "7c03ca74-eaf5-4541-8678-9b857634fdcb"
  },
  "buyerInfo": {
    "id": "f6c2c0f9-4e9f-a58d-a02d-9af2497294d9",
    "identityType": "MEMBER",
    "firstName": "Jane",
    "lastName": "Doe",
    "phone": "+15555555555",
    "email": "janedoe@gmail.com"
  },
  "_dateCreated": "2020-05-27T12:20:37.966Z",
  "currency": "ILS",
  "fulfillmentStatus": "NOT_FULFILLED",
  "archived": false,
  "activities": [
    {
      "type": "ORDER_PLACED",
      "timestamp": "2020-05-27T12:20:37.966Z"
    },
    {
      "type": "ORDER_PAID",
      "timestamp": "2020-05-27T12:20:37.994Z"
    }
  ],
  "number": 10019,
  "paymentStatus": "PAID",
  "shippingInfo": {
    "deliveryOption": "Free Shipping",
    "estimatedDeliveryTime": "4:30pm",
    "shippingRegion": "Domestic",
    "shipmentDetails": {
      "address": {
        "formatted": "company name\n235 W 23rd St\nNew York, New York 10011\nUnited States\n5555555555",
        "city": "New York",
        "country": "USA",
        "addressLine": "235 W 23rd St",
        "postalCode": "10011",
        "subdivision": "NY"
      },
      "firstName": "Jane",
      "lastName": "Doe",
      "email": "janedoe@gmail.com",
      "phone": "5555555555",
      "company": "company name",
      "tax": 0,
      "discount": 0,
      "priceData": null
    },
    "pickupDetails": null
  },
  "lineItems": [
    {
      "index": 1,
      "quantity": 1,
      "price": 5,
      "name": "my product's name",
      "translatedName": "Nombre traducido",
      "productId": "3fb6a3c8-988b-8755-04bd-5c59ae0b18ea",
      "totalPrice": 5,
      "lineItemType": "PHYSICAL",
      "options": [
        {
          "option": "Size",
          "selection": "Medium"
        }
      ],
      "customTextFields": [
        {
          "title": "Notes for delivery",
          "value": "Please leave at front door"
        }
      ],
      "weight": 1.42,
      "sku": "36523641234523",
      "discount": 0,
      "tax": 5,
      "taxIncludedInPrice": true,
      "priceData": {
        "price": "5",
        "totalPrice": 5,
        "taxIncludedInPrice": true
      },
      "mediaItem": {
        "altText": "This is a description of the image",
        "id": "fac9dc352bf7d54ed0458d64ce41a3ec.jpg",
        "src": "wix:image://v1/fac9dc352bf7d54ed0458d64ce41a3ec.jpg/file.jpg#originWidth=1348&originHeight=899",
        "type": "IMAGE"
      }
    }
  ],
  "totals": {
    "discount": 0,
    "quantity": 1,
    "shipping": 0,
    "subtotal": 5,
    "tax": 0,
    "total": 5,
    "weight": 1.42
  },
  "weightUnit": "KG",
  "customField": {
    "value": "Please call when outside",
    "title": "Notes for delivery",
    "translatedTitle": "Notas de entrega"
  },
  "discount": {
    "appliedCoupon": {
      "code": "47d259d6-7d1e-4ea5-a75c79ca9bb1",
      "couponId": "558b511f-6eb7-82d3-53fca7374dfa",
      "name": "Summer sale"
    }
  },
  "subscriptionInfo": {
    "id": "6b320feb-ddde-45be-950b-8ed277033579",
    "cycleNumber": 1,
    "subscriptionSettings": {
      "frequency": "MONTH",
      "autoRenewal": false,
      "billingCycles": 3
    },
    "subscriptionOptionInfo": {
      "id": "17c145c2-5d23-42c3-ac0a-e579e99c67fd",
      "title": "Coffee of the month",
      "description": "Monthly Coffee Sub"
    }
  }
}
"#;

        let _order: NewOrder = serde_json::from_str(JSON).unwrap();
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
use serde_json::Value;

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderWithOrder {
    pub twitch_username: String,
    pub order_id: OrderNumber,
    pub order: NewOrder,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SseEvent {
    NewOrder(OrderWithOrder),
}
