/// Enumerates common size systems specific for wearable products
///
/// https://schema.org/WearableSizeSystemEnumeration
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum WearableSizeSystemEnumeration {
    /// Australian size system for wearables.
    ///
    /// https://schema.org/WearableSizeSystemAU
    WearableSizeSystemAu,
    /// Brazilian size system for wearables.
    ///
    /// https://schema.org/WearableSizeSystemBR
    WearableSizeSystemBr,
    /// Chinese size system for wearables.
    ///
    /// https://schema.org/WearableSizeSystemCN
    WearableSizeSystemCn,
    /// Continental size system for wearables.
    ///
    /// https://schema.org/WearableSizeSystemContinental
    WearableSizeSystemContinental,
    /// German size system for wearables.
    ///
    /// https://schema.org/WearableSizeSystemDE
    WearableSizeSystemDe,
    /// EN 13402 (joint European standard for size labelling of clothes).
    ///
    /// https://schema.org/WearableSizeSystemEN13402
    WearableSizeSystemEn13402,
    /// European size system for wearables.
    ///
    /// https://schema.org/WearableSizeSystemEurope
    WearableSizeSystemEurope,
    /// French size system for wearables.
    ///
    /// https://schema.org/WearableSizeSystemFR
    WearableSizeSystemFr,
    /// GS1 (formerly NRF) size system for wearables.
    ///
    /// https://schema.org/WearableSizeSystemGS1
    WearableSizeSystemGs1,
    /// Italian size system for wearables.
    ///
    /// https://schema.org/WearableSizeSystemIT
    WearableSizeSystemIt,
    /// Japanese size system for wearables.
    ///
    /// https://schema.org/WearableSizeSystemJP
    WearableSizeSystemJp,
    /// Mexican size system for wearables.
    ///
    /// https://schema.org/WearableSizeSystemMX
    WearableSizeSystemMx,
    /// United Kingdom size system for wearables.
    ///
    /// https://schema.org/WearableSizeSystemUK
    WearableSizeSystemUk,
    /// United States size system for wearables.
    ///
    /// https://schema.org/WearableSizeSystemUS
    WearableSizeSystemUs,
}
