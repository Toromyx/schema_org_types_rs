use super::*;
/// <https://schema.org/seeks>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SeeksProperty {
	#[cfg(any(
		any(feature = "demand-schema", feature = "general-schema-section"),
		doc
	))]
	Demand(Demand),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::FailValue),
}
