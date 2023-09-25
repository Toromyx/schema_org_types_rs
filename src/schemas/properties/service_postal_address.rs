use super::*;
/// The address for accessing the service by mail.
///
/// <https://schema.org/servicePostalAddress>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ServicePostalAddressProperty {
    #[cfg(any(
        any(feature = "postal-address-schema", feature = "general-schema-section"),
        doc
    ))]
    PostalAddress(PostalAddress),
}
