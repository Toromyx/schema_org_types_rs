use super::*;
/// An additional offer that can only be obtained in combination with the first base offer (e.g. supplements and extensions that are available for a surcharge).
///
/// https://schema.org/addOn
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AddOnProperty {
    #[cfg(any(feature = "offer-schema", feature = "general-schema-section"))]
    Offer(Offer),
}
