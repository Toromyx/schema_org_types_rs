use super::*;
/// The version of the CreativeWork embodied by a specified resource.
///
/// https://schema.org/version
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum VersionProperty {
    #[cfg(any(feature = "number-schema", feature = "general-schema-section"))]
    Number(Number),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
