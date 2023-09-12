/// A value indicating which roadwheels will receive torque.
///
/// https://schema.org/DriveWheelConfigurationValue
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DriveWheelConfigurationValue {
    /// All-wheel Drive is a transmission layout where the engine drives all four wheels.
    ///
    /// https://schema.org/AllWheelDriveConfiguration
    AllWheelDriveConfiguration,
    /// Four-wheel drive is a transmission layout where the engine primarily drives two wheels with a part-time four-wheel drive capability.
    ///
    /// https://schema.org/FourWheelDriveConfiguration
    FourWheelDriveConfiguration,
    /// Front-wheel drive is a transmission layout where the engine drives the front wheels.
    ///
    /// https://schema.org/FrontWheelDriveConfiguration
    FrontWheelDriveConfiguration,
    /// Real-wheel drive is a transmission layout where the engine drives the rear wheels.
    ///
    /// https://schema.org/RearWheelDriveConfiguration
    RearWheelDriveConfiguration,
}
