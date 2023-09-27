use super::*;
/// <https://schema.org/serviceType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ServiceTypeProperty {
    #[cfg(any(
        any(
            feature = "government-benefits-type-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    GovernmentBenefitsType(GovernmentBenefitsType),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
