/// <https://schema.org/DrugPrescriptionStatus>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum DrugPrescriptionStatus {
    /// <https://schema.org/OTC>
    Otc,
    /// <https://schema.org/PrescriptionOnly>
    PrescriptionOnly,
}
