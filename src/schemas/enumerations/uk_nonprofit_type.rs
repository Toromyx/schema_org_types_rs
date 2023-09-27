/// <https://schema.org/UKNonprofitType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum UkNonprofitType {
    /// <https://schema.org/CharitableIncorporatedOrganization>
    CharitableIncorporatedOrganization,
    /// <https://schema.org/LimitedByGuaranteeCharity>
    LimitedByGuaranteeCharity,
    /// <https://schema.org/UKTrust>
    UkTrust,
    /// <https://schema.org/UnincorporatedAssociationCharity>
    UnincorporatedAssociationCharity,
}
