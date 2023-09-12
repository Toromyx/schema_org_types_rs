/// Used to indicate whether a product is EnergyStar certified.
///
/// https://schema.org/EnergyStarEnergyEfficiencyEnumeration
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EnergyStarEnergyEfficiencyEnumeration {
    /// Represents EnergyStar certification.
    ///
    /// https://schema.org/EnergyStarCertified
    EnergyStarCertified,
}
