use super::*;
/// <https://schema.org/size>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SizeProperty {
    #[cfg(any(
        any(feature = "defined-term-schema", feature = "pending-schema-section"),
        doc
    ))]
    DefinedTerm(DefinedTerm),
    #[cfg(any(
        any(
            feature = "quantitative-value-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    QuantitativeValue(QuantitativeValue),
    #[cfg(any(
        any(
            feature = "size-specification-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    SizeSpecification(SizeSpecification),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
