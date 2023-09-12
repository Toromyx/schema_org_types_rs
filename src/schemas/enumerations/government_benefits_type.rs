/// GovernmentBenefitsType enumerates several kinds of government benefits to support the COVID-19 situation. Note that this structure may not capture all benefits offered.
///
/// https://schema.org/GovernmentBenefitsType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GovernmentBenefitsType {
    /// BasicIncome: this is a benefit for basic income.
    ///
    /// https://schema.org/BasicIncome
    BasicIncome,
    /// BusinessSupport: this is a benefit for supporting businesses.
    ///
    /// https://schema.org/BusinessSupport
    BusinessSupport,
    /// DisabilitySupport: this is a benefit for disability support.
    ///
    /// https://schema.org/DisabilitySupport
    DisabilitySupport,
    /// HealthCare: this is a benefit for health care.
    ///
    /// https://schema.org/HealthCare
    HealthCare,
    /// OneTimePayments: this is a benefit for one-time payments for individuals.
    ///
    /// https://schema.org/OneTimePayments
    OneTimePayments,
    /// PaidLeave: this is a benefit for paid leave.
    ///
    /// https://schema.org/PaidLeave
    PaidLeave,
    /// ParentalSupport: this is a benefit for parental support.
    ///
    /// https://schema.org/ParentalSupport
    ParentalSupport,
    /// UnemploymentSupport: this is a benefit for unemployment support.
    ///
    /// https://schema.org/UnemploymentSupport
    UnemploymentSupport,
}
