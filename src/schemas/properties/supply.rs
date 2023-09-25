use super::*;
/// A sub-property of instrument. A supply consumed when performing instructions or a direction.
///
/// https://schema.org/supply
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SupplyProperty {
    #[cfg(any(
        any(feature = "how-to-supply-schema", feature = "general-schema-section"),
        doc
    ))]
    HowToSupply(HowToSupply),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
