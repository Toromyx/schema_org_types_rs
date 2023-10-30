use super::*;
/// <https://schema.org/warrantyPromise>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum WarrantyPromiseProperty {
	#[cfg(any(
		any(
			feature = "warranty-promise-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	WarrantyPromise(WarrantyPromise),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::FailValue),
}
