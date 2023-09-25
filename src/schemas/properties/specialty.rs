use super::*;
/// One of the domain specialities to which this web page's content applies.
///
/// https://schema.org/specialty
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SpecialtyProperty {
    #[cfg(any(
        any(feature = "specialty-schema", feature = "general-schema-section"),
        doc
    ))]
    Specialty(Specialty),
}
