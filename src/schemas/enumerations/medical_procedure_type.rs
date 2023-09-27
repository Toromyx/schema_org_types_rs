/// <https://schema.org/MedicalProcedureType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MedicalProcedureType {
    /// <https://schema.org/NoninvasiveProcedure>
    NoninvasiveProcedure,
    /// <https://schema.org/PercutaneousProcedure>
    PercutaneousProcedure,
}
