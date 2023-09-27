use super::*;
/// <https://schema.org/departureBoatTerminal>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DepartureBoatTerminalProperty {
    #[cfg(any(
        any(feature = "boat-terminal-schema", feature = "pending-schema-section"),
        doc
    ))]
    BoatTerminal(BoatTerminal),
}
