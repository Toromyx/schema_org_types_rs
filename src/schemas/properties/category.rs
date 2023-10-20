use super::*;
/// <https://schema.org/category>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CategoryProperty {
	#[cfg(any(
		any(feature = "category-code-schema", feature = "pending-schema-section"),
		doc
	))]
	CategoryCode(CategoryCode),
	#[cfg(any(any(feature = "thing-schema", feature = "general-schema-section"), doc))]
	Thing(Thing),
	#[cfg(any(
		any(
			feature = "physical-activity-category-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	PhysicalActivityCategory(PhysicalActivityCategory),
	#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
	Url(Url),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
	Text(Text),
}
