use super::*;
/// An Offer which must be accepted before the user can perform the Action. For example, the user may need to buy a movie before being able to watch it.
///
/// https://schema.org/expectsAcceptanceOf
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ExpectsAcceptanceOfProperty {
    #[cfg(any(any(feature = "offer-schema", feature = "general-schema-section"), doc))]
    Offer(Offer),
}
