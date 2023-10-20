use super::*;
/// <https://schema.org/model>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ModelProperty {
	#[cfg(any(
		any(feature = "product-model-schema", feature = "general-schema-section"),
		doc
	))]
	ProductModel(ProductModel),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
	Text(Text),
}
