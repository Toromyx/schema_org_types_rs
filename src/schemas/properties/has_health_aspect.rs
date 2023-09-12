use super::*;
/// Indicates the aspect or aspects specifically addressed in some [[HealthTopicContent]]. For example, that the content is an overview, or that it talks about treatment, self-care, treatments or their side-effects.
///
/// https://schema.org/hasHealthAspect
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasHealthAspectProperty {
    #[cfg(any(
        feature = "health-aspect-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    HealthAspectEnumeration(HealthAspectEnumeration),
}
