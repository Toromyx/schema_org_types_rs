/// <https://schema.org/DigitalDocumentPermissionType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DigitalDocumentPermissionType {
	/// <https://schema.org/CommentPermission>
	CommentPermission,
	/// <https://schema.org/ReadPermission>
	ReadPermission,
	/// <https://schema.org/WritePermission>
	WritePermission,
}
