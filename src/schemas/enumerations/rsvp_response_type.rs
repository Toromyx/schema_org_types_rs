/// RsvpResponseType is an enumeration type whose instances represent responding to an RSVP request.
///
/// https://schema.org/RsvpResponseType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
