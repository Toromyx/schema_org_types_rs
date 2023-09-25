/// A list of possible conditions for the item.
///
/// https://schema.org/OfferItemCondition
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum OfferItemCondition {
    /// Indicates that the item is damaged.
    ///
    /// https://schema.org/DamagedCondition
    DamagedCondition,
    /// Indicates that the item is new.
    ///
    /// https://schema.org/NewCondition
    NewCondition,
    /// Indicates that the item is refurbished.
    ///
    /// https://schema.org/RefurbishedCondition
    RefurbishedCondition,
    /// Indicates that the item is used.
    ///
    /// https://schema.org/UsedCondition
    UsedCondition,
}
