use super::*;
/// <https://schema.org/xpath>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum XpathProperty {
    #[cfg(any(
        any(feature = "x-path-type-schema", feature = "pending-schema-section"),
        doc
    ))]
    XPathType(XPathType),
}
