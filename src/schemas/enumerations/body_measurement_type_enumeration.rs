/// Enumerates types (or dimensions) of a person's body measurements, for example for fitting of clothes.
///
/// https://schema.org/BodyMeasurementTypeEnumeration
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BodyMeasurementTypeEnumeration {
    /// Arm length (measured between arms/shoulder line intersection and the prominent wrist bone). Used, for example, to fit shirts.
    ///
    /// https://schema.org/BodyMeasurementArm
    BodyMeasurementArm,
    /// Maximum girth of bust. Used, for example, to fit women's suits.
    ///
    /// https://schema.org/BodyMeasurementBust
    BodyMeasurementBust,
    /// Maximum girth of chest. Used, for example, to fit men's suits.
    ///
    /// https://schema.org/BodyMeasurementChest
    BodyMeasurementChest,
    /// Foot length (measured between end of the most prominent toe and the most prominent part of the heel). Used, for example, to measure socks.
    ///
    /// https://schema.org/BodyMeasurementFoot
    BodyMeasurementFoot,
    /// Maximum hand girth (measured over the knuckles of the open right hand excluding thumb, fingers together). Used, for example, to fit gloves.
    ///
    /// https://schema.org/BodyMeasurementHand
    BodyMeasurementHand,
    /// Maximum girth of head above the ears. Used, for example, to fit hats.
    ///
    /// https://schema.org/BodyMeasurementHead
    BodyMeasurementHead,
    /// Body height (measured between crown of head and soles of feet). Used, for example, to fit jackets.
    ///
    /// https://schema.org/BodyMeasurementHeight
    BodyMeasurementHeight,
    /// Girth of hips (measured around the buttocks). Used, for example, to fit skirts.
    ///
    /// https://schema.org/BodyMeasurementHips
    BodyMeasurementHips,
    /// Inside leg (measured between crotch and soles of feet). Used, for example, to fit pants.
    ///
    /// https://schema.org/BodyMeasurementInsideLeg
    BodyMeasurementInsideLeg,
    /// Girth of neck. Used, for example, to fit shirts.
    ///
    /// https://schema.org/BodyMeasurementNeck
    BodyMeasurementNeck,
    /// Girth of body just below the bust. Used, for example, to fit women's swimwear.
    ///
    /// https://schema.org/BodyMeasurementUnderbust
    BodyMeasurementUnderbust,
    /// Girth of natural waistline (between hip bones and lower ribs). Used, for example, to fit pants.
    ///
    /// https://schema.org/BodyMeasurementWaist
    BodyMeasurementWaist,
    /// Body weight. Used, for example, to measure pantyhose.
    ///
    /// https://schema.org/BodyMeasurementWeight
    BodyMeasurementWeight,
}
