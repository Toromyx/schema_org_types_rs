/// <https://schema.org/EUEnergyEfficiencyEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum EuEnergyEfficiencyEnumeration {
    /// <https://schema.org/EUEnergyEfficiencyCategoryA>
    EuEnergyEfficiencyCategoryA,
    /// <https://schema.org/EUEnergyEfficiencyCategoryA1Plus>
    EuEnergyEfficiencyCategoryA1Plus,
    /// <https://schema.org/EUEnergyEfficiencyCategoryA2Plus>
    EuEnergyEfficiencyCategoryA2Plus,
    /// <https://schema.org/EUEnergyEfficiencyCategoryA3Plus>
    EuEnergyEfficiencyCategoryA3Plus,
    /// <https://schema.org/EUEnergyEfficiencyCategoryB>
    EuEnergyEfficiencyCategoryB,
    /// <https://schema.org/EUEnergyEfficiencyCategoryC>
    EuEnergyEfficiencyCategoryC,
    /// <https://schema.org/EUEnergyEfficiencyCategoryD>
    EuEnergyEfficiencyCategoryD,
    /// <https://schema.org/EUEnergyEfficiencyCategoryE>
    EuEnergyEfficiencyCategoryE,
    /// <https://schema.org/EUEnergyEfficiencyCategoryF>
    EuEnergyEfficiencyCategoryF,
    /// <https://schema.org/EUEnergyEfficiencyCategoryG>
    EuEnergyEfficiencyCategoryG,
}
