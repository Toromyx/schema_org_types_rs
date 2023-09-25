use super::*;
/// A list of single or combined accessModes that are sufficient to understand all the intellectual content of a resource. Values should be drawn from the [approved vocabulary](https://www.w3.org/2021/a11y-discov-vocab/latest/#accessModeSufficient-vocabulary).
///
/// <https://schema.org/accessModeSufficient>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AccessModeSufficientProperty {
    #[cfg(any(
        any(feature = "item-list-schema", feature = "general-schema-section"),
        doc
    ))]
    ItemList(ItemList),
}
