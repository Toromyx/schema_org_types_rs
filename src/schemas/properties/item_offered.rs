use super::*;
/// <https://schema.org/itemOffered>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ItemOfferedProperty {
	#[cfg(any(
		any(feature = "aggregate-offer-schema", feature = "general-schema-section"),
		doc
	))]
	AggregateOffer(AggregateOffer),
	#[cfg(any(
		any(feature = "creative-work-schema", feature = "general-schema-section"),
		doc
	))]
	CreativeWork(CreativeWork),
	#[cfg(any(any(feature = "event-schema", feature = "general-schema-section"), doc))]
	Event(Event),
	#[cfg(any(
		any(feature = "menu-item-schema", feature = "general-schema-section"),
		doc
	))]
	MenuItem(MenuItem),
	#[cfg(any(
		any(feature = "product-schema", feature = "general-schema-section"),
		doc
	))]
	Product(Product),
	#[cfg(any(
		any(feature = "service-schema", feature = "general-schema-section"),
		doc
	))]
	Service(Service),
	#[cfg(any(any(feature = "trip-schema", feature = "general-schema-section"), doc))]
	Trip(Trip),
}
