use super::*;
/// A hospital with which the physician or office is affiliated.
///
/// <https://schema.org/hospitalAffiliation>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum HospitalAffiliationProperty {
    #[cfg(any(
        any(feature = "hospital-schema", feature = "general-schema-section"),
        doc
    ))]
    Hospital(Hospital),
}
