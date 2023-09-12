use super::*;
/// A number associated with a role in an organization, for example, the number on an athlete's jersey.
///
/// https://schema.org/numberedPosition
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum NumberedPositionProperty {
    #[cfg(any(feature = "number-schema", feature = "general-schema-section"))]
    Number(Number),
}
