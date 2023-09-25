use super::*;
/// The position of the steering wheel or similar device (mostly for cars).
///
/// https://schema.org/steeringPosition
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SteeringPositionProperty {
    #[cfg(any(
        any(
            feature = "steering-position-value-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    SteeringPositionValue(SteeringPositionValue),
}
