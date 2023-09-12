use super::*;
/// collectiondate - Date for which patient counts are reported.
///
/// https://schema.org/cvdCollectionDate
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CvdCollectionDateProperty {
    #[cfg(any(feature = "date-time-schema", feature = "general-schema-section"))]
    DateTime(DateTime),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
