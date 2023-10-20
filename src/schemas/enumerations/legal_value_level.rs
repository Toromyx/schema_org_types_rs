/// <https://schema.org/LegalValueLevel>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LegalValueLevel {
	/// <https://schema.org/AuthoritativeLegalValue>
	AuthoritativeLegalValue,
	/// <https://schema.org/DefinitiveLegalValue>
	DefinitiveLegalValue,
	/// <https://schema.org/OfficialLegalValue>
	OfficialLegalValue,
	/// <https://schema.org/UnofficialLegalValue>
	UnofficialLegalValue,
}
