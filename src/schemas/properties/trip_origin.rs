use super::*;
/// The location of origin of the trip, prior to any destination(s).
///
/// https://schema.org/tripOrigin
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TripOriginProperty {
    #[cfg(any(feature = "place-schema", feature = "general-schema-section"))]
    Place(Place),
}
