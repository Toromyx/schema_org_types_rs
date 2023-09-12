use super::*;
/// An item being offered (or demanded). The transactional nature of the offer or demand is documented using [[businessFunction]], e.g. sell, lease etc. While several common expected types are listed explicitly in this definition, others can be used. Using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
///
/// https://schema.org/itemOffered
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ItemOfferedProperty {
    #[cfg(any(feature = "aggregate-offer-schema", feature = "general-schema-section"))]
    AggregateOffer(AggregateOffer),
    #[cfg(any(feature = "creative-work-schema", feature = "general-schema-section"))]
    CreativeWork(CreativeWork),
    #[cfg(any(feature = "event-schema", feature = "general-schema-section"))]
    Event(Event),
    #[cfg(any(feature = "menu-item-schema", feature = "general-schema-section"))]
    MenuItem(MenuItem),
    #[cfg(any(feature = "product-schema", feature = "general-schema-section"))]
    Product(Product),
    #[cfg(any(feature = "service-schema", feature = "general-schema-section"))]
    Service(Service),
    #[cfg(any(feature = "trip-schema", feature = "general-schema-section"))]
    Trip(Trip),
}
