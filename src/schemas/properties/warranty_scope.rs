use super::*;
/// <https://schema.org/warrantyScope>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum WarrantyScopeProperty {
	#[cfg(any(
		any(feature = "warranty-scope-schema", feature = "general-schema-section"),
		doc
	))]
	WarrantyScope(WarrantyScope),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for WarrantyScopeProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "warranty-scope-schema", feature = "general-schema-section"),
					doc
				))]
				WarrantyScopeProperty::WarrantyScope(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for WarrantyScopeProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "warranty-scope-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<WarrantyScope as Deserialize>::deserialize(deserializer),
				WarrantyScopeProperty::WarrantyScope,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property warrantyScope",
			))
		}
	}
}
