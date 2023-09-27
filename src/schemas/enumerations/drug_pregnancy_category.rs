/// <https://schema.org/DrugPregnancyCategory>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
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
