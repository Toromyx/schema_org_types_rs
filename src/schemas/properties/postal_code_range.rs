use super::*;
/// A defined range of postal codes.
///
/// https://schema.org/postalCodeRange
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PostalCodeRangeProperty {
    #[cfg(any(
        feature = "postal-code-range-specification-schema",
        feature = "pending-schema-section"
    ))]
    PostalCodeRangeSpecification(PostalCodeRangeSpecification),
}
