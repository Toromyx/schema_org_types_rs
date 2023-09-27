/// <https://schema.org/DrugCostCategory>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DrugCostCategory {
    /// <https://schema.org/ReimbursementCap>
    ReimbursementCap,
    /// <https://schema.org/Retail>
    Retail,
    /// <https://schema.org/Wholesale>
    Wholesale,
}
