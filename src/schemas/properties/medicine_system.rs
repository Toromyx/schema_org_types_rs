use super::*;
/// The system of medicine that includes this MedicalEntity, for example 'evidence-based', 'homeopathic', 'chiropractic', etc.
///
/// https://schema.org/medicineSystem
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MedicineSystemProperty {
    #[cfg(any(
        feature = "medicine-system-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicineSystem(MedicineSystem),
}
