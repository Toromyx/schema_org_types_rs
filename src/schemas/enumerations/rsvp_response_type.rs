/// RsvpResponseType is an enumeration type whose instances represent responding to an RSVP request.
///
/// https://schema.org/RsvpResponseType
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum RsvpResponseType {
    /// The invitee may or may not attend.
    ///
    /// https://schema.org/RsvpResponseMaybe
    RsvpResponseMaybe,
    /// The invitee will not attend.
    ///
    /// https://schema.org/RsvpResponseNo
    RsvpResponseNo,
    /// The invitee will attend.
    ///
    /// https://schema.org/RsvpResponseYes
    RsvpResponseYes,
}
