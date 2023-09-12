use super::*;
/// A work featured in some event, e.g. exhibited in an ExhibitionEvent.
/// Specific subproperties are available for workPerformed (e.g. a play), or a workPresented (a Movie at a ScreeningEvent).
///
/// https://schema.org/workFeatured
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum WorkFeaturedProperty {
    #[cfg(any(feature = "creative-work-schema", feature = "general-schema-section"))]
    CreativeWork(CreativeWork),
}
