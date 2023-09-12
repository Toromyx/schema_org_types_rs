use super::*;
/// A date value in [ISO 8601 date format](http://en.wikipedia.org/wiki/ISO_8601).
///
/// https://schema.org/Date
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Date(
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub crate::date_types::Date,
);
impl std::ops::Deref for Date {
    type Target = crate::date_types::Date;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
