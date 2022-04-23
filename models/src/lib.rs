use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::wix::NewOrder;

pub mod wix {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct NewOrder {
        #[serde(rename = "_id")]
        pub _id: Uuid,
        #[serde(rename = "_updatedDate")]
        pub _updated_date: DateTime<Utc>,
        #[serde(rename = "buyerLanguage")]
        pub buyer_language: BuyerLanguage,
        #[serde(rename = "cartId")]
        pub cart_id: Option<Uuid>,
        #[serde(rename = "channelInfo")]
        pub channel_info: ChannelInfo,
        #[serde(rename = "enteredBy")]
        pub entered_by: EnteredBy,
        #[serde(rename = "billingInfo")]
        pub billing_info: Option<BillingInfo>,
        #[serde(rename = "buyerInfo")]
        pub buyer_info: BuyerInfo,
        #[serde(rename = "buyerNote")]
        pub buyer_note: Option<String>,
        #[serde(rename = "_dateCreated")]
        pub _date_created: DateTime<Utc>,
        #[serde(rename = "currency")]
        pub currency: String,
        #[serde(rename = "fulfillmentStatus")]
        pub fulfillment_status: FulfillmentStatus,
        #[serde(rename = "fulfillments")]
        pub fulfillments: Option<Vec<Fulfillment>>,
        #[serde(rename = "archived")]
        pub archived: bool,
        #[serde(rename = "activities")]
        pub activities: Vec<Activity>,
        #[serde(rename = "number")]
        pub number: u64,
        #[serde(rename = "paymentStatus")]
        pub payment_status: PaymentStatus,
        #[serde(rename = "shippingInfo")]
        pub shipping_info: Option<ShippingInfo>,
        #[serde(rename = "lineItems")]
        pub line_items: Vec<OrderLineItem>,
        #[serde(rename = "totals")]
        pub totals: Totals,
        #[serde(rename = "weightUnit")]
        pub weight_unit: Option<WeightUnit>,
        #[serde(rename = "customField")]
        pub custom_field: Option<CustomField>,
        #[serde(rename = "discount")]
        pub discount: Option<Discount>,
        #[serde(rename = "refund")]
        pub refunds: Option<Vec<Refund>>,
        // The following field is only present when
        // the order is a subscription
        #[serde(rename = "subscriptionInfo")]
        pub subscription_info: Option<SubscriptionInfo>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Refund {
        #[serde(rename = "id")]
        pub id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: DateTime<Utc>,
        #[serde(rename = "amount")]
        pub amount: String,
        #[serde(rename = "reason")]
        pub reason: Option<String>,
        #[serde(rename = "externalRefund")]
        pub external_refund: bool,
        #[serde(rename = "paymentProviderTransactionId")]
        pub payment_provider_transaction_id: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Fulfillment {
        #[serde(rename = "id")]
        pub id: String,
        #[serde(rename = "dateCreated")]
        pub date_created: DateTime<Utc>,
        #[serde(rename = "lineItems")]
        pub line_items: Vec<FulfillmentLineItem>,
        #[serde(rename = "tackingInfo")]
        pub tacking_info: Option<TrackingInfo>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct TrackingInfo {
        #[serde(rename = "trackingNumber")]
        pub tracking_number: String,
        #[serde(rename = "shippingProvider")]
        pub shipping_provider: String,
        #[serde(rename = "trackingLink")]
        pub tracking_link: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct FulfillmentLineItem {
        #[serde(rename = "index")]
        pub index: u32,
        #[serde(rename = "quantity")]
        pub quantity: Option<u32>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum BuyerLanguage {
        #[serde(rename = "en")]
        English,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct ChannelInfo {
        #[serde(rename = "externalOrderId")]
        pub external_order_id: Option<String>,
        #[serde(rename = "externalOrderUrl")]
        pub external_order_url: Option<String>,
        #[serde(rename = "type")]
        r#type: ChannelInfoType,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum ChannelInfoType {
        #[serde(rename = "WEB")]
        Web,
        #[serde(rename = "POS")]
        Pos,
        #[serde(rename = "EBAY")]
        Ebay,
        #[serde(rename = "OTHER_PLATFORM")]
        OtherPlatform,
        #[serde(rename = "WIX_APP_STORE")]
        WixAppStore,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct EnteredBy {
        #[serde(rename = "id")]
        pub id: Uuid,
        #[serde(rename = "identityType")]
        pub identity_type: EnteredByIdentityType,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum EnteredByIdentityType {
        #[serde(rename = "USER")]
        User,

        #[serde(rename = "MEMBER")]
        Member,

        #[serde(rename = "CONTACT")]
        Contact,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct BillingInfo {
        #[serde(rename = "address")]
        pub address: Option<Address>,
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
        #[serde(rename = "email")]
        pub email: Option<String>,
        #[serde(rename = "phone")]
        pub phone: Option<String>,
        #[serde(rename = "company")]
        pub company: Option<String>,
        #[serde(rename = "vatId")]
        pub vat_id: Option<VatId>,
        #[serde(rename = "externalTransactionId")]
        #[deprecated(note = "replaced with paymentProviderTransactionId")]
        pub external_transaction_id: Option<Uuid>,
        #[serde(rename = "paidDate")]
        pub paid_date: Option<DateTime<Utc>>,
        // REVIEW: Enum? ("VISA")
        #[serde(rename = "paymentMethod")]
        pub payment_method: Option<String>,
        // REVIEW: Uuid?
        #[serde(rename = "paymentGatewayTransactionId")]
        pub payment_gateway_transaction_id: Option<String>,
        #[serde(rename = "paymentProviderTransactionId")]
        pub payment_provider_transaction_id: Option<Uuid>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Address {
        #[serde(rename = "formatted")]
        pub formatted: Option<String>,
        #[serde(rename = "city")]
        pub city: String,
        #[serde(rename = "country")]
        pub country: String,
        #[serde(rename = "addressLine")]
        pub address_line: String,
        #[serde(rename = "addressLine2")]
        pub address_line_2: Option<String>,
        #[serde(rename = "streetAddress")]
        pub street_address: Option<StreetAddress>,
        #[serde(rename = "postalCode")]
        pub postal_code: String,
        #[serde(rename = "subdivision")]
        pub subdivision: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct StreetAddress {
        #[serde(rename = "name")]
        pub name: String,
        #[serde(rename = "number")]
        pub number: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct VatId {
        #[serde(rename = "number")]
        pub number: String,
        #[serde(rename = "type")]
        r#type: VatIdType,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum VatIdType {
        #[serde(rename = "CPF")]
        Cpf,
        #[serde(rename = "CNJP")]
        Cnjp,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct BuyerInfo {
        #[serde(rename = "id")]
        pub id: String,
        #[serde(rename = "email")]
        pub email: String,
        #[serde(rename = "firstName")]
        pub first_name: String,
        #[serde(rename = "lastName")]
        pub last_name: String,
        #[serde(rename = "phone")]
        pub phone: Option<String>,
        #[serde(rename = "identityType")]
        pub identity_type: IdentityType,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum IdentityType {
        #[serde(rename = "MEMBER")]
        Member,

        #[serde(rename = "CONTACT")]
        Contact,

        #[serde(rename = "ADMIN")]
        Admin,

        #[serde(rename = "VISITOR")]
        Visitor,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct PriceData {
        #[serde(rename = "price")]
        pub price: String,
        #[serde(rename = "totalPrice")]
        pub total_price: f64,
        #[serde(rename = "taxIncludedInPrice")]
        pub tax_included_in_price: bool,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct ShippingInfo {
        #[serde(rename = "deliverByDate")]
        pub deliver_by_date: Option<String>,
        #[serde(rename = "deliveryOption")]
        pub delivery_option: Option<String>,
        #[serde(rename = "estimatedDeliveryTime")]
        pub estimated_delivery_time: Option<String>,
        #[serde(rename = "shipmentDetails")]
        // REVIEW: Make *_details an enum?
        pub shipment_details: ShipmentDetails,
        #[serde(rename = "pickupDetails")]
        pub pickup_details: Option<PickupDetails>,
        #[serde(rename = "shippingRegion")]
        pub shipping_region: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct ShipmentDetails {
        #[serde(rename = "address")]
        pub address: Address,
        #[serde(rename = "firstName")]
        pub first_name: String,
        #[serde(rename = "lastName")]
        pub last_name: String,
        #[serde(rename = "email")]
        pub email: String,
        #[serde(rename = "phone")]
        pub phone: String,
        #[serde(rename = "company")]
        pub company: String,
        #[serde(rename = "tax")]
        pub tax: f64,
        #[serde(rename = "discount")]
        pub discount: f64,
        #[serde(rename = "priceData")]
        pub price_data: Option<ShipmentPriceData>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct PickupDetails {
        #[serde(rename = "pickupInstructions")]
        pub pickup_instructions: String,
        #[serde(rename = "pickupAddress")]
        pub pickup_address: Address,
        #[serde(rename = "firstName")]
        pub first_name: String,
        #[serde(rename = "lastName")]
        pub last_name: String,
        #[serde(rename = "email")]
        pub email: String,
        #[serde(rename = "phone")]
        pub phone: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct ShipmentPriceData {
        #[serde(rename = "price")]
        pub price: String,
        #[serde(rename = "taxIncludedInPrice")]
        pub tax_included_in_price: bool,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum FulfillmentStatus {
        #[serde(rename = "NOT_FULFILLED")]
        NotFulfilled,
        #[serde(rename = "FULFILLED")]
        Fulfilled,
        #[serde(rename = "CANCELED")]
        Canceled,
        #[serde(rename = "PARTIALLY_FULFILLED")]
        PartiallyFulfilled,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Activity {
        #[serde(rename = "type")]
        r#type: ActivityType,
        #[serde(rename = "timestamp")]
        pub timestamp: DateTime<Utc>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum ActivityType {
        #[serde(rename = "MERCHANT_COMMENT")]
        MerchantComment,
        #[serde(rename = "ORDER_PLACED")]
        OrderPlaced,
        #[serde(rename = "ORDER_PAID")]
        OrderPaid,
        #[serde(rename = "ORDER_FULFILLED")]
        OrderFulfilled,
        #[serde(rename = "ORDER_NOT_FULFILLED")]
        OrderNotFulfilled,
        #[serde(rename = "DOWNLOAD_LINK_SENT")]
        DownloadLinkSent,
        #[serde(rename = "PICKUP_READY_EMAIL_SENT")]
        PickupReadyEmailSent,
        #[serde(rename = "TRACKING_NUMBER_ADDED")]
        TrackingNumberAdded,
        #[serde(rename = "TRACKING_NUMBER_EDITED")]
        TrackingNumberEdited,
        #[serde(rename = "TRACKING_LINK_WAS_SET")]
        TrackingLinkWasSet,
        #[serde(rename = "SHIPPING_CONFIRMATION_EMAIL_SENT")]
        ShippingConfirmationEmailSent,
        #[serde(rename = "INVOICE_WAS_SET")]
        InvoiceWasSet,
        #[serde(rename = "INVOICE_WAS_REMOVED")]
        InvoiceWasRemoved,
        #[serde(rename = "INVOICE_WAS_SENT")]
        InvoiceWasSent,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum PaymentStatus {
        #[serde(rename = "NOT_PAID")]
        NotPaid,
        #[serde(rename = "PAID")]
        Paid,
        #[serde(rename = "PARTIALLY_REFUNDED")]
        PartiallyRefunded,
        #[serde(rename = "FULLY_REFUNDED")]
        FullyRefunded,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct OrderLineItem {
        #[serde(rename = "index")]
        pub index: Option<u64>,
        #[serde(rename = "quantity")]
        pub quantity: u64,
        #[serde(rename = "price")]
        pub price: f64,
        #[serde(rename = "name")]
        pub name: String,
        #[serde(rename = "translatedName")]
        pub translated_name: Option<String>,
        #[serde(rename = "productId")]
        pub product_id: Option<Uuid>,
        #[serde(rename = "totalPrice")]
        pub total_price: f64,
        #[serde(rename = "lineItemType")]
        pub line_item_type: Option<LineItemType>,
        #[serde(rename = "options")]
        pub options: Vec<OrderLineItemOption>,
        #[serde(rename = "customTextFields")]
        pub custom_text_fields: Option<Vec<CustomTextField>>,
        #[serde(rename = "weight")]
        pub weight: Option<f64>,
        #[serde(rename = "sku")]
        pub sku: Option<String>,
        #[serde(rename = "discount")]
        pub discount: Option<f64>,
        #[serde(rename = "tax")]
        pub tax: Option<f64>,
        #[serde(rename = "taxGroupId")]
        pub tax_group_id: Option<String>,
        #[serde(rename = "taxIncludedInPrice")]
        pub tax_included_in_price: bool,
        #[serde(rename = "fulfillerId")]
        pub fulfiller_id: Option<String>,
        #[serde(rename = "variantId")]
        pub variant_id: Option<String>,
        #[serde(rename = "priceData")]
        pub price_data: PriceData,
        #[serde(rename = "mediaItem")]
        pub media_item: OrderMediaItem,
        #[serde(rename = "notes")]
        pub notes: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct CustomTextField {
        #[serde(rename = "title")]
        pub title: String,
        #[serde(rename = "value")]
        pub value: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum LineItemType {
        #[serde(rename = "DIGITAL")]
        Digital,
        #[serde(rename = "PHYSICAL")]
        Physical,
        #[serde(rename = "CUSTOM_AMOUNT_ITEM")]
        CustomAmountItem,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]

    pub struct OrderLineItemOption {
        #[serde(rename = "option")]
        pub option: String,
        #[serde(rename = "selection")]
        pub selection: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct OrderMediaItem {
        #[serde(rename = "altText")]
        pub alt_text: Option<String>,
        #[serde(rename = "id")]
        pub id: String,
        #[serde(rename = "src")]
        pub src: String,
        #[serde(rename = "type")]
        r#type: OrderMediaItemType,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum OrderMediaItemType {
        #[serde(rename = "IMAGE")]
        Image,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Totals {
        #[serde(rename = "discount")]
        pub discount: f64,
        #[serde(rename = "quantity")]
        pub quantity: u64,
        #[serde(rename = "shipping")]
        pub shipping: f64,
        #[serde(rename = "subtotal")]
        pub subtotal: f64,
        #[serde(rename = "tax")]
        pub tax: f64,
        #[serde(rename = "total")]
        pub total: f64,
        #[serde(rename = "weight")]
        pub weight: f64,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum WeightUnit {
        #[serde(rename = "KG")]
        Kg,
        #[serde(rename = "LB")]
        Lb,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct CustomField {
        #[serde(rename = "value")]
        pub value: String,
        #[serde(rename = "title")]
        pub title: String,
        #[serde(rename = "translatedTitle")]
        pub translated_title: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Discount {
        #[serde(rename = "appliedCoupon")]
        pub applied_coupon: AppliedCoupon,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct AppliedCoupon {
        #[serde(rename = "code")]
        pub code: String,
        #[serde(rename = "couponId")]
        pub coupon_id: String,
        #[serde(rename = "name")]
        pub name: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct SubscriptionInfo {
        #[serde(rename = "id")]
        pub id: Uuid,
        #[serde(rename = "cycleNumber")]
        pub cycle_number: u32,
        #[serde(rename = "subscriptionSettings")]
        pub subscription_settings: SubscriptionSettings,
        #[serde(rename = "subscriptionOptionInfo")]
        pub subscription_option_info: SubscriptionOptionInfo,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct SubscriptionSettings {
        #[serde(rename = "frequency")]
        pub frequency: SubscroptionFrequency,
        #[serde(rename = "autoRenewal")]
        pub auto_renewal: bool,
        #[serde(rename = "billingCycles")]
        pub billing_cycles: u32,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum SubscroptionFrequency {
        #[serde(rename = "DAY")]
        Day,

        #[serde(rename = "WEEK")]
        Week,

        #[serde(rename = "MONTH")]
        Month,

        #[serde(rename = "YEAR")]
        Year,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct SubscriptionOptionInfo {
        #[serde(rename = "id")]
        pub id: Uuid,
        #[serde(rename = "title")]
        pub title: String,
        #[serde(rename = "description")]
        pub description: String,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientMsg {
    BreakCompleted { order_id: Uuid },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerMsg {
    NewOrder(NewOrder),
    BreakCompletedSuccess { order_id: Uuid },
}
