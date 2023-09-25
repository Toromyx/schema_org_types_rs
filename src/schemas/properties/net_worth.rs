use super::*;
/// The total financial value of the person as calculated by subtracting assets from liabilities.
///
/// <https://schema.org/netWorth>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum NetWorthProperty {
    #[cfg(any(
        any(feature = "monetary-amount-schema", feature = "general-schema-section"),
        doc
    ))]
    MonetaryAmount(MonetaryAmount),
    #[cfg(any(
        any(
            feature = "price-specification-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    PriceSpecification(PriceSpecification),
}
