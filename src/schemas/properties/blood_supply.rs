use super::*;
/// The blood vessel that carries blood from the heart to the muscle.
///
/// <https://schema.org/bloodSupply>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum BloodSupplyProperty {
    #[cfg(any(
        any(feature = "vessel-schema", feature = "health-lifesci-schema-section"),
        doc
    ))]
    Vessel(Vessel),
}
