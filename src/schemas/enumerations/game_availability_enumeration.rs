/// For a [[VideoGame]], such as used with a [[PlayGameAction]], an enumeration of the kind of game availability offered.
///
/// https://schema.org/GameAvailabilityEnumeration
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GameAvailabilityEnumeration {
    /// Indicates demo game availability, i.e. a somehow limited demonstration of the full game.
    ///
    /// https://schema.org/DemoGameAvailability
    DemoGameAvailability,
    /// Indicates full game availability.
    ///
    /// https://schema.org/FullGameAvailability
    FullGameAvailability,
}
