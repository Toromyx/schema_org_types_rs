use super::*;
/// <https://schema.org/businessFunction>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum BusinessFunctionProperty {
    #[cfg(any(
        any(
            feature = "business-function-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    BusinessFunction(BusinessFunction),
}
