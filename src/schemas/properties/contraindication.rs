use super::*;
/// A contraindication for this therapy.
///
/// https://schema.org/contraindication
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ContraindicationProperty {
    #[cfg(any(
        feature = "medical-contraindication-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalContraindication(MedicalContraindication),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
