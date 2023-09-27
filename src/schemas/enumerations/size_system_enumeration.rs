/// <https://schema.org/SizeSystemEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum SizeSystemEnumeration {
    /// <https://schema.org/SizeSystemImperial>
    SizeSystemImperial,
    /// <https://schema.org/SizeSystemMetric>
    SizeSystemMetric,
}
