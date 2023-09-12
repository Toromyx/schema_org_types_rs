use super::*;
/// The special opening hours of a certain place.\n\nUse this to explicitly override general opening hours brought in scope by [[openingHoursSpecification]] or [[openingHours]].
///
///
/// https://schema.org/specialOpeningHoursSpecification
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SpecialOpeningHoursSpecificationProperty {
    #[cfg(any(
        feature = "opening-hours-specification-schema",
        feature = "general-schema-section"
    ))]
    OpeningHoursSpecification(OpeningHoursSpecification),
}
