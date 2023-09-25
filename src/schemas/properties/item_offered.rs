use super::*;
/// An item being offered (or demanded). The transactional nature of the offer or demand is documented using [[businessFunction]], e.g. sell, lease etc. While several common expected types are listed explicitly in this definition, others can be used. Using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
///
/// <https://schema.org/itemOffered>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
