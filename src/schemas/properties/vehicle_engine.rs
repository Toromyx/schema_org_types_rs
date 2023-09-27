use super::*;
/// <https://schema.org/vehicleEngine>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
