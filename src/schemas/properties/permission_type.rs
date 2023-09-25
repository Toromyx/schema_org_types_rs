use super::*;
/// The type of permission granted the person, organization, or audience.
///
/// <https://schema.org/permissionType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
