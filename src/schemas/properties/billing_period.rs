use super::*;
/// The time interval used to compute the invoice.
///
/// <https://schema.org/billingPeriod>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum BillingPeriodProperty {
    #[cfg(any(
        any(feature = "duration-schema", feature = "general-schema-section"),
        doc
    ))]
    Duration(Duration),
}
