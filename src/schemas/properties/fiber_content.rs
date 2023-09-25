use super::*;
/// The number of grams of fiber.
///
/// <https://schema.org/fiberContent>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum FiberContentProperty {
    #[cfg(any(any(feature = "mass-schema", feature = "general-schema-section"), doc))]
    Mass(Mass),
}
