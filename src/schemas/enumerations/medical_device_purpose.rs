/// Categories of medical devices, organized by the purpose or intended use of the device.
///
/// https://schema.org/MedicalDevicePurpose
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MedicalDevicePurpose {
    /// A medical device used for diagnostic purposes.
    ///
    /// https://schema.org/Diagnostic
    Diagnostic,
    /// A medical device used for therapeutic purposes.
    ///
    /// https://schema.org/Therapeutic
    Therapeutic,
}
