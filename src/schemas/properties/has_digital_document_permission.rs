use super::*;
/// <https://schema.org/hasDigitalDocumentPermission>
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
