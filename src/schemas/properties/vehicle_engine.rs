use super::*;
/// Information about the engine or engines of the vehicle.
///
/// <https://schema.org/vehicleEngine>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum VehicleEngineProperty {
    #[cfg(any(
        any(
            feature = "engine-specification-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    EngineSpecification(EngineSpecification),
}
