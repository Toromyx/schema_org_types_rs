use super::*;
/// The elevation of a location ([WGS 84](https://en.wikipedia.org/wiki/World_Geodetic_System)). Values may be of the form 'NUMBER UNIT\_OF\_MEASUREMENT' (e.g., '1,000 m', '3,200 ft') while numbers alone should be assumed to be a value in meters.
///
/// https://schema.org/elevation
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ElevationProperty {
    #[cfg(any(feature = "number-schema", feature = "general-schema-section"))]
    Number(Number),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
