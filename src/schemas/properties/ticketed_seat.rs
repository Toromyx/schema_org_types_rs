use super::*;
/// The seat associated with the ticket.
///
/// <https://schema.org/ticketedSeat>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum TicketedSeatProperty {
    #[cfg(any(any(feature = "seat-schema", feature = "general-schema-section"), doc))]
    Seat(Seat),
}
