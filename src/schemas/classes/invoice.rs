use super::*;
/// <https://schema.org/Invoice>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Invoice {
	/// <https://schema.org/accountId>
	pub r#account_id: Vec<AccountIdProperty>,
	/// <https://schema.org/billingPeriod>
	pub r#billing_period: Vec<BillingPeriodProperty>,
	/// <https://schema.org/broker>
	pub r#broker: Vec<BrokerProperty>,
	/// <https://schema.org/category>
	pub r#category: Vec<CategoryProperty>,
	/// <https://schema.org/confirmationNumber>
	pub r#confirmation_number: Vec<ConfirmationNumberProperty>,
	/// <https://schema.org/customer>
	pub r#customer: Vec<CustomerProperty>,
	/// <https://schema.org/minimumPaymentDue>
	pub r#minimum_payment_due: Vec<MinimumPaymentDueProperty>,
	/// <https://schema.org/paymentDue>
	#[deprecated = "This schema is superseded by <https://schema.org/paymentDueDate>."]
	pub r#payment_due: Vec<PaymentDueProperty>,
	/// <https://schema.org/paymentDueDate>
	pub r#payment_due_date: Vec<PaymentDueDateProperty>,
	/// <https://schema.org/paymentMethod>
	pub r#payment_method: Vec<PaymentMethodProperty>,
	/// <https://schema.org/paymentMethodId>
	pub r#payment_method_id: Vec<PaymentMethodIdProperty>,
	/// <https://schema.org/paymentStatus>
	pub r#payment_status: Vec<PaymentStatusProperty>,
	/// <https://schema.org/provider>
	pub r#provider: Vec<ProviderProperty>,
	/// <https://schema.org/referencesOrder>
	pub r#references_order: Vec<ReferencesOrderProperty>,
	/// <https://schema.org/scheduledPaymentDate>
	pub r#scheduled_payment_date: Vec<ScheduledPaymentDateProperty>,
	/// <https://schema.org/totalPaymentDue>
	pub r#total_payment_due: Vec<TotalPaymentDueProperty>,
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
/// This trait is for properties from <https://schema.org/Invoice>.
pub trait InvoiceTrait {
	/// Get <https://schema.org/accountId> from [`Self`] as borrowed slice.
	fn get_account_id(&self) -> &[AccountIdProperty];
	/// Take <https://schema.org/accountId> from [`Self`] as owned vector.
	fn take_account_id(&mut self) -> Vec<AccountIdProperty>;
	/// Get <https://schema.org/billingPeriod> from [`Self`] as borrowed slice.
	fn get_billing_period(&self) -> &[BillingPeriodProperty];
	/// Take <https://schema.org/billingPeriod> from [`Self`] as owned vector.
	fn take_billing_period(&mut self) -> Vec<BillingPeriodProperty>;
	/// Get <https://schema.org/broker> from [`Self`] as borrowed slice.
	fn get_broker(&self) -> &[BrokerProperty];
	/// Take <https://schema.org/broker> from [`Self`] as owned vector.
	fn take_broker(&mut self) -> Vec<BrokerProperty>;
	/// Get <https://schema.org/category> from [`Self`] as borrowed slice.
	fn get_category(&self) -> &[CategoryProperty];
	/// Take <https://schema.org/category> from [`Self`] as owned vector.
	fn take_category(&mut self) -> Vec<CategoryProperty>;
	/// Get <https://schema.org/confirmationNumber> from [`Self`] as borrowed slice.
	fn get_confirmation_number(&self) -> &[ConfirmationNumberProperty];
	/// Take <https://schema.org/confirmationNumber> from [`Self`] as owned vector.
	fn take_confirmation_number(&mut self) -> Vec<ConfirmationNumberProperty>;
	/// Get <https://schema.org/customer> from [`Self`] as borrowed slice.
	fn get_customer(&self) -> &[CustomerProperty];
	/// Take <https://schema.org/customer> from [`Self`] as owned vector.
	fn take_customer(&mut self) -> Vec<CustomerProperty>;
	/// Get <https://schema.org/minimumPaymentDue> from [`Self`] as borrowed slice.
	fn get_minimum_payment_due(&self) -> &[MinimumPaymentDueProperty];
	/// Take <https://schema.org/minimumPaymentDue> from [`Self`] as owned vector.
	fn take_minimum_payment_due(&mut self) -> Vec<MinimumPaymentDueProperty>;
	/// Get <https://schema.org/paymentDue> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/paymentDueDate>."]
	fn get_payment_due(&self) -> &[PaymentDueProperty];
	/// Take <https://schema.org/paymentDue> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/paymentDueDate>."]
	fn take_payment_due(&mut self) -> Vec<PaymentDueProperty>;
	/// Get <https://schema.org/paymentDueDate> from [`Self`] as borrowed slice.
	fn get_payment_due_date(&self) -> &[PaymentDueDateProperty];
	/// Take <https://schema.org/paymentDueDate> from [`Self`] as owned vector.
	fn take_payment_due_date(&mut self) -> Vec<PaymentDueDateProperty>;
	/// Get <https://schema.org/paymentMethod> from [`Self`] as borrowed slice.
	fn get_payment_method(&self) -> &[PaymentMethodProperty];
	/// Take <https://schema.org/paymentMethod> from [`Self`] as owned vector.
	fn take_payment_method(&mut self) -> Vec<PaymentMethodProperty>;
	/// Get <https://schema.org/paymentMethodId> from [`Self`] as borrowed slice.
	fn get_payment_method_id(&self) -> &[PaymentMethodIdProperty];
	/// Take <https://schema.org/paymentMethodId> from [`Self`] as owned vector.
	fn take_payment_method_id(&mut self) -> Vec<PaymentMethodIdProperty>;
	/// Get <https://schema.org/paymentStatus> from [`Self`] as borrowed slice.
	fn get_payment_status(&self) -> &[PaymentStatusProperty];
	/// Take <https://schema.org/paymentStatus> from [`Self`] as owned vector.
	fn take_payment_status(&mut self) -> Vec<PaymentStatusProperty>;
	/// Get <https://schema.org/provider> from [`Self`] as borrowed slice.
	fn get_provider(&self) -> &[ProviderProperty];
	/// Take <https://schema.org/provider> from [`Self`] as owned vector.
	fn take_provider(&mut self) -> Vec<ProviderProperty>;
	/// Get <https://schema.org/referencesOrder> from [`Self`] as borrowed slice.
	fn get_references_order(&self) -> &[ReferencesOrderProperty];
	/// Take <https://schema.org/referencesOrder> from [`Self`] as owned vector.
	fn take_references_order(&mut self) -> Vec<ReferencesOrderProperty>;
	/// Get <https://schema.org/scheduledPaymentDate> from [`Self`] as borrowed slice.
	fn get_scheduled_payment_date(&self) -> &[ScheduledPaymentDateProperty];
	/// Take <https://schema.org/scheduledPaymentDate> from [`Self`] as owned vector.
	fn take_scheduled_payment_date(&mut self) -> Vec<ScheduledPaymentDateProperty>;
	/// Get <https://schema.org/totalPaymentDue> from [`Self`] as borrowed slice.
	fn get_total_payment_due(&self) -> &[TotalPaymentDueProperty];
	/// Take <https://schema.org/totalPaymentDue> from [`Self`] as owned vector.
	fn take_total_payment_due(&mut self) -> Vec<TotalPaymentDueProperty>;
}
impl InvoiceTrait for Invoice {
	fn get_account_id(&self) -> &[AccountIdProperty] {
		self.r#account_id.as_slice()
	}
	fn take_account_id(&mut self) -> Vec<AccountIdProperty> {
		std::mem::take(&mut self.r#account_id)
	}
	fn get_billing_period(&self) -> &[BillingPeriodProperty] {
		self.r#billing_period.as_slice()
	}
	fn take_billing_period(&mut self) -> Vec<BillingPeriodProperty> {
		std::mem::take(&mut self.r#billing_period)
	}
	fn get_broker(&self) -> &[BrokerProperty] {
		self.r#broker.as_slice()
	}
	fn take_broker(&mut self) -> Vec<BrokerProperty> {
		std::mem::take(&mut self.r#broker)
	}
	fn get_category(&self) -> &[CategoryProperty] {
		self.r#category.as_slice()
	}
	fn take_category(&mut self) -> Vec<CategoryProperty> {
		std::mem::take(&mut self.r#category)
	}
	fn get_confirmation_number(&self) -> &[ConfirmationNumberProperty] {
		self.r#confirmation_number.as_slice()
	}
	fn take_confirmation_number(&mut self) -> Vec<ConfirmationNumberProperty> {
		std::mem::take(&mut self.r#confirmation_number)
	}
	fn get_customer(&self) -> &[CustomerProperty] {
		self.r#customer.as_slice()
	}
	fn take_customer(&mut self) -> Vec<CustomerProperty> {
		std::mem::take(&mut self.r#customer)
	}
	fn get_minimum_payment_due(&self) -> &[MinimumPaymentDueProperty] {
		self.r#minimum_payment_due.as_slice()
	}
	fn take_minimum_payment_due(&mut self) -> Vec<MinimumPaymentDueProperty> {
		std::mem::take(&mut self.r#minimum_payment_due)
	}
	fn get_payment_due(&self) -> &[PaymentDueProperty] {
		self.r#payment_due.as_slice()
	}
	fn take_payment_due(&mut self) -> Vec<PaymentDueProperty> {
		std::mem::take(&mut self.r#payment_due)
	}
	fn get_payment_due_date(&self) -> &[PaymentDueDateProperty] {
		self.r#payment_due_date.as_slice()
	}
	fn take_payment_due_date(&mut self) -> Vec<PaymentDueDateProperty> {
		std::mem::take(&mut self.r#payment_due_date)
	}
	fn get_payment_method(&self) -> &[PaymentMethodProperty] {
		self.r#payment_method.as_slice()
	}
	fn take_payment_method(&mut self) -> Vec<PaymentMethodProperty> {
		std::mem::take(&mut self.r#payment_method)
	}
	fn get_payment_method_id(&self) -> &[PaymentMethodIdProperty] {
		self.r#payment_method_id.as_slice()
	}
	fn take_payment_method_id(&mut self) -> Vec<PaymentMethodIdProperty> {
		std::mem::take(&mut self.r#payment_method_id)
	}
	fn get_payment_status(&self) -> &[PaymentStatusProperty] {
		self.r#payment_status.as_slice()
	}
	fn take_payment_status(&mut self) -> Vec<PaymentStatusProperty> {
		std::mem::take(&mut self.r#payment_status)
	}
	fn get_provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn take_provider(&mut self) -> Vec<ProviderProperty> {
		std::mem::take(&mut self.r#provider)
	}
	fn get_references_order(&self) -> &[ReferencesOrderProperty] {
		self.r#references_order.as_slice()
	}
	fn take_references_order(&mut self) -> Vec<ReferencesOrderProperty> {
		std::mem::take(&mut self.r#references_order)
	}
	fn get_scheduled_payment_date(&self) -> &[ScheduledPaymentDateProperty] {
		self.r#scheduled_payment_date.as_slice()
	}
	fn take_scheduled_payment_date(&mut self) -> Vec<ScheduledPaymentDateProperty> {
		std::mem::take(&mut self.r#scheduled_payment_date)
	}
	fn get_total_payment_due(&self) -> &[TotalPaymentDueProperty] {
		self.r#total_payment_due.as_slice()
	}
	fn take_total_payment_due(&mut self) -> Vec<TotalPaymentDueProperty> {
		std::mem::take(&mut self.r#total_payment_due)
	}
}
impl ThingTrait for Invoice {
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
	impl Serialize for Invoice {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#account_id) as usize,
				!Vec::is_empty(&self.r#billing_period) as usize,
				!Vec::is_empty(&self.r#broker) as usize,
				!Vec::is_empty(&self.r#category) as usize,
				!Vec::is_empty(&self.r#confirmation_number) as usize,
				!Vec::is_empty(&self.r#customer) as usize,
				!Vec::is_empty(&self.r#minimum_payment_due) as usize,
				!Vec::is_empty(&self.r#payment_due) as usize,
				!Vec::is_empty(&self.r#payment_due_date) as usize,
				!Vec::is_empty(&self.r#payment_method) as usize,
				!Vec::is_empty(&self.r#payment_method_id) as usize,
				!Vec::is_empty(&self.r#payment_status) as usize,
				!Vec::is_empty(&self.r#provider) as usize,
				!Vec::is_empty(&self.r#references_order) as usize,
				!Vec::is_empty(&self.r#scheduled_payment_date) as usize,
				!Vec::is_empty(&self.r#total_payment_due) as usize,
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
			let mut serialize_struct = Serializer::serialize_struct(serializer, "Invoice", len)?;
			if !Vec::is_empty(&self.r#account_id) {
				serialize_struct.serialize_field("accountId", {
					struct SerializeWith<'a>(&'a Vec<AccountIdProperty>);
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
					&SerializeWith(&self.r#account_id)
				})?;
			} else {
				serialize_struct.skip_field("accountId")?;
			}
			if !Vec::is_empty(&self.r#billing_period) {
				serialize_struct.serialize_field("billingPeriod", {
					struct SerializeWith<'a>(&'a Vec<BillingPeriodProperty>);
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
					&SerializeWith(&self.r#billing_period)
				})?;
			} else {
				serialize_struct.skip_field("billingPeriod")?;
			}
			if !Vec::is_empty(&self.r#broker) {
				serialize_struct.serialize_field("broker", {
					struct SerializeWith<'a>(&'a Vec<BrokerProperty>);
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
					&SerializeWith(&self.r#broker)
				})?;
			} else {
				serialize_struct.skip_field("broker")?;
			}
			if !Vec::is_empty(&self.r#category) {
				serialize_struct.serialize_field("category", {
					struct SerializeWith<'a>(&'a Vec<CategoryProperty>);
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
					&SerializeWith(&self.r#category)
				})?;
			} else {
				serialize_struct.skip_field("category")?;
			}
			if !Vec::is_empty(&self.r#confirmation_number) {
				serialize_struct.serialize_field("confirmationNumber", {
					struct SerializeWith<'a>(&'a Vec<ConfirmationNumberProperty>);
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
					&SerializeWith(&self.r#confirmation_number)
				})?;
			} else {
				serialize_struct.skip_field("confirmationNumber")?;
			}
			if !Vec::is_empty(&self.r#customer) {
				serialize_struct.serialize_field("customer", {
					struct SerializeWith<'a>(&'a Vec<CustomerProperty>);
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
					&SerializeWith(&self.r#customer)
				})?;
			} else {
				serialize_struct.skip_field("customer")?;
			}
			if !Vec::is_empty(&self.r#minimum_payment_due) {
				serialize_struct.serialize_field("minimumPaymentDue", {
					struct SerializeWith<'a>(&'a Vec<MinimumPaymentDueProperty>);
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
					&SerializeWith(&self.r#minimum_payment_due)
				})?;
			} else {
				serialize_struct.skip_field("minimumPaymentDue")?;
			}
			if !Vec::is_empty(&self.r#payment_due) {
				serialize_struct.serialize_field("paymentDue", {
					struct SerializeWith<'a>(&'a Vec<PaymentDueProperty>);
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
					&SerializeWith(&self.r#payment_due)
				})?;
			} else {
				serialize_struct.skip_field("paymentDue")?;
			}
			if !Vec::is_empty(&self.r#payment_due_date) {
				serialize_struct.serialize_field("paymentDueDate", {
					struct SerializeWith<'a>(&'a Vec<PaymentDueDateProperty>);
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
					&SerializeWith(&self.r#payment_due_date)
				})?;
			} else {
				serialize_struct.skip_field("paymentDueDate")?;
			}
			if !Vec::is_empty(&self.r#payment_method) {
				serialize_struct.serialize_field("paymentMethod", {
					struct SerializeWith<'a>(&'a Vec<PaymentMethodProperty>);
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
					&SerializeWith(&self.r#payment_method)
				})?;
			} else {
				serialize_struct.skip_field("paymentMethod")?;
			}
			if !Vec::is_empty(&self.r#payment_method_id) {
				serialize_struct.serialize_field("paymentMethodId", {
					struct SerializeWith<'a>(&'a Vec<PaymentMethodIdProperty>);
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
					&SerializeWith(&self.r#payment_method_id)
				})?;
			} else {
				serialize_struct.skip_field("paymentMethodId")?;
			}
			if !Vec::is_empty(&self.r#payment_status) {
				serialize_struct.serialize_field("paymentStatus", {
					struct SerializeWith<'a>(&'a Vec<PaymentStatusProperty>);
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
					&SerializeWith(&self.r#payment_status)
				})?;
			} else {
				serialize_struct.skip_field("paymentStatus")?;
			}
			if !Vec::is_empty(&self.r#provider) {
				serialize_struct.serialize_field("provider", {
					struct SerializeWith<'a>(&'a Vec<ProviderProperty>);
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
					&SerializeWith(&self.r#provider)
				})?;
			} else {
				serialize_struct.skip_field("provider")?;
			}
			if !Vec::is_empty(&self.r#references_order) {
				serialize_struct.serialize_field("referencesOrder", {
					struct SerializeWith<'a>(&'a Vec<ReferencesOrderProperty>);
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
					&SerializeWith(&self.r#references_order)
				})?;
			} else {
				serialize_struct.skip_field("referencesOrder")?;
			}
			if !Vec::is_empty(&self.r#scheduled_payment_date) {
				serialize_struct.serialize_field("scheduledPaymentDate", {
					struct SerializeWith<'a>(&'a Vec<ScheduledPaymentDateProperty>);
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
					&SerializeWith(&self.r#scheduled_payment_date)
				})?;
			} else {
				serialize_struct.skip_field("scheduledPaymentDate")?;
			}
			if !Vec::is_empty(&self.r#total_payment_due) {
				serialize_struct.serialize_field("totalPaymentDue", {
					struct SerializeWith<'a>(&'a Vec<TotalPaymentDueProperty>);
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
					&SerializeWith(&self.r#total_payment_due)
				})?;
			} else {
				serialize_struct.skip_field("totalPaymentDue")?;
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
	impl<'de> Deserialize<'de> for Invoice {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AccountId,
				BillingPeriod,
				Broker,
				Category,
				ConfirmationNumber,
				Customer,
				MinimumPaymentDue,
				PaymentDue,
				PaymentDueDate,
				PaymentMethod,
				PaymentMethodId,
				PaymentStatus,
				Provider,
				ReferencesOrder,
				ScheduledPaymentDate,
				TotalPaymentDue,
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
						"accountId" => Ok(Field::AccountId),
						"billingPeriod" => Ok(Field::BillingPeriod),
						"broker" => Ok(Field::Broker),
						"category" => Ok(Field::Category),
						"confirmationNumber" => Ok(Field::ConfirmationNumber),
						"customer" => Ok(Field::Customer),
						"minimumPaymentDue" => Ok(Field::MinimumPaymentDue),
						"paymentDue" => Ok(Field::PaymentDue),
						"paymentDueDate" => Ok(Field::PaymentDueDate),
						"paymentMethod" => Ok(Field::PaymentMethod),
						"paymentMethodId" => Ok(Field::PaymentMethodId),
						"paymentStatus" => Ok(Field::PaymentStatus),
						"provider" => Ok(Field::Provider),
						"referencesOrder" => Ok(Field::ReferencesOrder),
						"scheduledPaymentDate" => Ok(Field::ScheduledPaymentDate),
						"totalPaymentDue" => Ok(Field::TotalPaymentDue),
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
						_ => Err(de::Error::unknown_field(value, FIELDS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"accountId" => Ok(Field::AccountId),
						b"billingPeriod" => Ok(Field::BillingPeriod),
						b"broker" => Ok(Field::Broker),
						b"category" => Ok(Field::Category),
						b"confirmationNumber" => Ok(Field::ConfirmationNumber),
						b"customer" => Ok(Field::Customer),
						b"minimumPaymentDue" => Ok(Field::MinimumPaymentDue),
						b"paymentDue" => Ok(Field::PaymentDue),
						b"paymentDueDate" => Ok(Field::PaymentDueDate),
						b"paymentMethod" => Ok(Field::PaymentMethod),
						b"paymentMethodId" => Ok(Field::PaymentMethodId),
						b"paymentStatus" => Ok(Field::PaymentStatus),
						b"provider" => Ok(Field::Provider),
						b"referencesOrder" => Ok(Field::ReferencesOrder),
						b"scheduledPaymentDate" => Ok(Field::ScheduledPaymentDate),
						b"totalPaymentDue" => Ok(Field::TotalPaymentDue),
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
						_ => {
							let value = &String::from_utf8_lossy(value);
							Err(de::Error::unknown_field(value, FIELDS))
						}
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
				type Value = Invoice;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Invoice")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#account_id_property = None;
					let mut r#billing_period_property = None;
					let mut r#broker_property = None;
					let mut r#category_property = None;
					let mut r#confirmation_number_property = None;
					let mut r#customer_property = None;
					let mut r#minimum_payment_due_property = None;
					let mut r#payment_due_property = None;
					let mut r#payment_due_date_property = None;
					let mut r#payment_method_property = None;
					let mut r#payment_method_id_property = None;
					let mut r#payment_status_property = None;
					let mut r#provider_property = None;
					let mut r#references_order_property = None;
					let mut r#scheduled_payment_date_property = None;
					let mut r#total_payment_due_property = None;
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
							Field::AccountId => {
								if r#account_id_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"accountId",
									));
								}
								r#account_id_property = Some({
									struct DeserializeWith(Vec<AccountIdProperty>);
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
							Field::BillingPeriod => {
								if r#billing_period_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"billingPeriod",
									));
								}
								r#billing_period_property = Some({
									struct DeserializeWith(Vec<BillingPeriodProperty>);
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
							Field::Broker => {
								if r#broker_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("broker"));
								}
								r#broker_property = Some({
									struct DeserializeWith(Vec<BrokerProperty>);
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
							Field::Category => {
								if r#category_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"category",
									));
								}
								r#category_property = Some({
									struct DeserializeWith(Vec<CategoryProperty>);
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
							Field::ConfirmationNumber => {
								if r#confirmation_number_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"confirmationNumber",
									));
								}
								r#confirmation_number_property = Some({
									struct DeserializeWith(Vec<ConfirmationNumberProperty>);
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
							Field::Customer => {
								if r#customer_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"customer",
									));
								}
								r#customer_property = Some({
									struct DeserializeWith(Vec<CustomerProperty>);
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
							Field::MinimumPaymentDue => {
								if r#minimum_payment_due_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"minimumPaymentDue",
									));
								}
								r#minimum_payment_due_property = Some({
									struct DeserializeWith(Vec<MinimumPaymentDueProperty>);
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
							Field::PaymentDue => {
								if r#payment_due_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"paymentDue",
									));
								}
								r#payment_due_property = Some({
									struct DeserializeWith(Vec<PaymentDueProperty>);
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
							Field::PaymentDueDate => {
								if r#payment_due_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"paymentDueDate",
									));
								}
								r#payment_due_date_property = Some({
									struct DeserializeWith(Vec<PaymentDueDateProperty>);
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
							Field::PaymentMethod => {
								if r#payment_method_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"paymentMethod",
									));
								}
								r#payment_method_property = Some({
									struct DeserializeWith(Vec<PaymentMethodProperty>);
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
							Field::PaymentMethodId => {
								if r#payment_method_id_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"paymentMethodId",
									));
								}
								r#payment_method_id_property = Some({
									struct DeserializeWith(Vec<PaymentMethodIdProperty>);
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
							Field::PaymentStatus => {
								if r#payment_status_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"paymentStatus",
									));
								}
								r#payment_status_property = Some({
									struct DeserializeWith(Vec<PaymentStatusProperty>);
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
							Field::Provider => {
								if r#provider_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"provider",
									));
								}
								r#provider_property = Some({
									struct DeserializeWith(Vec<ProviderProperty>);
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
							Field::ReferencesOrder => {
								if r#references_order_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"referencesOrder",
									));
								}
								r#references_order_property = Some({
									struct DeserializeWith(Vec<ReferencesOrderProperty>);
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
							Field::ScheduledPaymentDate => {
								if r#scheduled_payment_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"scheduledPaymentDate",
									));
								}
								r#scheduled_payment_date_property = Some({
									struct DeserializeWith(Vec<ScheduledPaymentDateProperty>);
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
							Field::TotalPaymentDue => {
								if r#total_payment_due_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"totalPaymentDue",
									));
								}
								r#total_payment_due_property = Some({
									struct DeserializeWith(Vec<TotalPaymentDueProperty>);
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
						}
					}
					Ok(Invoice {
						r#account_id: r#account_id_property.unwrap_or_default(),
						r#billing_period: r#billing_period_property.unwrap_or_default(),
						r#broker: r#broker_property.unwrap_or_default(),
						r#category: r#category_property.unwrap_or_default(),
						r#confirmation_number: r#confirmation_number_property.unwrap_or_default(),
						r#customer: r#customer_property.unwrap_or_default(),
						r#minimum_payment_due: r#minimum_payment_due_property.unwrap_or_default(),
						r#payment_due: r#payment_due_property.unwrap_or_default(),
						r#payment_due_date: r#payment_due_date_property.unwrap_or_default(),
						r#payment_method: r#payment_method_property.unwrap_or_default(),
						r#payment_method_id: r#payment_method_id_property.unwrap_or_default(),
						r#payment_status: r#payment_status_property.unwrap_or_default(),
						r#provider: r#provider_property.unwrap_or_default(),
						r#references_order: r#references_order_property.unwrap_or_default(),
						r#scheduled_payment_date: r#scheduled_payment_date_property
							.unwrap_or_default(),
						r#total_payment_due: r#total_payment_due_property.unwrap_or_default(),
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
				"accountId",
				"billingPeriod",
				"broker",
				"category",
				"confirmationNumber",
				"customer",
				"minimumPaymentDue",
				"paymentDue",
				"paymentDueDate",
				"paymentMethod",
				"paymentMethodId",
				"paymentStatus",
				"provider",
				"referencesOrder",
				"scheduledPaymentDate",
				"totalPaymentDue",
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
			deserializer.deserialize_struct("Invoice", FIELDS, ClassVisitor)
		}
	}
}
