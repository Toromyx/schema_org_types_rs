use super::*;
/// A combination of date and time of day in the form [-]CCYY-MM-DDThh:mm:ss[Z|(+|-)hh:mm] (see Chapter 5.4 of ISO 8601).
///
/// https://schema.org/DateTime
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DateTime(
    #[cfg_attr(
        feature = "serde",
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
