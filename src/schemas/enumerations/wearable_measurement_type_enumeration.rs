/// <https://schema.org/WearableMeasurementTypeEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum WearableMeasurementTypeEnumeration {
    /// <https://schema.org/WearableMeasurementBack>
    WearableMeasurementBack,
    /// <https://schema.org/WearableMeasurementChestOrBust>
    WearableMeasurementChestOrBust,
    /// <https://schema.org/WearableMeasurementCollar>
    WearableMeasurementCollar,
    /// <https://schema.org/WearableMeasurementCup>
    WearableMeasurementCup,
    /// <https://schema.org/WearableMeasurementHeight>
    WearableMeasurementHeight,
    /// <https://schema.org/WearableMeasurementHips>
    WearableMeasurementHips,
    /// <https://schema.org/WearableMeasurementInseam>
    WearableMeasurementInseam,
    /// <https://schema.org/WearableMeasurementLength>
    WearableMeasurementLength,
    /// <https://schema.org/WearableMeasurementOutsideLeg>
    WearableMeasurementOutsideLeg,
    /// <https://schema.org/WearableMeasurementSleeve>
    WearableMeasurementSleeve,
    /// <https://schema.org/WearableMeasurementWaist>
    WearableMeasurementWaist,
    /// <https://schema.org/WearableMeasurementWidth>
    WearableMeasurementWidth,
}
