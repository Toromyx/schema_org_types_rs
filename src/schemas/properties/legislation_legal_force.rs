use super::*;
/// Whether the legislation is currently in force, not in force, or partially in force.
///
/// https://schema.org/legislationLegalForce
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LegislationLegalForceProperty {
    #[cfg(any(
        feature = "legal-force-status-schema",
        feature = "pending-schema-section"
    ))]
    LegalForceStatus(LegalForceStatus),
}
