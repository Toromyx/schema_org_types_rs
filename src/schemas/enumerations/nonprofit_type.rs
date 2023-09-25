/// NonprofitType enumerates several kinds of official non-profit types of which a non-profit organization can be.
///
/// <https://schema.org/NonprofitType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum NonprofitType {}
