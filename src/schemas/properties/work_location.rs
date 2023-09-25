use super::*;
/// A contact location for a person's place of work.
///
/// https://schema.org/workLocation
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum WorkLocationProperty {
    #[cfg(any(
        any(feature = "contact-point-schema", feature = "general-schema-section"),
        doc
    ))]
    ContactPoint(ContactPoint),
    #[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
    Place(Place),
}
