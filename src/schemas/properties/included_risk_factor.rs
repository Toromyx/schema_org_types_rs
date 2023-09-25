use super::*;
/// A modifiable or non-modifiable risk factor included in the calculation, e.g. age, coexisting condition.
///
/// https://schema.org/includedRiskFactor
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum IncludedRiskFactorProperty {
    #[cfg(any(
        any(
            feature = "medical-risk-factor-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalRiskFactor(MedicalRiskFactor),
}
