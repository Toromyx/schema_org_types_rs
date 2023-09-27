use super::*;
/// <https://schema.org/driveWheelConfiguration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DriveWheelConfigurationProperty {
    #[cfg(any(
        any(
            feature = "drive-wheel-configuration-value-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    DriveWheelConfigurationValue(DriveWheelConfigurationValue),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
