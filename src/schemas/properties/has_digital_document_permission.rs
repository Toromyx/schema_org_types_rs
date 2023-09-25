use super::*;
/// A permission related to the access to this document (e.g. permission to read or write an electronic document). For a public document, specify a grantee with an Audience with audienceType equal to "public".
///
/// https://schema.org/hasDigitalDocumentPermission
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum HasDigitalDocumentPermissionProperty {
    #[cfg(any(
        any(
            feature = "digital-document-permission-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    DigitalDocumentPermission(DigitalDocumentPermission),
}
