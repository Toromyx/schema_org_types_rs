use super::*;
/// <https://schema.org/byMonthDay>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ByMonthDayProperty {
	#[cfg(any(
		any(feature = "integer-schema", feature = "general-schema-section"),
		doc
	))]
	Integer(Integer),
}
