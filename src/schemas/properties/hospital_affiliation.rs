use super::*;
/// A hospital with which the physician or office is affiliated.
///
/// https://schema.org/hospitalAffiliation
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HospitalAffiliationProperty {
    #[cfg(any(feature = "hospital-schema", feature = "general-schema-section"))]
    Hospital(Hospital),
}
