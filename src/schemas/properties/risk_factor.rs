use super::*;
/// <https://schema.org/riskFactor>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RiskFactorProperty {
    #[cfg(any(
        any(
            feature = "medical-risk-factor-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalRiskFactor(MedicalRiskFactor),
}
