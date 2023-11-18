use super::*;
/// <https://schema.org/Order>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Order {
	pub r#accepted_offer: Vec<AcceptedOfferProperty>,
	pub r#billing_address: Vec<BillingAddressProperty>,
	pub r#broker: Vec<BrokerProperty>,
	pub r#confirmation_number: Vec<ConfirmationNumberProperty>,
	pub r#customer: Vec<CustomerProperty>,
	pub r#discount: Vec<DiscountProperty>,
	pub r#discount_code: Vec<DiscountCodeProperty>,
	pub r#discount_currency: Vec<DiscountCurrencyProperty>,
	pub r#is_gift: Vec<IsGiftProperty>,
	pub r#merchant: Vec<MerchantProperty>,
	pub r#order_date: Vec<OrderDateProperty>,
	pub r#order_delivery: Vec<OrderDeliveryProperty>,
	pub r#order_number: Vec<OrderNumberProperty>,
	pub r#order_status: Vec<OrderStatusProperty>,
	pub r#ordered_item: Vec<OrderedItemProperty>,
	pub r#part_of_invoice: Vec<PartOfInvoiceProperty>,
	pub r#payment_due: Vec<PaymentDueProperty>,
	pub r#payment_due_date: Vec<PaymentDueDateProperty>,
	pub r#payment_method: Vec<PaymentMethodProperty>,
	pub r#payment_method_id: Vec<PaymentMethodIdProperty>,
	pub r#payment_url: Vec<PaymentUrlProperty>,
	pub r#seller: Vec<SellerProperty>,
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	pub r#alternate_name: Vec<AlternateNameProperty>,
	pub r#description: Vec<DescriptionProperty>,
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	pub r#identifier: Vec<IdentifierProperty>,
	pub r#image: Vec<ImageProperty>,
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	pub r#name: Vec<NameProperty>,
	pub r#potential_action: Vec<PotentialActionProperty>,
	pub r#same_as: Vec<SameAsProperty>,
	pub r#subject_of: Vec<SubjectOfProperty>,
	pub r#url: Vec<UrlProperty>,
}
pub trait OrderTrait {
	fn get_accepted_offer(&self) -> &[AcceptedOfferProperty];
	fn take_accepted_offer(&mut self) -> Vec<AcceptedOfferProperty>;
	fn get_billing_address(&self) -> &[BillingAddressProperty];
	fn take_billing_address(&mut self) -> Vec<BillingAddressProperty>;
	fn get_broker(&self) -> &[BrokerProperty];
	fn take_broker(&mut self) -> Vec<BrokerProperty>;
	fn get_confirmation_number(&self) -> &[ConfirmationNumberProperty];
	fn take_confirmation_number(&mut self) -> Vec<ConfirmationNumberProperty>;
	fn get_customer(&self) -> &[CustomerProperty];
	fn take_customer(&mut self) -> Vec<CustomerProperty>;
	fn get_discount(&self) -> &[DiscountProperty];
	fn take_discount(&mut self) -> Vec<DiscountProperty>;
	fn get_discount_code(&self) -> &[DiscountCodeProperty];
	fn take_discount_code(&mut self) -> Vec<DiscountCodeProperty>;
	fn get_discount_currency(&self) -> &[DiscountCurrencyProperty];
	fn take_discount_currency(&mut self) -> Vec<DiscountCurrencyProperty>;
	fn get_is_gift(&self) -> &[IsGiftProperty];
	fn take_is_gift(&mut self) -> Vec<IsGiftProperty>;
	fn get_merchant(&self) -> &[MerchantProperty];
	fn take_merchant(&mut self) -> Vec<MerchantProperty>;
	fn get_order_date(&self) -> &[OrderDateProperty];
	fn take_order_date(&mut self) -> Vec<OrderDateProperty>;
	fn get_order_delivery(&self) -> &[OrderDeliveryProperty];
	fn take_order_delivery(&mut self) -> Vec<OrderDeliveryProperty>;
	fn get_order_number(&self) -> &[OrderNumberProperty];
	fn take_order_number(&mut self) -> Vec<OrderNumberProperty>;
	fn get_order_status(&self) -> &[OrderStatusProperty];
	fn take_order_status(&mut self) -> Vec<OrderStatusProperty>;
	fn get_ordered_item(&self) -> &[OrderedItemProperty];
	fn take_ordered_item(&mut self) -> Vec<OrderedItemProperty>;
	fn get_part_of_invoice(&self) -> &[PartOfInvoiceProperty];
	fn take_part_of_invoice(&mut self) -> Vec<PartOfInvoiceProperty>;
	fn get_payment_due(&self) -> &[PaymentDueProperty];
	fn take_payment_due(&mut self) -> Vec<PaymentDueProperty>;
	fn get_payment_due_date(&self) -> &[PaymentDueDateProperty];
	fn take_payment_due_date(&mut self) -> Vec<PaymentDueDateProperty>;
	fn get_payment_method(&self) -> &[PaymentMethodProperty];
	fn take_payment_method(&mut self) -> Vec<PaymentMethodProperty>;
	fn get_payment_method_id(&self) -> &[PaymentMethodIdProperty];
	fn take_payment_method_id(&mut self) -> Vec<PaymentMethodIdProperty>;
	fn get_payment_url(&self) -> &[PaymentUrlProperty];
	fn take_payment_url(&mut self) -> Vec<PaymentUrlProperty>;
	fn get_seller(&self) -> &[SellerProperty];
	fn take_seller(&mut self) -> Vec<SellerProperty>;
}
impl OrderTrait for Order {
	fn get_accepted_offer(&self) -> &[AcceptedOfferProperty] {
		self.r#accepted_offer.as_slice()
	}
	fn take_accepted_offer(&mut self) -> Vec<AcceptedOfferProperty> {
		std::mem::take(&mut self.r#accepted_offer)
	}
	fn get_billing_address(&self) -> &[BillingAddressProperty] {
		self.r#billing_address.as_slice()
	}
	fn take_billing_address(&mut self) -> Vec<BillingAddressProperty> {
		std::mem::take(&mut self.r#billing_address)
	}
	fn get_broker(&self) -> &[BrokerProperty] {
		self.r#broker.as_slice()
	}
	fn take_broker(&mut self) -> Vec<BrokerProperty> {
		std::mem::take(&mut self.r#broker)
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
	fn get_discount(&self) -> &[DiscountProperty] {
		self.r#discount.as_slice()
	}
	fn take_discount(&mut self) -> Vec<DiscountProperty> {
		std::mem::take(&mut self.r#discount)
	}
	fn get_discount_code(&self) -> &[DiscountCodeProperty] {
		self.r#discount_code.as_slice()
	}
	fn take_discount_code(&mut self) -> Vec<DiscountCodeProperty> {
		std::mem::take(&mut self.r#discount_code)
	}
	fn get_discount_currency(&self) -> &[DiscountCurrencyProperty] {
		self.r#discount_currency.as_slice()
	}
	fn take_discount_currency(&mut self) -> Vec<DiscountCurrencyProperty> {
		std::mem::take(&mut self.r#discount_currency)
	}
	fn get_is_gift(&self) -> &[IsGiftProperty] {
		self.r#is_gift.as_slice()
	}
	fn take_is_gift(&mut self) -> Vec<IsGiftProperty> {
		std::mem::take(&mut self.r#is_gift)
	}
	fn get_merchant(&self) -> &[MerchantProperty] {
		self.r#merchant.as_slice()
	}
	fn take_merchant(&mut self) -> Vec<MerchantProperty> {
		std::mem::take(&mut self.r#merchant)
	}
	fn get_order_date(&self) -> &[OrderDateProperty] {
		self.r#order_date.as_slice()
	}
	fn take_order_date(&mut self) -> Vec<OrderDateProperty> {
		std::mem::take(&mut self.r#order_date)
	}
	fn get_order_delivery(&self) -> &[OrderDeliveryProperty] {
		self.r#order_delivery.as_slice()
	}
	fn take_order_delivery(&mut self) -> Vec<OrderDeliveryProperty> {
		std::mem::take(&mut self.r#order_delivery)
	}
	fn get_order_number(&self) -> &[OrderNumberProperty] {
		self.r#order_number.as_slice()
	}
	fn take_order_number(&mut self) -> Vec<OrderNumberProperty> {
		std::mem::take(&mut self.r#order_number)
	}
	fn get_order_status(&self) -> &[OrderStatusProperty] {
		self.r#order_status.as_slice()
	}
	fn take_order_status(&mut self) -> Vec<OrderStatusProperty> {
		std::mem::take(&mut self.r#order_status)
	}
	fn get_ordered_item(&self) -> &[OrderedItemProperty] {
		self.r#ordered_item.as_slice()
	}
	fn take_ordered_item(&mut self) -> Vec<OrderedItemProperty> {
		std::mem::take(&mut self.r#ordered_item)
	}
	fn get_part_of_invoice(&self) -> &[PartOfInvoiceProperty] {
		self.r#part_of_invoice.as_slice()
	}
	fn take_part_of_invoice(&mut self) -> Vec<PartOfInvoiceProperty> {
		std::mem::take(&mut self.r#part_of_invoice)
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
	fn get_payment_url(&self) -> &[PaymentUrlProperty] {
		self.r#payment_url.as_slice()
	}
	fn take_payment_url(&mut self) -> Vec<PaymentUrlProperty> {
		std::mem::take(&mut self.r#payment_url)
	}
	fn get_seller(&self) -> &[SellerProperty] {
		self.r#seller.as_slice()
	}
	fn take_seller(&mut self) -> Vec<SellerProperty> {
		std::mem::take(&mut self.r#seller)
	}
}
impl ThingTrait for Order {
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
	impl Serialize for Order {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#accepted_offer) as usize,
				!Vec::is_empty(&self.r#billing_address) as usize,
				!Vec::is_empty(&self.r#broker) as usize,
				!Vec::is_empty(&self.r#confirmation_number) as usize,
				!Vec::is_empty(&self.r#customer) as usize,
				!Vec::is_empty(&self.r#discount) as usize,
				!Vec::is_empty(&self.r#discount_code) as usize,
				!Vec::is_empty(&self.r#discount_currency) as usize,
				!Vec::is_empty(&self.r#is_gift) as usize,
				!Vec::is_empty(&self.r#merchant) as usize,
				!Vec::is_empty(&self.r#order_date) as usize,
				!Vec::is_empty(&self.r#order_delivery) as usize,
				!Vec::is_empty(&self.r#order_number) as usize,
				!Vec::is_empty(&self.r#order_status) as usize,
				!Vec::is_empty(&self.r#ordered_item) as usize,
				!Vec::is_empty(&self.r#part_of_invoice) as usize,
				!Vec::is_empty(&self.r#payment_due) as usize,
				!Vec::is_empty(&self.r#payment_due_date) as usize,
				!Vec::is_empty(&self.r#payment_method) as usize,
				!Vec::is_empty(&self.r#payment_method_id) as usize,
				!Vec::is_empty(&self.r#payment_url) as usize,
				!Vec::is_empty(&self.r#seller) as usize,
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
			let mut serialize_struct = Serializer::serialize_struct(serializer, "Order", len)?;
			if !Vec::is_empty(&self.r#accepted_offer) {
				serialize_struct.serialize_field("acceptedOffer", {
					struct SerializeWith<'a>(&'a Vec<AcceptedOfferProperty>);
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
					&SerializeWith(&self.r#accepted_offer)
				})?;
			} else {
				serialize_struct.skip_field("acceptedOffer")?;
			}
			if !Vec::is_empty(&self.r#billing_address) {
				serialize_struct.serialize_field("billingAddress", {
					struct SerializeWith<'a>(&'a Vec<BillingAddressProperty>);
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
					&SerializeWith(&self.r#billing_address)
				})?;
			} else {
				serialize_struct.skip_field("billingAddress")?;
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
			if !Vec::is_empty(&self.r#discount) {
				serialize_struct.serialize_field("discount", {
					struct SerializeWith<'a>(&'a Vec<DiscountProperty>);
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
					&SerializeWith(&self.r#discount)
				})?;
			} else {
				serialize_struct.skip_field("discount")?;
			}
			if !Vec::is_empty(&self.r#discount_code) {
				serialize_struct.serialize_field("discountCode", {
					struct SerializeWith<'a>(&'a Vec<DiscountCodeProperty>);
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
					&SerializeWith(&self.r#discount_code)
				})?;
			} else {
				serialize_struct.skip_field("discountCode")?;
			}
			if !Vec::is_empty(&self.r#discount_currency) {
				serialize_struct.serialize_field("discountCurrency", {
					struct SerializeWith<'a>(&'a Vec<DiscountCurrencyProperty>);
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
					&SerializeWith(&self.r#discount_currency)
				})?;
			} else {
				serialize_struct.skip_field("discountCurrency")?;
			}
			if !Vec::is_empty(&self.r#is_gift) {
				serialize_struct.serialize_field("isGift", {
					struct SerializeWith<'a>(&'a Vec<IsGiftProperty>);
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
					&SerializeWith(&self.r#is_gift)
				})?;
			} else {
				serialize_struct.skip_field("isGift")?;
			}
			if !Vec::is_empty(&self.r#merchant) {
				serialize_struct.serialize_field("merchant", {
					struct SerializeWith<'a>(&'a Vec<MerchantProperty>);
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
					&SerializeWith(&self.r#merchant)
				})?;
			} else {
				serialize_struct.skip_field("merchant")?;
			}
			if !Vec::is_empty(&self.r#order_date) {
				serialize_struct.serialize_field("orderDate", {
					struct SerializeWith<'a>(&'a Vec<OrderDateProperty>);
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
					&SerializeWith(&self.r#order_date)
				})?;
			} else {
				serialize_struct.skip_field("orderDate")?;
			}
			if !Vec::is_empty(&self.r#order_delivery) {
				serialize_struct.serialize_field("orderDelivery", {
					struct SerializeWith<'a>(&'a Vec<OrderDeliveryProperty>);
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
					&SerializeWith(&self.r#order_delivery)
				})?;
			} else {
				serialize_struct.skip_field("orderDelivery")?;
			}
			if !Vec::is_empty(&self.r#order_number) {
				serialize_struct.serialize_field("orderNumber", {
					struct SerializeWith<'a>(&'a Vec<OrderNumberProperty>);
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
					&SerializeWith(&self.r#order_number)
				})?;
			} else {
				serialize_struct.skip_field("orderNumber")?;
			}
			if !Vec::is_empty(&self.r#order_status) {
				serialize_struct.serialize_field("orderStatus", {
					struct SerializeWith<'a>(&'a Vec<OrderStatusProperty>);
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
					&SerializeWith(&self.r#order_status)
				})?;
			} else {
				serialize_struct.skip_field("orderStatus")?;
			}
			if !Vec::is_empty(&self.r#ordered_item) {
				serialize_struct.serialize_field("orderedItem", {
					struct SerializeWith<'a>(&'a Vec<OrderedItemProperty>);
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
					&SerializeWith(&self.r#ordered_item)
				})?;
			} else {
				serialize_struct.skip_field("orderedItem")?;
			}
			if !Vec::is_empty(&self.r#part_of_invoice) {
				serialize_struct.serialize_field("partOfInvoice", {
					struct SerializeWith<'a>(&'a Vec<PartOfInvoiceProperty>);
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
					&SerializeWith(&self.r#part_of_invoice)
				})?;
			} else {
				serialize_struct.skip_field("partOfInvoice")?;
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
			if !Vec::is_empty(&self.r#payment_url) {
				serialize_struct.serialize_field("paymentUrl", {
					struct SerializeWith<'a>(&'a Vec<PaymentUrlProperty>);
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
					&SerializeWith(&self.r#payment_url)
				})?;
			} else {
				serialize_struct.skip_field("paymentUrl")?;
			}
			if !Vec::is_empty(&self.r#seller) {
				serialize_struct.serialize_field("seller", {
					struct SerializeWith<'a>(&'a Vec<SellerProperty>);
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
					&SerializeWith(&self.r#seller)
				})?;
			} else {
				serialize_struct.skip_field("seller")?;
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
	impl<'de> Deserialize<'de> for Order {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AcceptedOffer,
				BillingAddress,
				Broker,
				ConfirmationNumber,
				Customer,
				Discount,
				DiscountCode,
				DiscountCurrency,
				IsGift,
				Merchant,
				OrderDate,
				OrderDelivery,
				OrderNumber,
				OrderStatus,
				OrderedItem,
				PartOfInvoice,
				PaymentDue,
				PaymentDueDate,
				PaymentMethod,
				PaymentMethodId,
				PaymentUrl,
				Seller,
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
						"acceptedOffer" => Ok(Field::AcceptedOffer),
						"billingAddress" => Ok(Field::BillingAddress),
						"broker" => Ok(Field::Broker),
						"confirmationNumber" => Ok(Field::ConfirmationNumber),
						"customer" => Ok(Field::Customer),
						"discount" => Ok(Field::Discount),
						"discountCode" => Ok(Field::DiscountCode),
						"discountCurrency" => Ok(Field::DiscountCurrency),
						"isGift" => Ok(Field::IsGift),
						"merchant" => Ok(Field::Merchant),
						"orderDate" => Ok(Field::OrderDate),
						"orderDelivery" => Ok(Field::OrderDelivery),
						"orderNumber" => Ok(Field::OrderNumber),
						"orderStatus" => Ok(Field::OrderStatus),
						"orderedItem" => Ok(Field::OrderedItem),
						"partOfInvoice" => Ok(Field::PartOfInvoice),
						"paymentDue" => Ok(Field::PaymentDue),
						"paymentDueDate" => Ok(Field::PaymentDueDate),
						"paymentMethod" => Ok(Field::PaymentMethod),
						"paymentMethodId" => Ok(Field::PaymentMethodId),
						"paymentUrl" => Ok(Field::PaymentUrl),
						"seller" => Ok(Field::Seller),
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
						b"acceptedOffer" => Ok(Field::AcceptedOffer),
						b"billingAddress" => Ok(Field::BillingAddress),
						b"broker" => Ok(Field::Broker),
						b"confirmationNumber" => Ok(Field::ConfirmationNumber),
						b"customer" => Ok(Field::Customer),
						b"discount" => Ok(Field::Discount),
						b"discountCode" => Ok(Field::DiscountCode),
						b"discountCurrency" => Ok(Field::DiscountCurrency),
						b"isGift" => Ok(Field::IsGift),
						b"merchant" => Ok(Field::Merchant),
						b"orderDate" => Ok(Field::OrderDate),
						b"orderDelivery" => Ok(Field::OrderDelivery),
						b"orderNumber" => Ok(Field::OrderNumber),
						b"orderStatus" => Ok(Field::OrderStatus),
						b"orderedItem" => Ok(Field::OrderedItem),
						b"partOfInvoice" => Ok(Field::PartOfInvoice),
						b"paymentDue" => Ok(Field::PaymentDue),
						b"paymentDueDate" => Ok(Field::PaymentDueDate),
						b"paymentMethod" => Ok(Field::PaymentMethod),
						b"paymentMethodId" => Ok(Field::PaymentMethodId),
						b"paymentUrl" => Ok(Field::PaymentUrl),
						b"seller" => Ok(Field::Seller),
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
				type Value = Order;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Order")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#accepted_offer_property = None;
					let mut r#billing_address_property = None;
					let mut r#broker_property = None;
					let mut r#confirmation_number_property = None;
					let mut r#customer_property = None;
					let mut r#discount_property = None;
					let mut r#discount_code_property = None;
					let mut r#discount_currency_property = None;
					let mut r#is_gift_property = None;
					let mut r#merchant_property = None;
					let mut r#order_date_property = None;
					let mut r#order_delivery_property = None;
					let mut r#order_number_property = None;
					let mut r#order_status_property = None;
					let mut r#ordered_item_property = None;
					let mut r#part_of_invoice_property = None;
					let mut r#payment_due_property = None;
					let mut r#payment_due_date_property = None;
					let mut r#payment_method_property = None;
					let mut r#payment_method_id_property = None;
					let mut r#payment_url_property = None;
					let mut r#seller_property = None;
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
							Field::AcceptedOffer => {
								if r#accepted_offer_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"acceptedOffer",
									));
								}
								r#accepted_offer_property = Some({
									struct DeserializeWith(Vec<AcceptedOfferProperty>);
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
							Field::BillingAddress => {
								if r#billing_address_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"billingAddress",
									));
								}
								r#billing_address_property = Some({
									struct DeserializeWith(Vec<BillingAddressProperty>);
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
							Field::Discount => {
								if r#discount_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"discount",
									));
								}
								r#discount_property = Some({
									struct DeserializeWith(Vec<DiscountProperty>);
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
							Field::DiscountCode => {
								if r#discount_code_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"discountCode",
									));
								}
								r#discount_code_property = Some({
									struct DeserializeWith(Vec<DiscountCodeProperty>);
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
							Field::DiscountCurrency => {
								if r#discount_currency_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"discountCurrency",
									));
								}
								r#discount_currency_property = Some({
									struct DeserializeWith(Vec<DiscountCurrencyProperty>);
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
							Field::IsGift => {
								if r#is_gift_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("isGift"));
								}
								r#is_gift_property = Some({
									struct DeserializeWith(Vec<IsGiftProperty>);
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
							Field::Merchant => {
								if r#merchant_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"merchant",
									));
								}
								r#merchant_property = Some({
									struct DeserializeWith(Vec<MerchantProperty>);
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
							Field::OrderDate => {
								if r#order_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"orderDate",
									));
								}
								r#order_date_property = Some({
									struct DeserializeWith(Vec<OrderDateProperty>);
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
							Field::OrderDelivery => {
								if r#order_delivery_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"orderDelivery",
									));
								}
								r#order_delivery_property = Some({
									struct DeserializeWith(Vec<OrderDeliveryProperty>);
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
							Field::OrderNumber => {
								if r#order_number_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"orderNumber",
									));
								}
								r#order_number_property = Some({
									struct DeserializeWith(Vec<OrderNumberProperty>);
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
							Field::OrderStatus => {
								if r#order_status_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"orderStatus",
									));
								}
								r#order_status_property = Some({
									struct DeserializeWith(Vec<OrderStatusProperty>);
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
							Field::OrderedItem => {
								if r#ordered_item_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"orderedItem",
									));
								}
								r#ordered_item_property = Some({
									struct DeserializeWith(Vec<OrderedItemProperty>);
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
							Field::PartOfInvoice => {
								if r#part_of_invoice_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"partOfInvoice",
									));
								}
								r#part_of_invoice_property = Some({
									struct DeserializeWith(Vec<PartOfInvoiceProperty>);
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
							Field::PaymentUrl => {
								if r#payment_url_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"paymentUrl",
									));
								}
								r#payment_url_property = Some({
									struct DeserializeWith(Vec<PaymentUrlProperty>);
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
							Field::Seller => {
								if r#seller_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("seller"));
								}
								r#seller_property = Some({
									struct DeserializeWith(Vec<SellerProperty>);
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
					Ok(Order {
						r#accepted_offer: r#accepted_offer_property.unwrap_or_default(),
						r#billing_address: r#billing_address_property.unwrap_or_default(),
						r#broker: r#broker_property.unwrap_or_default(),
						r#confirmation_number: r#confirmation_number_property.unwrap_or_default(),
						r#customer: r#customer_property.unwrap_or_default(),
						r#discount: r#discount_property.unwrap_or_default(),
						r#discount_code: r#discount_code_property.unwrap_or_default(),
						r#discount_currency: r#discount_currency_property.unwrap_or_default(),
						r#is_gift: r#is_gift_property.unwrap_or_default(),
						r#merchant: r#merchant_property.unwrap_or_default(),
						r#order_date: r#order_date_property.unwrap_or_default(),
						r#order_delivery: r#order_delivery_property.unwrap_or_default(),
						r#order_number: r#order_number_property.unwrap_or_default(),
						r#order_status: r#order_status_property.unwrap_or_default(),
						r#ordered_item: r#ordered_item_property.unwrap_or_default(),
						r#part_of_invoice: r#part_of_invoice_property.unwrap_or_default(),
						r#payment_due: r#payment_due_property.unwrap_or_default(),
						r#payment_due_date: r#payment_due_date_property.unwrap_or_default(),
						r#payment_method: r#payment_method_property.unwrap_or_default(),
						r#payment_method_id: r#payment_method_id_property.unwrap_or_default(),
						r#payment_url: r#payment_url_property.unwrap_or_default(),
						r#seller: r#seller_property.unwrap_or_default(),
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
				"acceptedOffer",
				"billingAddress",
				"broker",
				"confirmationNumber",
				"customer",
				"discount",
				"discountCode",
				"discountCurrency",
				"isGift",
				"merchant",
				"orderDate",
				"orderDelivery",
				"orderNumber",
				"orderStatus",
				"orderedItem",
				"partOfInvoice",
				"paymentDue",
				"paymentDueDate",
				"paymentMethod",
				"paymentMethodId",
				"paymentUrl",
				"seller",
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
			deserializer.deserialize_struct("Order", FIELDS, ClassVisitor)
		}
	}
}
