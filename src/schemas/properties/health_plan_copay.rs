use super::*;
/// The copay amount.
///
/// https://schema.org/healthPlanCopay
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HealthPlanCopayProperty {
    #[cfg(any(
        feature = "price-specification-schema",
        feature = "general-schema-section"
    ))]
    PriceSpecification(PriceSpecification),
}
