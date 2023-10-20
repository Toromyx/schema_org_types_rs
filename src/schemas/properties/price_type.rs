use super::*;
/// <https://schema.org/priceType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PriceTypeProperty {
	#[cfg(any(
		any(
			feature = "price-type-enumeration-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	PriceTypeEnumeration(PriceTypeEnumeration),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
	Text(Text),
}
