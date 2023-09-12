use super::*;
/// Educational background needed for the position or Occupation.
///
/// https://schema.org/educationRequirements
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EducationRequirementsProperty {
    #[cfg(any(
        feature = "educational-occupational-credential-schema",
        feature = "pending-schema-section"
    ))]
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
