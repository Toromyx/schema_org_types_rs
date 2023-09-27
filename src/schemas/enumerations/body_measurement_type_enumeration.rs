/// <https://schema.org/BodyMeasurementTypeEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum BodyMeasurementTypeEnumeration {
    /// <https://schema.org/BodyMeasurementArm>
    BodyMeasurementArm,
    /// <https://schema.org/BodyMeasurementBust>
    BodyMeasurementBust,
    /// <https://schema.org/BodyMeasurementChest>
    BodyMeasurementChest,
    /// <https://schema.org/BodyMeasurementFoot>
    BodyMeasurementFoot,
    /// <https://schema.org/BodyMeasurementHand>
    BodyMeasurementHand,
    /// <https://schema.org/BodyMeasurementHead>
    BodyMeasurementHead,
    /// <https://schema.org/BodyMeasurementHeight>
    BodyMeasurementHeight,
    /// <https://schema.org/BodyMeasurementHips>
    BodyMeasurementHips,
    /// <https://schema.org/BodyMeasurementInsideLeg>
    BodyMeasurementInsideLeg,
    /// <https://schema.org/BodyMeasurementNeck>
    BodyMeasurementNeck,
    /// <https://schema.org/BodyMeasurementUnderbust>
    BodyMeasurementUnderbust,
    /// <https://schema.org/BodyMeasurementWaist>
    BodyMeasurementWaist,
    /// <https://schema.org/BodyMeasurementWeight>
    BodyMeasurementWeight,
}
