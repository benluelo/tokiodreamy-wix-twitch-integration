use std::fmt::Display;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use sqlx::postgres::{PgHasArrayType, PgTypeInfo};

macro_rules! impl_HasPgArrayType {
    ($ty:ident) => {
        impl PgHasArrayType for $ty {
            fn array_type_info() -> PgTypeInfo {
                PgTypeInfo::with_name(concat!("_", stringify!($ty)))
            }
        }
    };
}

#[derive(
    Debug, Deserialize, Serialize, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, sqlx::Type, TS,
)]
#[serde(transparent)]
#[repr(transparent)]
#[sqlx(transparent)]
#[ts(export, export_to = "frontend/src/generated/")]
pub struct OrderNumber(i32);

impl Display for OrderNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, TS, sqlx::Type)]
#[ts(export, export_to = "frontend/src/generated/")]
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
                    .ok_or_else(|| {
                        TwitchUsernameError::IncorrectTitleForCustomField(cf.value.clone())
                    })
            })
            .ok_or(TwitchUsernameError::CustomFieldNotPresent)?
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum TwitchUsernameError {
    IncorrectTitleForCustomField(String),
    CustomFieldNotPresent,
}

#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone, PartialEq, Eq, TS)]
#[ts(export, export_to = "frontend/src/generated/")]
pub struct OrderLineItem {
    // /// REVIEW: Necessary?
    #[serde(rename = "index")]
    pub index: Option<i64>,

    #[serde(rename = "quantity")]
    pub quantity: i64,

    /// The name of the item.
    #[serde(rename = "name")]
    pub name: String,

    /// The different options for the item, such as which cards to keep from the
    /// breaks.
    #[serde(rename = "options")]
    pub options: Vec<OrderLineItemOption>,

    #[serde(rename = "customTextFields")]
    pub custom_text_fields: Option<Vec<CustomTextField>>,

    #[serde(rename = "mediaItem")]
    pub media_item: OrderMediaItem,

    #[serde(rename = "notes")]
    pub notes: Option<String>,
}

impl_HasPgArrayType! {
    OrderLineItem
}

/// I'm not sure what this is
#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone, PartialEq, Eq, TS)]
#[ts(export, export_to = "frontend/src/generated/")]
pub struct CustomTextField {
    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "value")]
    pub value: String,
}

impl_HasPgArrayType! {
    CustomTextField
}

/// Example: https://www.tokiodreamy.com/product-page/boltund-v-collection
///
/// ```json
/// {
///     "option": "Promo Card",
///     "selection": "Yes"
/// }
/// ```
#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone, PartialEq, Eq, TS)]
#[ts(export, export_to = "frontend/src/generated/")]
pub struct OrderLineItemOption {
    #[serde(rename = "option")]
    pub option: String,

    #[serde(rename = "selection")]
    pub selection: String,
}

impl_HasPgArrayType! {
    OrderLineItemOption
}

/// https://static.wixstatic.com/media/{id} for the image file
#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone, PartialEq, Eq, TS)]
#[ts(export, export_to = "frontend/src/generated/")]
pub struct OrderMediaItem {
    #[serde(rename = "altText")]
    pub alt_text: Option<String>,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "src")]
    pub src: String,
}

#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone, PartialEq, Eq, TS)]
#[ts(export, export_to = "frontend/src/generated/")]
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
