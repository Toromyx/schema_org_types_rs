use super::*;
/// <https://schema.org/requiresSubscription>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum RequiresSubscriptionProperty {
	#[cfg(any(
		any(
			feature = "media-subscription-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	MediaSubscription(MediaSubscription),
	#[cfg(any(
		any(feature = "boolean-schema", feature = "general-schema-section"),
		doc
	))]
	Boolean(Boolean),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for RequiresSubscriptionProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "media-subscription-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				RequiresSubscriptionProperty::MediaSubscription(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "boolean-schema", feature = "general-schema-section"),
					doc
				))]
				RequiresSubscriptionProperty::Boolean(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for RequiresSubscriptionProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(
					feature = "media-subscription-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<MediaSubscription as Deserialize>::deserialize(deserializer),
				RequiresSubscriptionProperty::MediaSubscription,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "boolean-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Boolean as Deserialize>::deserialize(deserializer),
				RequiresSubscriptionProperty::Boolean,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property requiresSubscription",
			))
		}
	}
}
