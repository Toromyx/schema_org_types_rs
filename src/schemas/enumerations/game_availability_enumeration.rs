/// <https://schema.org/GameAvailabilityEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum GameAvailabilityEnumeration {
    /// <https://schema.org/DemoGameAvailability>
    DemoGameAvailability,
    /// <https://schema.org/FullGameAvailability>
    FullGameAvailability,
}
