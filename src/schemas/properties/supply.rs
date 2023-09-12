use super::*;
/// A sub-property of instrument. A supply consumed when performing instructions or a direction.
///
/// https://schema.org/supply
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SupplyProperty {
    #[cfg(any(feature = "how-to-supply-schema", feature = "general-schema-section"))]
    HowToSupply(HowToSupply),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
