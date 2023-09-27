use super::*;
/// <https://schema.org/Time>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Time(
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub crate::date_types::Time,
);
impl std::ops::Deref for Time {
    type Target = crate::date_types::Time;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
