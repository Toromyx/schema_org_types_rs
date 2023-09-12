use super::*;
/// An Offer which must be accepted before the user can perform the Action. For example, the user may need to buy a movie before being able to watch it.
///
/// https://schema.org/expectsAcceptanceOf
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ExpectsAcceptanceOfProperty {
    #[cfg(any(feature = "offer-schema", feature = "general-schema-section"))]
    Offer(Offer),
}
