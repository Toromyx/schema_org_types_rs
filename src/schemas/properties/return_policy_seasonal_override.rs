use super::*;
/// <https://schema.org/returnPolicySeasonalOverride>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ReturnPolicySeasonalOverrideProperty {
    #[cfg(any(
        any(
            feature = "merchant-return-policy-seasonal-override-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    MerchantReturnPolicySeasonalOverride(MerchantReturnPolicySeasonalOverride),
}
