use super::*;
/// The TV series to which this episode or season belongs.
///
/// https://schema.org/partOfTVSeries
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PartOfTvSeriesProperty {
    #[cfg(any(feature = "tv-series-schema", feature = "general-schema-section"))]
    TvSeries(TvSeries),
}
