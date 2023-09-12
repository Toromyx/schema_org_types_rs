use super::*;
/// The terminal or port from which the boat arrives.
///
/// https://schema.org/arrivalBoatTerminal
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ArrivalBoatTerminalProperty {
    #[cfg(any(feature = "boat-terminal-schema", feature = "pending-schema-section"))]
    BoatTerminal(BoatTerminal),
}
