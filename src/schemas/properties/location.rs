use super::*;
/// The location of, for example, where an event is happening, where an organization is located, or where an action takes place.
///
/// <https://schema.org/location>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum LocationProperty {
    #[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
    Place(Place),
    #[cfg(any(
        any(feature = "postal-address-schema", feature = "general-schema-section"),
        doc
    ))]
    PostalAddress(PostalAddress),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
    #[cfg(any(
        any(
            feature = "virtual-location-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    VirtualLocation(VirtualLocation),
}
