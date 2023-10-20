use super::*;
/// <https://schema.org/costCategory>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
