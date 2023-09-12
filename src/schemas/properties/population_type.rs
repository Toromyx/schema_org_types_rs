use super::*;
/// Indicates the populationType common to all members of a [[StatisticalPopulation]] or all cases within the scope of a [[StatisticalVariable]].
///
/// https://schema.org/populationType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PopulationTypeProperty {
    #[cfg(any(feature = "class-schema", feature = "meta-schema-section"))]
    Class(Class),
}
