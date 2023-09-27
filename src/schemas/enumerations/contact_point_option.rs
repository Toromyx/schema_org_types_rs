/// <https://schema.org/ContactPointOption>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum ContactPointOption {
    /// <https://schema.org/HearingImpairedSupported>
    HearingImpairedSupported,
    /// <https://schema.org/TollFree>
    TollFree,
}
