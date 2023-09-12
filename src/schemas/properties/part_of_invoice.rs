use super::*;
/// The order is being paid as part of the referenced Invoice.
///
/// https://schema.org/partOfInvoice
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PartOfInvoiceProperty {
    #[cfg(any(feature = "invoice-schema", feature = "general-schema-section"))]
    Invoice(Invoice),
}
