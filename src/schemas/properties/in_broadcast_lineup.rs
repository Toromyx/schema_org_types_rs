use super::*;
/// <https://schema.org/inBroadcastLineup>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum InBroadcastLineupProperty {
    #[cfg(any(
        any(
            feature = "cable-or-satellite-service-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    CableOrSatelliteService(CableOrSatelliteService),
}
