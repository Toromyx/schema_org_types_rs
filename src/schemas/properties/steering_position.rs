use super::*;
/// <https://schema.org/steeringPosition>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
