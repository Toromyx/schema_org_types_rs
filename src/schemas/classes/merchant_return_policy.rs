use super::*;
/// <https://schema.org/MerchantReturnPolicy>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct MerchantReturnPolicy {
	#[cfg(any(
		any(
			feature = "additional-property-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#additional_property: Vec<AdditionalPropertyProperty>,
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
			feature = "applicable-country-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#applicable_country: Vec<ApplicableCountryProperty>,
	#[cfg(any(
		any(
			feature = "customer-remorse-return-fees-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#customer_remorse_return_fees: Vec<CustomerRemorseReturnFeesProperty>,
	#[cfg(any(
		any(
			feature = "customer-remorse-return-label-source-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#customer_remorse_return_label_source: Vec<CustomerRemorseReturnLabelSourceProperty>,
	#[cfg(any(
		any(
			feature = "customer-remorse-return-shipping-fees-amount-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#customer_remorse_return_shipping_fees_amount:
		Vec<CustomerRemorseReturnShippingFeesAmountProperty>,
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
			feature = "in-store-returns-offered-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#in_store_returns_offered: Vec<InStoreReturnsOfferedProperty>,
	#[cfg(any(
		any(
			feature = "item-condition-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#item_condition: Vec<ItemConditionProperty>,
	#[cfg(any(
		any(
			feature = "item-defect-return-fees-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#item_defect_return_fees: Vec<ItemDefectReturnFeesProperty>,
	#[cfg(any(
		any(
			feature = "item-defect-return-label-source-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#item_defect_return_label_source: Vec<ItemDefectReturnLabelSourceProperty>,
	#[cfg(any(
		any(
			feature = "item-defect-return-shipping-fees-amount-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#item_defect_return_shipping_fees_amount: Vec<ItemDefectReturnShippingFeesAmountProperty>,
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
			feature = "merchant-return-days-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#merchant_return_days: Vec<MerchantReturnDaysProperty>,
	#[cfg(any(
		any(
			feature = "merchant-return-link-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#merchant_return_link: Vec<MerchantReturnLinkProperty>,
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
		any(
			feature = "refund-type-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#refund_type: Vec<RefundTypeProperty>,
	#[cfg(any(
		any(
			feature = "restocking-fee-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#restocking_fee: Vec<RestockingFeeProperty>,
	#[cfg(any(
		any(
			feature = "return-fees-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#return_fees: Vec<ReturnFeesProperty>,
	#[cfg(any(
		any(
			feature = "return-label-source-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#return_label_source: Vec<ReturnLabelSourceProperty>,
	#[cfg(any(
		any(
			feature = "return-method-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#return_method: Vec<ReturnMethodProperty>,
	#[cfg(any(
		any(
			feature = "return-policy-category-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#return_policy_category: Vec<ReturnPolicyCategoryProperty>,
	#[cfg(any(
		any(
			feature = "return-policy-country-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#return_policy_country: Vec<ReturnPolicyCountryProperty>,
	#[cfg(any(
		any(
			feature = "return-policy-seasonal-override-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#return_policy_seasonal_override: Vec<ReturnPolicySeasonalOverrideProperty>,
	#[cfg(any(
		any(
			feature = "return-shipping-fees-amount-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#return_shipping_fees_amount: Vec<ReturnShippingFeesAmountProperty>,
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
		any(feature = "url-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#url: Vec<UrlProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MerchantReturnPolicy {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				if cfg!(any(
					any(
						feature = "additional-property-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#additional_property) as usize
				} else {
					0
				},
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
						feature = "applicable-country-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#applicable_country) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "customer-remorse-return-fees-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#customer_remorse_return_fees) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "customer-remorse-return-label-source-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#customer_remorse_return_label_source) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "customer-remorse-return-shipping-fees-amount-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#customer_remorse_return_shipping_fees_amount) as usize
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
						feature = "in-store-returns-offered-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#in_store_returns_offered) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "item-condition-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#item_condition) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "item-defect-return-fees-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#item_defect_return_fees) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "item-defect-return-label-source-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#item_defect_return_label_source) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "item-defect-return-shipping-fees-amount-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#item_defect_return_shipping_fees_amount) as usize
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
						feature = "merchant-return-days-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#merchant_return_days) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "merchant-return-link-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#merchant_return_link) as usize
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
						feature = "refund-type-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#refund_type) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "restocking-fee-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#restocking_fee) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "return-fees-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#return_fees) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "return-label-source-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#return_label_source) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "return-method-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#return_method) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "return-policy-category-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#return_policy_category) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "return-policy-country-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#return_policy_country) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "return-policy-seasonal-override-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#return_policy_seasonal_override) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "return-shipping-fees-amount-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#return_shipping_fees_amount) as usize
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
						feature = "url-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#url) as usize
				} else {
					0
				},
			]
			.iter()
			.sum();
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "MerchantReturnPolicy", len)?;
			#[cfg(any(
				any(
					feature = "additional-property-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#additional_property) {
				serialize_struct.serialize_field("additionalProperty", {
					struct SerializeWith<'a>(&'a Vec<AdditionalPropertyProperty>);
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
					&SerializeWith(&self.r#additional_property)
				})?;
			} else {
				serialize_struct.skip_field("additionalProperty")?;
			}
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
					feature = "applicable-country-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#applicable_country) {
				serialize_struct.serialize_field("applicableCountry", {
					struct SerializeWith<'a>(&'a Vec<ApplicableCountryProperty>);
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
					&SerializeWith(&self.r#applicable_country)
				})?;
			} else {
				serialize_struct.skip_field("applicableCountry")?;
			}
			#[cfg(any(
				any(
					feature = "customer-remorse-return-fees-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#customer_remorse_return_fees) {
				serialize_struct.serialize_field("customerRemorseReturnFees", {
					struct SerializeWith<'a>(&'a Vec<CustomerRemorseReturnFeesProperty>);
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
					&SerializeWith(&self.r#customer_remorse_return_fees)
				})?;
			} else {
				serialize_struct.skip_field("customerRemorseReturnFees")?;
			}
			#[cfg(any(
				any(
					feature = "customer-remorse-return-label-source-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#customer_remorse_return_label_source) {
				serialize_struct.serialize_field("customerRemorseReturnLabelSource", {
					struct SerializeWith<'a>(&'a Vec<CustomerRemorseReturnLabelSourceProperty>);
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
					&SerializeWith(&self.r#customer_remorse_return_label_source)
				})?;
			} else {
				serialize_struct.skip_field("customerRemorseReturnLabelSource")?;
			}
			#[cfg(any(
				any(
					feature = "customer-remorse-return-shipping-fees-amount-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#customer_remorse_return_shipping_fees_amount) {
				serialize_struct.serialize_field("customerRemorseReturnShippingFeesAmount", {
					struct SerializeWith<'a>(
						&'a Vec<CustomerRemorseReturnShippingFeesAmountProperty>,
					);
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
					&SerializeWith(&self.r#customer_remorse_return_shipping_fees_amount)
				})?;
			} else {
				serialize_struct.skip_field("customerRemorseReturnShippingFeesAmount")?;
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
					feature = "in-store-returns-offered-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#in_store_returns_offered) {
				serialize_struct.serialize_field("inStoreReturnsOffered", {
					struct SerializeWith<'a>(&'a Vec<InStoreReturnsOfferedProperty>);
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
					&SerializeWith(&self.r#in_store_returns_offered)
				})?;
			} else {
				serialize_struct.skip_field("inStoreReturnsOffered")?;
			}
			#[cfg(any(
				any(
					feature = "item-condition-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#item_condition) {
				serialize_struct.serialize_field("itemCondition", {
					struct SerializeWith<'a>(&'a Vec<ItemConditionProperty>);
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
					&SerializeWith(&self.r#item_condition)
				})?;
			} else {
				serialize_struct.skip_field("itemCondition")?;
			}
			#[cfg(any(
				any(
					feature = "item-defect-return-fees-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#item_defect_return_fees) {
				serialize_struct.serialize_field("itemDefectReturnFees", {
					struct SerializeWith<'a>(&'a Vec<ItemDefectReturnFeesProperty>);
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
					&SerializeWith(&self.r#item_defect_return_fees)
				})?;
			} else {
				serialize_struct.skip_field("itemDefectReturnFees")?;
			}
			#[cfg(any(
				any(
					feature = "item-defect-return-label-source-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#item_defect_return_label_source) {
				serialize_struct.serialize_field("itemDefectReturnLabelSource", {
					struct SerializeWith<'a>(&'a Vec<ItemDefectReturnLabelSourceProperty>);
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
					&SerializeWith(&self.r#item_defect_return_label_source)
				})?;
			} else {
				serialize_struct.skip_field("itemDefectReturnLabelSource")?;
			}
			#[cfg(any(
				any(
					feature = "item-defect-return-shipping-fees-amount-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#item_defect_return_shipping_fees_amount) {
				serialize_struct.serialize_field("itemDefectReturnShippingFeesAmount", {
					struct SerializeWith<'a>(&'a Vec<ItemDefectReturnShippingFeesAmountProperty>);
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
					&SerializeWith(&self.r#item_defect_return_shipping_fees_amount)
				})?;
			} else {
				serialize_struct.skip_field("itemDefectReturnShippingFeesAmount")?;
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
					feature = "merchant-return-days-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#merchant_return_days) {
				serialize_struct.serialize_field("merchantReturnDays", {
					struct SerializeWith<'a>(&'a Vec<MerchantReturnDaysProperty>);
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
					&SerializeWith(&self.r#merchant_return_days)
				})?;
			} else {
				serialize_struct.skip_field("merchantReturnDays")?;
			}
			#[cfg(any(
				any(
					feature = "merchant-return-link-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#merchant_return_link) {
				serialize_struct.serialize_field("merchantReturnLink", {
					struct SerializeWith<'a>(&'a Vec<MerchantReturnLinkProperty>);
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
					&SerializeWith(&self.r#merchant_return_link)
				})?;
			} else {
				serialize_struct.skip_field("merchantReturnLink")?;
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
				any(
					feature = "refund-type-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#refund_type) {
				serialize_struct.serialize_field("refundType", {
					struct SerializeWith<'a>(&'a Vec<RefundTypeProperty>);
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
					&SerializeWith(&self.r#refund_type)
				})?;
			} else {
				serialize_struct.skip_field("refundType")?;
			}
			#[cfg(any(
				any(
					feature = "restocking-fee-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#restocking_fee) {
				serialize_struct.serialize_field("restockingFee", {
					struct SerializeWith<'a>(&'a Vec<RestockingFeeProperty>);
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
					&SerializeWith(&self.r#restocking_fee)
				})?;
			} else {
				serialize_struct.skip_field("restockingFee")?;
			}
			#[cfg(any(
				any(
					feature = "return-fees-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#return_fees) {
				serialize_struct.serialize_field("returnFees", {
					struct SerializeWith<'a>(&'a Vec<ReturnFeesProperty>);
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
					&SerializeWith(&self.r#return_fees)
				})?;
			} else {
				serialize_struct.skip_field("returnFees")?;
			}
			#[cfg(any(
				any(
					feature = "return-label-source-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#return_label_source) {
				serialize_struct.serialize_field("returnLabelSource", {
					struct SerializeWith<'a>(&'a Vec<ReturnLabelSourceProperty>);
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
					&SerializeWith(&self.r#return_label_source)
				})?;
			} else {
				serialize_struct.skip_field("returnLabelSource")?;
			}
			#[cfg(any(
				any(
					feature = "return-method-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#return_method) {
				serialize_struct.serialize_field("returnMethod", {
					struct SerializeWith<'a>(&'a Vec<ReturnMethodProperty>);
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
					&SerializeWith(&self.r#return_method)
				})?;
			} else {
				serialize_struct.skip_field("returnMethod")?;
			}
			#[cfg(any(
				any(
					feature = "return-policy-category-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#return_policy_category) {
				serialize_struct.serialize_field("returnPolicyCategory", {
					struct SerializeWith<'a>(&'a Vec<ReturnPolicyCategoryProperty>);
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
					&SerializeWith(&self.r#return_policy_category)
				})?;
			} else {
				serialize_struct.skip_field("returnPolicyCategory")?;
			}
			#[cfg(any(
				any(
					feature = "return-policy-country-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#return_policy_country) {
				serialize_struct.serialize_field("returnPolicyCountry", {
					struct SerializeWith<'a>(&'a Vec<ReturnPolicyCountryProperty>);
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
					&SerializeWith(&self.r#return_policy_country)
				})?;
			} else {
				serialize_struct.skip_field("returnPolicyCountry")?;
			}
			#[cfg(any(
				any(
					feature = "return-policy-seasonal-override-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#return_policy_seasonal_override) {
				serialize_struct.serialize_field("returnPolicySeasonalOverride", {
					struct SerializeWith<'a>(&'a Vec<ReturnPolicySeasonalOverrideProperty>);
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
					&SerializeWith(&self.r#return_policy_seasonal_override)
				})?;
			} else {
				serialize_struct.skip_field("returnPolicySeasonalOverride")?;
			}
			#[cfg(any(
				any(
					feature = "return-shipping-fees-amount-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#return_shipping_fees_amount) {
				serialize_struct.serialize_field("returnShippingFeesAmount", {
					struct SerializeWith<'a>(&'a Vec<ReturnShippingFeesAmountProperty>);
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
					&SerializeWith(&self.r#return_shipping_fees_amount)
				})?;
			} else {
				serialize_struct.skip_field("returnShippingFeesAmount")?;
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
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for MerchantReturnPolicy {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				#[cfg(any(
					any(
						feature = "additional-property-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AdditionalProperty,
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
						feature = "applicable-country-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ApplicableCountry,
				#[cfg(any(
					any(
						feature = "customer-remorse-return-fees-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CustomerRemorseReturnFees,
				#[cfg(any(
					any(
						feature = "customer-remorse-return-label-source-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CustomerRemorseReturnLabelSource,
				#[cfg(any(
					any(
						feature = "customer-remorse-return-shipping-fees-amount-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CustomerRemorseReturnShippingFeesAmount,
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
						feature = "in-store-returns-offered-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				InStoreReturnsOffered,
				#[cfg(any(
					any(
						feature = "item-condition-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ItemCondition,
				#[cfg(any(
					any(
						feature = "item-defect-return-fees-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ItemDefectReturnFees,
				#[cfg(any(
					any(
						feature = "item-defect-return-label-source-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ItemDefectReturnLabelSource,
				#[cfg(any(
					any(
						feature = "item-defect-return-shipping-fees-amount-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ItemDefectReturnShippingFeesAmount,
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
						feature = "merchant-return-days-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				MerchantReturnDays,
				#[cfg(any(
					any(
						feature = "merchant-return-link-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				MerchantReturnLink,
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
					any(
						feature = "refund-type-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				RefundType,
				#[cfg(any(
					any(
						feature = "restocking-fee-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				RestockingFee,
				#[cfg(any(
					any(
						feature = "return-fees-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ReturnFees,
				#[cfg(any(
					any(
						feature = "return-label-source-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ReturnLabelSource,
				#[cfg(any(
					any(
						feature = "return-method-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ReturnMethod,
				#[cfg(any(
					any(
						feature = "return-policy-category-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ReturnPolicyCategory,
				#[cfg(any(
					any(
						feature = "return-policy-country-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ReturnPolicyCountry,
				#[cfg(any(
					any(
						feature = "return-policy-seasonal-override-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ReturnPolicySeasonalOverride,
				#[cfg(any(
					any(
						feature = "return-shipping-fees-amount-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ReturnShippingFeesAmount,
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
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				Url,
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
								feature = "additional-property-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"additionalProperty" => Ok(Field::AdditionalProperty),
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
								feature = "applicable-country-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"applicableCountry" => Ok(Field::ApplicableCountry),
						#[cfg(any(
							any(
								feature = "customer-remorse-return-fees-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"customerRemorseReturnFees" => Ok(Field::CustomerRemorseReturnFees),
						#[cfg(any(
							any(
								feature = "customer-remorse-return-label-source-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"customerRemorseReturnLabelSource" => Ok(Field::CustomerRemorseReturnLabelSource),
						#[cfg(any(
							any(
								feature = "customer-remorse-return-shipping-fees-amount-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"customerRemorseReturnShippingFeesAmount" => {
							Ok(Field::CustomerRemorseReturnShippingFeesAmount)
						}
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
								feature = "in-store-returns-offered-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"inStoreReturnsOffered" => Ok(Field::InStoreReturnsOffered),
						#[cfg(any(
							any(
								feature = "item-condition-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"itemCondition" => Ok(Field::ItemCondition),
						#[cfg(any(
							any(
								feature = "item-defect-return-fees-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"itemDefectReturnFees" => Ok(Field::ItemDefectReturnFees),
						#[cfg(any(
							any(
								feature = "item-defect-return-label-source-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"itemDefectReturnLabelSource" => Ok(Field::ItemDefectReturnLabelSource),
						#[cfg(any(
							any(
								feature = "item-defect-return-shipping-fees-amount-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"itemDefectReturnShippingFeesAmount" => Ok(Field::ItemDefectReturnShippingFeesAmount),
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
								feature = "merchant-return-days-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"merchantReturnDays" => Ok(Field::MerchantReturnDays),
						#[cfg(any(
							any(
								feature = "merchant-return-link-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"merchantReturnLink" => Ok(Field::MerchantReturnLink),
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
								feature = "refund-type-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"refundType" => Ok(Field::RefundType),
						#[cfg(any(
							any(
								feature = "restocking-fee-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"restockingFee" => Ok(Field::RestockingFee),
						#[cfg(any(
							any(
								feature = "return-fees-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"returnFees" => Ok(Field::ReturnFees),
						#[cfg(any(
							any(
								feature = "return-label-source-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"returnLabelSource" => Ok(Field::ReturnLabelSource),
						#[cfg(any(
							any(
								feature = "return-method-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"returnMethod" => Ok(Field::ReturnMethod),
						#[cfg(any(
							any(
								feature = "return-policy-category-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"returnPolicyCategory" => Ok(Field::ReturnPolicyCategory),
						#[cfg(any(
							any(
								feature = "return-policy-country-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"returnPolicyCountry" => Ok(Field::ReturnPolicyCountry),
						#[cfg(any(
							any(
								feature = "return-policy-seasonal-override-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"returnPolicySeasonalOverride" => Ok(Field::ReturnPolicySeasonalOverride),
						#[cfg(any(
							any(
								feature = "return-shipping-fees-amount-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"returnShippingFeesAmount" => Ok(Field::ReturnShippingFeesAmount),
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
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"url" => Ok(Field::Url),
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
								feature = "additional-property-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"additionalProperty" => Ok(Field::AdditionalProperty),
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
								feature = "applicable-country-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"applicableCountry" => Ok(Field::ApplicableCountry),
						#[cfg(any(
							any(
								feature = "customer-remorse-return-fees-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"customerRemorseReturnFees" => Ok(Field::CustomerRemorseReturnFees),
						#[cfg(any(
							any(
								feature = "customer-remorse-return-label-source-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"customerRemorseReturnLabelSource" => Ok(Field::CustomerRemorseReturnLabelSource),
						#[cfg(any(
							any(
								feature = "customer-remorse-return-shipping-fees-amount-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"customerRemorseReturnShippingFeesAmount" => {
							Ok(Field::CustomerRemorseReturnShippingFeesAmount)
						}
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
								feature = "in-store-returns-offered-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"inStoreReturnsOffered" => Ok(Field::InStoreReturnsOffered),
						#[cfg(any(
							any(
								feature = "item-condition-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"itemCondition" => Ok(Field::ItemCondition),
						#[cfg(any(
							any(
								feature = "item-defect-return-fees-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"itemDefectReturnFees" => Ok(Field::ItemDefectReturnFees),
						#[cfg(any(
							any(
								feature = "item-defect-return-label-source-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"itemDefectReturnLabelSource" => Ok(Field::ItemDefectReturnLabelSource),
						#[cfg(any(
							any(
								feature = "item-defect-return-shipping-fees-amount-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"itemDefectReturnShippingFeesAmount" => Ok(Field::ItemDefectReturnShippingFeesAmount),
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
								feature = "merchant-return-days-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"merchantReturnDays" => Ok(Field::MerchantReturnDays),
						#[cfg(any(
							any(
								feature = "merchant-return-link-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"merchantReturnLink" => Ok(Field::MerchantReturnLink),
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
								feature = "refund-type-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"refundType" => Ok(Field::RefundType),
						#[cfg(any(
							any(
								feature = "restocking-fee-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"restockingFee" => Ok(Field::RestockingFee),
						#[cfg(any(
							any(
								feature = "return-fees-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"returnFees" => Ok(Field::ReturnFees),
						#[cfg(any(
							any(
								feature = "return-label-source-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"returnLabelSource" => Ok(Field::ReturnLabelSource),
						#[cfg(any(
							any(
								feature = "return-method-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"returnMethod" => Ok(Field::ReturnMethod),
						#[cfg(any(
							any(
								feature = "return-policy-category-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"returnPolicyCategory" => Ok(Field::ReturnPolicyCategory),
						#[cfg(any(
							any(
								feature = "return-policy-country-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"returnPolicyCountry" => Ok(Field::ReturnPolicyCountry),
						#[cfg(any(
							any(
								feature = "return-policy-seasonal-override-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"returnPolicySeasonalOverride" => Ok(Field::ReturnPolicySeasonalOverride),
						#[cfg(any(
							any(
								feature = "return-shipping-fees-amount-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"returnShippingFeesAmount" => Ok(Field::ReturnShippingFeesAmount),
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
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"url" => Ok(Field::Url),
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
				type Value = MerchantReturnPolicy;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MerchantReturnPolicy")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					#[cfg(any(
						any(
							feature = "additional-property-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#additional_property_property = None;
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
							feature = "applicable-country-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#applicable_country_property = None;
					#[cfg(any(
						any(
							feature = "customer-remorse-return-fees-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#customer_remorse_return_fees_property = None;
					#[cfg(any(
						any(
							feature = "customer-remorse-return-label-source-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#customer_remorse_return_label_source_property = None;
					#[cfg(any(
						any(
							feature = "customer-remorse-return-shipping-fees-amount-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#customer_remorse_return_shipping_fees_amount_property = None;
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
							feature = "in-store-returns-offered-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#in_store_returns_offered_property = None;
					#[cfg(any(
						any(
							feature = "item-condition-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#item_condition_property = None;
					#[cfg(any(
						any(
							feature = "item-defect-return-fees-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#item_defect_return_fees_property = None;
					#[cfg(any(
						any(
							feature = "item-defect-return-label-source-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#item_defect_return_label_source_property = None;
					#[cfg(any(
						any(
							feature = "item-defect-return-shipping-fees-amount-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#item_defect_return_shipping_fees_amount_property = None;
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
							feature = "merchant-return-days-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#merchant_return_days_property = None;
					#[cfg(any(
						any(
							feature = "merchant-return-link-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#merchant_return_link_property = None;
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
						any(
							feature = "refund-type-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#refund_type_property = None;
					#[cfg(any(
						any(
							feature = "restocking-fee-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#restocking_fee_property = None;
					#[cfg(any(
						any(
							feature = "return-fees-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#return_fees_property = None;
					#[cfg(any(
						any(
							feature = "return-label-source-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#return_label_source_property = None;
					#[cfg(any(
						any(
							feature = "return-method-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#return_method_property = None;
					#[cfg(any(
						any(
							feature = "return-policy-category-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#return_policy_category_property = None;
					#[cfg(any(
						any(
							feature = "return-policy-country-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#return_policy_country_property = None;
					#[cfg(any(
						any(
							feature = "return-policy-seasonal-override-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#return_policy_seasonal_override_property = None;
					#[cfg(any(
						any(
							feature = "return-shipping-fees-amount-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#return_shipping_fees_amount_property = None;
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
						any(feature = "url-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#url_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
							#[cfg(any(
								any(
									feature = "additional-property-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AdditionalProperty => {
								if r#additional_property_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"additionalProperty",
									));
								}
								r#additional_property_property = Some({
									struct DeserializeWith(Vec<AdditionalPropertyProperty>);
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
									feature = "applicable-country-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ApplicableCountry => {
								if r#applicable_country_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"applicableCountry",
									));
								}
								r#applicable_country_property = Some({
									struct DeserializeWith(Vec<ApplicableCountryProperty>);
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
									feature = "customer-remorse-return-fees-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::CustomerRemorseReturnFees => {
								if r#customer_remorse_return_fees_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"customerRemorseReturnFees",
									));
								}
								r#customer_remorse_return_fees_property = Some({
									struct DeserializeWith(Vec<CustomerRemorseReturnFeesProperty>);
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
									feature = "customer-remorse-return-label-source-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::CustomerRemorseReturnLabelSource => {
								if r#customer_remorse_return_label_source_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"customerRemorseReturnLabelSource",
									));
								}
								r#customer_remorse_return_label_source_property = Some({
									struct DeserializeWith(
										Vec<CustomerRemorseReturnLabelSourceProperty>,
									);
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
									feature = "customer-remorse-return-shipping-fees-amount-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::CustomerRemorseReturnShippingFeesAmount => {
								if r#customer_remorse_return_shipping_fees_amount_property.is_some()
								{
									return Err(<A::Error as de::Error>::duplicate_field(
										"customerRemorseReturnShippingFeesAmount",
									));
								}
								r#customer_remorse_return_shipping_fees_amount_property = Some({
									struct DeserializeWith(
										Vec<CustomerRemorseReturnShippingFeesAmountProperty>,
									);
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
									feature = "in-store-returns-offered-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::InStoreReturnsOffered => {
								if r#in_store_returns_offered_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"inStoreReturnsOffered",
									));
								}
								r#in_store_returns_offered_property = Some({
									struct DeserializeWith(Vec<InStoreReturnsOfferedProperty>);
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
									feature = "item-condition-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ItemCondition => {
								if r#item_condition_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"itemCondition",
									));
								}
								r#item_condition_property = Some({
									struct DeserializeWith(Vec<ItemConditionProperty>);
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
									feature = "item-defect-return-fees-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ItemDefectReturnFees => {
								if r#item_defect_return_fees_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"itemDefectReturnFees",
									));
								}
								r#item_defect_return_fees_property = Some({
									struct DeserializeWith(Vec<ItemDefectReturnFeesProperty>);
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
									feature = "item-defect-return-label-source-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ItemDefectReturnLabelSource => {
								if r#item_defect_return_label_source_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"itemDefectReturnLabelSource",
									));
								}
								r#item_defect_return_label_source_property = Some({
									struct DeserializeWith(
										Vec<ItemDefectReturnLabelSourceProperty>,
									);
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
									feature = "item-defect-return-shipping-fees-amount-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ItemDefectReturnShippingFeesAmount => {
								if r#item_defect_return_shipping_fees_amount_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"itemDefectReturnShippingFeesAmount",
									));
								}
								r#item_defect_return_shipping_fees_amount_property = Some({
									struct DeserializeWith(
										Vec<ItemDefectReturnShippingFeesAmountProperty>,
									);
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
									feature = "merchant-return-days-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::MerchantReturnDays => {
								if r#merchant_return_days_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"merchantReturnDays",
									));
								}
								r#merchant_return_days_property = Some({
									struct DeserializeWith(Vec<MerchantReturnDaysProperty>);
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
									feature = "merchant-return-link-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::MerchantReturnLink => {
								if r#merchant_return_link_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"merchantReturnLink",
									));
								}
								r#merchant_return_link_property = Some({
									struct DeserializeWith(Vec<MerchantReturnLinkProperty>);
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
									feature = "refund-type-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::RefundType => {
								if r#refund_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"refundType",
									));
								}
								r#refund_type_property = Some({
									struct DeserializeWith(Vec<RefundTypeProperty>);
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
									feature = "restocking-fee-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::RestockingFee => {
								if r#restocking_fee_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"restockingFee",
									));
								}
								r#restocking_fee_property = Some({
									struct DeserializeWith(Vec<RestockingFeeProperty>);
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
									feature = "return-fees-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ReturnFees => {
								if r#return_fees_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"returnFees",
									));
								}
								r#return_fees_property = Some({
									struct DeserializeWith(Vec<ReturnFeesProperty>);
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
									feature = "return-label-source-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ReturnLabelSource => {
								if r#return_label_source_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"returnLabelSource",
									));
								}
								r#return_label_source_property = Some({
									struct DeserializeWith(Vec<ReturnLabelSourceProperty>);
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
									feature = "return-method-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ReturnMethod => {
								if r#return_method_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"returnMethod",
									));
								}
								r#return_method_property = Some({
									struct DeserializeWith(Vec<ReturnMethodProperty>);
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
									feature = "return-policy-category-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ReturnPolicyCategory => {
								if r#return_policy_category_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"returnPolicyCategory",
									));
								}
								r#return_policy_category_property = Some({
									struct DeserializeWith(Vec<ReturnPolicyCategoryProperty>);
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
									feature = "return-policy-country-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ReturnPolicyCountry => {
								if r#return_policy_country_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"returnPolicyCountry",
									));
								}
								r#return_policy_country_property = Some({
									struct DeserializeWith(Vec<ReturnPolicyCountryProperty>);
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
									feature = "return-policy-seasonal-override-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ReturnPolicySeasonalOverride => {
								if r#return_policy_seasonal_override_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"returnPolicySeasonalOverride",
									));
								}
								r#return_policy_seasonal_override_property = Some({
									struct DeserializeWith(
										Vec<ReturnPolicySeasonalOverrideProperty>,
									);
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
									feature = "return-shipping-fees-amount-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ReturnShippingFeesAmount => {
								if r#return_shipping_fees_amount_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"returnShippingFeesAmount",
									));
								}
								r#return_shipping_fees_amount_property = Some({
									struct DeserializeWith(Vec<ReturnShippingFeesAmountProperty>);
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
							_ => {
								let _ = map.next_value::<de::IgnoredAny>()?;
							}
						}
					}
					Ok(MerchantReturnPolicy {
						#[cfg(any(
							any(
								feature = "additional-property-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#additional_property: r#additional_property_property.unwrap_or_default(),
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
								feature = "applicable-country-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#applicable_country: r#applicable_country_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "customer-remorse-return-fees-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#customer_remorse_return_fees: r#customer_remorse_return_fees_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "customer-remorse-return-label-source-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#customer_remorse_return_label_source:
							r#customer_remorse_return_label_source_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "customer-remorse-return-shipping-fees-amount-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#customer_remorse_return_shipping_fees_amount:
							r#customer_remorse_return_shipping_fees_amount_property
								.unwrap_or_default(),
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
								feature = "in-store-returns-offered-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#in_store_returns_offered: r#in_store_returns_offered_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "item-condition-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#item_condition: r#item_condition_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "item-defect-return-fees-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#item_defect_return_fees: r#item_defect_return_fees_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "item-defect-return-label-source-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#item_defect_return_label_source:
							r#item_defect_return_label_source_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "item-defect-return-shipping-fees-amount-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#item_defect_return_shipping_fees_amount:
							r#item_defect_return_shipping_fees_amount_property.unwrap_or_default(),
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
								feature = "merchant-return-days-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#merchant_return_days: r#merchant_return_days_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "merchant-return-link-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#merchant_return_link: r#merchant_return_link_property.unwrap_or_default(),
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
								feature = "refund-type-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#refund_type: r#refund_type_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "restocking-fee-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#restocking_fee: r#restocking_fee_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "return-fees-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#return_fees: r#return_fees_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "return-label-source-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#return_label_source: r#return_label_source_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "return-method-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#return_method: r#return_method_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "return-policy-category-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#return_policy_category: r#return_policy_category_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "return-policy-country-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#return_policy_country: r#return_policy_country_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "return-policy-seasonal-override-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#return_policy_seasonal_override:
							r#return_policy_seasonal_override_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "return-shipping-fees-amount-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#return_shipping_fees_amount: r#return_shipping_fees_amount_property
							.unwrap_or_default(),
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
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#url: r#url_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				#[cfg(any(
					any(
						feature = "additional-property-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"additionalProperty",
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
						feature = "applicable-country-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"applicableCountry",
				#[cfg(any(
					any(
						feature = "customer-remorse-return-fees-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"customerRemorseReturnFees",
				#[cfg(any(
					any(
						feature = "customer-remorse-return-label-source-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"customerRemorseReturnLabelSource",
				#[cfg(any(
					any(
						feature = "customer-remorse-return-shipping-fees-amount-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"customerRemorseReturnShippingFeesAmount",
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
						feature = "in-store-returns-offered-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"inStoreReturnsOffered",
				#[cfg(any(
					any(
						feature = "item-condition-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"itemCondition",
				#[cfg(any(
					any(
						feature = "item-defect-return-fees-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"itemDefectReturnFees",
				#[cfg(any(
					any(
						feature = "item-defect-return-label-source-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"itemDefectReturnLabelSource",
				#[cfg(any(
					any(
						feature = "item-defect-return-shipping-fees-amount-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"itemDefectReturnShippingFeesAmount",
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
						feature = "merchant-return-days-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"merchantReturnDays",
				#[cfg(any(
					any(
						feature = "merchant-return-link-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"merchantReturnLink",
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
					any(
						feature = "refund-type-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"refundType",
				#[cfg(any(
					any(
						feature = "restocking-fee-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"restockingFee",
				#[cfg(any(
					any(
						feature = "return-fees-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"returnFees",
				#[cfg(any(
					any(
						feature = "return-label-source-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"returnLabelSource",
				#[cfg(any(
					any(
						feature = "return-method-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"returnMethod",
				#[cfg(any(
					any(
						feature = "return-policy-category-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"returnPolicyCategory",
				#[cfg(any(
					any(
						feature = "return-policy-country-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"returnPolicyCountry",
				#[cfg(any(
					any(
						feature = "return-policy-seasonal-override-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"returnPolicySeasonalOverride",
				#[cfg(any(
					any(
						feature = "return-shipping-fees-amount-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"returnShippingFeesAmount",
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
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				"url",
			];
			deserializer.deserialize_struct("MerchantReturnPolicy", FIELDS, ClassVisitor)
		}
	}
}
