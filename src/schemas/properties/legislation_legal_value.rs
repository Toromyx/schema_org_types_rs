use super::*;
/// <https://schema.org/legislationLegalValue>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LegislationLegalValueProperty {
    #[cfg(any(
        any(
            feature = "legal-value-level-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    LegalValueLevel(LegalValueLevel),
}
