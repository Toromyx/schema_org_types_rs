use super::*;
/// <https://schema.org/experienceRequirements>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ExperienceRequirementsProperty {
	#[cfg(any(
		any(
			feature = "occupational-experience-requirements-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	OccupationalExperienceRequirements(OccupationalExperienceRequirements),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
	Text(Text),
}
