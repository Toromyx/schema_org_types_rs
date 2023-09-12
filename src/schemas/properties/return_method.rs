use super::*;
/// The type of return method offered, specified from an enumeration.
///
/// https://schema.org/returnMethod
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ReturnMethodProperty {
    #[cfg(any(
        feature = "return-method-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    ReturnMethodEnumeration(ReturnMethodEnumeration),
}
