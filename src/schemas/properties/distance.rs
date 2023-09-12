use super::*;
/// The distance travelled, e.g. exercising or travelling.
///
/// https://schema.org/distance
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DistanceProperty {
    #[cfg(any(feature = "distance-schema", feature = "general-schema-section"))]
    Distance(Distance),
}
