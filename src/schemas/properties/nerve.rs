use super::*;
/// The underlying innervation associated with the muscle.
///
/// <https://schema.org/nerve>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum NerveProperty {
    #[cfg(any(
        any(feature = "nerve-schema", feature = "health-lifesci-schema-section"),
        doc
    ))]
    Nerve(Nerve),
}
