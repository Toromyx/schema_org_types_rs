use super::*;
/// <https://schema.org/servicePostalAddress>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ServicePostalAddressProperty {
	#[cfg(any(
		any(feature = "postal-address-schema", feature = "general-schema-section"),
		doc
	))]
	PostalAddress(PostalAddress),
}
