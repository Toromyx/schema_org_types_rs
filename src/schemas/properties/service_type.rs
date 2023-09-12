use super::*;
/// The type of service being offered, e.g. veterans' benefits, emergency relief, etc.
///
/// https://schema.org/serviceType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ServiceTypeProperty {
    #[cfg(any(
        feature = "government-benefits-type-schema",
        feature = "pending-schema-section"
    ))]
    GovernmentBenefitsType(GovernmentBenefitsType),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
