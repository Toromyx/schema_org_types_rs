/// NLNonprofitType: Non-profit organization type originating from the Netherlands.
///
/// https://schema.org/NLNonprofitType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NlNonprofitType {
    /// NonprofitANBI: Non-profit type referring to a Public Benefit Organization (NL).
    ///
    /// https://schema.org/NonprofitANBI
    NonprofitAnbi,
    /// NonprofitSBBI: Non-profit type referring to a Social Interest Promoting Institution (NL).
    ///
    /// https://schema.org/NonprofitSBBI
    NonprofitSbbi,
}
