use super::*;
/// The neurological pathway extension that involves muscle control.
///
/// https://schema.org/nerveMotor
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum NerveMotorProperty {
    #[cfg(any(
        any(feature = "muscle-schema", feature = "health-lifesci-schema-section"),
        doc
    ))]
    Muscle(Muscle),
}
