use super::*;
/// The legal value of this legislation file. The same legislation can be written in multiple files with different legal values. Typically a digitally signed PDF have a "stronger" legal value than the HTML file of the same act.
///
/// <https://schema.org/legislationLegalValue>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
