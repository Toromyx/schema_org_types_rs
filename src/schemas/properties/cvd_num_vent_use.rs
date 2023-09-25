use super::*;
/// numventuse - MECHANICAL VENTILATORS IN USE: Total number of ventilators in use.
///
/// <https://schema.org/cvdNumVentUse>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum CvdNumVentUseProperty {
    #[cfg(any(
        any(feature = "number-schema", feature = "general-schema-section"),
        doc
    ))]
    Number(Number),
}
