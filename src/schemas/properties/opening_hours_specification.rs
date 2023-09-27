use super::*;
/// <https://schema.org/openingHoursSpecification>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
