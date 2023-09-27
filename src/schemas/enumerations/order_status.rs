/// <https://schema.org/OrderStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OrderStatus {
    /// <https://schema.org/OrderCancelled>
    OrderCancelled,
    /// <https://schema.org/OrderDelivered>
    OrderDelivered,
    /// <https://schema.org/OrderInTransit>
    OrderInTransit,
    /// <https://schema.org/OrderPaymentDue>
    OrderPaymentDue,
    /// <https://schema.org/OrderPickupAvailable>
    OrderPickupAvailable,
    /// <https://schema.org/OrderProblem>
    OrderProblem,
    /// <https://schema.org/OrderProcessing>
    OrderProcessing,
    /// <https://schema.org/OrderReturned>
    OrderReturned,
}
