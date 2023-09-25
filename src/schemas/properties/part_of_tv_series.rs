use super::*;
/// The TV series to which this episode or season belongs.
///
/// https://schema.org/partOfTVSeries
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PartOfTvSeriesProperty {
    #[cfg(any(
        any(feature = "tv-series-schema", feature = "general-schema-section"),
        doc
    ))]
    TvSeries(TvSeries),
}
