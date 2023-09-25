use super::*;
/// Typical preparation that a patient must undergo before having the procedure performed.
///
/// <https://schema.org/preparation>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PreparationProperty {
    #[cfg(any(
        any(
            feature = "medical-entity-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalEntity(MedicalEntity),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
