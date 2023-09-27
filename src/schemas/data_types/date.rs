use super::*;
/// <https://schema.org/Date>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Date(
    #[cfg_attr(
        any(feature = "serde", doc),
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
