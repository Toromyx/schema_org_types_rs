use super::*;
/// Another drug that is known to interact with this drug in a way that impacts the effect of this drug or causes a risk to the patient. Note: disease interactions are typically captured as contraindications.
///
/// https://schema.org/interactingDrug
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InteractingDrugProperty {
    #[cfg(any(feature = "drug-schema", feature = "health-lifesci-schema-section"))]
    Drug(Drug),
}
