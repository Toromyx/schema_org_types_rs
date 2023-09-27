use super::*;
/// <https://schema.org/permissionType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PermissionTypeProperty {
    #[cfg(any(
        any(
            feature = "digital-document-permission-type-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    DigitalDocumentPermissionType(DigitalDocumentPermissionType),
}
