use super::*;
/// <https://schema.org/touristType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TouristTypeProperty {
    #[cfg(any(
        any(feature = "audience-schema", feature = "general-schema-section"),
        doc
    ))]
    Audience(Audience),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
