use super::*;
/// The position of the steering wheel or similar device (mostly for cars).
///
/// https://schema.org/steeringPosition
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SteeringPositionProperty {
    #[cfg(any(
        feature = "steering-position-value-schema",
        feature = "general-schema-section"
    ))]
    SteeringPositionValue(SteeringPositionValue),
}