use super::*;
/// <https://schema.org/foodEstablishment>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
