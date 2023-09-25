use super::*;
/// Indicates the aspect or aspects specifically addressed in some [[HealthTopicContent]]. For example, that the content is an overview, or that it talks about treatment, self-care, treatments or their side-effects.
///
/// https://schema.org/hasHealthAspect
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum HasHealthAspectProperty {
    #[cfg(any(
        any(
            feature = "health-aspect-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    HealthAspectEnumeration(HealthAspectEnumeration),
}
