use super::*;
/// The transaction volume, in a monetary unit, for which the offer or price specification is valid, e.g. for indicating a minimal purchasing volume, to express free shipping above a certain order volume, or to limit the acceptance of credit cards to purchases to a certain minimal amount.
///
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
