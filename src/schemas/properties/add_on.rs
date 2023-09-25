use super::*;
/// An additional offer that can only be obtained in combination with the first base offer (e.g. supplements and extensions that are available for a surcharge).
///
/// <https://schema.org/addOn>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AddOnProperty {
    #[cfg(any(any(feature = "offer-schema", feature = "general-schema-section"), doc))]
    Offer(Offer),
}
