use super::*;
/// Physical address of the item.
///
/// https://schema.org/address
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AddressProperty {
    #[cfg(any(feature = "postal-address-schema", feature = "general-schema-section"))]
    PostalAddress(PostalAddress),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
