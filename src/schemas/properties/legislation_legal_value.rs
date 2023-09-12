use super::*;
/// The legal value of this legislation file. The same legislation can be written in multiple files with different legal values. Typically a digitally signed PDF have a "stronger" legal value than the HTML file of the same act.
///
/// https://schema.org/legislationLegalValue
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LegislationLegalValueProperty {
    #[cfg(any(
        feature = "legal-value-level-schema",
        feature = "pending-schema-section"
    ))]
    LegalValueLevel(LegalValueLevel),
}
