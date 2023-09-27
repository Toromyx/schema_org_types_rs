use super::*;
/// <https://schema.org/sportsActivityLocation>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SportsActivityLocationProperty {
    #[cfg(any(
        any(
            feature = "sports-activity-location-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    SportsActivityLocation(SportsActivityLocation),
}
