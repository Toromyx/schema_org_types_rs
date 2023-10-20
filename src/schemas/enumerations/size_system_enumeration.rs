/// <https://schema.org/SizeSystemEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SizeSystemEnumeration {
	/// <https://schema.org/SizeSystemImperial>
	SizeSystemImperial,
	/// <https://schema.org/SizeSystemMetric>
	SizeSystemMetric,
}
