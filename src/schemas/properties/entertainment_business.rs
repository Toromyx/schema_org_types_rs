use super::*;
/// A sub property of location. The entertainment business where the action occurred.
///
/// https://schema.org/entertainmentBusiness
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EntertainmentBusinessProperty {
    #[cfg(any(
        feature = "entertainment-business-schema",
        feature = "general-schema-section"
    ))]
    EntertainmentBusiness(EntertainmentBusiness),
}
