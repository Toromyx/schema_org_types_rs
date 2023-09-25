/// Indicates whether this drug is available by prescription or over-the-counter.
///
/// <https://schema.org/DrugPrescriptionStatus>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum DrugPrescriptionStatus {
    /// The character of a medical substance, typically a medicine, of being available over the counter or not.
    ///
    /// <https://schema.org/OTC>
    Otc,
    /// Available by prescription only.
    ///
    /// <https://schema.org/PrescriptionOnly>
    PrescriptionOnly,
}
