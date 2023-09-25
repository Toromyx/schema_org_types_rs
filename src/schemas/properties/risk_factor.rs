use super::*;
/// A modifiable or non-modifiable factor that increases the risk of a patient contracting this condition, e.g. age,  coexisting condition.
///
/// https://schema.org/riskFactor
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
