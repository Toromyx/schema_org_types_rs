use super::*;
/// The movie presented during this event.
///
/// https://schema.org/workPresented
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum WorkPresentedProperty {
    #[cfg(any(any(feature = "movie-schema", feature = "general-schema-section"), doc))]
    Movie(Movie),
}
