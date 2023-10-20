use super::*;
/// <https://schema.org/hasMerchantReturnPolicy>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasMerchantReturnPolicyProperty {
	#[cfg(any(
		any(
			feature = "merchant-return-policy-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	MerchantReturnPolicy(MerchantReturnPolicy),
}
