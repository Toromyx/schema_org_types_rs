use super::*;
/// <https://schema.org/hospitalAffiliation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HospitalAffiliationProperty {
	#[cfg(any(
		any(feature = "hospital-schema", feature = "general-schema-section"),
		doc
	))]
	Hospital(Hospital),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::FailValue),
}
