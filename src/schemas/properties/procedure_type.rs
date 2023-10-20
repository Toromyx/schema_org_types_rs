use super::*;
/// <https://schema.org/procedureType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
