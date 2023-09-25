use super::*;
/// The class of infectious agent (bacteria, prion, etc.) that causes the disease.
///
/// https://schema.org/infectiousAgentClass
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum InfectiousAgentClassProperty {
    #[cfg(any(
        any(
            feature = "infectious-agent-class-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    InfectiousAgentClass(InfectiousAgentClass),
}
