use super::*;
/// A contact location for a person's place of work.
///
/// https://schema.org/workLocation
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum WorkLocationProperty {
    #[cfg(any(feature = "contact-point-schema", feature = "general-schema-section"))]
    ContactPoint(ContactPoint),
    #[cfg(any(feature = "place-schema", feature = "general-schema-section"))]
    Place(Place),
}
