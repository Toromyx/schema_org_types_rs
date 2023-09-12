use super::*;
/// The type of procedure, for example Surgical, Noninvasive, or Percutaneous.
///
/// https://schema.org/procedureType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ProcedureTypeProperty {
    #[cfg(any(
        feature = "medical-procedure-type-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalProcedureType(MedicalProcedureType),
}
