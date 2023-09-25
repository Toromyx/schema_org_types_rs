use super::*;
/// A description of the qualification, award, certificate, diploma or other occupational credential awarded as a consequence of successful completion of this course or program.
///
/// <https://schema.org/occupationalCredentialAwarded>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum OccupationalCredentialAwardedProperty {
    #[cfg(any(
        any(
            feature = "educational-occupational-credential-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
}
