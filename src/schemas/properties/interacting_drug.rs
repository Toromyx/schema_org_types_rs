use super::*;
/// Another drug that is known to interact with this drug in a way that impacts the effect of this drug or causes a risk to the patient. Note: disease interactions are typically captured as contraindications.
///
/// https://schema.org/interactingDrug
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum InteractingDrugProperty {
    #[cfg(any(
        any(feature = "drug-schema", feature = "health-lifesci-schema-section"),
        doc
    ))]
    Drug(Drug),
}
