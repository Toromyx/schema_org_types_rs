use super::*;
/// The warranty promise(s) included in the offer.
///
/// <https://schema.org/warranty>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum WarrantyProperty {
    #[cfg(any(
        any(
            feature = "warranty-promise-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    WarrantyPromise(WarrantyPromise),
}
