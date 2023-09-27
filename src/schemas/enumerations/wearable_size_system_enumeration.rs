/// <https://schema.org/WearableSizeSystemEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum WearableSizeSystemEnumeration {
    /// <https://schema.org/WearableSizeSystemAU>
    WearableSizeSystemAu,
    /// <https://schema.org/WearableSizeSystemBR>
    WearableSizeSystemBr,
    /// <https://schema.org/WearableSizeSystemCN>
    WearableSizeSystemCn,
    /// <https://schema.org/WearableSizeSystemContinental>
    WearableSizeSystemContinental,
    /// <https://schema.org/WearableSizeSystemDE>
    WearableSizeSystemDe,
    /// <https://schema.org/WearableSizeSystemEN13402>
    WearableSizeSystemEn13402,
    /// <https://schema.org/WearableSizeSystemEurope>
    WearableSizeSystemEurope,
    /// <https://schema.org/WearableSizeSystemFR>
    WearableSizeSystemFr,
    /// <https://schema.org/WearableSizeSystemGS1>
    WearableSizeSystemGs1,
    /// <https://schema.org/WearableSizeSystemIT>
    WearableSizeSystemIt,
    /// <https://schema.org/WearableSizeSystemJP>
    WearableSizeSystemJp,
    /// <https://schema.org/WearableSizeSystemMX>
    WearableSizeSystemMx,
    /// <https://schema.org/WearableSizeSystemUK>
    WearableSizeSystemUk,
    /// <https://schema.org/WearableSizeSystemUS>
    WearableSizeSystemUs,
}
