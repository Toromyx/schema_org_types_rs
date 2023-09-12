use super::*;
/// The warranty promise(s) included in the offer.
///
/// https://schema.org/warrantyPromise
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum WarrantyPromiseProperty {
    #[cfg(any(
        feature = "warranty-promise-schema",
        feature = "general-schema-section"
    ))]
    WarrantyPromise(WarrantyPromise),
}
