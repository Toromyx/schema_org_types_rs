use super::*;
/// <https://schema.org/experienceRequirements>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
