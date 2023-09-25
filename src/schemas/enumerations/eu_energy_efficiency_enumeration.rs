/// Enumerates the EU energy efficiency classes A-G as well as A+, A++, and A+++ as defined in EU directive 2017/1369.
///
/// <https://schema.org/EUEnergyEfficiencyEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum EuEnergyEfficiencyEnumeration {
    /// Represents EU Energy Efficiency Class A as defined in EU energy labeling regulations.
    ///
    /// <https://schema.org/EUEnergyEfficiencyCategoryA>
    EuEnergyEfficiencyCategoryA,
    /// Represents EU Energy Efficiency Class A+ as defined in EU energy labeling regulations.
    ///
    /// <https://schema.org/EUEnergyEfficiencyCategoryA1Plus>
    EuEnergyEfficiencyCategoryA1Plus,
    /// Represents EU Energy Efficiency Class A++ as defined in EU energy labeling regulations.
    ///
    /// <https://schema.org/EUEnergyEfficiencyCategoryA2Plus>
    EuEnergyEfficiencyCategoryA2Plus,
    /// Represents EU Energy Efficiency Class A+++ as defined in EU energy labeling regulations.
    ///
    /// <https://schema.org/EUEnergyEfficiencyCategoryA3Plus>
    EuEnergyEfficiencyCategoryA3Plus,
    /// Represents EU Energy Efficiency Class B as defined in EU energy labeling regulations.
    ///
    /// <https://schema.org/EUEnergyEfficiencyCategoryB>
    EuEnergyEfficiencyCategoryB,
    /// Represents EU Energy Efficiency Class C as defined in EU energy labeling regulations.
    ///
    /// <https://schema.org/EUEnergyEfficiencyCategoryC>
    EuEnergyEfficiencyCategoryC,
    /// Represents EU Energy Efficiency Class D as defined in EU energy labeling regulations.
    ///
    /// <https://schema.org/EUEnergyEfficiencyCategoryD>
    EuEnergyEfficiencyCategoryD,
    /// Represents EU Energy Efficiency Class E as defined in EU energy labeling regulations.
    ///
    /// <https://schema.org/EUEnergyEfficiencyCategoryE>
    EuEnergyEfficiencyCategoryE,
    /// Represents EU Energy Efficiency Class F as defined in EU energy labeling regulations.
    ///
    /// <https://schema.org/EUEnergyEfficiencyCategoryF>
    EuEnergyEfficiencyCategoryF,
    /// Represents EU Energy Efficiency Class G as defined in EU energy labeling regulations.
    ///
    /// <https://schema.org/EUEnergyEfficiencyCategoryG>
    EuEnergyEfficiencyCategoryG,
}
