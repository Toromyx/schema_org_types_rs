use super::*;
/// The number to access the service by text message.
///
/// https://schema.org/serviceSmsNumber
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ServiceSmsNumberProperty {
    #[cfg(any(
        any(feature = "contact-point-schema", feature = "general-schema-section"),
        doc
    ))]
    ContactPoint(ContactPoint),
}
