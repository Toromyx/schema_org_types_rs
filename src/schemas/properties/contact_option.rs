use super::*;
/// An option available on this contact point (e.g. a toll-free number or support for hearing-impaired callers).
///
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
