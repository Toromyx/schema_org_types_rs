use super::*;
/// <https://schema.org/Demand>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Demand {
	#[cfg(any(
		any(
			feature = "accepted-payment-method-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#accepted_payment_method: Vec<AcceptedPaymentMethodProperty>,
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
			feature = "advance-booking-requirement-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#advance_booking_requirement: Vec<AdvanceBookingRequirementProperty>,
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
			feature = "area-served-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#area_served: Vec<AreaServedProperty>,
	#[cfg(any(
		any(feature = "asin-property-schema", feature = "pending-schema-section"),
		doc
	))]
	pub r#asin: Vec<AsinProperty>,
	#[cfg(any(
		any(
			feature = "availability-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#availability: Vec<AvailabilityProperty>,
	#[cfg(any(
		any(
			feature = "availability-ends-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#availability_ends: Vec<AvailabilityEndsProperty>,
	#[cfg(any(
		any(
			feature = "availability-starts-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#availability_starts: Vec<AvailabilityStartsProperty>,
	#[cfg(any(
		any(
			feature = "available-at-or-from-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#available_at_or_from: Vec<AvailableAtOrFromProperty>,
	#[cfg(any(
		any(
			feature = "available-delivery-method-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#available_delivery_method: Vec<AvailableDeliveryMethodProperty>,
	#[cfg(any(
		any(
			feature = "business-function-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#business_function: Vec<BusinessFunctionProperty>,
	#[cfg(any(
		any(
			feature = "delivery-lead-time-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#delivery_lead_time: Vec<DeliveryLeadTimeProperty>,
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
			feature = "eligible-customer-type-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#eligible_customer_type: Vec<EligibleCustomerTypeProperty>,
	#[cfg(any(
		any(
			feature = "eligible-duration-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#eligible_duration: Vec<EligibleDurationProperty>,
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
			feature = "eligible-region-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#eligible_region: Vec<EligibleRegionProperty>,
	#[cfg(any(
		any(
			feature = "eligible-transaction-volume-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#eligible_transaction_volume: Vec<EligibleTransactionVolumeProperty>,
	#[cfg(any(
		any(feature = "gtin-property-schema", feature = "pending-schema-section"),
		doc
	))]
	pub r#gtin: Vec<GtinProperty>,
	#[cfg(any(
		any(
			feature = "gtin-12-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#gtin_12: Vec<Gtin12Property>,
	#[cfg(any(
		any(
			feature = "gtin-13-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#gtin_13: Vec<Gtin13Property>,
	#[cfg(any(
		any(
			feature = "gtin-14-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#gtin_14: Vec<Gtin14Property>,
	#[cfg(any(
		any(feature = "gtin-8-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#gtin_8: Vec<Gtin8Property>,
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
			feature = "includes-object-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#includes_object: Vec<IncludesObjectProperty>,
	#[cfg(any(
		any(
			feature = "ineligible-region-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#ineligible_region: Vec<IneligibleRegionProperty>,
	#[cfg(any(
		any(
			feature = "inventory-level-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#inventory_level: Vec<InventoryLevelProperty>,
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
			feature = "item-offered-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#item_offered: Vec<ItemOfferedProperty>,
	#[cfg(any(
		any(
			feature = "main-entity-of-page-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	#[cfg(any(
		any(feature = "mpn-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#mpn: Vec<MpnProperty>,
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
			feature = "price-specification-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#price_specification: Vec<PriceSpecificationProperty>,
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
			feature = "serial-number-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#serial_number: Vec<SerialNumberProperty>,
	#[cfg(any(
		any(feature = "sku-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#sku: Vec<SkuProperty>,
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
			feature = "warranty-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#warranty: Vec<WarrantyProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for Demand {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				if cfg!(any(
					any(
						feature = "accepted-payment-method-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#accepted_payment_method) as usize
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
						feature = "advance-booking-requirement-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#advance_booking_requirement) as usize
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
						feature = "area-served-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#area_served) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "asin-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#asin) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "availability-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#availability) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "availability-ends-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#availability_ends) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "availability-starts-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#availability_starts) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "available-at-or-from-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#available_at_or_from) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "available-delivery-method-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#available_delivery_method) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "business-function-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#business_function) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "delivery-lead-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#delivery_lead_time) as usize
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
						feature = "eligible-customer-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#eligible_customer_type) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "eligible-duration-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#eligible_duration) as usize
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
						feature = "eligible-region-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#eligible_region) as usize
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
						feature = "gtin-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#gtin) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "gtin-12-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#gtin_12) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "gtin-13-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#gtin_13) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "gtin-14-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#gtin_14) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "gtin-8-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#gtin_8) as usize
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
						feature = "includes-object-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#includes_object) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "ineligible-region-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#ineligible_region) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "inventory-level-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#inventory_level) as usize
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
						feature = "item-offered-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#item_offered) as usize
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
						feature = "mpn-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#mpn) as usize
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
						feature = "price-specification-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#price_specification) as usize
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
						feature = "serial-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#serial_number) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "sku-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#sku) as usize
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
						feature = "warranty-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#warranty) as usize
				} else {
					0
				},
			]
			.iter()
			.sum();
			let mut serialize_struct = Serializer::serialize_struct(serializer, "Demand", len)?;
			#[cfg(any(
				any(
					feature = "accepted-payment-method-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#accepted_payment_method) {
				serialize_struct.serialize_field("acceptedPaymentMethod", {
					struct SerializeWith<'a>(&'a Vec<AcceptedPaymentMethodProperty>);
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
					&SerializeWith(&self.r#accepted_payment_method)
				})?;
			} else {
				serialize_struct.skip_field("acceptedPaymentMethod")?;
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
					feature = "advance-booking-requirement-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#advance_booking_requirement) {
				serialize_struct.serialize_field("advanceBookingRequirement", {
					struct SerializeWith<'a>(&'a Vec<AdvanceBookingRequirementProperty>);
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
					&SerializeWith(&self.r#advance_booking_requirement)
				})?;
			} else {
				serialize_struct.skip_field("advanceBookingRequirement")?;
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
					feature = "area-served-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#area_served) {
				serialize_struct.serialize_field("areaServed", {
					struct SerializeWith<'a>(&'a Vec<AreaServedProperty>);
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
					&SerializeWith(&self.r#area_served)
				})?;
			} else {
				serialize_struct.skip_field("areaServed")?;
			}
			#[cfg(any(
				any(feature = "asin-property-schema", feature = "pending-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#asin) {
				serialize_struct.serialize_field("asin", {
					struct SerializeWith<'a>(&'a Vec<AsinProperty>);
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
					&SerializeWith(&self.r#asin)
				})?;
			} else {
				serialize_struct.skip_field("asin")?;
			}
			#[cfg(any(
				any(
					feature = "availability-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#availability) {
				serialize_struct.serialize_field("availability", {
					struct SerializeWith<'a>(&'a Vec<AvailabilityProperty>);
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
					&SerializeWith(&self.r#availability)
				})?;
			} else {
				serialize_struct.skip_field("availability")?;
			}
			#[cfg(any(
				any(
					feature = "availability-ends-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#availability_ends) {
				serialize_struct.serialize_field("availabilityEnds", {
					struct SerializeWith<'a>(&'a Vec<AvailabilityEndsProperty>);
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
					&SerializeWith(&self.r#availability_ends)
				})?;
			} else {
				serialize_struct.skip_field("availabilityEnds")?;
			}
			#[cfg(any(
				any(
					feature = "availability-starts-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#availability_starts) {
				serialize_struct.serialize_field("availabilityStarts", {
					struct SerializeWith<'a>(&'a Vec<AvailabilityStartsProperty>);
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
					&SerializeWith(&self.r#availability_starts)
				})?;
			} else {
				serialize_struct.skip_field("availabilityStarts")?;
			}
			#[cfg(any(
				any(
					feature = "available-at-or-from-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#available_at_or_from) {
				serialize_struct.serialize_field("availableAtOrFrom", {
					struct SerializeWith<'a>(&'a Vec<AvailableAtOrFromProperty>);
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
					&SerializeWith(&self.r#available_at_or_from)
				})?;
			} else {
				serialize_struct.skip_field("availableAtOrFrom")?;
			}
			#[cfg(any(
				any(
					feature = "available-delivery-method-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#available_delivery_method) {
				serialize_struct.serialize_field("availableDeliveryMethod", {
					struct SerializeWith<'a>(&'a Vec<AvailableDeliveryMethodProperty>);
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
					&SerializeWith(&self.r#available_delivery_method)
				})?;
			} else {
				serialize_struct.skip_field("availableDeliveryMethod")?;
			}
			#[cfg(any(
				any(
					feature = "business-function-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#business_function) {
				serialize_struct.serialize_field("businessFunction", {
					struct SerializeWith<'a>(&'a Vec<BusinessFunctionProperty>);
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
					&SerializeWith(&self.r#business_function)
				})?;
			} else {
				serialize_struct.skip_field("businessFunction")?;
			}
			#[cfg(any(
				any(
					feature = "delivery-lead-time-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#delivery_lead_time) {
				serialize_struct.serialize_field("deliveryLeadTime", {
					struct SerializeWith<'a>(&'a Vec<DeliveryLeadTimeProperty>);
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
					&SerializeWith(&self.r#delivery_lead_time)
				})?;
			} else {
				serialize_struct.skip_field("deliveryLeadTime")?;
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
					feature = "eligible-customer-type-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#eligible_customer_type) {
				serialize_struct.serialize_field("eligibleCustomerType", {
					struct SerializeWith<'a>(&'a Vec<EligibleCustomerTypeProperty>);
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
					&SerializeWith(&self.r#eligible_customer_type)
				})?;
			} else {
				serialize_struct.skip_field("eligibleCustomerType")?;
			}
			#[cfg(any(
				any(
					feature = "eligible-duration-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#eligible_duration) {
				serialize_struct.serialize_field("eligibleDuration", {
					struct SerializeWith<'a>(&'a Vec<EligibleDurationProperty>);
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
					&SerializeWith(&self.r#eligible_duration)
				})?;
			} else {
				serialize_struct.skip_field("eligibleDuration")?;
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
					feature = "eligible-region-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#eligible_region) {
				serialize_struct.serialize_field("eligibleRegion", {
					struct SerializeWith<'a>(&'a Vec<EligibleRegionProperty>);
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
					&SerializeWith(&self.r#eligible_region)
				})?;
			} else {
				serialize_struct.skip_field("eligibleRegion")?;
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
				any(feature = "gtin-property-schema", feature = "pending-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#gtin) {
				serialize_struct.serialize_field("gtin", {
					struct SerializeWith<'a>(&'a Vec<GtinProperty>);
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
					&SerializeWith(&self.r#gtin)
				})?;
			} else {
				serialize_struct.skip_field("gtin")?;
			}
			#[cfg(any(
				any(
					feature = "gtin-12-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#gtin_12) {
				serialize_struct.serialize_field("gtin12", {
					struct SerializeWith<'a>(&'a Vec<Gtin12Property>);
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
					&SerializeWith(&self.r#gtin_12)
				})?;
			} else {
				serialize_struct.skip_field("gtin12")?;
			}
			#[cfg(any(
				any(
					feature = "gtin-13-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#gtin_13) {
				serialize_struct.serialize_field("gtin13", {
					struct SerializeWith<'a>(&'a Vec<Gtin13Property>);
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
					&SerializeWith(&self.r#gtin_13)
				})?;
			} else {
				serialize_struct.skip_field("gtin13")?;
			}
			#[cfg(any(
				any(
					feature = "gtin-14-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#gtin_14) {
				serialize_struct.serialize_field("gtin14", {
					struct SerializeWith<'a>(&'a Vec<Gtin14Property>);
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
					&SerializeWith(&self.r#gtin_14)
				})?;
			} else {
				serialize_struct.skip_field("gtin14")?;
			}
			#[cfg(any(
				any(feature = "gtin-8-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#gtin_8) {
				serialize_struct.serialize_field("gtin8", {
					struct SerializeWith<'a>(&'a Vec<Gtin8Property>);
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
					&SerializeWith(&self.r#gtin_8)
				})?;
			} else {
				serialize_struct.skip_field("gtin8")?;
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
					feature = "includes-object-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#includes_object) {
				serialize_struct.serialize_field("includesObject", {
					struct SerializeWith<'a>(&'a Vec<IncludesObjectProperty>);
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
					&SerializeWith(&self.r#includes_object)
				})?;
			} else {
				serialize_struct.skip_field("includesObject")?;
			}
			#[cfg(any(
				any(
					feature = "ineligible-region-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#ineligible_region) {
				serialize_struct.serialize_field("ineligibleRegion", {
					struct SerializeWith<'a>(&'a Vec<IneligibleRegionProperty>);
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
					&SerializeWith(&self.r#ineligible_region)
				})?;
			} else {
				serialize_struct.skip_field("ineligibleRegion")?;
			}
			#[cfg(any(
				any(
					feature = "inventory-level-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#inventory_level) {
				serialize_struct.serialize_field("inventoryLevel", {
					struct SerializeWith<'a>(&'a Vec<InventoryLevelProperty>);
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
					&SerializeWith(&self.r#inventory_level)
				})?;
			} else {
				serialize_struct.skip_field("inventoryLevel")?;
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
					feature = "item-offered-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#item_offered) {
				serialize_struct.serialize_field("itemOffered", {
					struct SerializeWith<'a>(&'a Vec<ItemOfferedProperty>);
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
					&SerializeWith(&self.r#item_offered)
				})?;
			} else {
				serialize_struct.skip_field("itemOffered")?;
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
				any(feature = "mpn-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#mpn) {
				serialize_struct.serialize_field("mpn", {
					struct SerializeWith<'a>(&'a Vec<MpnProperty>);
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
					&SerializeWith(&self.r#mpn)
				})?;
			} else {
				serialize_struct.skip_field("mpn")?;
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
					feature = "price-specification-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#price_specification) {
				serialize_struct.serialize_field("priceSpecification", {
					struct SerializeWith<'a>(&'a Vec<PriceSpecificationProperty>);
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
					&SerializeWith(&self.r#price_specification)
				})?;
			} else {
				serialize_struct.skip_field("priceSpecification")?;
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
					feature = "serial-number-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#serial_number) {
				serialize_struct.serialize_field("serialNumber", {
					struct SerializeWith<'a>(&'a Vec<SerialNumberProperty>);
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
					&SerializeWith(&self.r#serial_number)
				})?;
			} else {
				serialize_struct.skip_field("serialNumber")?;
			}
			#[cfg(any(
				any(feature = "sku-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#sku) {
				serialize_struct.serialize_field("sku", {
					struct SerializeWith<'a>(&'a Vec<SkuProperty>);
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
					&SerializeWith(&self.r#sku)
				})?;
			} else {
				serialize_struct.skip_field("sku")?;
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
					feature = "warranty-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#warranty) {
				serialize_struct.serialize_field("warranty", {
					struct SerializeWith<'a>(&'a Vec<WarrantyProperty>);
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
					&SerializeWith(&self.r#warranty)
				})?;
			} else {
				serialize_struct.skip_field("warranty")?;
			}
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for Demand {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				#[cfg(any(
					any(
						feature = "accepted-payment-method-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AcceptedPaymentMethod,
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
						feature = "advance-booking-requirement-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AdvanceBookingRequirement,
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
						feature = "area-served-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AreaServed,
				#[cfg(any(
					any(feature = "asin-property-schema", feature = "pending-schema-section"),
					doc
				))]
				Asin,
				#[cfg(any(
					any(
						feature = "availability-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Availability,
				#[cfg(any(
					any(
						feature = "availability-ends-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AvailabilityEnds,
				#[cfg(any(
					any(
						feature = "availability-starts-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AvailabilityStarts,
				#[cfg(any(
					any(
						feature = "available-at-or-from-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AvailableAtOrFrom,
				#[cfg(any(
					any(
						feature = "available-delivery-method-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AvailableDeliveryMethod,
				#[cfg(any(
					any(
						feature = "business-function-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				BusinessFunction,
				#[cfg(any(
					any(
						feature = "delivery-lead-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DeliveryLeadTime,
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
						feature = "eligible-customer-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EligibleCustomerType,
				#[cfg(any(
					any(
						feature = "eligible-duration-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EligibleDuration,
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
						feature = "eligible-region-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EligibleRegion,
				#[cfg(any(
					any(
						feature = "eligible-transaction-volume-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EligibleTransactionVolume,
				#[cfg(any(
					any(feature = "gtin-property-schema", feature = "pending-schema-section"),
					doc
				))]
				Gtin,
				#[cfg(any(
					any(
						feature = "gtin-12-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Gtin12,
				#[cfg(any(
					any(
						feature = "gtin-13-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Gtin13,
				#[cfg(any(
					any(
						feature = "gtin-14-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Gtin14,
				#[cfg(any(
					any(feature = "gtin-8-property-schema", feature = "general-schema-section"),
					doc
				))]
				Gtin8,
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
						feature = "includes-object-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IncludesObject,
				#[cfg(any(
					any(
						feature = "ineligible-region-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				IneligibleRegion,
				#[cfg(any(
					any(
						feature = "inventory-level-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				InventoryLevel,
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
						feature = "item-offered-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ItemOffered,
				#[cfg(any(
					any(
						feature = "main-entity-of-page-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MainEntityOfPage,
				#[cfg(any(
					any(feature = "mpn-property-schema", feature = "general-schema-section"),
					doc
				))]
				Mpn,
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
						feature = "price-specification-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PriceSpecification,
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
						feature = "serial-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SerialNumber,
				#[cfg(any(
					any(feature = "sku-property-schema", feature = "general-schema-section"),
					doc
				))]
				Sku,
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
						feature = "warranty-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Warranty,
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
								feature = "accepted-payment-method-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"acceptedPaymentMethod" => Ok(Field::AcceptedPaymentMethod),
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
								feature = "advance-booking-requirement-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"advanceBookingRequirement" => Ok(Field::AdvanceBookingRequirement),
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
								feature = "area-served-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"areaServed" => Ok(Field::AreaServed),
						#[cfg(any(
							any(
								feature = "asin-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"asin" => Ok(Field::Asin),
						#[cfg(any(
							any(
								feature = "availability-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"availability" => Ok(Field::Availability),
						#[cfg(any(
							any(
								feature = "availability-ends-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"availabilityEnds" => Ok(Field::AvailabilityEnds),
						#[cfg(any(
							any(
								feature = "availability-starts-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"availabilityStarts" => Ok(Field::AvailabilityStarts),
						#[cfg(any(
							any(
								feature = "available-at-or-from-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"availableAtOrFrom" => Ok(Field::AvailableAtOrFrom),
						#[cfg(any(
							any(
								feature = "available-delivery-method-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"availableDeliveryMethod" => Ok(Field::AvailableDeliveryMethod),
						#[cfg(any(
							any(
								feature = "business-function-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"businessFunction" => Ok(Field::BusinessFunction),
						#[cfg(any(
							any(
								feature = "delivery-lead-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"deliveryLeadTime" => Ok(Field::DeliveryLeadTime),
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
								feature = "eligible-customer-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"eligibleCustomerType" => Ok(Field::EligibleCustomerType),
						#[cfg(any(
							any(
								feature = "eligible-duration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"eligibleDuration" => Ok(Field::EligibleDuration),
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
								feature = "eligible-region-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"eligibleRegion" => Ok(Field::EligibleRegion),
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
								feature = "gtin-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"gtin" => Ok(Field::Gtin),
						#[cfg(any(
							any(
								feature = "gtin-12-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"gtin12" => Ok(Field::Gtin12),
						#[cfg(any(
							any(
								feature = "gtin-13-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"gtin13" => Ok(Field::Gtin13),
						#[cfg(any(
							any(
								feature = "gtin-14-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"gtin14" => Ok(Field::Gtin14),
						#[cfg(any(
							any(
								feature = "gtin-8-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"gtin8" => Ok(Field::Gtin8),
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
								feature = "includes-object-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"includesObject" => Ok(Field::IncludesObject),
						#[cfg(any(
							any(
								feature = "ineligible-region-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"ineligibleRegion" => Ok(Field::IneligibleRegion),
						#[cfg(any(
							any(
								feature = "inventory-level-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"inventoryLevel" => Ok(Field::InventoryLevel),
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
								feature = "item-offered-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"itemOffered" => Ok(Field::ItemOffered),
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
								feature = "mpn-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"mpn" => Ok(Field::Mpn),
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
								feature = "price-specification-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"priceSpecification" => Ok(Field::PriceSpecification),
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
								feature = "serial-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"serialNumber" => Ok(Field::SerialNumber),
						#[cfg(any(
							any(
								feature = "sku-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"sku" => Ok(Field::Sku),
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
								feature = "warranty-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"warranty" => Ok(Field::Warranty),
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
								feature = "accepted-payment-method-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"acceptedPaymentMethod" => Ok(Field::AcceptedPaymentMethod),
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
								feature = "advance-booking-requirement-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"advanceBookingRequirement" => Ok(Field::AdvanceBookingRequirement),
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
								feature = "area-served-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"areaServed" => Ok(Field::AreaServed),
						#[cfg(any(
							any(
								feature = "asin-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"asin" => Ok(Field::Asin),
						#[cfg(any(
							any(
								feature = "availability-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"availability" => Ok(Field::Availability),
						#[cfg(any(
							any(
								feature = "availability-ends-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"availabilityEnds" => Ok(Field::AvailabilityEnds),
						#[cfg(any(
							any(
								feature = "availability-starts-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"availabilityStarts" => Ok(Field::AvailabilityStarts),
						#[cfg(any(
							any(
								feature = "available-at-or-from-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"availableAtOrFrom" => Ok(Field::AvailableAtOrFrom),
						#[cfg(any(
							any(
								feature = "available-delivery-method-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"availableDeliveryMethod" => Ok(Field::AvailableDeliveryMethod),
						#[cfg(any(
							any(
								feature = "business-function-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"businessFunction" => Ok(Field::BusinessFunction),
						#[cfg(any(
							any(
								feature = "delivery-lead-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"deliveryLeadTime" => Ok(Field::DeliveryLeadTime),
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
								feature = "eligible-customer-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"eligibleCustomerType" => Ok(Field::EligibleCustomerType),
						#[cfg(any(
							any(
								feature = "eligible-duration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"eligibleDuration" => Ok(Field::EligibleDuration),
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
								feature = "eligible-region-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"eligibleRegion" => Ok(Field::EligibleRegion),
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
								feature = "gtin-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"gtin" => Ok(Field::Gtin),
						#[cfg(any(
							any(
								feature = "gtin-12-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"gtin12" => Ok(Field::Gtin12),
						#[cfg(any(
							any(
								feature = "gtin-13-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"gtin13" => Ok(Field::Gtin13),
						#[cfg(any(
							any(
								feature = "gtin-14-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"gtin14" => Ok(Field::Gtin14),
						#[cfg(any(
							any(
								feature = "gtin-8-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"gtin8" => Ok(Field::Gtin8),
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
								feature = "includes-object-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"includesObject" => Ok(Field::IncludesObject),
						#[cfg(any(
							any(
								feature = "ineligible-region-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"ineligibleRegion" => Ok(Field::IneligibleRegion),
						#[cfg(any(
							any(
								feature = "inventory-level-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"inventoryLevel" => Ok(Field::InventoryLevel),
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
								feature = "item-offered-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"itemOffered" => Ok(Field::ItemOffered),
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
								feature = "mpn-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"mpn" => Ok(Field::Mpn),
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
								feature = "price-specification-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"priceSpecification" => Ok(Field::PriceSpecification),
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
								feature = "serial-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"serialNumber" => Ok(Field::SerialNumber),
						#[cfg(any(
							any(
								feature = "sku-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"sku" => Ok(Field::Sku),
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
								feature = "warranty-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"warranty" => Ok(Field::Warranty),
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
				type Value = Demand;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Demand")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					#[cfg(any(
						any(
							feature = "accepted-payment-method-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#accepted_payment_method_property = None;
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
							feature = "advance-booking-requirement-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#advance_booking_requirement_property = None;
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
							feature = "area-served-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#area_served_property = None;
					#[cfg(any(
						any(feature = "asin-property-schema", feature = "pending-schema-section"),
						doc
					))]
					let mut r#asin_property = None;
					#[cfg(any(
						any(
							feature = "availability-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#availability_property = None;
					#[cfg(any(
						any(
							feature = "availability-ends-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#availability_ends_property = None;
					#[cfg(any(
						any(
							feature = "availability-starts-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#availability_starts_property = None;
					#[cfg(any(
						any(
							feature = "available-at-or-from-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#available_at_or_from_property = None;
					#[cfg(any(
						any(
							feature = "available-delivery-method-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#available_delivery_method_property = None;
					#[cfg(any(
						any(
							feature = "business-function-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#business_function_property = None;
					#[cfg(any(
						any(
							feature = "delivery-lead-time-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#delivery_lead_time_property = None;
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
							feature = "eligible-customer-type-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#eligible_customer_type_property = None;
					#[cfg(any(
						any(
							feature = "eligible-duration-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#eligible_duration_property = None;
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
							feature = "eligible-region-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#eligible_region_property = None;
					#[cfg(any(
						any(
							feature = "eligible-transaction-volume-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#eligible_transaction_volume_property = None;
					#[cfg(any(
						any(feature = "gtin-property-schema", feature = "pending-schema-section"),
						doc
					))]
					let mut r#gtin_property = None;
					#[cfg(any(
						any(
							feature = "gtin-12-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#gtin_12_property = None;
					#[cfg(any(
						any(
							feature = "gtin-13-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#gtin_13_property = None;
					#[cfg(any(
						any(
							feature = "gtin-14-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#gtin_14_property = None;
					#[cfg(any(
						any(
							feature = "gtin-8-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#gtin_8_property = None;
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
							feature = "includes-object-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#includes_object_property = None;
					#[cfg(any(
						any(
							feature = "ineligible-region-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#ineligible_region_property = None;
					#[cfg(any(
						any(
							feature = "inventory-level-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#inventory_level_property = None;
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
							feature = "item-offered-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#item_offered_property = None;
					#[cfg(any(
						any(
							feature = "main-entity-of-page-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#main_entity_of_page_property = None;
					#[cfg(any(
						any(feature = "mpn-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#mpn_property = None;
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
							feature = "price-specification-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#price_specification_property = None;
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
							feature = "serial-number-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#serial_number_property = None;
					#[cfg(any(
						any(feature = "sku-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#sku_property = None;
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
							feature = "warranty-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#warranty_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
							#[cfg(any(
								any(
									feature = "accepted-payment-method-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AcceptedPaymentMethod => {
								if r#accepted_payment_method_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"acceptedPaymentMethod",
									));
								}
								r#accepted_payment_method_property = Some({
									struct DeserializeWith(Vec<AcceptedPaymentMethodProperty>);
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
									feature = "advance-booking-requirement-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AdvanceBookingRequirement => {
								if r#advance_booking_requirement_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"advanceBookingRequirement",
									));
								}
								r#advance_booking_requirement_property = Some({
									struct DeserializeWith(Vec<AdvanceBookingRequirementProperty>);
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
									feature = "area-served-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AreaServed => {
								if r#area_served_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"areaServed",
									));
								}
								r#area_served_property = Some({
									struct DeserializeWith(Vec<AreaServedProperty>);
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
									feature = "asin-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::Asin => {
								if r#asin_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("asin"));
								}
								r#asin_property = Some({
									struct DeserializeWith(Vec<AsinProperty>);
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
									feature = "availability-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Availability => {
								if r#availability_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"availability",
									));
								}
								r#availability_property = Some({
									struct DeserializeWith(Vec<AvailabilityProperty>);
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
									feature = "availability-ends-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AvailabilityEnds => {
								if r#availability_ends_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"availabilityEnds",
									));
								}
								r#availability_ends_property = Some({
									struct DeserializeWith(Vec<AvailabilityEndsProperty>);
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
									feature = "availability-starts-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AvailabilityStarts => {
								if r#availability_starts_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"availabilityStarts",
									));
								}
								r#availability_starts_property = Some({
									struct DeserializeWith(Vec<AvailabilityStartsProperty>);
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
									feature = "available-at-or-from-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AvailableAtOrFrom => {
								if r#available_at_or_from_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"availableAtOrFrom",
									));
								}
								r#available_at_or_from_property = Some({
									struct DeserializeWith(Vec<AvailableAtOrFromProperty>);
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
									feature = "available-delivery-method-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AvailableDeliveryMethod => {
								if r#available_delivery_method_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"availableDeliveryMethod",
									));
								}
								r#available_delivery_method_property = Some({
									struct DeserializeWith(Vec<AvailableDeliveryMethodProperty>);
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
									feature = "business-function-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::BusinessFunction => {
								if r#business_function_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"businessFunction",
									));
								}
								r#business_function_property = Some({
									struct DeserializeWith(Vec<BusinessFunctionProperty>);
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
									feature = "delivery-lead-time-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::DeliveryLeadTime => {
								if r#delivery_lead_time_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"deliveryLeadTime",
									));
								}
								r#delivery_lead_time_property = Some({
									struct DeserializeWith(Vec<DeliveryLeadTimeProperty>);
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
									feature = "eligible-customer-type-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::EligibleCustomerType => {
								if r#eligible_customer_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"eligibleCustomerType",
									));
								}
								r#eligible_customer_type_property = Some({
									struct DeserializeWith(Vec<EligibleCustomerTypeProperty>);
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
									feature = "eligible-duration-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::EligibleDuration => {
								if r#eligible_duration_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"eligibleDuration",
									));
								}
								r#eligible_duration_property = Some({
									struct DeserializeWith(Vec<EligibleDurationProperty>);
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
									feature = "eligible-region-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::EligibleRegion => {
								if r#eligible_region_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"eligibleRegion",
									));
								}
								r#eligible_region_property = Some({
									struct DeserializeWith(Vec<EligibleRegionProperty>);
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
									feature = "gtin-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::Gtin => {
								if r#gtin_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("gtin"));
								}
								r#gtin_property = Some({
									struct DeserializeWith(Vec<GtinProperty>);
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
									feature = "gtin-12-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Gtin12 => {
								if r#gtin_12_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("gtin12"));
								}
								r#gtin_12_property = Some({
									struct DeserializeWith(Vec<Gtin12Property>);
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
									feature = "gtin-13-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Gtin13 => {
								if r#gtin_13_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("gtin13"));
								}
								r#gtin_13_property = Some({
									struct DeserializeWith(Vec<Gtin13Property>);
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
									feature = "gtin-14-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Gtin14 => {
								if r#gtin_14_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("gtin14"));
								}
								r#gtin_14_property = Some({
									struct DeserializeWith(Vec<Gtin14Property>);
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
									feature = "gtin-8-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Gtin8 => {
								if r#gtin_8_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("gtin8"));
								}
								r#gtin_8_property = Some({
									struct DeserializeWith(Vec<Gtin8Property>);
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
									feature = "includes-object-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::IncludesObject => {
								if r#includes_object_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"includesObject",
									));
								}
								r#includes_object_property = Some({
									struct DeserializeWith(Vec<IncludesObjectProperty>);
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
									feature = "ineligible-region-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::IneligibleRegion => {
								if r#ineligible_region_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"ineligibleRegion",
									));
								}
								r#ineligible_region_property = Some({
									struct DeserializeWith(Vec<IneligibleRegionProperty>);
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
									feature = "inventory-level-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::InventoryLevel => {
								if r#inventory_level_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"inventoryLevel",
									));
								}
								r#inventory_level_property = Some({
									struct DeserializeWith(Vec<InventoryLevelProperty>);
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
									feature = "item-offered-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ItemOffered => {
								if r#item_offered_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"itemOffered",
									));
								}
								r#item_offered_property = Some({
									struct DeserializeWith(Vec<ItemOfferedProperty>);
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
									feature = "mpn-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Mpn => {
								if r#mpn_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("mpn"));
								}
								r#mpn_property = Some({
									struct DeserializeWith(Vec<MpnProperty>);
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
									feature = "price-specification-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::PriceSpecification => {
								if r#price_specification_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"priceSpecification",
									));
								}
								r#price_specification_property = Some({
									struct DeserializeWith(Vec<PriceSpecificationProperty>);
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
									feature = "serial-number-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::SerialNumber => {
								if r#serial_number_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"serialNumber",
									));
								}
								r#serial_number_property = Some({
									struct DeserializeWith(Vec<SerialNumberProperty>);
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
									feature = "sku-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Sku => {
								if r#sku_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("sku"));
								}
								r#sku_property = Some({
									struct DeserializeWith(Vec<SkuProperty>);
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
									feature = "warranty-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Warranty => {
								if r#warranty_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"warranty",
									));
								}
								r#warranty_property = Some({
									struct DeserializeWith(Vec<WarrantyProperty>);
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
					Ok(Demand {
						#[cfg(any(
							any(
								feature = "accepted-payment-method-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#accepted_payment_method: r#accepted_payment_method_property
							.unwrap_or_default(),
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
								feature = "advance-booking-requirement-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#advance_booking_requirement: r#advance_booking_requirement_property
							.unwrap_or_default(),
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
								feature = "area-served-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#area_served: r#area_served_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "asin-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#asin: r#asin_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "availability-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#availability: r#availability_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "availability-ends-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#availability_ends: r#availability_ends_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "availability-starts-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#availability_starts: r#availability_starts_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "available-at-or-from-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#available_at_or_from: r#available_at_or_from_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "available-delivery-method-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#available_delivery_method: r#available_delivery_method_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "business-function-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#business_function: r#business_function_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "delivery-lead-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#delivery_lead_time: r#delivery_lead_time_property.unwrap_or_default(),
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
								feature = "eligible-customer-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#eligible_customer_type: r#eligible_customer_type_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "eligible-duration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#eligible_duration: r#eligible_duration_property.unwrap_or_default(),
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
								feature = "eligible-region-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#eligible_region: r#eligible_region_property.unwrap_or_default(),
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
								feature = "gtin-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#gtin: r#gtin_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "gtin-12-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#gtin_12: r#gtin_12_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "gtin-13-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#gtin_13: r#gtin_13_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "gtin-14-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#gtin_14: r#gtin_14_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "gtin-8-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#gtin_8: r#gtin_8_property.unwrap_or_default(),
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
								feature = "includes-object-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#includes_object: r#includes_object_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "ineligible-region-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#ineligible_region: r#ineligible_region_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "inventory-level-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#inventory_level: r#inventory_level_property.unwrap_or_default(),
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
								feature = "item-offered-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#item_offered: r#item_offered_property.unwrap_or_default(),
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
								feature = "mpn-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#mpn: r#mpn_property.unwrap_or_default(),
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
								feature = "price-specification-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#price_specification: r#price_specification_property.unwrap_or_default(),
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
								feature = "serial-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#serial_number: r#serial_number_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "sku-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#sku: r#sku_property.unwrap_or_default(),
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
								feature = "warranty-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#warranty: r#warranty_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				#[cfg(any(
					any(
						feature = "accepted-payment-method-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"acceptedPaymentMethod",
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
						feature = "advance-booking-requirement-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"advanceBookingRequirement",
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
						feature = "area-served-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"areaServed",
				#[cfg(any(
					any(feature = "asin-property-schema", feature = "pending-schema-section"),
					doc
				))]
				"asin",
				#[cfg(any(
					any(
						feature = "availability-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"availability",
				#[cfg(any(
					any(
						feature = "availability-ends-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"availabilityEnds",
				#[cfg(any(
					any(
						feature = "availability-starts-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"availabilityStarts",
				#[cfg(any(
					any(
						feature = "available-at-or-from-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"availableAtOrFrom",
				#[cfg(any(
					any(
						feature = "available-delivery-method-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"availableDeliveryMethod",
				#[cfg(any(
					any(
						feature = "business-function-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"businessFunction",
				#[cfg(any(
					any(
						feature = "delivery-lead-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"deliveryLeadTime",
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
						feature = "eligible-customer-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"eligibleCustomerType",
				#[cfg(any(
					any(
						feature = "eligible-duration-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"eligibleDuration",
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
						feature = "eligible-region-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"eligibleRegion",
				#[cfg(any(
					any(
						feature = "eligible-transaction-volume-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"eligibleTransactionVolume",
				#[cfg(any(
					any(feature = "gtin-property-schema", feature = "pending-schema-section"),
					doc
				))]
				"gtin",
				#[cfg(any(
					any(
						feature = "gtin-12-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"gtin12",
				#[cfg(any(
					any(
						feature = "gtin-13-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"gtin13",
				#[cfg(any(
					any(
						feature = "gtin-14-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"gtin14",
				#[cfg(any(
					any(feature = "gtin-8-property-schema", feature = "general-schema-section"),
					doc
				))]
				"gtin8",
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
						feature = "includes-object-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"includesObject",
				#[cfg(any(
					any(
						feature = "ineligible-region-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"ineligibleRegion",
				#[cfg(any(
					any(
						feature = "inventory-level-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"inventoryLevel",
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
						feature = "item-offered-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"itemOffered",
				#[cfg(any(
					any(
						feature = "main-entity-of-page-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"mainEntityOfPage",
				#[cfg(any(
					any(feature = "mpn-property-schema", feature = "general-schema-section"),
					doc
				))]
				"mpn",
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
						feature = "price-specification-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"priceSpecification",
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
						feature = "serial-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"serialNumber",
				#[cfg(any(
					any(feature = "sku-property-schema", feature = "general-schema-section"),
					doc
				))]
				"sku",
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
						feature = "warranty-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"warranty",
			];
			deserializer.deserialize_struct("Demand", FIELDS, ClassVisitor)
		}
	}
}
