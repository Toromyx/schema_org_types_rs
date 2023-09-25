use super::*;
/// The type of return method offered, specified from an enumeration.
///
/// <https://schema.org/returnMethod>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ReturnMethodProperty {
    #[cfg(any(
        any(
            feature = "return-method-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    ReturnMethodEnumeration(ReturnMethodEnumeration),
}
