use super::*;
/// A sub property of location. The specific food establishment where the action occurred.
///
/// https://schema.org/foodEstablishment
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FoodEstablishmentProperty {
    #[cfg(any(
        feature = "food-establishment-schema",
        feature = "general-schema-section"
    ))]
    FoodEstablishment(FoodEstablishment),
    #[cfg(any(feature = "place-schema", feature = "general-schema-section"))]
    Place(Place),
}
