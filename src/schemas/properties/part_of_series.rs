use super::*;
/// <https://schema.org/partOfSeries>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PartOfSeriesProperty {
	#[cfg(any(
		any(
			feature = "creative-work-series-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	CreativeWorkSeries(CreativeWorkSeries),
}
