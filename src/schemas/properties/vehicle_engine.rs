use super::*;
/// Information about the engine or engines of the vehicle.
///
/// https://schema.org/vehicleEngine
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum VehicleEngineProperty {
    #[cfg(any(
        feature = "engine-specification-schema",
        feature = "general-schema-section"
    ))]
    EngineSpecification(EngineSpecification),
}
