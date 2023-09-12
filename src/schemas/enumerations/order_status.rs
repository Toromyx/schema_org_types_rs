/// Enumerated status values for Order.
///
/// https://schema.org/OrderStatus
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OrderStatus {
    /// OrderStatus representing cancellation of an order.
    ///
    /// https://schema.org/OrderCancelled
    OrderCancelled,
    /// OrderStatus representing successful delivery of an order.
    ///
    /// https://schema.org/OrderDelivered
    OrderDelivered,
    /// OrderStatus representing that an order is in transit.
    ///
    /// https://schema.org/OrderInTransit
    OrderInTransit,
    /// OrderStatus representing that payment is due on an order.
    ///
    /// https://schema.org/OrderPaymentDue
    OrderPaymentDue,
    /// OrderStatus representing availability of an order for pickup.
    ///
    /// https://schema.org/OrderPickupAvailable
    OrderPickupAvailable,
    /// OrderStatus representing that there is a problem with the order.
    ///
    /// https://schema.org/OrderProblem
    OrderProblem,
    /// OrderStatus representing that an order is being processed.
    ///
    /// https://schema.org/OrderProcessing
    OrderProcessing,
    /// OrderStatus representing that an order has been returned.
    ///
    /// https://schema.org/OrderReturned
    OrderReturned,
}
