use super::*;
/// <https://schema.org/bloodSupply>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BloodSupplyProperty {
    #[cfg(any(
        any(feature = "vessel-schema", feature = "health-lifesci-schema-section"),
        doc
    ))]
    Vessel(Vessel),
}
