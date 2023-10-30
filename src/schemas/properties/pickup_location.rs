use super::*;
/// <https://schema.org/pickupLocation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PickupLocationProperty {
	#[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
	Place(Place),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::FailValue),
}
