use super::*;
/// A credential awarded to the Person or Organization.
///
/// https://schema.org/hasCredential
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum HasCredentialProperty {
    #[cfg(any(
        any(
            feature = "educational-occupational-credential-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    EducationalOccupationalCredential(EducationalOccupationalCredential),
}
