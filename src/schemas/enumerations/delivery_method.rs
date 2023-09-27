/// <https://schema.org/DeliveryMethod>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeliveryMethod {
    /// <https://schema.org/LockerDelivery>
    LockerDelivery,
    /// <https://schema.org/OnSitePickup>
    OnSitePickup,
    /// <https://schema.org/ParcelService>
    ParcelService,
}
