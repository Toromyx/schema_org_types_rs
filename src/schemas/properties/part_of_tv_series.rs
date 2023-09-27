use super::*;
/// <https://schema.org/partOfTVSeries>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PartOfTvSeriesProperty {
    #[cfg(any(
        any(feature = "tv-series-schema", feature = "general-schema-section"),
        doc
    ))]
    TvSeries(TvSeries),
}
