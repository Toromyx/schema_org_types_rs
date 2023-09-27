use super::*;
/// <https://schema.org/xpath>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum XpathProperty {
    #[cfg(any(
        any(feature = "x-path-type-schema", feature = "pending-schema-section"),
        doc
    ))]
    XPathType(XPathType),
}
