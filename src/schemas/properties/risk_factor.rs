use super::*;
/// A modifiable or non-modifiable factor that increases the risk of a patient contracting this condition, e.g. age,  coexisting condition.
///
/// https://schema.org/riskFactor
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RiskFactorProperty {
    #[cfg(any(
        feature = "medical-risk-factor-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalRiskFactor(MedicalRiskFactor),
}
