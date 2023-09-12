use super::*;
/// A category for the item. Greater signs or slashes can be used to informally indicate a category hierarchy.
///
/// https://schema.org/category
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CategoryProperty {
    #[cfg(any(feature = "category-code-schema", feature = "pending-schema-section"))]
    CategoryCode(CategoryCode),
    #[cfg(any(
        feature = "physical-activity-category-schema",
        feature = "health-lifesci-schema-section"
    ))]
    PhysicalActivityCategory(PhysicalActivityCategory),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
    #[cfg(any(feature = "thing-schema", feature = "general-schema-section"))]
    Thing(Thing),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
