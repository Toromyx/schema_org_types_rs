/// Enumerates common size systems for different categories of products, for example "EN-13402" or "UK" for wearables or "Imperial" for screws.
///
/// <https://schema.org/SizeSystemEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum SizeSystemEnumeration {
    /// Imperial size system.
    ///
    /// <https://schema.org/SizeSystemImperial>
    SizeSystemImperial,
    /// Metric size system.
    ///
    /// <https://schema.org/SizeSystemMetric>
    SizeSystemMetric,
}
