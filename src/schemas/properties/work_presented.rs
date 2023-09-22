use super::*;
/// The movie presented during this event.
///
/// https://schema.org/workPresented
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum WorkPresentedProperty {
    #[cfg(any(feature = "movie-schema", feature = "general-schema-section"))]
    Movie(Movie),
}