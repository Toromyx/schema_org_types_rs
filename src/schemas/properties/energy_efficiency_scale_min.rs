use super::*;
/// <https://schema.org/energyEfficiencyScaleMin>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum EnergyEfficiencyScaleMinProperty {
	/// <https://schema.org/EUEnergyEfficiencyEnumeration>
	EuEnergyEfficiencyEnumeration(EuEnergyEfficiencyEnumeration),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for EnergyEfficiencyScaleMinProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				EnergyEfficiencyScaleMinProperty::EuEnergyEfficiencyEnumeration(ref inner) => {
					inner.serialize(serializer)
				}
				#[cfg(all(feature = "fallible", feature = "serde"))]
				EnergyEfficiencyScaleMinProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for EnergyEfficiencyScaleMinProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<EuEnergyEfficiencyEnumeration as Deserialize>::deserialize(deserializer),
				EnergyEfficiencyScaleMinProperty::EuEnergyEfficiencyEnumeration,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				EnergyEfficiencyScaleMinProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property energyEfficiencyScaleMin or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property energyEfficiencyScaleMin";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
