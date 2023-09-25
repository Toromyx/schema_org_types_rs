use super::*;
/// A floorplan of some [[Accommodation]].
///
/// <https://schema.org/accommodationFloorPlan>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AccommodationFloorPlanProperty {
    #[cfg(any(
        any(feature = "floor-plan-schema", feature = "pending-schema-section"),
        doc
    ))]
    FloorPlan(FloorPlan),
}
