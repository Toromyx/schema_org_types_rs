use super::*;
/// The drive wheel configuration, i.e. which roadwheels will receive torque from the vehicle's engine via the drivetrain.
///
/// https://schema.org/driveWheelConfiguration
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DriveWheelConfigurationProperty {
    #[cfg(any(
        feature = "drive-wheel-configuration-value-schema",
        feature = "general-schema-section"
    ))]
    DriveWheelConfigurationValue(DriveWheelConfigurationValue),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
