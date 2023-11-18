use super::*;
/// <https://schema.org/MerchantReturnPolicy>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct MerchantReturnPolicy {
	/// <https://schema.org/additionalProperty>
	pub r#additional_property: Vec<AdditionalPropertyProperty>,
	/// <https://schema.org/applicableCountry>
	pub r#applicable_country: Vec<ApplicableCountryProperty>,
	/// <https://schema.org/customerRemorseReturnFees>
	pub r#customer_remorse_return_fees: Vec<CustomerRemorseReturnFeesProperty>,
	/// <https://schema.org/customerRemorseReturnLabelSource>
	pub r#customer_remorse_return_label_source: Vec<CustomerRemorseReturnLabelSourceProperty>,
	/// <https://schema.org/customerRemorseReturnShippingFeesAmount>
	pub r#customer_remorse_return_shipping_fees_amount:
		Vec<CustomerRemorseReturnShippingFeesAmountProperty>,
	/// <https://schema.org/inStoreReturnsOffered>
	pub r#in_store_returns_offered: Vec<InStoreReturnsOfferedProperty>,
	/// <https://schema.org/itemCondition>
	pub r#item_condition: Vec<ItemConditionProperty>,
	/// <https://schema.org/itemDefectReturnFees>
	pub r#item_defect_return_fees: Vec<ItemDefectReturnFeesProperty>,
	/// <https://schema.org/itemDefectReturnLabelSource>
	pub r#item_defect_return_label_source: Vec<ItemDefectReturnLabelSourceProperty>,
	/// <https://schema.org/itemDefectReturnShippingFeesAmount>
	pub r#item_defect_return_shipping_fees_amount: Vec<ItemDefectReturnShippingFeesAmountProperty>,
	/// <https://schema.org/merchantReturnDays>
	pub r#merchant_return_days: Vec<MerchantReturnDaysProperty>,
	/// <https://schema.org/merchantReturnLink>
	pub r#merchant_return_link: Vec<MerchantReturnLinkProperty>,
	/// <https://schema.org/refundType>
	pub r#refund_type: Vec<RefundTypeProperty>,
	/// <https://schema.org/restockingFee>
	pub r#restocking_fee: Vec<RestockingFeeProperty>,
	/// <https://schema.org/returnFees>
	pub r#return_fees: Vec<ReturnFeesProperty>,
	/// <https://schema.org/returnLabelSource>
	pub r#return_label_source: Vec<ReturnLabelSourceProperty>,
	/// <https://schema.org/returnMethod>
	pub r#return_method: Vec<ReturnMethodProperty>,
	/// <https://schema.org/returnPolicyCategory>
	pub r#return_policy_category: Vec<ReturnPolicyCategoryProperty>,
	/// <https://schema.org/returnPolicyCountry>
	pub r#return_policy_country: Vec<ReturnPolicyCountryProperty>,
	/// <https://schema.org/returnPolicySeasonalOverride>
	pub r#return_policy_seasonal_override: Vec<ReturnPolicySeasonalOverrideProperty>,
	/// <https://schema.org/returnShippingFeesAmount>
	pub r#return_shipping_fees_amount: Vec<ReturnShippingFeesAmountProperty>,
	/// <https://schema.org/additionalType>
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	/// <https://schema.org/alternateName>
	pub r#alternate_name: Vec<AlternateNameProperty>,
	/// <https://schema.org/description>
	pub r#description: Vec<DescriptionProperty>,
	/// <https://schema.org/disambiguatingDescription>
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	/// <https://schema.org/identifier>
	pub r#identifier: Vec<IdentifierProperty>,
	/// <https://schema.org/image>
	pub r#image: Vec<ImageProperty>,
	/// <https://schema.org/mainEntityOfPage>
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	/// <https://schema.org/name>
	pub r#name: Vec<NameProperty>,
	/// <https://schema.org/potentialAction>
	pub r#potential_action: Vec<PotentialActionProperty>,
	/// <https://schema.org/sameAs>
	pub r#same_as: Vec<SameAsProperty>,
	/// <https://schema.org/subjectOf>
	pub r#subject_of: Vec<SubjectOfProperty>,
	/// <https://schema.org/url>
	pub r#url: Vec<UrlProperty>,
}
/// This trait is for properties from <https://schema.org/MerchantReturnPolicy>.
pub trait MerchantReturnPolicyTrait {
	/// Get <https://schema.org/additionalProperty> from [`Self`] as borrowed slice.
	fn get_additional_property(&self) -> &[AdditionalPropertyProperty];
	/// Take <https://schema.org/additionalProperty> from [`Self`] as owned vector.
	fn take_additional_property(&mut self) -> Vec<AdditionalPropertyProperty>;
	/// Get <https://schema.org/applicableCountry> from [`Self`] as borrowed slice.
	fn get_applicable_country(&self) -> &[ApplicableCountryProperty];
	/// Take <https://schema.org/applicableCountry> from [`Self`] as owned vector.
	fn take_applicable_country(&mut self) -> Vec<ApplicableCountryProperty>;
	/// Get <https://schema.org/customerRemorseReturnFees> from [`Self`] as borrowed slice.
	fn get_customer_remorse_return_fees(&self) -> &[CustomerRemorseReturnFeesProperty];
	/// Take <https://schema.org/customerRemorseReturnFees> from [`Self`] as owned vector.
	fn take_customer_remorse_return_fees(&mut self) -> Vec<CustomerRemorseReturnFeesProperty>;
	/// Get <https://schema.org/customerRemorseReturnLabelSource> from [`Self`] as borrowed slice.
	fn get_customer_remorse_return_label_source(
		&self,
	) -> &[CustomerRemorseReturnLabelSourceProperty];
	/// Take <https://schema.org/customerRemorseReturnLabelSource> from [`Self`] as owned vector.
	fn take_customer_remorse_return_label_source(
		&mut self,
	) -> Vec<CustomerRemorseReturnLabelSourceProperty>;
	/// Get <https://schema.org/customerRemorseReturnShippingFeesAmount> from [`Self`] as borrowed slice.
	fn get_customer_remorse_return_shipping_fees_amount(
		&self,
	) -> &[CustomerRemorseReturnShippingFeesAmountProperty];
	/// Take <https://schema.org/customerRemorseReturnShippingFeesAmount> from [`Self`] as owned vector.
	fn take_customer_remorse_return_shipping_fees_amount(
		&mut self,
	) -> Vec<CustomerRemorseReturnShippingFeesAmountProperty>;
	/// Get <https://schema.org/inStoreReturnsOffered> from [`Self`] as borrowed slice.
	fn get_in_store_returns_offered(&self) -> &[InStoreReturnsOfferedProperty];
	/// Take <https://schema.org/inStoreReturnsOffered> from [`Self`] as owned vector.
	fn take_in_store_returns_offered(&mut self) -> Vec<InStoreReturnsOfferedProperty>;
	/// Get <https://schema.org/itemCondition> from [`Self`] as borrowed slice.
	fn get_item_condition(&self) -> &[ItemConditionProperty];
	/// Take <https://schema.org/itemCondition> from [`Self`] as owned vector.
	fn take_item_condition(&mut self) -> Vec<ItemConditionProperty>;
	/// Get <https://schema.org/itemDefectReturnFees> from [`Self`] as borrowed slice.
	fn get_item_defect_return_fees(&self) -> &[ItemDefectReturnFeesProperty];
	/// Take <https://schema.org/itemDefectReturnFees> from [`Self`] as owned vector.
	fn take_item_defect_return_fees(&mut self) -> Vec<ItemDefectReturnFeesProperty>;
	/// Get <https://schema.org/itemDefectReturnLabelSource> from [`Self`] as borrowed slice.
	fn get_item_defect_return_label_source(&self) -> &[ItemDefectReturnLabelSourceProperty];
	/// Take <https://schema.org/itemDefectReturnLabelSource> from [`Self`] as owned vector.
	fn take_item_defect_return_label_source(&mut self) -> Vec<ItemDefectReturnLabelSourceProperty>;
	/// Get <https://schema.org/itemDefectReturnShippingFeesAmount> from [`Self`] as borrowed slice.
	fn get_item_defect_return_shipping_fees_amount(
		&self,
	) -> &[ItemDefectReturnShippingFeesAmountProperty];
	/// Take <https://schema.org/itemDefectReturnShippingFeesAmount> from [`Self`] as owned vector.
	fn take_item_defect_return_shipping_fees_amount(
		&mut self,
	) -> Vec<ItemDefectReturnShippingFeesAmountProperty>;
	/// Get <https://schema.org/merchantReturnDays> from [`Self`] as borrowed slice.
	fn get_merchant_return_days(&self) -> &[MerchantReturnDaysProperty];
	/// Take <https://schema.org/merchantReturnDays> from [`Self`] as owned vector.
	fn take_merchant_return_days(&mut self) -> Vec<MerchantReturnDaysProperty>;
	/// Get <https://schema.org/merchantReturnLink> from [`Self`] as borrowed slice.
	fn get_merchant_return_link(&self) -> &[MerchantReturnLinkProperty];
	/// Take <https://schema.org/merchantReturnLink> from [`Self`] as owned vector.
	fn take_merchant_return_link(&mut self) -> Vec<MerchantReturnLinkProperty>;
	/// Get <https://schema.org/refundType> from [`Self`] as borrowed slice.
	fn get_refund_type(&self) -> &[RefundTypeProperty];
	/// Take <https://schema.org/refundType> from [`Self`] as owned vector.
	fn take_refund_type(&mut self) -> Vec<RefundTypeProperty>;
	/// Get <https://schema.org/restockingFee> from [`Self`] as borrowed slice.
	fn get_restocking_fee(&self) -> &[RestockingFeeProperty];
	/// Take <https://schema.org/restockingFee> from [`Self`] as owned vector.
	fn take_restocking_fee(&mut self) -> Vec<RestockingFeeProperty>;
	/// Get <https://schema.org/returnFees> from [`Self`] as borrowed slice.
	fn get_return_fees(&self) -> &[ReturnFeesProperty];
	/// Take <https://schema.org/returnFees> from [`Self`] as owned vector.
	fn take_return_fees(&mut self) -> Vec<ReturnFeesProperty>;
	/// Get <https://schema.org/returnLabelSource> from [`Self`] as borrowed slice.
	fn get_return_label_source(&self) -> &[ReturnLabelSourceProperty];
	/// Take <https://schema.org/returnLabelSource> from [`Self`] as owned vector.
	fn take_return_label_source(&mut self) -> Vec<ReturnLabelSourceProperty>;
	/// Get <https://schema.org/returnMethod> from [`Self`] as borrowed slice.
	fn get_return_method(&self) -> &[ReturnMethodProperty];
	/// Take <https://schema.org/returnMethod> from [`Self`] as owned vector.
	fn take_return_method(&mut self) -> Vec<ReturnMethodProperty>;
	/// Get <https://schema.org/returnPolicyCategory> from [`Self`] as borrowed slice.
	fn get_return_policy_category(&self) -> &[ReturnPolicyCategoryProperty];
	/// Take <https://schema.org/returnPolicyCategory> from [`Self`] as owned vector.
	fn take_return_policy_category(&mut self) -> Vec<ReturnPolicyCategoryProperty>;
	/// Get <https://schema.org/returnPolicyCountry> from [`Self`] as borrowed slice.
	fn get_return_policy_country(&self) -> &[ReturnPolicyCountryProperty];
	/// Take <https://schema.org/returnPolicyCountry> from [`Self`] as owned vector.
	fn take_return_policy_country(&mut self) -> Vec<ReturnPolicyCountryProperty>;
	/// Get <https://schema.org/returnPolicySeasonalOverride> from [`Self`] as borrowed slice.
	fn get_return_policy_seasonal_override(&self) -> &[ReturnPolicySeasonalOverrideProperty];
	/// Take <https://schema.org/returnPolicySeasonalOverride> from [`Self`] as owned vector.
	fn take_return_policy_seasonal_override(&mut self)
	-> Vec<ReturnPolicySeasonalOverrideProperty>;
	/// Get <https://schema.org/returnShippingFeesAmount> from [`Self`] as borrowed slice.
	fn get_return_shipping_fees_amount(&self) -> &[ReturnShippingFeesAmountProperty];
	/// Take <https://schema.org/returnShippingFeesAmount> from [`Self`] as owned vector.
	fn take_return_shipping_fees_amount(&mut self) -> Vec<ReturnShippingFeesAmountProperty>;
}
impl MerchantReturnPolicyTrait for MerchantReturnPolicy {
	fn get_additional_property(&self) -> &[AdditionalPropertyProperty] {
		self.r#additional_property.as_slice()
	}
	fn take_additional_property(&mut self) -> Vec<AdditionalPropertyProperty> {
		std::mem::take(&mut self.r#additional_property)
	}
	fn get_applicable_country(&self) -> &[ApplicableCountryProperty] {
		self.r#applicable_country.as_slice()
	}
	fn take_applicable_country(&mut self) -> Vec<ApplicableCountryProperty> {
		std::mem::take(&mut self.r#applicable_country)
	}
	fn get_customer_remorse_return_fees(&self) -> &[CustomerRemorseReturnFeesProperty] {
		self.r#customer_remorse_return_fees.as_slice()
	}
	fn take_customer_remorse_return_fees(&mut self) -> Vec<CustomerRemorseReturnFeesProperty> {
		std::mem::take(&mut self.r#customer_remorse_return_fees)
	}
	fn get_customer_remorse_return_label_source(
		&self,
	) -> &[CustomerRemorseReturnLabelSourceProperty] {
		self.r#customer_remorse_return_label_source.as_slice()
	}
	fn take_customer_remorse_return_label_source(
		&mut self,
	) -> Vec<CustomerRemorseReturnLabelSourceProperty> {
		std::mem::take(&mut self.r#customer_remorse_return_label_source)
	}
	fn get_customer_remorse_return_shipping_fees_amount(
		&self,
	) -> &[CustomerRemorseReturnShippingFeesAmountProperty] {
		self.r#customer_remorse_return_shipping_fees_amount
			.as_slice()
	}
	fn take_customer_remorse_return_shipping_fees_amount(
		&mut self,
	) -> Vec<CustomerRemorseReturnShippingFeesAmountProperty> {
		std::mem::take(&mut self.r#customer_remorse_return_shipping_fees_amount)
	}
	fn get_in_store_returns_offered(&self) -> &[InStoreReturnsOfferedProperty] {
		self.r#in_store_returns_offered.as_slice()
	}
	fn take_in_store_returns_offered(&mut self) -> Vec<InStoreReturnsOfferedProperty> {
		std::mem::take(&mut self.r#in_store_returns_offered)
	}
	fn get_item_condition(&self) -> &[ItemConditionProperty] {
		self.r#item_condition.as_slice()
	}
	fn take_item_condition(&mut self) -> Vec<ItemConditionProperty> {
		std::mem::take(&mut self.r#item_condition)
	}
	fn get_item_defect_return_fees(&self) -> &[ItemDefectReturnFeesProperty] {
		self.r#item_defect_return_fees.as_slice()
	}
	fn take_item_defect_return_fees(&mut self) -> Vec<ItemDefectReturnFeesProperty> {
		std::mem::take(&mut self.r#item_defect_return_fees)
	}
	fn get_item_defect_return_label_source(&self) -> &[ItemDefectReturnLabelSourceProperty] {
		self.r#item_defect_return_label_source.as_slice()
	}
	fn take_item_defect_return_label_source(&mut self) -> Vec<ItemDefectReturnLabelSourceProperty> {
		std::mem::take(&mut self.r#item_defect_return_label_source)
	}
	fn get_item_defect_return_shipping_fees_amount(
		&self,
	) -> &[ItemDefectReturnShippingFeesAmountProperty] {
		self.r#item_defect_return_shipping_fees_amount.as_slice()
	}
	fn take_item_defect_return_shipping_fees_amount(
		&mut self,
	) -> Vec<ItemDefectReturnShippingFeesAmountProperty> {
		std::mem::take(&mut self.r#item_defect_return_shipping_fees_amount)
	}
	fn get_merchant_return_days(&self) -> &[MerchantReturnDaysProperty] {
		self.r#merchant_return_days.as_slice()
	}
	fn take_merchant_return_days(&mut self) -> Vec<MerchantReturnDaysProperty> {
		std::mem::take(&mut self.r#merchant_return_days)
	}
	fn get_merchant_return_link(&self) -> &[MerchantReturnLinkProperty] {
		self.r#merchant_return_link.as_slice()
	}
	fn take_merchant_return_link(&mut self) -> Vec<MerchantReturnLinkProperty> {
		std::mem::take(&mut self.r#merchant_return_link)
	}
	fn get_refund_type(&self) -> &[RefundTypeProperty] {
		self.r#refund_type.as_slice()
	}
	fn take_refund_type(&mut self) -> Vec<RefundTypeProperty> {
		std::mem::take(&mut self.r#refund_type)
	}
	fn get_restocking_fee(&self) -> &[RestockingFeeProperty] {
		self.r#restocking_fee.as_slice()
	}
	fn take_restocking_fee(&mut self) -> Vec<RestockingFeeProperty> {
		std::mem::take(&mut self.r#restocking_fee)
	}
	fn get_return_fees(&self) -> &[ReturnFeesProperty] {
		self.r#return_fees.as_slice()
	}
	fn take_return_fees(&mut self) -> Vec<ReturnFeesProperty> {
		std::mem::take(&mut self.r#return_fees)
	}
	fn get_return_label_source(&self) -> &[ReturnLabelSourceProperty] {
		self.r#return_label_source.as_slice()
	}
	fn take_return_label_source(&mut self) -> Vec<ReturnLabelSourceProperty> {
		std::mem::take(&mut self.r#return_label_source)
	}
	fn get_return_method(&self) -> &[ReturnMethodProperty] {
		self.r#return_method.as_slice()
	}
	fn take_return_method(&mut self) -> Vec<ReturnMethodProperty> {
		std::mem::take(&mut self.r#return_method)
	}
	fn get_return_policy_category(&self) -> &[ReturnPolicyCategoryProperty] {
		self.r#return_policy_category.as_slice()
	}
	fn take_return_policy_category(&mut self) -> Vec<ReturnPolicyCategoryProperty> {
		std::mem::take(&mut self.r#return_policy_category)
	}
	fn get_return_policy_country(&self) -> &[ReturnPolicyCountryProperty] {
		self.r#return_policy_country.as_slice()
	}
	fn take_return_policy_country(&mut self) -> Vec<ReturnPolicyCountryProperty> {
		std::mem::take(&mut self.r#return_policy_country)
	}
	fn get_return_policy_seasonal_override(&self) -> &[ReturnPolicySeasonalOverrideProperty] {
		self.r#return_policy_seasonal_override.as_slice()
	}
	fn take_return_policy_seasonal_override(
		&mut self,
	) -> Vec<ReturnPolicySeasonalOverrideProperty> {
		std::mem::take(&mut self.r#return_policy_seasonal_override)
	}
	fn get_return_shipping_fees_amount(&self) -> &[ReturnShippingFeesAmountProperty] {
		self.r#return_shipping_fees_amount.as_slice()
	}
	fn take_return_shipping_fees_amount(&mut self) -> Vec<ReturnShippingFeesAmountProperty> {
		std::mem::take(&mut self.r#return_shipping_fees_amount)
	}
}
impl ThingTrait for MerchantReturnPolicy {
	fn get_additional_type(&self) -> &[AdditionalTypeProperty] {
		self.r#additional_type.as_slice()
	}
	fn take_additional_type(&mut self) -> Vec<AdditionalTypeProperty> {
		std::mem::take(&mut self.r#additional_type)
	}
	fn get_alternate_name(&self) -> &[AlternateNameProperty] {
		self.r#alternate_name.as_slice()
	}
	fn take_alternate_name(&mut self) -> Vec<AlternateNameProperty> {
		std::mem::take(&mut self.r#alternate_name)
	}
	fn get_description(&self) -> &[DescriptionProperty] {
		self.r#description.as_slice()
	}
	fn take_description(&mut self) -> Vec<DescriptionProperty> {
		std::mem::take(&mut self.r#description)
	}
	fn get_disambiguating_description(&self) -> &[DisambiguatingDescriptionProperty] {
		self.r#disambiguating_description.as_slice()
	}
	fn take_disambiguating_description(&mut self) -> Vec<DisambiguatingDescriptionProperty> {
		std::mem::take(&mut self.r#disambiguating_description)
	}
	fn get_identifier(&self) -> &[IdentifierProperty] {
		self.r#identifier.as_slice()
	}
	fn take_identifier(&mut self) -> Vec<IdentifierProperty> {
		std::mem::take(&mut self.r#identifier)
	}
	fn get_image(&self) -> &[ImageProperty] {
		self.r#image.as_slice()
	}
	fn take_image(&mut self) -> Vec<ImageProperty> {
		std::mem::take(&mut self.r#image)
	}
	fn get_main_entity_of_page(&self) -> &[MainEntityOfPageProperty] {
		self.r#main_entity_of_page.as_slice()
	}
	fn take_main_entity_of_page(&mut self) -> Vec<MainEntityOfPageProperty> {
		std::mem::take(&mut self.r#main_entity_of_page)
	}
	fn get_name(&self) -> &[NameProperty] {
		self.r#name.as_slice()
	}
	fn take_name(&mut self) -> Vec<NameProperty> {
		std::mem::take(&mut self.r#name)
	}
	fn get_potential_action(&self) -> &[PotentialActionProperty] {
		self.r#potential_action.as_slice()
	}
	fn take_potential_action(&mut self) -> Vec<PotentialActionProperty> {
		std::mem::take(&mut self.r#potential_action)
	}
	fn get_same_as(&self) -> &[SameAsProperty] {
		self.r#same_as.as_slice()
	}
	fn take_same_as(&mut self) -> Vec<SameAsProperty> {
		std::mem::take(&mut self.r#same_as)
	}
	fn get_subject_of(&self) -> &[SubjectOfProperty] {
		self.r#subject_of.as_slice()
	}
	fn take_subject_of(&mut self) -> Vec<SubjectOfProperty> {
		std::mem::take(&mut self.r#subject_of)
	}
	fn get_url(&self) -> &[UrlProperty] {
		self.r#url.as_slice()
	}
	fn take_url(&mut self) -> Vec<UrlProperty> {
		std::mem::take(&mut self.r#url)
	}
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
				!Vec::is_empty(&self.r#applicable_country) as usize,
				!Vec::is_empty(&self.r#customer_remorse_return_fees) as usize,
				!Vec::is_empty(&self.r#customer_remorse_return_label_source) as usize,
				!Vec::is_empty(&self.r#customer_remorse_return_shipping_fees_amount) as usize,
				!Vec::is_empty(&self.r#in_store_returns_offered) as usize,
				!Vec::is_empty(&self.r#item_condition) as usize,
				!Vec::is_empty(&self.r#item_defect_return_fees) as usize,
				!Vec::is_empty(&self.r#item_defect_return_label_source) as usize,
				!Vec::is_empty(&self.r#item_defect_return_shipping_fees_amount) as usize,
				!Vec::is_empty(&self.r#merchant_return_days) as usize,
				!Vec::is_empty(&self.r#merchant_return_link) as usize,
				!Vec::is_empty(&self.r#refund_type) as usize,
				!Vec::is_empty(&self.r#restocking_fee) as usize,
				!Vec::is_empty(&self.r#return_fees) as usize,
				!Vec::is_empty(&self.r#return_label_source) as usize,
				!Vec::is_empty(&self.r#return_method) as usize,
				!Vec::is_empty(&self.r#return_policy_category) as usize,
				!Vec::is_empty(&self.r#return_policy_country) as usize,
				!Vec::is_empty(&self.r#return_policy_seasonal_override) as usize,
				!Vec::is_empty(&self.r#return_shipping_fees_amount) as usize,
				!Vec::is_empty(&self.r#additional_type) as usize,
				!Vec::is_empty(&self.r#alternate_name) as usize,
				!Vec::is_empty(&self.r#description) as usize,
				!Vec::is_empty(&self.r#disambiguating_description) as usize,
				!Vec::is_empty(&self.r#identifier) as usize,
				!Vec::is_empty(&self.r#image) as usize,
				!Vec::is_empty(&self.r#main_entity_of_page) as usize,
				!Vec::is_empty(&self.r#name) as usize,
				!Vec::is_empty(&self.r#potential_action) as usize,
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
				ApplicableCountry,
				CustomerRemorseReturnFees,
				CustomerRemorseReturnLabelSource,
				CustomerRemorseReturnShippingFeesAmount,
				InStoreReturnsOffered,
				ItemCondition,
				ItemDefectReturnFees,
				ItemDefectReturnLabelSource,
				ItemDefectReturnShippingFeesAmount,
				MerchantReturnDays,
				MerchantReturnLink,
				RefundType,
				RestockingFee,
				ReturnFees,
				ReturnLabelSource,
				ReturnMethod,
				ReturnPolicyCategory,
				ReturnPolicyCountry,
				ReturnPolicySeasonalOverride,
				ReturnShippingFeesAmount,
				AdditionalType,
				AlternateName,
				Description,
				DisambiguatingDescription,
				Identifier,
				Image,
				MainEntityOfPage,
				Name,
				PotentialAction,
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
						"applicableCountry" => Ok(Field::ApplicableCountry),
						"customerRemorseReturnFees" => Ok(Field::CustomerRemorseReturnFees),
						"customerRemorseReturnLabelSource" => {
							Ok(Field::CustomerRemorseReturnLabelSource)
						}
						"customerRemorseReturnShippingFeesAmount" => {
							Ok(Field::CustomerRemorseReturnShippingFeesAmount)
						}
						"inStoreReturnsOffered" => Ok(Field::InStoreReturnsOffered),
						"itemCondition" => Ok(Field::ItemCondition),
						"itemDefectReturnFees" => Ok(Field::ItemDefectReturnFees),
						"itemDefectReturnLabelSource" => Ok(Field::ItemDefectReturnLabelSource),
						"itemDefectReturnShippingFeesAmount" => {
							Ok(Field::ItemDefectReturnShippingFeesAmount)
						}
						"merchantReturnDays" => Ok(Field::MerchantReturnDays),
						"merchantReturnLink" => Ok(Field::MerchantReturnLink),
						"refundType" => Ok(Field::RefundType),
						"restockingFee" => Ok(Field::RestockingFee),
						"returnFees" => Ok(Field::ReturnFees),
						"returnLabelSource" => Ok(Field::ReturnLabelSource),
						"returnMethod" => Ok(Field::ReturnMethod),
						"returnPolicyCategory" => Ok(Field::ReturnPolicyCategory),
						"returnPolicyCountry" => Ok(Field::ReturnPolicyCountry),
						"returnPolicySeasonalOverride" => Ok(Field::ReturnPolicySeasonalOverride),
						"returnShippingFeesAmount" => Ok(Field::ReturnShippingFeesAmount),
						"additionalType" => Ok(Field::AdditionalType),
						"alternateName" => Ok(Field::AlternateName),
						"description" => Ok(Field::Description),
						"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						"identifier" => Ok(Field::Identifier),
						"image" => Ok(Field::Image),
						"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						"name" => Ok(Field::Name),
						"potentialAction" => Ok(Field::PotentialAction),
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
						b"applicableCountry" => Ok(Field::ApplicableCountry),
						b"customerRemorseReturnFees" => Ok(Field::CustomerRemorseReturnFees),
						b"customerRemorseReturnLabelSource" => {
							Ok(Field::CustomerRemorseReturnLabelSource)
						}
						b"customerRemorseReturnShippingFeesAmount" => {
							Ok(Field::CustomerRemorseReturnShippingFeesAmount)
						}
						b"inStoreReturnsOffered" => Ok(Field::InStoreReturnsOffered),
						b"itemCondition" => Ok(Field::ItemCondition),
						b"itemDefectReturnFees" => Ok(Field::ItemDefectReturnFees),
						b"itemDefectReturnLabelSource" => Ok(Field::ItemDefectReturnLabelSource),
						b"itemDefectReturnShippingFeesAmount" => {
							Ok(Field::ItemDefectReturnShippingFeesAmount)
						}
						b"merchantReturnDays" => Ok(Field::MerchantReturnDays),
						b"merchantReturnLink" => Ok(Field::MerchantReturnLink),
						b"refundType" => Ok(Field::RefundType),
						b"restockingFee" => Ok(Field::RestockingFee),
						b"returnFees" => Ok(Field::ReturnFees),
						b"returnLabelSource" => Ok(Field::ReturnLabelSource),
						b"returnMethod" => Ok(Field::ReturnMethod),
						b"returnPolicyCategory" => Ok(Field::ReturnPolicyCategory),
						b"returnPolicyCountry" => Ok(Field::ReturnPolicyCountry),
						b"returnPolicySeasonalOverride" => Ok(Field::ReturnPolicySeasonalOverride),
						b"returnShippingFeesAmount" => Ok(Field::ReturnShippingFeesAmount),
						b"additionalType" => Ok(Field::AdditionalType),
						b"alternateName" => Ok(Field::AlternateName),
						b"description" => Ok(Field::Description),
						b"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						b"identifier" => Ok(Field::Identifier),
						b"image" => Ok(Field::Image),
						b"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						b"name" => Ok(Field::Name),
						b"potentialAction" => Ok(Field::PotentialAction),
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
					let mut r#applicable_country_property = None;
					let mut r#customer_remorse_return_fees_property = None;
					let mut r#customer_remorse_return_label_source_property = None;
					let mut r#customer_remorse_return_shipping_fees_amount_property = None;
					let mut r#in_store_returns_offered_property = None;
					let mut r#item_condition_property = None;
					let mut r#item_defect_return_fees_property = None;
					let mut r#item_defect_return_label_source_property = None;
					let mut r#item_defect_return_shipping_fees_amount_property = None;
					let mut r#merchant_return_days_property = None;
					let mut r#merchant_return_link_property = None;
					let mut r#refund_type_property = None;
					let mut r#restocking_fee_property = None;
					let mut r#return_fees_property = None;
					let mut r#return_label_source_property = None;
					let mut r#return_method_property = None;
					let mut r#return_policy_category_property = None;
					let mut r#return_policy_country_property = None;
					let mut r#return_policy_seasonal_override_property = None;
					let mut r#return_shipping_fees_amount_property = None;
					let mut r#additional_type_property = None;
					let mut r#alternate_name_property = None;
					let mut r#description_property = None;
					let mut r#disambiguating_description_property = None;
					let mut r#identifier_property = None;
					let mut r#image_property = None;
					let mut r#main_entity_of_page_property = None;
					let mut r#name_property = None;
					let mut r#potential_action_property = None;
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
						r#applicable_country: r#applicable_country_property.unwrap_or_default(),
						r#customer_remorse_return_fees: r#customer_remorse_return_fees_property
							.unwrap_or_default(),
						r#customer_remorse_return_label_source:
							r#customer_remorse_return_label_source_property.unwrap_or_default(),
						r#customer_remorse_return_shipping_fees_amount:
							r#customer_remorse_return_shipping_fees_amount_property
								.unwrap_or_default(),
						r#in_store_returns_offered: r#in_store_returns_offered_property
							.unwrap_or_default(),
						r#item_condition: r#item_condition_property.unwrap_or_default(),
						r#item_defect_return_fees: r#item_defect_return_fees_property
							.unwrap_or_default(),
						r#item_defect_return_label_source:
							r#item_defect_return_label_source_property.unwrap_or_default(),
						r#item_defect_return_shipping_fees_amount:
							r#item_defect_return_shipping_fees_amount_property.unwrap_or_default(),
						r#merchant_return_days: r#merchant_return_days_property.unwrap_or_default(),
						r#merchant_return_link: r#merchant_return_link_property.unwrap_or_default(),
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
						r#additional_type: r#additional_type_property.unwrap_or_default(),
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
						r#description: r#description_property.unwrap_or_default(),
						r#disambiguating_description: r#disambiguating_description_property
							.unwrap_or_default(),
						r#identifier: r#identifier_property.unwrap_or_default(),
						r#image: r#image_property.unwrap_or_default(),
						r#main_entity_of_page: r#main_entity_of_page_property.unwrap_or_default(),
						r#name: r#name_property.unwrap_or_default(),
						r#potential_action: r#potential_action_property.unwrap_or_default(),
						r#same_as: r#same_as_property.unwrap_or_default(),
						r#subject_of: r#subject_of_property.unwrap_or_default(),
						r#url: r#url_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				"additionalProperty",
				"applicableCountry",
				"customerRemorseReturnFees",
				"customerRemorseReturnLabelSource",
				"customerRemorseReturnShippingFeesAmount",
				"inStoreReturnsOffered",
				"itemCondition",
				"itemDefectReturnFees",
				"itemDefectReturnLabelSource",
				"itemDefectReturnShippingFeesAmount",
				"merchantReturnDays",
				"merchantReturnLink",
				"refundType",
				"restockingFee",
				"returnFees",
				"returnLabelSource",
				"returnMethod",
				"returnPolicyCategory",
				"returnPolicyCountry",
				"returnPolicySeasonalOverride",
				"returnShippingFeesAmount",
				"additionalType",
				"alternateName",
				"description",
				"disambiguatingDescription",
				"identifier",
				"image",
				"mainEntityOfPage",
				"name",
				"potentialAction",
				"sameAs",
				"subjectOf",
				"url",
			];
			deserializer.deserialize_struct("MerchantReturnPolicy", FIELDS, ClassVisitor)
		}
	}
}
