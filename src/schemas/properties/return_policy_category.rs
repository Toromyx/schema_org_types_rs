use super::*;
/// <https://schema.org/returnPolicyCategory>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ReturnPolicyCategoryProperty {
	#[cfg(any(
		any(
			feature = "merchant-return-enumeration-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	MerchantReturnEnumeration(MerchantReturnEnumeration),
}
