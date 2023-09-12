use super::*;
/// The seat associated with the ticket.
///
/// https://schema.org/ticketedSeat
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TicketedSeatProperty {
    #[cfg(any(feature = "seat-schema", feature = "general-schema-section"))]
    Seat(Seat),
}
