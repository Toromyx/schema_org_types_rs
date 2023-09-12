use super::*;
/// The location(s) applicants can apply from. This is usually used for telecommuting jobs where the applicant does not need to be in a physical office. Note: This should not be used for citizenship or work visa requirements.
///
/// https://schema.org/applicantLocationRequirements
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ApplicantLocationRequirementsProperty {
    #[cfg(any(
        feature = "administrative-area-schema",
        feature = "general-schema-section"
    ))]
    AdministrativeArea(AdministrativeArea),
}
