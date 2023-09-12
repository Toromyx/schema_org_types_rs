use super::*;
/// An intended audience, i.e. a group for whom something was created.
///
/// https://schema.org/audience
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AudienceProperty {
    #[cfg(any(feature = "audience-schema", feature = "general-schema-section"))]
    Audience(Audience),
}
