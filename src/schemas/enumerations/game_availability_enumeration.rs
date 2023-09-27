/// <https://schema.org/GameAvailabilityEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GameAvailabilityEnumeration {
    /// <https://schema.org/DemoGameAvailability>
    DemoGameAvailability,
    /// <https://schema.org/FullGameAvailability>
    FullGameAvailability,
}
