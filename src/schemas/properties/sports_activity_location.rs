use super::*;
/// A sub property of location. The sports activity location where this action occurred.
///
/// https://schema.org/sportsActivityLocation
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SportsActivityLocationProperty {
    #[cfg(any(
        feature = "sports-activity-location-schema",
        feature = "general-schema-section"
    ))]
    SportsActivityLocation(SportsActivityLocation),
}
