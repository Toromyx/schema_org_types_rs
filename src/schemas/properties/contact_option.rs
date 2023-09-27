use super::*;
/// <https://schema.org/contactOption>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ContactOptionProperty {
    #[cfg(any(
        any(
            feature = "contact-point-option-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    ContactPointOption(ContactPointOption),
}
