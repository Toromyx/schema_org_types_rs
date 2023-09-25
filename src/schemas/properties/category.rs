use super::*;
/// A category for the item. Greater signs or slashes can be used to informally indicate a category hierarchy.
///
/// <https://schema.org/category>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum CategoryProperty {
    #[cfg(any(
        any(feature = "category-code-schema", feature = "pending-schema-section"),
        doc
    ))]
    CategoryCode(CategoryCode),
    #[cfg(any(
        any(
            feature = "physical-activity-category-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    PhysicalActivityCategory(PhysicalActivityCategory),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
    #[cfg(any(any(feature = "thing-schema", feature = "general-schema-section"), doc))]
    Thing(Thing),
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
}
