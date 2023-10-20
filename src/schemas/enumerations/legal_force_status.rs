/// <https://schema.org/LegalForceStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LegalForceStatus {
	/// <https://schema.org/InForce>
	InForce,
	/// <https://schema.org/NotInForce>
	NotInForce,
	/// <https://schema.org/PartiallyInForce>
	PartiallyInForce,
}
