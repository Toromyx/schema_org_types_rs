/// <https://schema.org/DrugCostCategory>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum DrugCostCategory {
    /// <https://schema.org/ReimbursementCap>
    ReimbursementCap,
    /// <https://schema.org/Retail>
    Retail,
    /// <https://schema.org/Wholesale>
    Wholesale,
}
