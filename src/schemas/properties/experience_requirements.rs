use super::*;
/// Description of skills and experience needed for the position or Occupation.
///
/// https://schema.org/experienceRequirements
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ExperienceRequirementsProperty {
    #[cfg(any(
        feature = "occupational-experience-requirements-schema",
        feature = "pending-schema-section"
    ))]
    OccupationalExperienceRequirements(OccupationalExperienceRequirements),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
