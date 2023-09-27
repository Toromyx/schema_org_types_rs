/// <https://schema.org/ContactPointOption>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ContactPointOption {
    /// <https://schema.org/HearingImpairedSupported>
    HearingImpairedSupported,
    /// <https://schema.org/TollFree>
    TollFree,
}
