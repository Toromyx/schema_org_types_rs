use super::*;
/// The type of procedure, for example Surgical, Noninvasive, or Percutaneous.
///
/// <https://schema.org/procedureType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ProcedureTypeProperty {
    #[cfg(any(
        any(
            feature = "medical-procedure-type-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalProcedureType(MedicalProcedureType),
}
