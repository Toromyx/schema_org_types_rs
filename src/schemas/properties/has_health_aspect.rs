use super::*;
/// <https://schema.org/hasHealthAspect>
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
