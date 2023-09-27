use super::*;
/// <https://schema.org/returnPolicySeasonalOverride>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
