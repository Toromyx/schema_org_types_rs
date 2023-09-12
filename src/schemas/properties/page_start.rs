use super::*;
/// The page on which the work starts; for example "135" or "xiii".
///
/// https://schema.org/pageStart
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PageStartProperty {
    #[cfg(any(feature = "integer-schema", feature = "general-schema-section"))]
    Integer(Integer),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
