use super::*;
/// A sub property of location. The specific food event where the action occurred.
///
/// https://schema.org/foodEvent
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FoodEventProperty {
    #[cfg(any(feature = "food-event-schema", feature = "general-schema-section"))]
    FoodEvent(FoodEvent),
}
