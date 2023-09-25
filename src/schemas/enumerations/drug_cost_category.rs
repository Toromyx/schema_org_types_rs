/// Enumerated categories of medical drug costs.
///
/// <https://schema.org/DrugCostCategory>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum DrugCostCategory {
    /// The drug's cost represents the maximum reimbursement paid by an insurer for the drug.
    ///
    /// <https://schema.org/ReimbursementCap>
    ReimbursementCap,
    /// The drug's cost represents the retail cost of the drug.
    ///
    /// <https://schema.org/Retail>
    Retail,
    /// The drug's cost represents the wholesale acquisition cost of the drug.
    ///
    /// <https://schema.org/Wholesale>
    Wholesale,
}
