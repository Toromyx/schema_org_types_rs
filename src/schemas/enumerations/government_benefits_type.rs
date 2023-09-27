/// <https://schema.org/GovernmentBenefitsType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum GovernmentBenefitsType {
    /// <https://schema.org/BasicIncome>
    BasicIncome,
    /// <https://schema.org/BusinessSupport>
    BusinessSupport,
    /// <https://schema.org/DisabilitySupport>
    DisabilitySupport,
    /// <https://schema.org/HealthCare>
    HealthCare,
    /// <https://schema.org/OneTimePayments>
    OneTimePayments,
    /// <https://schema.org/PaidLeave>
    PaidLeave,
    /// <https://schema.org/ParentalSupport>
    ParentalSupport,
    /// <https://schema.org/UnemploymentSupport>
    UnemploymentSupport,
}
