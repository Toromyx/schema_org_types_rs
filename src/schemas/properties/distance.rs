use super::*;
/// The distance travelled, e.g. exercising or travelling.
///
/// <https://schema.org/distance>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DistanceProperty {
    #[cfg(any(
        any(feature = "distance-schema", feature = "general-schema-section"),
        doc
    ))]
    Distance(Distance),
}
