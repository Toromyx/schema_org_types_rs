use super::*;
/// The number of credits or units awarded by a Course or required to complete an EducationalOccupationalProgram.
///
/// https://schema.org/numberOfCredits
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum NumberOfCreditsProperty {
    #[cfg(any(feature = "integer-schema", feature = "general-schema-section"))]
    Integer(Integer),
    #[cfg(any(
        feature = "structured-value-schema",
        feature = "general-schema-section"
    ))]
    StructuredValue(StructuredValue),
}
