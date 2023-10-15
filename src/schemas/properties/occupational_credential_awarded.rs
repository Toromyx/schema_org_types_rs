use super::*;
/// <https://schema.org/occupationalCredentialAwarded>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OccupationalCredentialAwardedProperty {
    #[cfg(any(
        any(
            feature = "educational-occupational-credential-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
