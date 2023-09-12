use super::*;
/// Attraction suitable for type(s) of tourist. E.g. children, visitors from a particular country, etc.
///
/// https://schema.org/touristType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TouristTypeProperty {
    #[cfg(any(feature = "audience-schema", feature = "general-schema-section"))]
    Audience(Audience),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
