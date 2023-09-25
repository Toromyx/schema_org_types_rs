use super::*;
/// A sub property of location. The specific food establishment where the action occurred.
///
/// https://schema.org/foodEstablishment
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum FoodEstablishmentProperty {
    #[cfg(any(
        any(
            feature = "food-establishment-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    FoodEstablishment(FoodEstablishment),
    #[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
    Place(Place),
}
