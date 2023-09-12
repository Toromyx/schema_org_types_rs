use super::*;
/// The location in which the strength is available.
///
/// https://schema.org/availableIn
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AvailableInProperty {
    #[cfg(any(
        feature = "administrative-area-schema",
        feature = "general-schema-section"
    ))]
    AdministrativeArea(AdministrativeArea),
}
