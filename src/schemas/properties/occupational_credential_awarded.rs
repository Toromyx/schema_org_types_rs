use super::*;
/// A description of the qualification, award, certificate, diploma or other occupational credential awarded as a consequence of successful completion of this course or program.
///
/// https://schema.org/occupationalCredentialAwarded
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OccupationalCredentialAwardedProperty {
    #[cfg(any(
        feature = "educational-occupational-credential-schema",
        feature = "pending-schema-section"
    ))]
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
