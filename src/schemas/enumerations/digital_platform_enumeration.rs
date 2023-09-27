/// <https://schema.org/DigitalPlatformEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
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
