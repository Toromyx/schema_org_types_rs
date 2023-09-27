use super::*;
/// <https://schema.org/offers>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum OffersProperty {
    #[cfg(any(
        any(feature = "demand-schema", feature = "general-schema-section"),
        doc
    ))]
    Demand(Demand),
    #[cfg(any(any(feature = "offer-schema", feature = "general-schema-section"), doc))]
    Offer(Offer),
}
