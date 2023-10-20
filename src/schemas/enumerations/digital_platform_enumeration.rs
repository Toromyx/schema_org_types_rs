/// <https://schema.org/DigitalPlatformEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DigitalPlatformEnumeration {
	/// <https://schema.org/AndroidPlatform>
	AndroidPlatform,
	/// <https://schema.org/DesktopWebPlatform>
	DesktopWebPlatform,
	/// <https://schema.org/GenericWebPlatform>
	GenericWebPlatform,
	/// <https://schema.org/IOSPlatform>
	IosPlatform,
	/// <https://schema.org/MobileWebPlatform>
	MobileWebPlatform,
}
