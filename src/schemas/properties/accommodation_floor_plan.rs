use super::*;
/// A floorplan of some [[Accommodation]].
///
/// https://schema.org/accommodationFloorPlan
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AccommodationFloorPlanProperty {
    #[cfg(any(feature = "floor-plan-schema", feature = "pending-schema-section"))]
    FloorPlan(FloorPlan),
}
