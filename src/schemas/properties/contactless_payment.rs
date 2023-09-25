use super::*;
/// A secure method for consumers to purchase products or services via debit, credit or smartcards by using RFID or NFC technology.
///
/// <https://schema.org/contactlessPayment>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ContactlessPaymentProperty {
    #[cfg(any(
        any(feature = "boolean-schema", feature = "general-schema-section"),
        doc
    ))]
    Boolean(Boolean),
}
