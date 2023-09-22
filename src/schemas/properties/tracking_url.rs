use super::*;
/// Tracking url for the parcel delivery.
///
/// https://schema.org/trackingUrl
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TrackingUrlProperty {
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}