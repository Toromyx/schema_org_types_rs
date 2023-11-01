use super::*;
/// <https://schema.org/arrivalBusStop>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ArrivalBusStopProperty {
	#[cfg(any(
		any(feature = "bus-station-schema", feature = "general-schema-section"),
		doc
	))]
	BusStation(BusStation),
	#[cfg(any(
		any(feature = "bus-stop-schema", feature = "general-schema-section"),
		doc
	))]
	BusStop(BusStop),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for ArrivalBusStopProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "bus-station-schema", feature = "general-schema-section"),
					doc
				))]
				ArrivalBusStopProperty::BusStation(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "bus-stop-schema", feature = "general-schema-section"),
					doc
				))]
				ArrivalBusStopProperty::BusStop(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for ArrivalBusStopProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "bus-station-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<BusStation as Deserialize>::deserialize(deserializer),
				ArrivalBusStopProperty::BusStation,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "bus-stop-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<BusStop as Deserialize>::deserialize(deserializer),
				ArrivalBusStopProperty::BusStop,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property arrivalBusStop",
			))
		}
	}
}
