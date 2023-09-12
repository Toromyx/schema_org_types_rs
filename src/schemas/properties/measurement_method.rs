use super::*;
/// A subproperty of [[measurementTechnique]] that can be used for specifying specific methods, in particular via [[MeasurementMethodEnum]].
///
/// https://schema.org/measurementMethod
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MeasurementMethodProperty {
    #[cfg(any(feature = "defined-term-schema", feature = "pending-schema-section"))]
    DefinedTerm(DefinedTerm),
    #[cfg(any(
        feature = "measurement-method-enum-schema",
        feature = "pending-schema-section"
    ))]
    MeasurementMethodEnum(MeasurementMethodEnum),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
