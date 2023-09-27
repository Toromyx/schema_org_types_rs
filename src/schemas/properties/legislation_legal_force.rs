use super::*;
/// <https://schema.org/legislationLegalForce>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum LegislationLegalForceProperty {
    #[cfg(any(
        any(
            feature = "legal-force-status-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    LegalForceStatus(LegalForceStatus),
}
