use super::*;
/// Days of the week when the merchant typically operates, indicated via opening hours markup.
///
/// https://schema.org/businessDays
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BusinessDaysProperty {
    #[cfg(any(
        feature = "opening-hours-specification-schema",
        feature = "general-schema-section"
    ))]
    OpeningHoursSpecification(OpeningHoursSpecification),
}
