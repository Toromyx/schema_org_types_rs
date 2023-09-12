/// Enumerated options related to a ContactPoint.
///
/// https://schema.org/ContactPointOption
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ContactPointOption {
    /// Uses devices to support users with hearing impairments.
    ///
    /// https://schema.org/HearingImpairedSupported
    HearingImpairedSupported,
    /// The associated telephone number is toll free.
    ///
    /// https://schema.org/TollFree
    TollFree,
}
