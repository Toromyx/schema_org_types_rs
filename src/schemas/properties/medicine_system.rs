use super::*;
/// The system of medicine that includes this MedicalEntity, for example 'evidence-based', 'homeopathic', 'chiropractic', etc.
///
/// https://schema.org/medicineSystem
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum MedicineSystemProperty {
    #[cfg(any(
        any(
            feature = "medicine-system-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicineSystem(MedicineSystem),
}
