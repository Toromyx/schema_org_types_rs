/// A list of possible product availability options.
///
/// https://schema.org/ItemAvailability
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum ItemAvailability {
    /// Indicates that the item is available on back order.
    ///
    /// https://schema.org/BackOrder
    BackOrder,
    /// Indicates that the item has been discontinued.
    ///
    /// https://schema.org/Discontinued
    Discontinued,
    /// Indicates that the item is in stock.
    ///
    /// https://schema.org/InStock
    InStock,
    /// Indicates that the item is available only at physical locations.
    ///
    /// https://schema.org/InStoreOnly
    InStoreOnly,
    /// Indicates that the item has limited availability.
    ///
    /// https://schema.org/LimitedAvailability
    LimitedAvailability,
    /// Indicates that the item is available only online.
    ///
    /// https://schema.org/OnlineOnly
    OnlineOnly,
    /// Indicates that the item is out of stock.
    ///
    /// https://schema.org/OutOfStock
    OutOfStock,
    /// Indicates that the item is available for pre-order.
    ///
    /// https://schema.org/PreOrder
    PreOrder,
    /// Indicates that the item is available for ordering and delivery before general availability.
    ///
    /// https://schema.org/PreSale
    PreSale,
    /// Indicates that the item has sold out.
    ///
    /// https://schema.org/SoldOut
    SoldOut,
}
