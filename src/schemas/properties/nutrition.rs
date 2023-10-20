use super::*;
/// <https://schema.org/nutrition>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum NutritionProperty {
	#[cfg(any(
		any(
			feature = "nutrition-information-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	NutritionInformation(NutritionInformation),
}
