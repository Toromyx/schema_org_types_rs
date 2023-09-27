/// <https://schema.org/DeliveryMethod>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum DeliveryMethod {
    /// <https://schema.org/LockerDelivery>
    LockerDelivery,
    /// <https://schema.org/OnSitePickup>
    OnSitePickup,
    /// <https://schema.org/ParcelService>
    ParcelService,
}
