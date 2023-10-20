/// <https://schema.org/GenderType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GenderType {
	/// <https://schema.org/Female>
	Female,
	/// <https://schema.org/Male>
	Male,
}
