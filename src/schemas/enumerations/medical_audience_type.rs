/// <https://schema.org/MedicalAudienceType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MedicalAudienceType {
	/// <https://schema.org/Clinician>
	Clinician,
	/// <https://schema.org/MedicalResearcher>
	MedicalResearcher,
}
