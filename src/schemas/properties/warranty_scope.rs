use super::*;
/// <https://schema.org/warrantyScope>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum WarrantyScopeProperty {
	#[cfg(any(
		any(feature = "warranty-scope-schema", feature = "general-schema-section"),
		doc
	))]
	WarrantyScope(WarrantyScope),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::FailValue),
}
