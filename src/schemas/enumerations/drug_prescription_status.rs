/// <https://schema.org/DrugPrescriptionStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DrugPrescriptionStatus {
    /// <https://schema.org/OTC>
    Otc,
    /// <https://schema.org/PrescriptionOnly>
    PrescriptionOnly,
}
