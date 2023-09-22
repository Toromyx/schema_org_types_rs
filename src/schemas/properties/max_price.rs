use super::*;
/// The highest price if the price is a range.
///
/// https://schema.org/maxPrice
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MaxPriceProperty {
    #[cfg(any(feature = "number-schema", feature = "general-schema-section"))]
    Number(Number),
}