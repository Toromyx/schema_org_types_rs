use super::*;
/// <https://schema.org/accommodationFloorPlan>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AccommodationFloorPlanProperty {
    #[cfg(any(
        any(feature = "floor-plan-schema", feature = "pending-schema-section"),
        doc
    ))]
    FloorPlan(FloorPlan),
}
