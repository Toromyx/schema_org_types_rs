/// <https://schema.org/OfferItemCondition>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
