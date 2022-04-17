use serde::{Deserialize, Serialize};
use time::{serde::rfc3339, Date, OffsetDateTime, Time};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewOrder {
    #[serde(rename = "_id")]
    _id: Uuid,
    #[serde(rename = "_updatedDate", with = "rfc3339")]
    _updated_date: OffsetDateTime,
    #[serde(rename = "buyerLanguage")]
    buyer_language: BuyerLanguage,
    #[serde(rename = "cartId")]
    cart_id: Option<Uuid>,
    #[serde(rename = "channelInfo")]
    channel_info: ChannelInfo,
    #[serde(rename = "enteredBy")]
    entered_by: EnteredBy,
    #[serde(rename = "billingInfo")]
    billing_info: Option<BillingInfo>,
    #[serde(rename = "buyerInfo")]
    buyer_info: BuyerInfo,
    #[serde(rename = "buyerNote")]
    buyer_note: Option<String>,
    #[serde(rename = "_dateCreated", with = "rfc3339")]
    _date_created: OffsetDateTime,
    #[serde(rename = "currency")]
    currency: String,
    #[serde(rename = "fulfillmentStatus")]
    fulfillment_status: FulfillmentStatus,
    #[serde(rename = "fulfillments")]
    fulfillments: Option<Vec<Fulfillment>>,
    #[serde(rename = "archived")]
    archived: bool,
    #[serde(rename = "activities")]
    activities: Vec<Activity>,
    #[serde(rename = "number")]
    number: u64,
    #[serde(rename = "paymentStatus")]
    payment_status: PaymentStatus,
    #[serde(rename = "shippingInfo")]
    shipping_info: Option<ShippingInfo>,
    #[serde(rename = "lineItems")]
    line_items: Vec<OrderLineItem>,
    #[serde(rename = "totals")]
    totals: Totals,
    #[serde(rename = "weightUnit")]
    weight_unit: Option<WeightUnit>,
    #[serde(rename = "customField")]
    custom_field: Option<CustomField>,
    #[serde(rename = "discount")]
    discount: Option<Discount>,
    #[serde(rename = "refund")]
    refunds: Option<Vec<Refund>>,
    // The following field is only present when
    // the order is a subscription
    #[serde(rename = "subscriptionInfo")]
    subscription_info: Option<SubscriptionInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Refund {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "dateCreated", with = "rfc3339")]
    date_created: OffsetDateTime,
    #[serde(rename = "amount")]
    amount: String,
    #[serde(rename = "reason")]
    reason: Option<String>,
    #[serde(rename = "externalRefund")]
    external_refund: bool,
    #[serde(rename = "paymentProviderTransactionId")]
    payment_provider_transaction_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Fulfillment {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "dateCreated", with = "rfc3339")]
    date_created: OffsetDateTime,
    #[serde(rename = "lineItems")]
    line_items: Vec<FulfillmentLineItem>,
    #[serde(rename = "tackingInfo")]
    tacking_info: Option<TrackingInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackingInfo {
    #[serde(rename = "trackingNumber")]
    tracking_number: String,
    #[serde(rename = "shippingProvider")]
    shipping_provider: String,
    #[serde(rename = "trackingLink")]
    tracking_link: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FulfillmentLineItem {
    #[serde(rename = "index")]
    index: u32,
    #[serde(rename = "quantity")]
    quantity: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum BuyerLanguage {
    #[serde(rename = "en")]
    English,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChannelInfo {
    #[serde(rename = "externalOrderId")]
    external_order_id: Option<String>,
    #[serde(rename = "externalOrderUrl")]
    external_order_url: Option<String>,
    #[serde(rename = "type")]
    r#type: ChannelInfoType,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct EnteredBy {
    #[serde(rename = "id")]
    id: Uuid,
    #[serde(rename = "identityType")]
    identity_type: EnteredByIdentityType,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EnteredByIdentityType {
    #[serde(rename = "USER")]
    User,

    #[serde(rename = "MEMBER")]
    Member,

    #[serde(rename = "CONTACT")]
    Contact,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BillingInfo {
    #[serde(rename = "address")]
    address: Option<Address>,
    #[serde(rename = "firstName")]
    first_name: Option<String>,
    #[serde(rename = "lastName")]
    last_name: Option<String>,
    #[serde(rename = "email")]
    email: Option<String>,
    #[serde(rename = "phone")]
    phone: Option<String>,
    #[serde(rename = "company")]
    company: Option<String>,
    #[serde(rename = "vatId")]
    vat_id: Option<VatId>,
    #[serde(rename = "externalTransactionId")]
    #[deprecated(note = "replaced with paymentProviderTransactionId")]
    external_transaction_id: Option<Uuid>,
    #[serde(rename = "paidDate", with = "rfc3339::option")]
    paid_date: Option<OffsetDateTime>,
    // REVIEW: Enum? ("VISA")
    #[serde(rename = "paymentMethod")]
    payment_method: Option<String>,
    // REVIEW: Uuid?
    #[serde(rename = "paymentGatewayTransactionId")]
    payment_gateway_transaction_id: Option<String>,
    #[serde(rename = "paymentProviderTransactionId")]
    payment_provider_transaction_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    #[serde(rename = "formatted")]
    formatted: Option<String>,
    #[serde(rename = "city")]
    city: String,
    #[serde(rename = "country")]
    country: String,
    #[serde(rename = "addressLine")]
    address_line: String,
    #[serde(rename = "addressLine2")]
    address_line_2: Option<String>,
    #[serde(rename = "streetAddress")]
    street_address: Option<StreetAddress>,
    #[serde(rename = "postalCode")]
    postal_code: String,
    #[serde(rename = "subdivision")]
    subdivision: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StreetAddress {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "number")]
    number: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VatId {
    #[serde(rename = "number")]
    number: String,
    #[serde(rename = "type")]
    r#type: VatIdType,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum VatIdType {
    #[serde(rename = "CPF")]
    Cpf,
    #[serde(rename = "CNJP")]
    Cnjp,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BuyerInfo {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(rename = "phone")]
    phone: Option<String>,
    #[serde(rename = "identityType")]
    identity_type: IdentityType,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct PriceData {
    #[serde(rename = "price")]
    price: String,
    #[serde(rename = "totalPrice")]
    total_price: f64,
    #[serde(rename = "taxIncludedInPrice")]
    tax_included_in_price: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShippingInfo {
    #[serde(rename = "deliverByDate")]
    deliver_by_date: Option<String>,
    #[serde(rename = "deliveryOption")]
    delivery_option: Option<String>,
    #[serde(rename = "estimatedDeliveryTime")]
    estimated_delivery_time: Option<String>,
    #[serde(rename = "shipmentDetails")]
    // REVIEW: Make *_details an enum?
    shipment_details: ShipmentDetails,
    #[serde(rename = "pickupDetails")]
    pickup_details: Option<PickupDetails>,
    #[serde(rename = "shippingRegion")]
    shipping_region: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShipmentDetails {
    #[serde(rename = "address")]
    address: Address,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "phone")]
    phone: String,
    #[serde(rename = "company")]
    company: String,
    #[serde(rename = "tax")]
    tax: f64,
    #[serde(rename = "discount")]
    discount: f64,
    #[serde(rename = "priceData")]
    price_data: Option<ShipmentPriceData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PickupDetails {
    #[serde(rename = "pickupInstructions")]
    pickup_instructions: String,
    #[serde(rename = "pickupAddress")]
    pickup_address: Address,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "phone")]
    phone: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShipmentPriceData {
    #[serde(rename = "price")]
    price: String,
    #[serde(rename = "taxIncludedInPrice")]
    tax_included_in_price: bool,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Activity {
    #[serde(rename = "type")]
    r#type: ActivityType,
    #[serde(rename = "timestamp", with = "rfc3339")]
    timestamp: OffsetDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderLineItem {
    #[serde(rename = "index")]
    index: Option<u64>,
    #[serde(rename = "quantity")]
    quantity: u64,
    #[serde(rename = "price")]
    price: f64,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "translatedName")]
    translated_name: Option<String>,
    #[serde(rename = "productId")]
    product_id: Option<Uuid>,
    #[serde(rename = "totalPrice")]
    total_price: f64,
    #[serde(rename = "lineItemType")]
    line_item_type: Option<LineItemType>,
    #[serde(rename = "options")]
    options: Vec<OrderLineItemOption>,
    #[serde(rename = "customTextFields")]
    custom_text_fields: Option<Vec<CustomTextField>>,
    #[serde(rename = "weight")]
    weight: Option<f64>,
    #[serde(rename = "sku")]
    sku: Option<String>,
    #[serde(rename = "discount")]
    discount: Option<f64>,
    #[serde(rename = "tax")]
    tax: Option<f64>,
    #[serde(rename = "taxGroupId")]
    tax_group_id: Option<String>,
    #[serde(rename = "taxIncludedInPrice")]
    tax_included_in_price: bool,
    #[serde(rename = "fulfillerId")]
    fulfiller_id: Option<String>,
    #[serde(rename = "variantId")]
    variant_id: Option<String>,
    #[serde(rename = "priceData")]
    price_data: PriceData,
    #[serde(rename = "mediaItem")]
    media_item: OrderMediaItem,
    #[serde(rename = "notes")]
    notes: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomTextField {
    #[serde(rename = "title")]
    title: String,
    #[serde(rename = "value")]
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LineItemType {
    #[serde(rename = "DIGITAL")]
    Digital,
    #[serde(rename = "PHYSICAL")]
    Physical,
    #[serde(rename = "CUSTOM_AMOUNT_ITEM")]
    CustomAmountItem,
}
#[derive(Debug, Deserialize, Serialize)]

pub struct OrderLineItemOption {
    #[serde(rename = "option")]
    option: String,
    #[serde(rename = "selection")]
    selection: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderMediaItem {
    #[serde(rename = "altText")]
    alt_text: Option<String>,
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "src")]
    src: String,
    #[serde(rename = "type")]
    r#type: OrderMediaItemType,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OrderMediaItemType {
    #[serde(rename = "IMAGE")]
    Image,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Totals {
    #[serde(rename = "discount")]
    discount: f64,
    #[serde(rename = "quantity")]
    quantity: u64,
    #[serde(rename = "shipping")]
    shipping: f64,
    #[serde(rename = "subtotal")]
    subtotal: f64,
    #[serde(rename = "tax")]
    tax: f64,
    #[serde(rename = "total")]
    total: f64,
    #[serde(rename = "weight")]
    weight: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum WeightUnit {
    #[serde(rename = "KG")]
    Kg,
    #[serde(rename = "LB")]
    Lb,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomField {
    #[serde(rename = "value")]
    value: String,
    #[serde(rename = "title")]
    title: String,
    #[serde(rename = "translatedTitle")]
    translated_title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Discount {
    #[serde(rename = "appliedCoupon")]
    applied_coupon: AppliedCoupon,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppliedCoupon {
    #[serde(rename = "code")]
    code: String,
    #[serde(rename = "couponId")]
    coupon_id: String,
    #[serde(rename = "name")]
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriptionInfo {
    #[serde(rename = "id")]
    id: Uuid,
    #[serde(rename = "cycleNumber")]
    cycle_number: u32,
    #[serde(rename = "subscriptionSettings")]
    subscription_settings: SubscriptionSettings,
    #[serde(rename = "subscriptionOptionInfo")]
    subscription_option_info: SubscriptionOptionInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriptionSettings {
    #[serde(rename = "frequency")]
    frequency: SubscroptionFrequency,
    #[serde(rename = "autoRenewal")]
    auto_renewal: bool,
    #[serde(rename = "billingCycles")]
    billing_cycles: u32,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriptionOptionInfo {
    #[serde(rename = "id")]
    id: Uuid,
    #[serde(rename = "title")]
    title: String,
    #[serde(rename = "description")]
    description: String,
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

    let order: NewOrder = serde_json::from_str(JSON).unwrap();
}
