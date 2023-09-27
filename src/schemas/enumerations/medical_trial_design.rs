/// <https://schema.org/MedicalTrialDesign>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MedicalTrialDesign {
    /// <https://schema.org/DoubleBlindedTrial>
    DoubleBlindedTrial,
    /// <https://schema.org/InternationalTrial>
    InternationalTrial,
    /// <https://schema.org/MultiCenterTrial>
    MultiCenterTrial,
    /// <https://schema.org/OpenTrial>
    OpenTrial,
    /// <https://schema.org/PlaceboControlledTrial>
    PlaceboControlledTrial,
    /// <https://schema.org/RandomizedTrial>
    RandomizedTrial,
    /// <https://schema.org/SingleBlindedTrial>
    SingleBlindedTrial,
    /// <https://schema.org/SingleCenterTrial>
    SingleCenterTrial,
    /// <https://schema.org/TripleBlindedTrial>
    TripleBlindedTrial,
}
