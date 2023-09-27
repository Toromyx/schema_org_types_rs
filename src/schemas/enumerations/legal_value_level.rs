/// <https://schema.org/LegalValueLevel>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
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
