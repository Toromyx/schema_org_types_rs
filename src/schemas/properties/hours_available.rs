use super::*;
/// The hours during which this service or contact is available.
///
/// https://schema.org/hoursAvailable
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HoursAvailableProperty {
    #[cfg(any(
        feature = "opening-hours-specification-schema",
        feature = "general-schema-section"
    ))]
    OpeningHoursSpecification(OpeningHoursSpecification),
}
