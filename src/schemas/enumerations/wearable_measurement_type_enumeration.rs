/// Enumerates common types of measurement for wearables products.
///
/// <https://schema.org/WearableMeasurementTypeEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum WearableMeasurementTypeEnumeration {
    /// Measurement of the back section, for example of a jacket
    ///
    /// <https://schema.org/WearableMeasurementBack>
    WearableMeasurementBack,
    /// Measurement of the chest/bust section, for example of a suit
    ///
    /// <https://schema.org/WearableMeasurementChestOrBust>
    WearableMeasurementChestOrBust,
    /// Measurement of the collar, for example of a shirt
    ///
    /// <https://schema.org/WearableMeasurementCollar>
    WearableMeasurementCollar,
    /// Measurement of the cup, for example of a bra
    ///
    /// <https://schema.org/WearableMeasurementCup>
    WearableMeasurementCup,
    /// Measurement of the height, for example the heel height of a shoe
    ///
    /// <https://schema.org/WearableMeasurementHeight>
    WearableMeasurementHeight,
    /// Measurement of the hip section, for example of a skirt
    ///
    /// <https://schema.org/WearableMeasurementHips>
    WearableMeasurementHips,
    /// Measurement of the inseam, for example of pants
    ///
    /// <https://schema.org/WearableMeasurementInseam>
    WearableMeasurementInseam,
    /// Represents the length, for example of a dress
    ///
    /// <https://schema.org/WearableMeasurementLength>
    WearableMeasurementLength,
    /// Measurement of the outside leg, for example of pants
    ///
    /// <https://schema.org/WearableMeasurementOutsideLeg>
    WearableMeasurementOutsideLeg,
    /// Measurement of the sleeve length, for example of a shirt
    ///
    /// <https://schema.org/WearableMeasurementSleeve>
    WearableMeasurementSleeve,
    /// Measurement of the waist section, for example of pants
    ///
    /// <https://schema.org/WearableMeasurementWaist>
    WearableMeasurementWaist,
    /// Measurement of the width, for example of shoes
    ///
    /// <https://schema.org/WearableMeasurementWidth>
    WearableMeasurementWidth,
}
