/// A type of permission which can be granted for accessing a digital document.
///
/// https://schema.org/DigitalDocumentPermissionType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DigitalDocumentPermissionType {
    /// Permission to add comments to the document.
    ///
    /// https://schema.org/CommentPermission
    CommentPermission,
    /// Permission to read or view the document.
    ///
    /// https://schema.org/ReadPermission
    ReadPermission,
    /// Permission to write or edit the document.
    ///
    /// https://schema.org/WritePermission
    WritePermission,
}
