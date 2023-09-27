/// <https://schema.org/DrugPregnancyCategory>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DrugPregnancyCategory {
    /// <https://schema.org/FDAcategoryA>
    FdAcategoryA,
    /// <https://schema.org/FDAcategoryB>
    FdAcategoryB,
    /// <https://schema.org/FDAcategoryC>
    FdAcategoryC,
    /// <https://schema.org/FDAcategoryD>
    FdAcategoryD,
    /// <https://schema.org/FDAcategoryX>
    FdAcategoryX,
    /// <https://schema.org/FDAnotEvaluated>
    FdAnotEvaluated,
}
