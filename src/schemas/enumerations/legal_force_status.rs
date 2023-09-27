/// <https://schema.org/LegalForceStatus>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum LegalForceStatus {
    /// <https://schema.org/InForce>
    InForce,
    /// <https://schema.org/NotInForce>
    NotInForce,
    /// <https://schema.org/PartiallyInForce>
    PartiallyInForce,
}
