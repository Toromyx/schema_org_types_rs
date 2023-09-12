use super::*;
/// Specific qualifications required for this role or Occupation.
///
/// https://schema.org/qualifications
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum QualificationsProperty {
    #[cfg(any(
        feature = "educational-occupational-credential-schema",
        feature = "pending-schema-section"
    ))]
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
