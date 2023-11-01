use super::*;
/// <https://schema.org/Order>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Order {
	#[cfg(any(
		any(
			feature = "accepted-offer-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#accepted_offer: Vec<AcceptedOfferProperty>,
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
			feature = "billing-address-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#billing_address: Vec<BillingAddressProperty>,
	#[cfg(any(
		any(feature = "broker-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#broker: Vec<BrokerProperty>,
	#[cfg(any(
		any(
			feature = "confirmation-number-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#confirmation_number: Vec<ConfirmationNumberProperty>,
	#[cfg(any(
		any(
			feature = "customer-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#customer: Vec<CustomerProperty>,
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
			feature = "discount-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#discount: Vec<DiscountProperty>,
	#[cfg(any(
		any(
			feature = "discount-code-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#discount_code: Vec<DiscountCodeProperty>,
	#[cfg(any(
		any(
			feature = "discount-currency-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#discount_currency: Vec<DiscountCurrencyProperty>,
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
			feature = "is-gift-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_gift: Vec<IsGiftProperty>,
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
			feature = "merchant-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#merchant: Vec<MerchantProperty>,
	#[cfg(any(
		any(feature = "name-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#name: Vec<NameProperty>,
	#[cfg(any(
		any(
			feature = "order-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#order_date: Vec<OrderDateProperty>,
	#[cfg(any(
		any(
			feature = "order-delivery-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#order_delivery: Vec<OrderDeliveryProperty>,
	#[cfg(any(
		any(
			feature = "order-number-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#order_number: Vec<OrderNumberProperty>,
	#[cfg(any(
		any(
			feature = "order-status-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#order_status: Vec<OrderStatusProperty>,
	#[cfg(any(
		any(
			feature = "ordered-item-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#ordered_item: Vec<OrderedItemProperty>,
	#[cfg(any(
		any(
			feature = "part-of-invoice-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#part_of_invoice: Vec<PartOfInvoiceProperty>,
	#[cfg(any(
		any(
			feature = "payment-due-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#payment_due: Vec<PaymentDueProperty>,
	#[cfg(any(
		any(
			feature = "payment-due-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#payment_due_date: Vec<PaymentDueDateProperty>,
	#[cfg(any(
		any(
			feature = "payment-method-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#payment_method: Vec<PaymentMethodProperty>,
	#[cfg(any(
		any(
			feature = "payment-method-id-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#payment_method_id: Vec<PaymentMethodIdProperty>,
	#[cfg(any(
		any(
			feature = "payment-url-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#payment_url: Vec<PaymentUrlProperty>,
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
			feature = "same-as-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#same_as: Vec<SameAsProperty>,
	#[cfg(any(
		any(feature = "seller-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#seller: Vec<SellerProperty>,
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
	impl Serialize for Order {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				if cfg!(any(
					any(
						feature = "accepted-offer-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#accepted_offer) as usize
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
						feature = "billing-address-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#billing_address) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "broker-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#broker) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "confirmation-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#confirmation_number) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "customer-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#customer) as usize
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
						feature = "discount-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#discount) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "discount-code-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#discount_code) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "discount-currency-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#discount_currency) as usize
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
						feature = "is-gift-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_gift) as usize
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
						feature = "merchant-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#merchant) as usize
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
						feature = "order-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#order_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "order-delivery-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#order_delivery) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "order-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#order_number) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "order-status-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#order_status) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "ordered-item-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#ordered_item) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "part-of-invoice-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#part_of_invoice) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "payment-due-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#payment_due) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "payment-due-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#payment_due_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "payment-method-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#payment_method) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "payment-method-id-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#payment_method_id) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "payment-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#payment_url) as usize
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
						feature = "seller-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#seller) as usize
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
			let mut serialize_struct = Serializer::serialize_struct(serializer, "Order", len)?;
			#[cfg(any(
				any(
					feature = "accepted-offer-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "billing-address-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(feature = "broker-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "confirmation-number-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "customer-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "discount-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "discount-code-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "discount-currency-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "is-gift-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "merchant-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "order-date-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "order-delivery-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "order-number-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "order-status-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "ordered-item-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "part-of-invoice-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "payment-due-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "payment-due-date-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "payment-method-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "payment-method-id-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "payment-url-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
				any(feature = "seller-property-schema", feature = "general-schema-section"),
				doc
			))]
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
	impl<'de> Deserialize<'de> for Order {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				#[cfg(any(
					any(
						feature = "accepted-offer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AcceptedOffer,
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
						feature = "billing-address-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				BillingAddress,
				#[cfg(any(
					any(feature = "broker-property-schema", feature = "general-schema-section"),
					doc
				))]
				Broker,
				#[cfg(any(
					any(
						feature = "confirmation-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ConfirmationNumber,
				#[cfg(any(
					any(
						feature = "customer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Customer,
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
						feature = "discount-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Discount,
				#[cfg(any(
					any(
						feature = "discount-code-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DiscountCode,
				#[cfg(any(
					any(
						feature = "discount-currency-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DiscountCurrency,
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
						feature = "is-gift-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsGift,
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
						feature = "merchant-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Merchant,
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				Name,
				#[cfg(any(
					any(
						feature = "order-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				OrderDate,
				#[cfg(any(
					any(
						feature = "order-delivery-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				OrderDelivery,
				#[cfg(any(
					any(
						feature = "order-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				OrderNumber,
				#[cfg(any(
					any(
						feature = "order-status-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				OrderStatus,
				#[cfg(any(
					any(
						feature = "ordered-item-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				OrderedItem,
				#[cfg(any(
					any(
						feature = "part-of-invoice-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PartOfInvoice,
				#[cfg(any(
					any(
						feature = "payment-due-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PaymentDue,
				#[cfg(any(
					any(
						feature = "payment-due-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PaymentDueDate,
				#[cfg(any(
					any(
						feature = "payment-method-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PaymentMethod,
				#[cfg(any(
					any(
						feature = "payment-method-id-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PaymentMethodId,
				#[cfg(any(
					any(
						feature = "payment-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PaymentUrl,
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
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SameAs,
				#[cfg(any(
					any(feature = "seller-property-schema", feature = "general-schema-section"),
					doc
				))]
				Seller,
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
								feature = "accepted-offer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"acceptedOffer" => Ok(Field::AcceptedOffer),
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
								feature = "billing-address-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"billingAddress" => Ok(Field::BillingAddress),
						#[cfg(any(
							any(
								feature = "broker-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"broker" => Ok(Field::Broker),
						#[cfg(any(
							any(
								feature = "confirmation-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"confirmationNumber" => Ok(Field::ConfirmationNumber),
						#[cfg(any(
							any(
								feature = "customer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"customer" => Ok(Field::Customer),
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
								feature = "discount-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"discount" => Ok(Field::Discount),
						#[cfg(any(
							any(
								feature = "discount-code-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"discountCode" => Ok(Field::DiscountCode),
						#[cfg(any(
							any(
								feature = "discount-currency-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"discountCurrency" => Ok(Field::DiscountCurrency),
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
								feature = "is-gift-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isGift" => Ok(Field::IsGift),
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
								feature = "merchant-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"merchant" => Ok(Field::Merchant),
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
								feature = "order-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"orderDate" => Ok(Field::OrderDate),
						#[cfg(any(
							any(
								feature = "order-delivery-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"orderDelivery" => Ok(Field::OrderDelivery),
						#[cfg(any(
							any(
								feature = "order-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"orderNumber" => Ok(Field::OrderNumber),
						#[cfg(any(
							any(
								feature = "order-status-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"orderStatus" => Ok(Field::OrderStatus),
						#[cfg(any(
							any(
								feature = "ordered-item-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"orderedItem" => Ok(Field::OrderedItem),
						#[cfg(any(
							any(
								feature = "part-of-invoice-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"partOfInvoice" => Ok(Field::PartOfInvoice),
						#[cfg(any(
							any(
								feature = "payment-due-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"paymentDue" => Ok(Field::PaymentDue),
						#[cfg(any(
							any(
								feature = "payment-due-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"paymentDueDate" => Ok(Field::PaymentDueDate),
						#[cfg(any(
							any(
								feature = "payment-method-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"paymentMethod" => Ok(Field::PaymentMethod),
						#[cfg(any(
							any(
								feature = "payment-method-id-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"paymentMethodId" => Ok(Field::PaymentMethodId),
						#[cfg(any(
							any(
								feature = "payment-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"paymentUrl" => Ok(Field::PaymentUrl),
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
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"sameAs" => Ok(Field::SameAs),
						#[cfg(any(
							any(
								feature = "seller-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"seller" => Ok(Field::Seller),
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
								feature = "accepted-offer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"acceptedOffer" => Ok(Field::AcceptedOffer),
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
								feature = "billing-address-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"billingAddress" => Ok(Field::BillingAddress),
						#[cfg(any(
							any(
								feature = "broker-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"broker" => Ok(Field::Broker),
						#[cfg(any(
							any(
								feature = "confirmation-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"confirmationNumber" => Ok(Field::ConfirmationNumber),
						#[cfg(any(
							any(
								feature = "customer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"customer" => Ok(Field::Customer),
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
								feature = "discount-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"discount" => Ok(Field::Discount),
						#[cfg(any(
							any(
								feature = "discount-code-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"discountCode" => Ok(Field::DiscountCode),
						#[cfg(any(
							any(
								feature = "discount-currency-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"discountCurrency" => Ok(Field::DiscountCurrency),
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
								feature = "is-gift-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isGift" => Ok(Field::IsGift),
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
								feature = "merchant-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"merchant" => Ok(Field::Merchant),
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
								feature = "order-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"orderDate" => Ok(Field::OrderDate),
						#[cfg(any(
							any(
								feature = "order-delivery-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"orderDelivery" => Ok(Field::OrderDelivery),
						#[cfg(any(
							any(
								feature = "order-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"orderNumber" => Ok(Field::OrderNumber),
						#[cfg(any(
							any(
								feature = "order-status-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"orderStatus" => Ok(Field::OrderStatus),
						#[cfg(any(
							any(
								feature = "ordered-item-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"orderedItem" => Ok(Field::OrderedItem),
						#[cfg(any(
							any(
								feature = "part-of-invoice-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"partOfInvoice" => Ok(Field::PartOfInvoice),
						#[cfg(any(
							any(
								feature = "payment-due-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"paymentDue" => Ok(Field::PaymentDue),
						#[cfg(any(
							any(
								feature = "payment-due-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"paymentDueDate" => Ok(Field::PaymentDueDate),
						#[cfg(any(
							any(
								feature = "payment-method-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"paymentMethod" => Ok(Field::PaymentMethod),
						#[cfg(any(
							any(
								feature = "payment-method-id-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"paymentMethodId" => Ok(Field::PaymentMethodId),
						#[cfg(any(
							any(
								feature = "payment-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"paymentUrl" => Ok(Field::PaymentUrl),
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
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"sameAs" => Ok(Field::SameAs),
						#[cfg(any(
							any(
								feature = "seller-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"seller" => Ok(Field::Seller),
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
				type Value = Order;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Order")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					#[cfg(any(
						any(
							feature = "accepted-offer-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#accepted_offer_property = None;
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
							feature = "billing-address-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#billing_address_property = None;
					#[cfg(any(
						any(
							feature = "broker-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#broker_property = None;
					#[cfg(any(
						any(
							feature = "confirmation-number-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#confirmation_number_property = None;
					#[cfg(any(
						any(
							feature = "customer-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#customer_property = None;
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
							feature = "discount-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#discount_property = None;
					#[cfg(any(
						any(
							feature = "discount-code-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#discount_code_property = None;
					#[cfg(any(
						any(
							feature = "discount-currency-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#discount_currency_property = None;
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
							feature = "is-gift-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_gift_property = None;
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
							feature = "merchant-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#merchant_property = None;
					#[cfg(any(
						any(feature = "name-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#name_property = None;
					#[cfg(any(
						any(
							feature = "order-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#order_date_property = None;
					#[cfg(any(
						any(
							feature = "order-delivery-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#order_delivery_property = None;
					#[cfg(any(
						any(
							feature = "order-number-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#order_number_property = None;
					#[cfg(any(
						any(
							feature = "order-status-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#order_status_property = None;
					#[cfg(any(
						any(
							feature = "ordered-item-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#ordered_item_property = None;
					#[cfg(any(
						any(
							feature = "part-of-invoice-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#part_of_invoice_property = None;
					#[cfg(any(
						any(
							feature = "payment-due-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#payment_due_property = None;
					#[cfg(any(
						any(
							feature = "payment-due-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#payment_due_date_property = None;
					#[cfg(any(
						any(
							feature = "payment-method-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#payment_method_property = None;
					#[cfg(any(
						any(
							feature = "payment-method-id-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#payment_method_id_property = None;
					#[cfg(any(
						any(
							feature = "payment-url-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#payment_url_property = None;
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
							feature = "same-as-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#same_as_property = None;
					#[cfg(any(
						any(
							feature = "seller-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#seller_property = None;
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
									feature = "accepted-offer-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "billing-address-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "broker-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "confirmation-number-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "customer-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "discount-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "discount-code-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "discount-currency-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "is-gift-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "merchant-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "order-date-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "order-delivery-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "order-number-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "order-status-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "ordered-item-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "part-of-invoice-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "payment-due-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "payment-due-date-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "payment-method-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "payment-method-id-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "payment-url-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "seller-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
					Ok(Order {
						#[cfg(any(
							any(
								feature = "accepted-offer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#accepted_offer: r#accepted_offer_property.unwrap_or_default(),
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
								feature = "billing-address-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#billing_address: r#billing_address_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "broker-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#broker: r#broker_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "confirmation-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#confirmation_number: r#confirmation_number_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "customer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#customer: r#customer_property.unwrap_or_default(),
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
								feature = "discount-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#discount: r#discount_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "discount-code-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#discount_code: r#discount_code_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "discount-currency-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#discount_currency: r#discount_currency_property.unwrap_or_default(),
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
								feature = "is-gift-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_gift: r#is_gift_property.unwrap_or_default(),
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
								feature = "merchant-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#merchant: r#merchant_property.unwrap_or_default(),
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
								feature = "order-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#order_date: r#order_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "order-delivery-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#order_delivery: r#order_delivery_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "order-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#order_number: r#order_number_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "order-status-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#order_status: r#order_status_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "ordered-item-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#ordered_item: r#ordered_item_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "part-of-invoice-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#part_of_invoice: r#part_of_invoice_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "payment-due-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#payment_due: r#payment_due_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "payment-due-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#payment_due_date: r#payment_due_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "payment-method-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#payment_method: r#payment_method_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "payment-method-id-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#payment_method_id: r#payment_method_id_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "payment-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#payment_url: r#payment_url_property.unwrap_or_default(),
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
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#same_as: r#same_as_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "seller-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#seller: r#seller_property.unwrap_or_default(),
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
						feature = "accepted-offer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"acceptedOffer",
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
						feature = "billing-address-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"billingAddress",
				#[cfg(any(
					any(feature = "broker-property-schema", feature = "general-schema-section"),
					doc
				))]
				"broker",
				#[cfg(any(
					any(
						feature = "confirmation-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"confirmationNumber",
				#[cfg(any(
					any(
						feature = "customer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"customer",
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
						feature = "discount-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"discount",
				#[cfg(any(
					any(
						feature = "discount-code-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"discountCode",
				#[cfg(any(
					any(
						feature = "discount-currency-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"discountCurrency",
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
						feature = "is-gift-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isGift",
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
						feature = "merchant-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"merchant",
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				"name",
				#[cfg(any(
					any(
						feature = "order-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"orderDate",
				#[cfg(any(
					any(
						feature = "order-delivery-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"orderDelivery",
				#[cfg(any(
					any(
						feature = "order-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"orderNumber",
				#[cfg(any(
					any(
						feature = "order-status-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"orderStatus",
				#[cfg(any(
					any(
						feature = "ordered-item-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"orderedItem",
				#[cfg(any(
					any(
						feature = "part-of-invoice-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"partOfInvoice",
				#[cfg(any(
					any(
						feature = "payment-due-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"paymentDue",
				#[cfg(any(
					any(
						feature = "payment-due-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"paymentDueDate",
				#[cfg(any(
					any(
						feature = "payment-method-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"paymentMethod",
				#[cfg(any(
					any(
						feature = "payment-method-id-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"paymentMethodId",
				#[cfg(any(
					any(
						feature = "payment-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"paymentUrl",
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
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"sameAs",
				#[cfg(any(
					any(feature = "seller-property-schema", feature = "general-schema-section"),
					doc
				))]
				"seller",
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
			deserializer.deserialize_struct("Order", FIELDS, ClassVisitor)
		}
	}
}
