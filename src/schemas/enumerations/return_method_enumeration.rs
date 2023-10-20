/// <https://schema.org/ReturnMethodEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReturnMethodEnumeration {
	/// <https://schema.org/ReturnAtKiosk>
	ReturnAtKiosk,
	/// <https://schema.org/ReturnByMail>
	ReturnByMail,
	/// <https://schema.org/ReturnInStore>
	ReturnInStore,
}
