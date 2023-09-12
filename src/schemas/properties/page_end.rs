use super::*;
/// The page on which the work ends; for example "138" or "xvi".
///
/// https://schema.org/pageEnd
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PageEndProperty {
    #[cfg(any(feature = "integer-schema", feature = "general-schema-section"))]
    Integer(Integer),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
