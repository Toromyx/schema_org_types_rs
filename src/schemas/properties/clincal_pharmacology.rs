use super::*;
/// Description of the absorption and elimination of drugs, including their concentration (pharmacokinetics, pK) and biological effects (pharmacodynamics, pD).
///
/// <https://schema.org/clincalPharmacology>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ClincalPharmacologyProperty {
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
