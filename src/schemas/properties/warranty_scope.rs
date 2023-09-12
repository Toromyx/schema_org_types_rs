use super::*;
/// The scope of the warranty promise.
///
/// https://schema.org/warrantyScope
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum WarrantyScopeProperty {
    #[cfg(any(feature = "warranty-scope-schema", feature = "general-schema-section"))]
    WarrantyScope(WarrantyScope),
}
