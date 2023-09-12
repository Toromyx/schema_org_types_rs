use super::*;
/// Indicates whether the vehicle has been used for special purposes, like commercial rental, driving school, or as a taxi. The legislation in many countries requires this information to be revealed when offering a car for sale.
///
/// https://schema.org/vehicleSpecialUsage
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum VehicleSpecialUsageProperty {
    #[cfg(any(feature = "car-usage-type-schema", feature = "auto-schema-section"))]
    CarUsageType(CarUsageType),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
