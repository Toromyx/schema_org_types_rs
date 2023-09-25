/// Enumerated options related to a ContactPoint.
///
/// <https://schema.org/ContactPointOption>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum ContactPointOption {
    /// Uses devices to support users with hearing impairments.
    ///
    /// <https://schema.org/HearingImpairedSupported>
    HearingImpairedSupported,
    /// The associated telephone number is toll free.
    ///
    /// <https://schema.org/TollFree>
    TollFree,
}
