use super::*;
/// Software application help.
///
/// https://schema.org/softwareHelp
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SoftwareHelpProperty {
    #[cfg(any(feature = "creative-work-schema", feature = "general-schema-section"))]
    CreativeWork(CreativeWork),
}
