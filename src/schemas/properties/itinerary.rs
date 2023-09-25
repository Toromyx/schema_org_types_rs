use super::*;
/// Destination(s) ( [[Place]] ) that make up a trip. For a trip where destination order is important use [[ItemList]] to specify that order (see examples).
///
/// <https://schema.org/itinerary>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ItineraryProperty {
    #[cfg(any(
        any(feature = "item-list-schema", feature = "general-schema-section"),
        doc
    ))]
    ItemList(ItemList),
    #[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
    Place(Place),
}
