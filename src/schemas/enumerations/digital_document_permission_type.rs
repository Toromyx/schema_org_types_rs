/// <https://schema.org/DigitalDocumentPermissionType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum DigitalDocumentPermissionType {
    /// <https://schema.org/CommentPermission>
    CommentPermission,
    /// <https://schema.org/ReadPermission>
    ReadPermission,
    /// <https://schema.org/WritePermission>
    WritePermission,
}
