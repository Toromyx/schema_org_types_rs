use super::*;
/// A combination of date and time of day in the form [-]CCYY-MM-DDThh:mm:ss[Z|(+|-)hh:mm] (see Chapter 5.4 of ISO 8601).
///
/// https://schema.org/DateTime
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct DateTime(
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub crate::date_types::DateTime,
);
impl std::ops::Deref for DateTime {
    type Target = crate::date_types::DateTime;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
