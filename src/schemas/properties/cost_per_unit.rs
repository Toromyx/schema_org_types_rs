use super::*;
/// The cost per unit of the drug.
///
/// <https://schema.org/costPerUnit>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum CostPerUnitProperty {
    #[cfg(any(
        any(feature = "number-schema", feature = "general-schema-section"),
        doc
    ))]
    Number(Number),
    #[cfg(any(
        any(
            feature = "qualitative-value-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    QualitativeValue(QualitativeValue),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
