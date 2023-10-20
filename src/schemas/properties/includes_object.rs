use super::*;
/// <https://schema.org/includesObject>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IncludesObjectProperty {
	#[cfg(any(
		any(
			feature = "type-and-quantity-node-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	TypeAndQuantityNode(TypeAndQuantityNode),
}
