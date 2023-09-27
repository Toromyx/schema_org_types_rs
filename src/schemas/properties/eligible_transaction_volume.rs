use super::*;
/// <https://schema.org/eligibleTransactionVolume>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum EligibleTransactionVolumeProperty {
    #[cfg(any(
        any(
            feature = "price-specification-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    PriceSpecification(PriceSpecification),
}
