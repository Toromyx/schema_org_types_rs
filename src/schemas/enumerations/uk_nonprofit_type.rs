/// <https://schema.org/UKNonprofitType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
