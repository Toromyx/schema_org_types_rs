/// Indicates whether this drug is available by prescription or over-the-counter.
///
/// https://schema.org/DrugPrescriptionStatus
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DrugPrescriptionStatus {
    /// The character of a medical substance, typically a medicine, of being available over the counter or not.
    ///
    /// https://schema.org/OTC
    Otc,
    /// Available by prescription only.
    ///
    /// https://schema.org/PrescriptionOnly
    PrescriptionOnly,
}
