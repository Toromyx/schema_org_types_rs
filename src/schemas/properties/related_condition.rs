use super::*;
/// A medical condition associated with this anatomy.
///
/// https://schema.org/relatedCondition
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RelatedConditionProperty {
    #[cfg(any(
        feature = "medical-condition-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalCondition(MedicalCondition),
}
