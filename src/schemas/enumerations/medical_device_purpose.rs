/// <https://schema.org/MedicalDevicePurpose>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MedicalDevicePurpose {
	/// <https://schema.org/Diagnostic>
	Diagnostic,
	/// <https://schema.org/Therapeutic>
	Therapeutic,
}
