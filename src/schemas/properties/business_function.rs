use super::*;
/// The business function (e.g. sell, lease, repair, dispose) of the offer or component of a bundle (TypeAndQuantityNode). The default is http://purl.org/goodrelations/v1#Sell.
///
/// https://schema.org/businessFunction
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BusinessFunctionProperty {
    #[cfg(any(
        feature = "business-function-schema",
        feature = "general-schema-section"
    ))]
    BusinessFunction(BusinessFunction),
}
