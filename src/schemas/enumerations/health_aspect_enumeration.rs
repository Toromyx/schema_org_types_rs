/// <https://schema.org/HealthAspectEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HealthAspectEnumeration {
	/// <https://schema.org/AllergiesHealthAspect>
	AllergiesHealthAspect,
	/// <https://schema.org/BenefitsHealthAspect>
	BenefitsHealthAspect,
	/// <https://schema.org/CausesHealthAspect>
	CausesHealthAspect,
	/// <https://schema.org/ContagiousnessHealthAspect>
	ContagiousnessHealthAspect,
	/// <https://schema.org/EffectivenessHealthAspect>
	EffectivenessHealthAspect,
	/// <https://schema.org/GettingAccessHealthAspect>
	GettingAccessHealthAspect,
	/// <https://schema.org/HowItWorksHealthAspect>
	HowItWorksHealthAspect,
	/// <https://schema.org/HowOrWhereHealthAspect>
	HowOrWhereHealthAspect,
	/// <https://schema.org/IngredientsHealthAspect>
	IngredientsHealthAspect,
	/// <https://schema.org/LivingWithHealthAspect>
	LivingWithHealthAspect,
	/// <https://schema.org/MayTreatHealthAspect>
	MayTreatHealthAspect,
	/// <https://schema.org/MisconceptionsHealthAspect>
	MisconceptionsHealthAspect,
	/// <https://schema.org/OverviewHealthAspect>
	OverviewHealthAspect,
	/// <https://schema.org/PatientExperienceHealthAspect>
	PatientExperienceHealthAspect,
	/// <https://schema.org/PregnancyHealthAspect>
	PregnancyHealthAspect,
	/// <https://schema.org/PreventionHealthAspect>
	PreventionHealthAspect,
	/// <https://schema.org/PrognosisHealthAspect>
	PrognosisHealthAspect,
	/// <https://schema.org/RelatedTopicsHealthAspect>
	RelatedTopicsHealthAspect,
	/// <https://schema.org/RisksOrComplicationsHealthAspect>
	RisksOrComplicationsHealthAspect,
	/// <https://schema.org/SafetyHealthAspect>
	SafetyHealthAspect,
	/// <https://schema.org/ScreeningHealthAspect>
	ScreeningHealthAspect,
	/// <https://schema.org/SeeDoctorHealthAspect>
	SeeDoctorHealthAspect,
	/// <https://schema.org/SelfCareHealthAspect>
	SelfCareHealthAspect,
	/// <https://schema.org/SideEffectsHealthAspect>
	SideEffectsHealthAspect,
	/// <https://schema.org/StagesHealthAspect>
	StagesHealthAspect,
	/// <https://schema.org/SymptomsHealthAspect>
	SymptomsHealthAspect,
	/// <https://schema.org/TreatmentsHealthAspect>
	TreatmentsHealthAspect,
	/// <https://schema.org/TypesHealthAspect>
	TypesHealthAspect,
	/// <https://schema.org/UsageOrScheduleHealthAspect>
	UsageOrScheduleHealthAspect,
}
