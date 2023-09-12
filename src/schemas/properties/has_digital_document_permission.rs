use super::*;
/// A permission related to the access to this document (e.g. permission to read or write an electronic document). For a public document, specify a grantee with an Audience with audienceType equal to "public".
///
/// https://schema.org/hasDigitalDocumentPermission
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasDigitalDocumentPermissionProperty {
    #[cfg(any(
        feature = "digital-document-permission-schema",
        feature = "general-schema-section"
    ))]
    DigitalDocumentPermission(DigitalDocumentPermission),
}
