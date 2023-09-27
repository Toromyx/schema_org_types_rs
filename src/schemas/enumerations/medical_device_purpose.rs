/// <https://schema.org/MedicalDevicePurpose>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MedicalDevicePurpose {
    /// <https://schema.org/Diagnostic>
    Diagnostic,
    /// <https://schema.org/Therapeutic>
    Therapeutic,
}
