use super::*;
/// <https://schema.org/applicationContact>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ApplicationContactProperty {
    #[cfg(any(
        any(feature = "contact-point-schema", feature = "general-schema-section"),
        doc
    ))]
    ContactPoint(ContactPoint),
}
