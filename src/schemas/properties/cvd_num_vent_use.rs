use super::*;
/// numventuse - MECHANICAL VENTILATORS IN USE: Total number of ventilators in use.
///
/// https://schema.org/cvdNumVentUse
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CvdNumVentUseProperty {
    #[cfg(any(feature = "number-schema", feature = "general-schema-section"))]
    Number(Number),
}
