use super::*;
/// An honorific suffix following a Person's name such as M.D./PhD/MSCSW.
///
/// https://schema.org/honorificSuffix
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HonorificSuffixProperty {
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
