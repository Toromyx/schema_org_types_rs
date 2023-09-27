use super::*;
/// <https://schema.org/nerve>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum NerveProperty {
    #[cfg(any(
        any(feature = "nerve-schema", feature = "health-lifesci-schema-section"),
        doc
    ))]
    Nerve(Nerve),
}
