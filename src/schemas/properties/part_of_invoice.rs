use super::*;
/// The order is being paid as part of the referenced Invoice.
///
/// https://schema.org/partOfInvoice
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PartOfInvoiceProperty {
    #[cfg(any(
        any(feature = "invoice-schema", feature = "general-schema-section"),
        doc
    ))]
    Invoice(Invoice),
}
