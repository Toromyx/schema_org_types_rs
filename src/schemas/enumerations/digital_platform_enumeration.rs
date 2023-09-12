/// Enumerates some common technology platforms, for use with properties such as [[actionPlatform]]. It is not supposed to be comprehensive - when a suitable code is not enumerated here, textual or URL values can be used instead. These codes are at a fairly high level and do not deal with versioning and other nuance. Additional codes can be suggested [in github](https://github.com/schemaorg/schemaorg/issues/3057).
///
/// https://schema.org/DigitalPlatformEnumeration
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DigitalPlatformEnumeration {
    /// Represents the broad notion of Android-based operating systems.
    ///
    /// https://schema.org/AndroidPlatform
    AndroidPlatform,
    /// Represents the broad notion of 'desktop' browsers as a Web Platform.
    ///
    /// https://schema.org/DesktopWebPlatform
    DesktopWebPlatform,
    /// Represents the generic notion of the Web Platform. More specific codes include [[MobileWebPlatform]] and [[DesktopWebPlatform]], as an incomplete list.
    ///
    /// https://schema.org/GenericWebPlatform
    GenericWebPlatform,
    /// Represents the broad notion of iOS-based operating systems.
    ///
    /// https://schema.org/IOSPlatform
    IosPlatform,
    /// Represents the broad notion of 'mobile' browsers as a Web Platform.
    ///
    /// https://schema.org/MobileWebPlatform
    MobileWebPlatform,
}
