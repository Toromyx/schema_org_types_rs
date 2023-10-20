/// <https://schema.org/DriveWheelConfigurationValue>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
