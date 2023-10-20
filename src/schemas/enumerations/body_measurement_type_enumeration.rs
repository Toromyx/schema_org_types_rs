/// <https://schema.org/BodyMeasurementTypeEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
