use super::*;
/// The category of cost, such as wholesale, retail, reimbursement cap, etc.
///
/// https://schema.org/costCategory
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum CostCategoryProperty {
    #[cfg(any(
        any(
            feature = "drug-cost-category-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    DrugCostCategory(DrugCostCategory),
}
