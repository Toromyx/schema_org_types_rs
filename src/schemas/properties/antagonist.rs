use super::*;
/// The muscle whose action counteracts the specified muscle.
///
/// https://schema.org/antagonist
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AntagonistProperty {
    #[cfg(any(feature = "muscle-schema", feature = "health-lifesci-schema-section"))]
    Muscle(Muscle),
}
