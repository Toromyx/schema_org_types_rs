use super::*;
/// <https://schema.org/includesAttraction>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IncludesAttractionProperty {
	#[cfg(any(
		any(
			feature = "tourist-attraction-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	TouristAttraction(TouristAttraction),
}
