/// NLNonprofitType: Non-profit organization type originating from the Netherlands.
///
/// https://schema.org/NLNonprofitType
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
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
