/// An enumeration of genders.
///
/// https://schema.org/GenderType
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum GenderType {
    /// The female gender.
    ///
    /// https://schema.org/Female
    Female,
    /// The male gender.
    ///
    /// https://schema.org/Male
    Male,
}
