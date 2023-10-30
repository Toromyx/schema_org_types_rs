use super::*;
/// <https://schema.org/predecessorOf>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PredecessorOfProperty {
	#[cfg(any(
		any(feature = "product-model-schema", feature = "general-schema-section"),
		doc
	))]
	ProductModel(ProductModel),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::FailValue),
}
