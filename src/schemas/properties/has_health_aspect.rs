use super::*;
/// <https://schema.org/hasHealthAspect>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasHealthAspectProperty {
	#[cfg(any(
		any(
			feature = "health-aspect-enumeration-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	HealthAspectEnumeration(HealthAspectEnumeration),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::FailValue),
}
