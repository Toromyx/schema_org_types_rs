use super::*;
/// A ticket associated with the reservation.
///
/// https://schema.org/reservedTicket
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ReservedTicketProperty {
    #[cfg(any(
        any(feature = "ticket-schema", feature = "general-schema-section"),
        doc
    ))]
    Ticket(Ticket),
}
