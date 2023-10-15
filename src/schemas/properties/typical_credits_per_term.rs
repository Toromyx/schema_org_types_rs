use super::*;
/// <https://schema.org/typicalCreditsPerTerm>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TypicalCreditsPerTermProperty {
    #[cfg(any(
        any(
            feature = "structured-value-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    StructuredValue(StructuredValue),
    #[cfg(any(
        any(feature = "integer-schema", feature = "general-schema-section"),
        doc
    ))]
    Integer(Integer),
}
