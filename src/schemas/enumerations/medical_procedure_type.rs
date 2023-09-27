/// <https://schema.org/MedicalProcedureType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MedicalProcedureType {
    /// <https://schema.org/NoninvasiveProcedure>
    NoninvasiveProcedure,
    /// <https://schema.org/PercutaneousProcedure>
    PercutaneousProcedure,
}
