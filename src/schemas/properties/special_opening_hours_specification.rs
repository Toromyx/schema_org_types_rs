use super::*;
/// The special opening hours of a certain place.\n\nUse this to explicitly override general opening hours brought in scope by [[openingHoursSpecification]] or [[openingHours]].
///
///
/// https://schema.org/specialOpeningHoursSpecification
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SpecialOpeningHoursSpecificationProperty {
    #[cfg(any(
        any(
            feature = "opening-hours-specification-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    OpeningHoursSpecification(OpeningHoursSpecification),
}
