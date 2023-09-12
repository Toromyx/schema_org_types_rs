use super::*;
/// The type of permission granted the person, organization, or audience.
///
/// https://schema.org/permissionType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PermissionTypeProperty {
    #[cfg(any(
        feature = "digital-document-permission-type-schema",
        feature = "general-schema-section"
    ))]
    DigitalDocumentPermissionType(DigitalDocumentPermissionType),
}
