/// <https://schema.org/RestrictedDiet>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RestrictedDiet {
	/// <https://schema.org/DiabeticDiet>
	DiabeticDiet,
	/// <https://schema.org/GlutenFreeDiet>
	GlutenFreeDiet,
	/// <https://schema.org/HalalDiet>
	HalalDiet,
	/// <https://schema.org/HinduDiet>
	HinduDiet,
	/// <https://schema.org/KosherDiet>
	KosherDiet,
	/// <https://schema.org/LowCalorieDiet>
	LowCalorieDiet,
	/// <https://schema.org/LowFatDiet>
	LowFatDiet,
	/// <https://schema.org/LowLactoseDiet>
	LowLactoseDiet,
	/// <https://schema.org/LowSaltDiet>
	LowSaltDiet,
	/// <https://schema.org/VeganDiet>
	VeganDiet,
	/// <https://schema.org/VegetarianDiet>
	VegetarianDiet,
}
