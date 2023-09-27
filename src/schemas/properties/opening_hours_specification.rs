use super::*;
/// <https://schema.org/openingHoursSpecification>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OpeningHoursSpecificationProperty {
    #[cfg(any(
        any(
            feature = "opening-hours-specification-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    OpeningHoursSpecification(OpeningHoursSpecification),
}
