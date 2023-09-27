/// <https://schema.org/OfferItemCondition>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum OfferItemCondition {
    /// <https://schema.org/DamagedCondition>
    DamagedCondition,
    /// <https://schema.org/NewCondition>
    NewCondition,
    /// <https://schema.org/RefurbishedCondition>
    RefurbishedCondition,
    /// <https://schema.org/UsedCondition>
    UsedCondition,
}
