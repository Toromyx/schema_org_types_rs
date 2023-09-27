/// <https://schema.org/GenderType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum GenderType {
    /// <https://schema.org/Female>
    Female,
    /// <https://schema.org/Male>
    Male,
}
