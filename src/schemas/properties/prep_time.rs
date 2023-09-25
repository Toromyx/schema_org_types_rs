use super::*;
/// The length of time it takes to prepare the items to be used in instructions or a direction, in [ISO 8601 duration format](http://en.wikipedia.org/wiki/ISO_8601).
///
/// <https://schema.org/prepTime>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PrepTimeProperty {
    #[cfg(any(
        any(feature = "duration-schema", feature = "general-schema-section"),
        doc
    ))]
    Duration(Duration),
}
