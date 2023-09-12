/// Enumerated categories of medical drug costs.
///
/// https://schema.org/DrugCostCategory
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DrugCostCategory {
    /// The drug's cost represents the maximum reimbursement paid by an insurer for the drug.
    ///
    /// https://schema.org/ReimbursementCap
    ReimbursementCap,
    /// The drug's cost represents the retail cost of the drug.
    ///
    /// https://schema.org/Retail
    Retail,
    /// The drug's cost represents the wholesale acquisition cost of the drug.
    ///
    /// https://schema.org/Wholesale
    Wholesale,
}
