/// <https://schema.org/RsvpResponseType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum RsvpResponseType {
    /// <https://schema.org/RsvpResponseMaybe>
    RsvpResponseMaybe,
    /// <https://schema.org/RsvpResponseNo>
    RsvpResponseNo,
    /// <https://schema.org/RsvpResponseYes>
    RsvpResponseYes,
}
