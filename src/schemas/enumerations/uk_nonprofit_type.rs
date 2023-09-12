/// UKNonprofitType: Non-profit organization type originating from the United Kingdom.
///
/// https://schema.org/UKNonprofitType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UkNonprofitType {
    /// CharitableIncorporatedOrganization: Non-profit type referring to a Charitable Incorporated Organization (UK).
    ///
    /// https://schema.org/CharitableIncorporatedOrganization
    CharitableIncorporatedOrganization,
    /// LimitedByGuaranteeCharity: Non-profit type referring to a charitable company that is limited by guarantee (UK).
    ///
    /// https://schema.org/LimitedByGuaranteeCharity
    LimitedByGuaranteeCharity,
    /// UKTrust: Non-profit type referring to a UK trust.
    ///
    /// https://schema.org/UKTrust
    UkTrust,
    /// UnincorporatedAssociationCharity: Non-profit type referring to a charitable company that is not incorporated (UK).
    ///
    /// https://schema.org/UnincorporatedAssociationCharity
    UnincorporatedAssociationCharity,
}
