use super::*;
/// Textual description of the unit type (including suite vs. room, size of bed, etc.).
///
/// https://schema.org/lodgingUnitType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LodgingUnitTypeProperty {
    #[cfg(any(
        feature = "qualitative-value-schema",
        feature = "general-schema-section"
    ))]
    QualitativeValue(QualitativeValue),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
