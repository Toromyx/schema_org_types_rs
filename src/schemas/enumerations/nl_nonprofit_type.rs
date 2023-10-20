/// <https://schema.org/NLNonprofitType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NlNonprofitType {
	/// <https://schema.org/NonprofitANBI>
	NonprofitAnbi,
	/// <https://schema.org/NonprofitSBBI>
	NonprofitSbbi,
}
