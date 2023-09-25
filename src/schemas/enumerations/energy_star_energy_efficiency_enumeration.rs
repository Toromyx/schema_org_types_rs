/// Used to indicate whether a product is EnergyStar certified.
///
/// <https://schema.org/EnergyStarEnergyEfficiencyEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum EnergyStarEnergyEfficiencyEnumeration {
    /// Represents EnergyStar certification.
    ///
    /// <https://schema.org/EnergyStarCertified>
    EnergyStarCertified,
}
