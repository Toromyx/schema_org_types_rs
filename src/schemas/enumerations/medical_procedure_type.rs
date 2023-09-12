/// An enumeration that describes different types of medical procedures.
///
/// https://schema.org/MedicalProcedureType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MedicalProcedureType {
    /// A type of medical procedure that involves noninvasive techniques.
    ///
    /// https://schema.org/NoninvasiveProcedure
    NoninvasiveProcedure,
    /// A type of medical procedure that involves percutaneous techniques, where access to organs or tissue is achieved via needle-puncture of the skin. For example, catheter-based procedures like stent delivery.
    ///
    /// https://schema.org/PercutaneousProcedure
    PercutaneousProcedure,
}
