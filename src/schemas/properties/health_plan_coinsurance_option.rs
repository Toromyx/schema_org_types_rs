use super::*;
/// Whether the coinsurance applies before or after deductible, etc. TODO: Is this a closed set?
///
/// https://schema.org/healthPlanCoinsuranceOption
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HealthPlanCoinsuranceOptionProperty {
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
