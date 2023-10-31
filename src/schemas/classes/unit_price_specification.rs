use super::*;
/// <https://schema.org/UnitPriceSpecification>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct UnitPriceSpecification {
	#[cfg(any(
		any(
			feature = "additional-type-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	#[cfg(any(
		any(
			feature = "alternate-name-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#alternate_name: Vec<AlternateNameProperty>,
	#[cfg(any(
		any(
			feature = "billing-duration-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#billing_duration: Vec<BillingDurationProperty>,
	#[cfg(any(
		any(
			feature = "billing-increment-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#billing_increment: Vec<BillingIncrementProperty>,
	#[cfg(any(
		any(
			feature = "billing-start-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#billing_start: Vec<BillingStartProperty>,
	#[cfg(any(
		any(
			feature = "description-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#description: Vec<DescriptionProperty>,
	#[cfg(any(
		any(
			feature = "disambiguating-description-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	#[cfg(any(
		any(
			feature = "eligible-quantity-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#eligible_quantity: Vec<EligibleQuantityProperty>,
	#[cfg(any(
		any(
			feature = "eligible-transaction-volume-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#eligible_transaction_volume: Vec<EligibleTransactionVolumeProperty>,
	#[cfg(any(
		any(
			feature = "identifier-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#identifier: Vec<IdentifierProperty>,
	#[cfg(any(
		any(feature = "image-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#image: Vec<ImageProperty>,
	#[cfg(any(
		any(
			feature = "main-entity-of-page-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	#[cfg(any(
		any(
			feature = "max-price-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#max_price: Vec<MaxPriceProperty>,
	#[cfg(any(
		any(
			feature = "min-price-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#min_price: Vec<MinPriceProperty>,
	#[cfg(any(
		any(feature = "name-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#name: Vec<NameProperty>,
	#[cfg(any(
		any(
			feature = "potential-action-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#potential_action: Vec<PotentialActionProperty>,
	#[cfg(any(
		any(feature = "price-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#price: Vec<PriceProperty>,
	#[cfg(any(
		any(
			feature = "price-component-type-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#price_component_type: Vec<PriceComponentTypeProperty>,
	#[cfg(any(
		any(
			feature = "price-currency-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#price_currency: Vec<PriceCurrencyProperty>,
	#[cfg(any(
		any(
			feature = "price-type-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#price_type: Vec<PriceTypeProperty>,
	#[cfg(any(
		any(
			feature = "reference-quantity-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#reference_quantity: Vec<ReferenceQuantityProperty>,
	#[cfg(any(
		any(
			feature = "same-as-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#same_as: Vec<SameAsProperty>,
	#[cfg(any(
		any(
			feature = "subject-of-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#subject_of: Vec<SubjectOfProperty>,
	#[cfg(any(
		any(
			feature = "unit-code-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#unit_code: Vec<UnitCodeProperty>,
	#[cfg(any(
		any(
			feature = "unit-text-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#unit_text: Vec<UnitTextProperty>,
	#[cfg(any(
		any(feature = "url-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#url: Vec<UrlProperty>,
	#[cfg(any(
		any(
			feature = "valid-from-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#valid_from: Vec<ValidFromProperty>,
	#[cfg(any(
		any(
			feature = "valid-through-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#valid_through: Vec<ValidThroughProperty>,
	#[cfg(any(
		any(
			feature = "value-added-tax-included-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#value_added_tax_included: Vec<ValueAddedTaxIncludedProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for UnitPriceSpecification {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				if cfg!(any(
					any(
						feature = "additional-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#additional_type) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#alternate_name) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "billing-duration-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#billing_duration) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "billing-increment-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#billing_increment) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "billing-start-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#billing_start) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "description-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#description) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "disambiguating-description-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#disambiguating_description) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "eligible-quantity-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#eligible_quantity) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "eligible-transaction-volume-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#eligible_transaction_volume) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "identifier-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#identifier) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "image-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#image) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "main-entity-of-page-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#main_entity_of_page) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "max-price-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#max_price) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "min-price-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#min_price) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "name-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#name) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "potential-action-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#potential_action) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "price-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#price) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "price-component-type-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#price_component_type) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "price-currency-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#price_currency) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "price-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#price_type) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "reference-quantity-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#reference_quantity) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#same_as) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "subject-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#subject_of) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "unit-code-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#unit_code) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "unit-text-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#unit_text) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "url-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#url) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "valid-from-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#valid_from) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "valid-through-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#valid_through) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "value-added-tax-included-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#value_added_tax_included) as usize
				} else {
					0
				},
			]
			.iter()
			.sum();
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "UnitPriceSpecification", len)?;
			#[cfg(any(
				any(
					feature = "additional-type-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#additional_type) {
				serialize_struct.serialize_field("additionalType", {
					struct SerializeWith<'a>(&'a Vec<AdditionalTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#additional_type)
				})?;
			} else {
				serialize_struct.skip_field("additionalType")?;
			}
			#[cfg(any(
				any(
					feature = "alternate-name-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#alternate_name) {
				serialize_struct.serialize_field("alternateName", {
					struct SerializeWith<'a>(&'a Vec<AlternateNameProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#alternate_name)
				})?;
			} else {
				serialize_struct.skip_field("alternateName")?;
			}
			#[cfg(any(
				any(
					feature = "billing-duration-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#billing_duration) {
				serialize_struct.serialize_field("billingDuration", {
					struct SerializeWith<'a>(&'a Vec<BillingDurationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#billing_duration)
				})?;
			} else {
				serialize_struct.skip_field("billingDuration")?;
			}
			#[cfg(any(
				any(
					feature = "billing-increment-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#billing_increment) {
				serialize_struct.serialize_field("billingIncrement", {
					struct SerializeWith<'a>(&'a Vec<BillingIncrementProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#billing_increment)
				})?;
			} else {
				serialize_struct.skip_field("billingIncrement")?;
			}
			#[cfg(any(
				any(
					feature = "billing-start-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#billing_start) {
				serialize_struct.serialize_field("billingStart", {
					struct SerializeWith<'a>(&'a Vec<BillingStartProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#billing_start)
				})?;
			} else {
				serialize_struct.skip_field("billingStart")?;
			}
			#[cfg(any(
				any(
					feature = "description-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#description) {
				serialize_struct.serialize_field("description", {
					struct SerializeWith<'a>(&'a Vec<DescriptionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#description)
				})?;
			} else {
				serialize_struct.skip_field("description")?;
			}
			#[cfg(any(
				any(
					feature = "disambiguating-description-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#disambiguating_description) {
				serialize_struct.serialize_field("disambiguatingDescription", {
					struct SerializeWith<'a>(&'a Vec<DisambiguatingDescriptionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#disambiguating_description)
				})?;
			} else {
				serialize_struct.skip_field("disambiguatingDescription")?;
			}
			#[cfg(any(
				any(
					feature = "eligible-quantity-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#eligible_quantity) {
				serialize_struct.serialize_field("eligibleQuantity", {
					struct SerializeWith<'a>(&'a Vec<EligibleQuantityProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#eligible_quantity)
				})?;
			} else {
				serialize_struct.skip_field("eligibleQuantity")?;
			}
			#[cfg(any(
				any(
					feature = "eligible-transaction-volume-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#eligible_transaction_volume) {
				serialize_struct.serialize_field("eligibleTransactionVolume", {
					struct SerializeWith<'a>(&'a Vec<EligibleTransactionVolumeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#eligible_transaction_volume)
				})?;
			} else {
				serialize_struct.skip_field("eligibleTransactionVolume")?;
			}
			#[cfg(any(
				any(
					feature = "identifier-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#identifier) {
				serialize_struct.serialize_field("identifier", {
					struct SerializeWith<'a>(&'a Vec<IdentifierProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#identifier)
				})?;
			} else {
				serialize_struct.skip_field("identifier")?;
			}
			#[cfg(any(
				any(feature = "image-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#image) {
				serialize_struct.serialize_field("image", {
					struct SerializeWith<'a>(&'a Vec<ImageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#image)
				})?;
			} else {
				serialize_struct.skip_field("image")?;
			}
			#[cfg(any(
				any(
					feature = "main-entity-of-page-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#main_entity_of_page) {
				serialize_struct.serialize_field("mainEntityOfPage", {
					struct SerializeWith<'a>(&'a Vec<MainEntityOfPageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#main_entity_of_page)
				})?;
			} else {
				serialize_struct.skip_field("mainEntityOfPage")?;
			}
			#[cfg(any(
				any(
					feature = "max-price-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#max_price) {
				serialize_struct.serialize_field("maxPrice", {
					struct SerializeWith<'a>(&'a Vec<MaxPriceProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#max_price)
				})?;
			} else {
				serialize_struct.skip_field("maxPrice")?;
			}
			#[cfg(any(
				any(
					feature = "min-price-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#min_price) {
				serialize_struct.serialize_field("minPrice", {
					struct SerializeWith<'a>(&'a Vec<MinPriceProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#min_price)
				})?;
			} else {
				serialize_struct.skip_field("minPrice")?;
			}
			#[cfg(any(
				any(feature = "name-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#name) {
				serialize_struct.serialize_field("name", {
					struct SerializeWith<'a>(&'a Vec<NameProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#name)
				})?;
			} else {
				serialize_struct.skip_field("name")?;
			}
			#[cfg(any(
				any(
					feature = "potential-action-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#potential_action) {
				serialize_struct.serialize_field("potentialAction", {
					struct SerializeWith<'a>(&'a Vec<PotentialActionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#potential_action)
				})?;
			} else {
				serialize_struct.skip_field("potentialAction")?;
			}
			#[cfg(any(
				any(feature = "price-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#price) {
				serialize_struct.serialize_field("price", {
					struct SerializeWith<'a>(&'a Vec<PriceProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#price)
				})?;
			} else {
				serialize_struct.skip_field("price")?;
			}
			#[cfg(any(
				any(
					feature = "price-component-type-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#price_component_type) {
				serialize_struct.serialize_field("priceComponentType", {
					struct SerializeWith<'a>(&'a Vec<PriceComponentTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#price_component_type)
				})?;
			} else {
				serialize_struct.skip_field("priceComponentType")?;
			}
			#[cfg(any(
				any(
					feature = "price-currency-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#price_currency) {
				serialize_struct.serialize_field("priceCurrency", {
					struct SerializeWith<'a>(&'a Vec<PriceCurrencyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#price_currency)
				})?;
			} else {
				serialize_struct.skip_field("priceCurrency")?;
			}
			#[cfg(any(
				any(
					feature = "price-type-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#price_type) {
				serialize_struct.serialize_field("priceType", {
					struct SerializeWith<'a>(&'a Vec<PriceTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#price_type)
				})?;
			} else {
				serialize_struct.skip_field("priceType")?;
			}
			#[cfg(any(
				any(
					feature = "reference-quantity-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#reference_quantity) {
				serialize_struct.serialize_field("referenceQuantity", {
					struct SerializeWith<'a>(&'a Vec<ReferenceQuantityProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#reference_quantity)
				})?;
			} else {
				serialize_struct.skip_field("referenceQuantity")?;
			}
			#[cfg(any(
				any(
					feature = "same-as-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#same_as) {
				serialize_struct.serialize_field("sameAs", {
					struct SerializeWith<'a>(&'a Vec<SameAsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#same_as)
				})?;
			} else {
				serialize_struct.skip_field("sameAs")?;
			}
			#[cfg(any(
				any(
					feature = "subject-of-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#subject_of) {
				serialize_struct.serialize_field("subjectOf", {
					struct SerializeWith<'a>(&'a Vec<SubjectOfProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#subject_of)
				})?;
			} else {
				serialize_struct.skip_field("subjectOf")?;
			}
			#[cfg(any(
				any(
					feature = "unit-code-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#unit_code) {
				serialize_struct.serialize_field("unitCode", {
					struct SerializeWith<'a>(&'a Vec<UnitCodeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#unit_code)
				})?;
			} else {
				serialize_struct.skip_field("unitCode")?;
			}
			#[cfg(any(
				any(
					feature = "unit-text-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#unit_text) {
				serialize_struct.serialize_field("unitText", {
					struct SerializeWith<'a>(&'a Vec<UnitTextProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#unit_text)
				})?;
			} else {
				serialize_struct.skip_field("unitText")?;
			}
			#[cfg(any(
				any(feature = "url-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#url) {
				serialize_struct.serialize_field("url", {
					struct SerializeWith<'a>(&'a Vec<UrlProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#url)
				})?;
			} else {
				serialize_struct.skip_field("url")?;
			}
			#[cfg(any(
				any(
					feature = "valid-from-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#valid_from) {
				serialize_struct.serialize_field("validFrom", {
					struct SerializeWith<'a>(&'a Vec<ValidFromProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#valid_from)
				})?;
			} else {
				serialize_struct.skip_field("validFrom")?;
			}
			#[cfg(any(
				any(
					feature = "valid-through-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#valid_through) {
				serialize_struct.serialize_field("validThrough", {
					struct SerializeWith<'a>(&'a Vec<ValidThroughProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#valid_through)
				})?;
			} else {
				serialize_struct.skip_field("validThrough")?;
			}
			#[cfg(any(
				any(
					feature = "value-added-tax-included-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#value_added_tax_included) {
				serialize_struct.serialize_field("valueAddedTaxIncluded", {
					struct SerializeWith<'a>(&'a Vec<ValueAddedTaxIncludedProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#value_added_tax_included)
				})?;
			} else {
				serialize_struct.skip_field("valueAddedTaxIncluded")?;
			}
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for UnitPriceSpecification {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				#[cfg(any(
					any(
						feature = "additional-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AdditionalType,
				#[cfg(any(
					any(
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AlternateName,
				#[cfg(any(
					any(
						feature = "billing-duration-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				BillingDuration,
				#[cfg(any(
					any(
						feature = "billing-increment-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				BillingIncrement,
				#[cfg(any(
					any(
						feature = "billing-start-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				BillingStart,
				#[cfg(any(
					any(
						feature = "description-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Description,
				#[cfg(any(
					any(
						feature = "disambiguating-description-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DisambiguatingDescription,
				#[cfg(any(
					any(
						feature = "eligible-quantity-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EligibleQuantity,
				#[cfg(any(
					any(
						feature = "eligible-transaction-volume-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EligibleTransactionVolume,
				#[cfg(any(
					any(
						feature = "identifier-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Identifier,
				#[cfg(any(
					any(feature = "image-property-schema", feature = "general-schema-section"),
					doc
				))]
				Image,
				#[cfg(any(
					any(
						feature = "main-entity-of-page-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MainEntityOfPage,
				#[cfg(any(
					any(
						feature = "max-price-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MaxPrice,
				#[cfg(any(
					any(
						feature = "min-price-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MinPrice,
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				Name,
				#[cfg(any(
					any(
						feature = "potential-action-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PotentialAction,
				#[cfg(any(
					any(feature = "price-property-schema", feature = "general-schema-section"),
					doc
				))]
				Price,
				#[cfg(any(
					any(
						feature = "price-component-type-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				PriceComponentType,
				#[cfg(any(
					any(
						feature = "price-currency-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PriceCurrency,
				#[cfg(any(
					any(
						feature = "price-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PriceType,
				#[cfg(any(
					any(
						feature = "reference-quantity-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ReferenceQuantity,
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SameAs,
				#[cfg(any(
					any(
						feature = "subject-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SubjectOf,
				#[cfg(any(
					any(
						feature = "unit-code-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				UnitCode,
				#[cfg(any(
					any(
						feature = "unit-text-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				UnitText,
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				Url,
				#[cfg(any(
					any(
						feature = "valid-from-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ValidFrom,
				#[cfg(any(
					any(
						feature = "valid-through-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ValidThrough,
				#[cfg(any(
					any(
						feature = "value-added-tax-included-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ValueAddedTaxIncluded,
				Ignore,
			}
			struct FieldVisitor;
			impl<'de> Visitor<'de> for FieldVisitor {
				type Value = Field;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("field identifier")
				}
				fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						#[cfg(any(
							any(
								feature = "additional-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"additionalType" => Ok(Field::AdditionalType),
						#[cfg(any(
							any(
								feature = "alternate-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"alternateName" => Ok(Field::AlternateName),
						#[cfg(any(
							any(
								feature = "billing-duration-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"billingDuration" => Ok(Field::BillingDuration),
						#[cfg(any(
							any(
								feature = "billing-increment-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"billingIncrement" => Ok(Field::BillingIncrement),
						#[cfg(any(
							any(
								feature = "billing-start-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"billingStart" => Ok(Field::BillingStart),
						#[cfg(any(
							any(
								feature = "description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"description" => Ok(Field::Description),
						#[cfg(any(
							any(
								feature = "disambiguating-description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						#[cfg(any(
							any(
								feature = "eligible-quantity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"eligibleQuantity" => Ok(Field::EligibleQuantity),
						#[cfg(any(
							any(
								feature = "eligible-transaction-volume-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"eligibleTransactionVolume" => Ok(Field::EligibleTransactionVolume),
						#[cfg(any(
							any(
								feature = "identifier-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"identifier" => Ok(Field::Identifier),
						#[cfg(any(
							any(
								feature = "image-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"image" => Ok(Field::Image),
						#[cfg(any(
							any(
								feature = "main-entity-of-page-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						#[cfg(any(
							any(
								feature = "max-price-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"maxPrice" => Ok(Field::MaxPrice),
						#[cfg(any(
							any(
								feature = "min-price-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"minPrice" => Ok(Field::MinPrice),
						#[cfg(any(
							any(
								feature = "name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"name" => Ok(Field::Name),
						#[cfg(any(
							any(
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"potentialAction" => Ok(Field::PotentialAction),
						#[cfg(any(
							any(
								feature = "price-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"price" => Ok(Field::Price),
						#[cfg(any(
							any(
								feature = "price-component-type-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"priceComponentType" => Ok(Field::PriceComponentType),
						#[cfg(any(
							any(
								feature = "price-currency-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"priceCurrency" => Ok(Field::PriceCurrency),
						#[cfg(any(
							any(
								feature = "price-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"priceType" => Ok(Field::PriceType),
						#[cfg(any(
							any(
								feature = "reference-quantity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"referenceQuantity" => Ok(Field::ReferenceQuantity),
						#[cfg(any(
							any(
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"sameAs" => Ok(Field::SameAs),
						#[cfg(any(
							any(
								feature = "subject-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"subjectOf" => Ok(Field::SubjectOf),
						#[cfg(any(
							any(
								feature = "unit-code-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"unitCode" => Ok(Field::UnitCode),
						#[cfg(any(
							any(
								feature = "unit-text-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"unitText" => Ok(Field::UnitText),
						#[cfg(any(
							any(
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"url" => Ok(Field::Url),
						#[cfg(any(
							any(
								feature = "valid-from-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"validFrom" => Ok(Field::ValidFrom),
						#[cfg(any(
							any(
								feature = "valid-through-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"validThrough" => Ok(Field::ValidThrough),
						#[cfg(any(
							any(
								feature = "value-added-tax-included-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"valueAddedTaxIncluded" => Ok(Field::ValueAddedTaxIncluded),
						_ => Ok(Field::Ignore),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						#[cfg(any(
							any(
								feature = "additional-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"additionalType" => Ok(Field::AdditionalType),
						#[cfg(any(
							any(
								feature = "alternate-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"alternateName" => Ok(Field::AlternateName),
						#[cfg(any(
							any(
								feature = "billing-duration-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"billingDuration" => Ok(Field::BillingDuration),
						#[cfg(any(
							any(
								feature = "billing-increment-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"billingIncrement" => Ok(Field::BillingIncrement),
						#[cfg(any(
							any(
								feature = "billing-start-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"billingStart" => Ok(Field::BillingStart),
						#[cfg(any(
							any(
								feature = "description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"description" => Ok(Field::Description),
						#[cfg(any(
							any(
								feature = "disambiguating-description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						#[cfg(any(
							any(
								feature = "eligible-quantity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"eligibleQuantity" => Ok(Field::EligibleQuantity),
						#[cfg(any(
							any(
								feature = "eligible-transaction-volume-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"eligibleTransactionVolume" => Ok(Field::EligibleTransactionVolume),
						#[cfg(any(
							any(
								feature = "identifier-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"identifier" => Ok(Field::Identifier),
						#[cfg(any(
							any(
								feature = "image-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"image" => Ok(Field::Image),
						#[cfg(any(
							any(
								feature = "main-entity-of-page-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						#[cfg(any(
							any(
								feature = "max-price-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"maxPrice" => Ok(Field::MaxPrice),
						#[cfg(any(
							any(
								feature = "min-price-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"minPrice" => Ok(Field::MinPrice),
						#[cfg(any(
							any(
								feature = "name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"name" => Ok(Field::Name),
						#[cfg(any(
							any(
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"potentialAction" => Ok(Field::PotentialAction),
						#[cfg(any(
							any(
								feature = "price-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"price" => Ok(Field::Price),
						#[cfg(any(
							any(
								feature = "price-component-type-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"priceComponentType" => Ok(Field::PriceComponentType),
						#[cfg(any(
							any(
								feature = "price-currency-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"priceCurrency" => Ok(Field::PriceCurrency),
						#[cfg(any(
							any(
								feature = "price-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"priceType" => Ok(Field::PriceType),
						#[cfg(any(
							any(
								feature = "reference-quantity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"referenceQuantity" => Ok(Field::ReferenceQuantity),
						#[cfg(any(
							any(
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"sameAs" => Ok(Field::SameAs),
						#[cfg(any(
							any(
								feature = "subject-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"subjectOf" => Ok(Field::SubjectOf),
						#[cfg(any(
							any(
								feature = "unit-code-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"unitCode" => Ok(Field::UnitCode),
						#[cfg(any(
							any(
								feature = "unit-text-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"unitText" => Ok(Field::UnitText),
						#[cfg(any(
							any(
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"url" => Ok(Field::Url),
						#[cfg(any(
							any(
								feature = "valid-from-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"validFrom" => Ok(Field::ValidFrom),
						#[cfg(any(
							any(
								feature = "valid-through-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"validThrough" => Ok(Field::ValidThrough),
						#[cfg(any(
							any(
								feature = "value-added-tax-included-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"valueAddedTaxIncluded" => Ok(Field::ValueAddedTaxIncluded),
						_ => Ok(Field::Ignore),
					}
				}
			}
			impl<'de> Deserialize<'de> for Field {
				fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
				where
					D: Deserializer<'de>,
				{
					deserializer.deserialize_identifier(FieldVisitor)
				}
			}
			struct ClassVisitor;
			impl<'de> Visitor<'de> for ClassVisitor {
				type Value = UnitPriceSpecification;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema UnitPriceSpecification")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					#[cfg(any(
						any(
							feature = "additional-type-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#additional_type_property = None;
					#[cfg(any(
						any(
							feature = "alternate-name-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#alternate_name_property = None;
					#[cfg(any(
						any(
							feature = "billing-duration-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#billing_duration_property = None;
					#[cfg(any(
						any(
							feature = "billing-increment-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#billing_increment_property = None;
					#[cfg(any(
						any(
							feature = "billing-start-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#billing_start_property = None;
					#[cfg(any(
						any(
							feature = "description-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#description_property = None;
					#[cfg(any(
						any(
							feature = "disambiguating-description-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#disambiguating_description_property = None;
					#[cfg(any(
						any(
							feature = "eligible-quantity-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#eligible_quantity_property = None;
					#[cfg(any(
						any(
							feature = "eligible-transaction-volume-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#eligible_transaction_volume_property = None;
					#[cfg(any(
						any(
							feature = "identifier-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#identifier_property = None;
					#[cfg(any(
						any(feature = "image-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#image_property = None;
					#[cfg(any(
						any(
							feature = "main-entity-of-page-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#main_entity_of_page_property = None;
					#[cfg(any(
						any(
							feature = "max-price-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#max_price_property = None;
					#[cfg(any(
						any(
							feature = "min-price-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#min_price_property = None;
					#[cfg(any(
						any(feature = "name-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#name_property = None;
					#[cfg(any(
						any(
							feature = "potential-action-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#potential_action_property = None;
					#[cfg(any(
						any(feature = "price-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#price_property = None;
					#[cfg(any(
						any(
							feature = "price-component-type-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#price_component_type_property = None;
					#[cfg(any(
						any(
							feature = "price-currency-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#price_currency_property = None;
					#[cfg(any(
						any(
							feature = "price-type-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#price_type_property = None;
					#[cfg(any(
						any(
							feature = "reference-quantity-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#reference_quantity_property = None;
					#[cfg(any(
						any(
							feature = "same-as-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#same_as_property = None;
					#[cfg(any(
						any(
							feature = "subject-of-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#subject_of_property = None;
					#[cfg(any(
						any(
							feature = "unit-code-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#unit_code_property = None;
					#[cfg(any(
						any(
							feature = "unit-text-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#unit_text_property = None;
					#[cfg(any(
						any(feature = "url-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#url_property = None;
					#[cfg(any(
						any(
							feature = "valid-from-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#valid_from_property = None;
					#[cfg(any(
						any(
							feature = "valid-through-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#valid_through_property = None;
					#[cfg(any(
						any(
							feature = "value-added-tax-included-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#value_added_tax_included_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
							#[cfg(any(
								any(
									feature = "additional-type-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AdditionalType => {
								if r#additional_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"additionalType",
									));
								}
								r#additional_type_property = Some({
									struct DeserializeWith(Vec<AdditionalTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "alternate-name-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AlternateName => {
								if r#alternate_name_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"alternateName",
									));
								}
								r#alternate_name_property = Some({
									struct DeserializeWith(Vec<AlternateNameProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "billing-duration-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::BillingDuration => {
								if r#billing_duration_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"billingDuration",
									));
								}
								r#billing_duration_property = Some({
									struct DeserializeWith(Vec<BillingDurationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "billing-increment-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::BillingIncrement => {
								if r#billing_increment_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"billingIncrement",
									));
								}
								r#billing_increment_property = Some({
									struct DeserializeWith(Vec<BillingIncrementProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "billing-start-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::BillingStart => {
								if r#billing_start_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"billingStart",
									));
								}
								r#billing_start_property = Some({
									struct DeserializeWith(Vec<BillingStartProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "description-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Description => {
								if r#description_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"description",
									));
								}
								r#description_property = Some({
									struct DeserializeWith(Vec<DescriptionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "disambiguating-description-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::DisambiguatingDescription => {
								if r#disambiguating_description_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"disambiguatingDescription",
									));
								}
								r#disambiguating_description_property = Some({
									struct DeserializeWith(Vec<DisambiguatingDescriptionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "eligible-quantity-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::EligibleQuantity => {
								if r#eligible_quantity_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"eligibleQuantity",
									));
								}
								r#eligible_quantity_property = Some({
									struct DeserializeWith(Vec<EligibleQuantityProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "eligible-transaction-volume-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::EligibleTransactionVolume => {
								if r#eligible_transaction_volume_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"eligibleTransactionVolume",
									));
								}
								r#eligible_transaction_volume_property = Some({
									struct DeserializeWith(Vec<EligibleTransactionVolumeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "identifier-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Identifier => {
								if r#identifier_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"identifier",
									));
								}
								r#identifier_property = Some({
									struct DeserializeWith(Vec<IdentifierProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "image-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Image => {
								if r#image_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("image"));
								}
								r#image_property = Some({
									struct DeserializeWith(Vec<ImageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "main-entity-of-page-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::MainEntityOfPage => {
								if r#main_entity_of_page_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"mainEntityOfPage",
									));
								}
								r#main_entity_of_page_property = Some({
									struct DeserializeWith(Vec<MainEntityOfPageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "max-price-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::MaxPrice => {
								if r#max_price_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"maxPrice",
									));
								}
								r#max_price_property = Some({
									struct DeserializeWith(Vec<MaxPriceProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "min-price-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::MinPrice => {
								if r#min_price_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"minPrice",
									));
								}
								r#min_price_property = Some({
									struct DeserializeWith(Vec<MinPriceProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "name-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Name => {
								if r#name_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("name"));
								}
								r#name_property = Some({
									struct DeserializeWith(Vec<NameProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "potential-action-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::PotentialAction => {
								if r#potential_action_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"potentialAction",
									));
								}
								r#potential_action_property = Some({
									struct DeserializeWith(Vec<PotentialActionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "price-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Price => {
								if r#price_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("price"));
								}
								r#price_property = Some({
									struct DeserializeWith(Vec<PriceProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "price-component-type-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::PriceComponentType => {
								if r#price_component_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"priceComponentType",
									));
								}
								r#price_component_type_property = Some({
									struct DeserializeWith(Vec<PriceComponentTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "price-currency-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::PriceCurrency => {
								if r#price_currency_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"priceCurrency",
									));
								}
								r#price_currency_property = Some({
									struct DeserializeWith(Vec<PriceCurrencyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "price-type-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::PriceType => {
								if r#price_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"priceType",
									));
								}
								r#price_type_property = Some({
									struct DeserializeWith(Vec<PriceTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "reference-quantity-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ReferenceQuantity => {
								if r#reference_quantity_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"referenceQuantity",
									));
								}
								r#reference_quantity_property = Some({
									struct DeserializeWith(Vec<ReferenceQuantityProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "same-as-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::SameAs => {
								if r#same_as_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("sameAs"));
								}
								r#same_as_property = Some({
									struct DeserializeWith(Vec<SameAsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "subject-of-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::SubjectOf => {
								if r#subject_of_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"subjectOf",
									));
								}
								r#subject_of_property = Some({
									struct DeserializeWith(Vec<SubjectOfProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "unit-code-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::UnitCode => {
								if r#unit_code_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"unitCode",
									));
								}
								r#unit_code_property = Some({
									struct DeserializeWith(Vec<UnitCodeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "unit-text-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::UnitText => {
								if r#unit_text_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"unitText",
									));
								}
								r#unit_text_property = Some({
									struct DeserializeWith(Vec<UnitTextProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "url-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Url => {
								if r#url_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("url"));
								}
								r#url_property = Some({
									struct DeserializeWith(Vec<UrlProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "valid-from-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ValidFrom => {
								if r#valid_from_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"validFrom",
									));
								}
								r#valid_from_property = Some({
									struct DeserializeWith(Vec<ValidFromProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "valid-through-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ValidThrough => {
								if r#valid_through_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"validThrough",
									));
								}
								r#valid_through_property = Some({
									struct DeserializeWith(Vec<ValidThroughProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "value-added-tax-included-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ValueAddedTaxIncluded => {
								if r#value_added_tax_included_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"valueAddedTaxIncluded",
									));
								}
								r#value_added_tax_included_property = Some({
									struct DeserializeWith(Vec<ValueAddedTaxIncludedProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							_ => {
								let _ = map.next_value::<de::IgnoredAny>()?;
							}
						}
					}
					Ok(UnitPriceSpecification {
						#[cfg(any(
							any(
								feature = "additional-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#additional_type: r#additional_type_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "alternate-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "billing-duration-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#billing_duration: r#billing_duration_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "billing-increment-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#billing_increment: r#billing_increment_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "billing-start-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#billing_start: r#billing_start_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#description: r#description_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "disambiguating-description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#disambiguating_description: r#disambiguating_description_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "eligible-quantity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#eligible_quantity: r#eligible_quantity_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "eligible-transaction-volume-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#eligible_transaction_volume: r#eligible_transaction_volume_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "identifier-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#identifier: r#identifier_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "image-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#image: r#image_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "main-entity-of-page-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#main_entity_of_page: r#main_entity_of_page_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "max-price-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#max_price: r#max_price_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "min-price-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#min_price: r#min_price_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#name: r#name_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#potential_action: r#potential_action_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "price-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#price: r#price_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "price-component-type-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#price_component_type: r#price_component_type_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "price-currency-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#price_currency: r#price_currency_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "price-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#price_type: r#price_type_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "reference-quantity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#reference_quantity: r#reference_quantity_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#same_as: r#same_as_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "subject-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#subject_of: r#subject_of_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "unit-code-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#unit_code: r#unit_code_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "unit-text-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#unit_text: r#unit_text_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#url: r#url_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "valid-from-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#valid_from: r#valid_from_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "valid-through-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#valid_through: r#valid_through_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "value-added-tax-included-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#value_added_tax_included: r#value_added_tax_included_property
							.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				#[cfg(any(
					any(
						feature = "additional-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"additionalType",
				#[cfg(any(
					any(
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"alternateName",
				#[cfg(any(
					any(
						feature = "billing-duration-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"billingDuration",
				#[cfg(any(
					any(
						feature = "billing-increment-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"billingIncrement",
				#[cfg(any(
					any(
						feature = "billing-start-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"billingStart",
				#[cfg(any(
					any(
						feature = "description-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"description",
				#[cfg(any(
					any(
						feature = "disambiguating-description-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"disambiguatingDescription",
				#[cfg(any(
					any(
						feature = "eligible-quantity-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"eligibleQuantity",
				#[cfg(any(
					any(
						feature = "eligible-transaction-volume-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"eligibleTransactionVolume",
				#[cfg(any(
					any(
						feature = "identifier-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"identifier",
				#[cfg(any(
					any(feature = "image-property-schema", feature = "general-schema-section"),
					doc
				))]
				"image",
				#[cfg(any(
					any(
						feature = "main-entity-of-page-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"mainEntityOfPage",
				#[cfg(any(
					any(
						feature = "max-price-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"maxPrice",
				#[cfg(any(
					any(
						feature = "min-price-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"minPrice",
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				"name",
				#[cfg(any(
					any(
						feature = "potential-action-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"potentialAction",
				#[cfg(any(
					any(feature = "price-property-schema", feature = "general-schema-section"),
					doc
				))]
				"price",
				#[cfg(any(
					any(
						feature = "price-component-type-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"priceComponentType",
				#[cfg(any(
					any(
						feature = "price-currency-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"priceCurrency",
				#[cfg(any(
					any(
						feature = "price-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"priceType",
				#[cfg(any(
					any(
						feature = "reference-quantity-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"referenceQuantity",
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"sameAs",
				#[cfg(any(
					any(
						feature = "subject-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"subjectOf",
				#[cfg(any(
					any(
						feature = "unit-code-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"unitCode",
				#[cfg(any(
					any(
						feature = "unit-text-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"unitText",
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				"url",
				#[cfg(any(
					any(
						feature = "valid-from-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"validFrom",
				#[cfg(any(
					any(
						feature = "valid-through-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"validThrough",
				#[cfg(any(
					any(
						feature = "value-added-tax-included-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"valueAddedTaxIncluded",
			];
			deserializer.deserialize_struct("UnitPriceSpecification", FIELDS, ClassVisitor)
		}
	}
}
