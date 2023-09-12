use super::*;
/// A credential awarded to the Person or Organization.
///
/// https://schema.org/hasCredential
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasCredentialProperty {
    #[cfg(any(
        feature = "educational-occupational-credential-schema",
        feature = "pending-schema-section"
    ))]
    EducationalOccupationalCredential(EducationalOccupationalCredential),
}
