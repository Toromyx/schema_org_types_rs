use super::*;
/// <https://schema.org/arrivalBoatTerminal>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ArrivalBoatTerminalProperty {
    #[cfg(any(
        any(feature = "boat-terminal-schema", feature = "pending-schema-section"),
        doc
    ))]
    BoatTerminal(BoatTerminal),
}
