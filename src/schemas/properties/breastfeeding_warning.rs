use super::*;
/// Any precaution, guidance, contraindication, etc. related to this drug's use by breastfeeding mothers.
///
/// https://schema.org/breastfeedingWarning
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BreastfeedingWarningProperty {
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
