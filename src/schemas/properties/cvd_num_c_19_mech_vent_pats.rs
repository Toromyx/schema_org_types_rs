use super::*;
/// <https://schema.org/cvdNumC19MechVentPats>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CvdNumC19MechVentPatsProperty {
	#[cfg(any(
		any(feature = "number-schema", feature = "general-schema-section"),
		doc
	))]
	Number(Number),
}
