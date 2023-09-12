/// A list of possible conditions for the item.
///
/// https://schema.org/OfferItemCondition
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
