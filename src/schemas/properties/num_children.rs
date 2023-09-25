use super::*;
/// The number of children staying in the unit.
///
/// <https://schema.org/numChildren>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum NumChildrenProperty {
    #[cfg(any(
        any(feature = "integer-schema", feature = "general-schema-section"),
        doc
    ))]
    Integer(Integer),
    #[cfg(any(
        any(
            feature = "quantitative-value-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    QuantitativeValue(QuantitativeValue),
}
