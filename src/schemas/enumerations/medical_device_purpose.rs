/// Categories of medical devices, organized by the purpose or intended use of the device.
///
/// <https://schema.org/MedicalDevicePurpose>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MedicalDevicePurpose {
    /// A medical device used for diagnostic purposes.
    ///
    /// <https://schema.org/Diagnostic>
    Diagnostic,
    /// A medical device used for therapeutic purposes.
    ///
    /// <https://schema.org/Therapeutic>
    Therapeutic,
}
