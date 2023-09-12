/// An enumeration of genders.
///
/// https://schema.org/GenderType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
