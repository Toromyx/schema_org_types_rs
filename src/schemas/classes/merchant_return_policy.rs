use super::*;
/// <https://schema.org/MerchantReturnPolicy>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct MerchantReturnPolicy {
	pub r#additional_property: Vec<AdditionalPropertyProperty>,
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	pub r#alternate_name: Vec<AlternateNameProperty>,
	pub r#applicable_country: Vec<ApplicableCountryProperty>,
	pub r#customer_remorse_return_fees: Vec<CustomerRemorseReturnFeesProperty>,
	pub r#customer_remorse_return_label_source: Vec<CustomerRemorseReturnLabelSourceProperty>,
	pub r#customer_remorse_return_shipping_fees_amount:
		Vec<CustomerRemorseReturnShippingFeesAmountProperty>,
	pub r#description: Vec<DescriptionProperty>,
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	pub r#identifier: Vec<IdentifierProperty>,
	pub r#image: Vec<ImageProperty>,
	pub r#in_store_returns_offered: Vec<InStoreReturnsOfferedProperty>,
	pub r#item_condition: Vec<ItemConditionProperty>,
	pub r#item_defect_return_fees: Vec<ItemDefectReturnFeesProperty>,
	pub r#item_defect_return_label_source: Vec<ItemDefectReturnLabelSourceProperty>,
	pub r#item_defect_return_shipping_fees_amount: Vec<ItemDefectReturnShippingFeesAmountProperty>,
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	pub r#merchant_return_days: Vec<MerchantReturnDaysProperty>,
	pub r#merchant_return_link: Vec<MerchantReturnLinkProperty>,
	pub r#name: Vec<NameProperty>,
	pub r#potential_action: Vec<PotentialActionProperty>,
	pub r#refund_type: Vec<RefundTypeProperty>,
	pub r#restocking_fee: Vec<RestockingFeeProperty>,
	pub r#return_fees: Vec<ReturnFeesProperty>,
	pub r#return_label_source: Vec<ReturnLabelSourceProperty>,
	pub r#return_method: Vec<ReturnMethodProperty>,
	pub r#return_policy_category: Vec<ReturnPolicyCategoryProperty>,
	pub r#return_policy_country: Vec<ReturnPolicyCountryProperty>,
	pub r#return_policy_seasonal_override: Vec<ReturnPolicySeasonalOverrideProperty>,
	pub r#return_shipping_fees_amount: Vec<ReturnShippingFeesAmountProperty>,
	pub r#same_as: Vec<SameAsProperty>,
	pub r#subject_of: Vec<SubjectOfProperty>,
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
				!Vec::is_empty(&self.r#additional_property) as usize,
				!Vec::is_empty(&self.r#additional_type) as usize,
				!Vec::is_empty(&self.r#alternate_name) as usize,
				!Vec::is_empty(&self.r#applicable_country) as usize,
				!Vec::is_empty(&self.r#customer_remorse_return_fees) as usize,
				!Vec::is_empty(&self.r#customer_remorse_return_label_source) as usize,
				!Vec::is_empty(&self.r#customer_remorse_return_shipping_fees_amount) as usize,
				!Vec::is_empty(&self.r#description) as usize,
				!Vec::is_empty(&self.r#disambiguating_description) as usize,
				!Vec::is_empty(&self.r#identifier) as usize,
				!Vec::is_empty(&self.r#image) as usize,
				!Vec::is_empty(&self.r#in_store_returns_offered) as usize,
				!Vec::is_empty(&self.r#item_condition) as usize,
				!Vec::is_empty(&self.r#item_defect_return_fees) as usize,
				!Vec::is_empty(&self.r#item_defect_return_label_source) as usize,
				!Vec::is_empty(&self.r#item_defect_return_shipping_fees_amount) as usize,
				!Vec::is_empty(&self.r#main_entity_of_page) as usize,
				!Vec::is_empty(&self.r#merchant_return_days) as usize,
				!Vec::is_empty(&self.r#merchant_return_link) as usize,
				!Vec::is_empty(&self.r#name) as usize,
				!Vec::is_empty(&self.r#potential_action) as usize,
				!Vec::is_empty(&self.r#refund_type) as usize,
				!Vec::is_empty(&self.r#restocking_fee) as usize,
				!Vec::is_empty(&self.r#return_fees) as usize,
				!Vec::is_empty(&self.r#return_label_source) as usize,
				!Vec::is_empty(&self.r#return_method) as usize,
				!Vec::is_empty(&self.r#return_policy_category) as usize,
				!Vec::is_empty(&self.r#return_policy_country) as usize,
				!Vec::is_empty(&self.r#return_policy_seasonal_override) as usize,
				!Vec::is_empty(&self.r#return_shipping_fees_amount) as usize,
				!Vec::is_empty(&self.r#same_as) as usize,
				!Vec::is_empty(&self.r#subject_of) as usize,
				!Vec::is_empty(&self.r#url) as usize,
			]
			.iter()
			.sum();
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "MerchantReturnPolicy", len)?;
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
				AdditionalProperty,
				AdditionalType,
				AlternateName,
				ApplicableCountry,
				CustomerRemorseReturnFees,
				CustomerRemorseReturnLabelSource,
				CustomerRemorseReturnShippingFeesAmount,
				Description,
				DisambiguatingDescription,
				Identifier,
				Image,
				InStoreReturnsOffered,
				ItemCondition,
				ItemDefectReturnFees,
				ItemDefectReturnLabelSource,
				ItemDefectReturnShippingFeesAmount,
				MainEntityOfPage,
				MerchantReturnDays,
				MerchantReturnLink,
				Name,
				PotentialAction,
				RefundType,
				RestockingFee,
				ReturnFees,
				ReturnLabelSource,
				ReturnMethod,
				ReturnPolicyCategory,
				ReturnPolicyCountry,
				ReturnPolicySeasonalOverride,
				ReturnShippingFeesAmount,
				SameAs,
				SubjectOf,
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
						"additionalProperty" => Ok(Field::AdditionalProperty),
						"additionalType" => Ok(Field::AdditionalType),
						"alternateName" => Ok(Field::AlternateName),
						"applicableCountry" => Ok(Field::ApplicableCountry),
						"customerRemorseReturnFees" => Ok(Field::CustomerRemorseReturnFees),
						"customerRemorseReturnLabelSource" => {
							Ok(Field::CustomerRemorseReturnLabelSource)
						}
						"customerRemorseReturnShippingFeesAmount" => {
							Ok(Field::CustomerRemorseReturnShippingFeesAmount)
						}
						"description" => Ok(Field::Description),
						"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						"identifier" => Ok(Field::Identifier),
						"image" => Ok(Field::Image),
						"inStoreReturnsOffered" => Ok(Field::InStoreReturnsOffered),
						"itemCondition" => Ok(Field::ItemCondition),
						"itemDefectReturnFees" => Ok(Field::ItemDefectReturnFees),
						"itemDefectReturnLabelSource" => Ok(Field::ItemDefectReturnLabelSource),
						"itemDefectReturnShippingFeesAmount" => {
							Ok(Field::ItemDefectReturnShippingFeesAmount)
						}
						"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						"merchantReturnDays" => Ok(Field::MerchantReturnDays),
						"merchantReturnLink" => Ok(Field::MerchantReturnLink),
						"name" => Ok(Field::Name),
						"potentialAction" => Ok(Field::PotentialAction),
						"refundType" => Ok(Field::RefundType),
						"restockingFee" => Ok(Field::RestockingFee),
						"returnFees" => Ok(Field::ReturnFees),
						"returnLabelSource" => Ok(Field::ReturnLabelSource),
						"returnMethod" => Ok(Field::ReturnMethod),
						"returnPolicyCategory" => Ok(Field::ReturnPolicyCategory),
						"returnPolicyCountry" => Ok(Field::ReturnPolicyCountry),
						"returnPolicySeasonalOverride" => Ok(Field::ReturnPolicySeasonalOverride),
						"returnShippingFeesAmount" => Ok(Field::ReturnShippingFeesAmount),
						"sameAs" => Ok(Field::SameAs),
						"subjectOf" => Ok(Field::SubjectOf),
						"url" => Ok(Field::Url),
						_ => Ok(Field::Ignore),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"additionalProperty" => Ok(Field::AdditionalProperty),
						b"additionalType" => Ok(Field::AdditionalType),
						b"alternateName" => Ok(Field::AlternateName),
						b"applicableCountry" => Ok(Field::ApplicableCountry),
						b"customerRemorseReturnFees" => Ok(Field::CustomerRemorseReturnFees),
						b"customerRemorseReturnLabelSource" => {
							Ok(Field::CustomerRemorseReturnLabelSource)
						}
						b"customerRemorseReturnShippingFeesAmount" => {
							Ok(Field::CustomerRemorseReturnShippingFeesAmount)
						}
						b"description" => Ok(Field::Description),
						b"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						b"identifier" => Ok(Field::Identifier),
						b"image" => Ok(Field::Image),
						b"inStoreReturnsOffered" => Ok(Field::InStoreReturnsOffered),
						b"itemCondition" => Ok(Field::ItemCondition),
						b"itemDefectReturnFees" => Ok(Field::ItemDefectReturnFees),
						b"itemDefectReturnLabelSource" => Ok(Field::ItemDefectReturnLabelSource),
						b"itemDefectReturnShippingFeesAmount" => {
							Ok(Field::ItemDefectReturnShippingFeesAmount)
						}
						b"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						b"merchantReturnDays" => Ok(Field::MerchantReturnDays),
						b"merchantReturnLink" => Ok(Field::MerchantReturnLink),
						b"name" => Ok(Field::Name),
						b"potentialAction" => Ok(Field::PotentialAction),
						b"refundType" => Ok(Field::RefundType),
						b"restockingFee" => Ok(Field::RestockingFee),
						b"returnFees" => Ok(Field::ReturnFees),
						b"returnLabelSource" => Ok(Field::ReturnLabelSource),
						b"returnMethod" => Ok(Field::ReturnMethod),
						b"returnPolicyCategory" => Ok(Field::ReturnPolicyCategory),
						b"returnPolicyCountry" => Ok(Field::ReturnPolicyCountry),
						b"returnPolicySeasonalOverride" => Ok(Field::ReturnPolicySeasonalOverride),
						b"returnShippingFeesAmount" => Ok(Field::ReturnShippingFeesAmount),
						b"sameAs" => Ok(Field::SameAs),
						b"subjectOf" => Ok(Field::SubjectOf),
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
					let mut r#additional_property_property = None;
					let mut r#additional_type_property = None;
					let mut r#alternate_name_property = None;
					let mut r#applicable_country_property = None;
					let mut r#customer_remorse_return_fees_property = None;
					let mut r#customer_remorse_return_label_source_property = None;
					let mut r#customer_remorse_return_shipping_fees_amount_property = None;
					let mut r#description_property = None;
					let mut r#disambiguating_description_property = None;
					let mut r#identifier_property = None;
					let mut r#image_property = None;
					let mut r#in_store_returns_offered_property = None;
					let mut r#item_condition_property = None;
					let mut r#item_defect_return_fees_property = None;
					let mut r#item_defect_return_label_source_property = None;
					let mut r#item_defect_return_shipping_fees_amount_property = None;
					let mut r#main_entity_of_page_property = None;
					let mut r#merchant_return_days_property = None;
					let mut r#merchant_return_link_property = None;
					let mut r#name_property = None;
					let mut r#potential_action_property = None;
					let mut r#refund_type_property = None;
					let mut r#restocking_fee_property = None;
					let mut r#return_fees_property = None;
					let mut r#return_label_source_property = None;
					let mut r#return_method_property = None;
					let mut r#return_policy_category_property = None;
					let mut r#return_policy_country_property = None;
					let mut r#return_policy_seasonal_override_property = None;
					let mut r#return_shipping_fees_amount_property = None;
					let mut r#same_as_property = None;
					let mut r#subject_of_property = None;
					let mut r#url_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
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
						r#additional_property: r#additional_property_property.unwrap_or_default(),
						r#additional_type: r#additional_type_property.unwrap_or_default(),
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
						r#applicable_country: r#applicable_country_property.unwrap_or_default(),
						r#customer_remorse_return_fees: r#customer_remorse_return_fees_property
							.unwrap_or_default(),
						r#customer_remorse_return_label_source:
							r#customer_remorse_return_label_source_property.unwrap_or_default(),
						r#customer_remorse_return_shipping_fees_amount:
							r#customer_remorse_return_shipping_fees_amount_property
								.unwrap_or_default(),
						r#description: r#description_property.unwrap_or_default(),
						r#disambiguating_description: r#disambiguating_description_property
							.unwrap_or_default(),
						r#identifier: r#identifier_property.unwrap_or_default(),
						r#image: r#image_property.unwrap_or_default(),
						r#in_store_returns_offered: r#in_store_returns_offered_property
							.unwrap_or_default(),
						r#item_condition: r#item_condition_property.unwrap_or_default(),
						r#item_defect_return_fees: r#item_defect_return_fees_property
							.unwrap_or_default(),
						r#item_defect_return_label_source:
							r#item_defect_return_label_source_property.unwrap_or_default(),
						r#item_defect_return_shipping_fees_amount:
							r#item_defect_return_shipping_fees_amount_property.unwrap_or_default(),
						r#main_entity_of_page: r#main_entity_of_page_property.unwrap_or_default(),
						r#merchant_return_days: r#merchant_return_days_property.unwrap_or_default(),
						r#merchant_return_link: r#merchant_return_link_property.unwrap_or_default(),
						r#name: r#name_property.unwrap_or_default(),
						r#potential_action: r#potential_action_property.unwrap_or_default(),
						r#refund_type: r#refund_type_property.unwrap_or_default(),
						r#restocking_fee: r#restocking_fee_property.unwrap_or_default(),
						r#return_fees: r#return_fees_property.unwrap_or_default(),
						r#return_label_source: r#return_label_source_property.unwrap_or_default(),
						r#return_method: r#return_method_property.unwrap_or_default(),
						r#return_policy_category: r#return_policy_category_property
							.unwrap_or_default(),
						r#return_policy_country: r#return_policy_country_property
							.unwrap_or_default(),
						r#return_policy_seasonal_override:
							r#return_policy_seasonal_override_property.unwrap_or_default(),
						r#return_shipping_fees_amount: r#return_shipping_fees_amount_property
							.unwrap_or_default(),
						r#same_as: r#same_as_property.unwrap_or_default(),
						r#subject_of: r#subject_of_property.unwrap_or_default(),
						r#url: r#url_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				"additionalProperty",
				"additionalType",
				"alternateName",
				"applicableCountry",
				"customerRemorseReturnFees",
				"customerRemorseReturnLabelSource",
				"customerRemorseReturnShippingFeesAmount",
				"description",
				"disambiguatingDescription",
				"identifier",
				"image",
				"inStoreReturnsOffered",
				"itemCondition",
				"itemDefectReturnFees",
				"itemDefectReturnLabelSource",
				"itemDefectReturnShippingFeesAmount",
				"mainEntityOfPage",
				"merchantReturnDays",
				"merchantReturnLink",
				"name",
				"potentialAction",
				"refundType",
				"restockingFee",
				"returnFees",
				"returnLabelSource",
				"returnMethod",
				"returnPolicyCategory",
				"returnPolicyCountry",
				"returnPolicySeasonalOverride",
				"returnShippingFeesAmount",
				"sameAs",
				"subjectOf",
				"url",
			];
			deserializer.deserialize_struct("MerchantReturnPolicy", FIELDS, ClassVisitor)
		}
	}
}
