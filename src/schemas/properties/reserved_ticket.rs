use super::*;
/// A ticket associated with the reservation.
///
/// https://schema.org/reservedTicket
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ReservedTicketProperty {
    #[cfg(any(feature = "ticket-schema", feature = "general-schema-section"))]
    Ticket(Ticket),
}
