use super::*;
/// An option available on this contact point (e.g. a toll-free number or support for hearing-impaired callers).
///
/// https://schema.org/contactOption
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ContactOptionProperty {
    #[cfg(any(
        feature = "contact-point-option-schema",
        feature = "general-schema-section"
    ))]
    ContactPointOption(ContactPointOption),
}
