use super::*;
/// Indicates a target EntryPoint, or url, for an Action.
///
/// https://schema.org/target
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TargetProperty {
    #[cfg(any(feature = "entry-point-schema", feature = "general-schema-section"))]
    EntryPoint(EntryPoint),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
