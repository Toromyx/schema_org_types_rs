use super::*;
/// The location of, for example, where an event is happening, where an organization is located, or where an action takes place.
///
/// https://schema.org/location
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LocationProperty {
    #[cfg(any(feature = "place-schema", feature = "general-schema-section"))]
    Place(Place),
    #[cfg(any(feature = "postal-address-schema", feature = "general-schema-section"))]
    PostalAddress(PostalAddress),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
    #[cfg(any(
        feature = "virtual-location-schema",
        feature = "pending-schema-section"
    ))]
    VirtualLocation(VirtualLocation),
}
