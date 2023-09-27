/// <https://schema.org/PhysicalActivityCategory>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum PhysicalActivityCategory {
    /// <https://schema.org/AerobicActivity>
    AerobicActivity,
    /// <https://schema.org/AnaerobicActivity>
    AnaerobicActivity,
    /// <https://schema.org/Balance>
    Balance,
    /// <https://schema.org/Flexibility>
    Flexibility,
    /// <https://schema.org/LeisureTimeActivity>
    LeisureTimeActivity,
    /// <https://schema.org/OccupationalActivity>
    OccupationalActivity,
    /// <https://schema.org/StrengthTraining>
    StrengthTraining,
}
