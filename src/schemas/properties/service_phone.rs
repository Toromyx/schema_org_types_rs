use super::*;
/// The phone number to use to access the service.
///
/// https://schema.org/servicePhone
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ServicePhoneProperty {
    #[cfg(any(feature = "contact-point-schema", feature = "general-schema-section"))]
    ContactPoint(ContactPoint),
}