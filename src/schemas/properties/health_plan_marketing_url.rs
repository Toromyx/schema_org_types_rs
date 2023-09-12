use super::*;
/// The URL that goes directly to the plan brochure for the specific standard plan or plan variation.
///
/// https://schema.org/healthPlanMarketingUrl
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HealthPlanMarketingUrlProperty {
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
