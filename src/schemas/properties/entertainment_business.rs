use super::*;
/// A sub property of location. The entertainment business where the action occurred.
///
/// <https://schema.org/entertainmentBusiness>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum EntertainmentBusinessProperty {
    #[cfg(any(
        any(
            feature = "entertainment-business-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    EntertainmentBusiness(EntertainmentBusiness),
}
