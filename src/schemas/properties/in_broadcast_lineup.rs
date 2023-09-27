use super::*;
/// <https://schema.org/inBroadcastLineup>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
