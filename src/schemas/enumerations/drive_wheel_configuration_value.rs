/// <https://schema.org/DriveWheelConfigurationValue>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum DriveWheelConfigurationValue {
    /// <https://schema.org/AllWheelDriveConfiguration>
    AllWheelDriveConfiguration,
    /// <https://schema.org/FourWheelDriveConfiguration>
    FourWheelDriveConfiguration,
    /// <https://schema.org/FrontWheelDriveConfiguration>
    FrontWheelDriveConfiguration,
    /// <https://schema.org/RearWheelDriveConfiguration>
    RearWheelDriveConfiguration,
}
