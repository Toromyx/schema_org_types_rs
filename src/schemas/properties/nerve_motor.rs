use super::*;
/// <https://schema.org/nerveMotor>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum NerveMotorProperty {
    #[cfg(any(
        any(feature = "muscle-schema", feature = "health-lifesci-schema-section"),
        doc
    ))]
    Muscle(Muscle),
}
