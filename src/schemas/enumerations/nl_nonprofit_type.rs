/// <https://schema.org/NLNonprofitType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum NlNonprofitType {
    /// <https://schema.org/NonprofitANBI>
    NonprofitAnbi,
    /// <https://schema.org/NonprofitSBBI>
    NonprofitSbbi,
}
