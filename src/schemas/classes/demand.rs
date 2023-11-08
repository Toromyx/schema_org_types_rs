use super::*;
/// <https://schema.org/Demand>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Demand {
	pub r#accepted_payment_method: Vec<AcceptedPaymentMethodProperty>,
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	pub r#advance_booking_requirement: Vec<AdvanceBookingRequirementProperty>,
	pub r#alternate_name: Vec<AlternateNameProperty>,
	pub r#area_served: Vec<AreaServedProperty>,
	pub r#asin: Vec<AsinProperty>,
	pub r#availability: Vec<AvailabilityProperty>,
	pub r#availability_ends: Vec<AvailabilityEndsProperty>,
	pub r#availability_starts: Vec<AvailabilityStartsProperty>,
	pub r#available_at_or_from: Vec<AvailableAtOrFromProperty>,
	pub r#available_delivery_method: Vec<AvailableDeliveryMethodProperty>,
	pub r#business_function: Vec<BusinessFunctionProperty>,
	pub r#delivery_lead_time: Vec<DeliveryLeadTimeProperty>,
	pub r#description: Vec<DescriptionProperty>,
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	pub r#eligible_customer_type: Vec<EligibleCustomerTypeProperty>,
	pub r#eligible_duration: Vec<EligibleDurationProperty>,
	pub r#eligible_quantity: Vec<EligibleQuantityProperty>,
	pub r#eligible_region: Vec<EligibleRegionProperty>,
	pub r#eligible_transaction_volume: Vec<EligibleTransactionVolumeProperty>,
	pub r#gtin: Vec<GtinProperty>,
	pub r#gtin_12: Vec<Gtin12Property>,
	pub r#gtin_13: Vec<Gtin13Property>,
	pub r#gtin_14: Vec<Gtin14Property>,
	pub r#gtin_8: Vec<Gtin8Property>,
	pub r#identifier: Vec<IdentifierProperty>,
	pub r#image: Vec<ImageProperty>,
	pub r#includes_object: Vec<IncludesObjectProperty>,
	pub r#ineligible_region: Vec<IneligibleRegionProperty>,
	pub r#inventory_level: Vec<InventoryLevelProperty>,
	pub r#item_condition: Vec<ItemConditionProperty>,
	pub r#item_offered: Vec<ItemOfferedProperty>,
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	pub r#mpn: Vec<MpnProperty>,
	pub r#name: Vec<NameProperty>,
	pub r#potential_action: Vec<PotentialActionProperty>,
	pub r#price_specification: Vec<PriceSpecificationProperty>,
	pub r#same_as: Vec<SameAsProperty>,
	pub r#seller: Vec<SellerProperty>,
	pub r#serial_number: Vec<SerialNumberProperty>,
	pub r#sku: Vec<SkuProperty>,
	pub r#subject_of: Vec<SubjectOfProperty>,
	pub r#url: Vec<UrlProperty>,
	pub r#valid_from: Vec<ValidFromProperty>,
	pub r#valid_through: Vec<ValidThroughProperty>,
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
				!Vec::is_empty(&self.r#accepted_payment_method) as usize,
				!Vec::is_empty(&self.r#additional_type) as usize,
				!Vec::is_empty(&self.r#advance_booking_requirement) as usize,
				!Vec::is_empty(&self.r#alternate_name) as usize,
				!Vec::is_empty(&self.r#area_served) as usize,
				!Vec::is_empty(&self.r#asin) as usize,
				!Vec::is_empty(&self.r#availability) as usize,
				!Vec::is_empty(&self.r#availability_ends) as usize,
				!Vec::is_empty(&self.r#availability_starts) as usize,
				!Vec::is_empty(&self.r#available_at_or_from) as usize,
				!Vec::is_empty(&self.r#available_delivery_method) as usize,
				!Vec::is_empty(&self.r#business_function) as usize,
				!Vec::is_empty(&self.r#delivery_lead_time) as usize,
				!Vec::is_empty(&self.r#description) as usize,
				!Vec::is_empty(&self.r#disambiguating_description) as usize,
				!Vec::is_empty(&self.r#eligible_customer_type) as usize,
				!Vec::is_empty(&self.r#eligible_duration) as usize,
				!Vec::is_empty(&self.r#eligible_quantity) as usize,
				!Vec::is_empty(&self.r#eligible_region) as usize,
				!Vec::is_empty(&self.r#eligible_transaction_volume) as usize,
				!Vec::is_empty(&self.r#gtin) as usize,
				!Vec::is_empty(&self.r#gtin_12) as usize,
				!Vec::is_empty(&self.r#gtin_13) as usize,
				!Vec::is_empty(&self.r#gtin_14) as usize,
				!Vec::is_empty(&self.r#gtin_8) as usize,
				!Vec::is_empty(&self.r#identifier) as usize,
				!Vec::is_empty(&self.r#image) as usize,
				!Vec::is_empty(&self.r#includes_object) as usize,
				!Vec::is_empty(&self.r#ineligible_region) as usize,
				!Vec::is_empty(&self.r#inventory_level) as usize,
				!Vec::is_empty(&self.r#item_condition) as usize,
				!Vec::is_empty(&self.r#item_offered) as usize,
				!Vec::is_empty(&self.r#main_entity_of_page) as usize,
				!Vec::is_empty(&self.r#mpn) as usize,
				!Vec::is_empty(&self.r#name) as usize,
				!Vec::is_empty(&self.r#potential_action) as usize,
				!Vec::is_empty(&self.r#price_specification) as usize,
				!Vec::is_empty(&self.r#same_as) as usize,
				!Vec::is_empty(&self.r#seller) as usize,
				!Vec::is_empty(&self.r#serial_number) as usize,
				!Vec::is_empty(&self.r#sku) as usize,
				!Vec::is_empty(&self.r#subject_of) as usize,
				!Vec::is_empty(&self.r#url) as usize,
				!Vec::is_empty(&self.r#valid_from) as usize,
				!Vec::is_empty(&self.r#valid_through) as usize,
				!Vec::is_empty(&self.r#warranty) as usize,
			]
			.iter()
			.sum();
			let mut serialize_struct = Serializer::serialize_struct(serializer, "Demand", len)?;
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
				AcceptedPaymentMethod,
				AdditionalType,
				AdvanceBookingRequirement,
				AlternateName,
				AreaServed,
				Asin,
				Availability,
				AvailabilityEnds,
				AvailabilityStarts,
				AvailableAtOrFrom,
				AvailableDeliveryMethod,
				BusinessFunction,
				DeliveryLeadTime,
				Description,
				DisambiguatingDescription,
				EligibleCustomerType,
				EligibleDuration,
				EligibleQuantity,
				EligibleRegion,
				EligibleTransactionVolume,
				Gtin,
				Gtin12,
				Gtin13,
				Gtin14,
				Gtin8,
				Identifier,
				Image,
				IncludesObject,
				IneligibleRegion,
				InventoryLevel,
				ItemCondition,
				ItemOffered,
				MainEntityOfPage,
				Mpn,
				Name,
				PotentialAction,
				PriceSpecification,
				SameAs,
				Seller,
				SerialNumber,
				Sku,
				SubjectOf,
				Url,
				ValidFrom,
				ValidThrough,
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
						"acceptedPaymentMethod" => Ok(Field::AcceptedPaymentMethod),
						"additionalType" => Ok(Field::AdditionalType),
						"advanceBookingRequirement" => Ok(Field::AdvanceBookingRequirement),
						"alternateName" => Ok(Field::AlternateName),
						"areaServed" => Ok(Field::AreaServed),
						"asin" => Ok(Field::Asin),
						"availability" => Ok(Field::Availability),
						"availabilityEnds" => Ok(Field::AvailabilityEnds),
						"availabilityStarts" => Ok(Field::AvailabilityStarts),
						"availableAtOrFrom" => Ok(Field::AvailableAtOrFrom),
						"availableDeliveryMethod" => Ok(Field::AvailableDeliveryMethod),
						"businessFunction" => Ok(Field::BusinessFunction),
						"deliveryLeadTime" => Ok(Field::DeliveryLeadTime),
						"description" => Ok(Field::Description),
						"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						"eligibleCustomerType" => Ok(Field::EligibleCustomerType),
						"eligibleDuration" => Ok(Field::EligibleDuration),
						"eligibleQuantity" => Ok(Field::EligibleQuantity),
						"eligibleRegion" => Ok(Field::EligibleRegion),
						"eligibleTransactionVolume" => Ok(Field::EligibleTransactionVolume),
						"gtin" => Ok(Field::Gtin),
						"gtin12" => Ok(Field::Gtin12),
						"gtin13" => Ok(Field::Gtin13),
						"gtin14" => Ok(Field::Gtin14),
						"gtin8" => Ok(Field::Gtin8),
						"identifier" => Ok(Field::Identifier),
						"image" => Ok(Field::Image),
						"includesObject" => Ok(Field::IncludesObject),
						"ineligibleRegion" => Ok(Field::IneligibleRegion),
						"inventoryLevel" => Ok(Field::InventoryLevel),
						"itemCondition" => Ok(Field::ItemCondition),
						"itemOffered" => Ok(Field::ItemOffered),
						"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						"mpn" => Ok(Field::Mpn),
						"name" => Ok(Field::Name),
						"potentialAction" => Ok(Field::PotentialAction),
						"priceSpecification" => Ok(Field::PriceSpecification),
						"sameAs" => Ok(Field::SameAs),
						"seller" => Ok(Field::Seller),
						"serialNumber" => Ok(Field::SerialNumber),
						"sku" => Ok(Field::Sku),
						"subjectOf" => Ok(Field::SubjectOf),
						"url" => Ok(Field::Url),
						"validFrom" => Ok(Field::ValidFrom),
						"validThrough" => Ok(Field::ValidThrough),
						"warranty" => Ok(Field::Warranty),
						_ => Ok(Field::Ignore),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"acceptedPaymentMethod" => Ok(Field::AcceptedPaymentMethod),
						b"additionalType" => Ok(Field::AdditionalType),
						b"advanceBookingRequirement" => Ok(Field::AdvanceBookingRequirement),
						b"alternateName" => Ok(Field::AlternateName),
						b"areaServed" => Ok(Field::AreaServed),
						b"asin" => Ok(Field::Asin),
						b"availability" => Ok(Field::Availability),
						b"availabilityEnds" => Ok(Field::AvailabilityEnds),
						b"availabilityStarts" => Ok(Field::AvailabilityStarts),
						b"availableAtOrFrom" => Ok(Field::AvailableAtOrFrom),
						b"availableDeliveryMethod" => Ok(Field::AvailableDeliveryMethod),
						b"businessFunction" => Ok(Field::BusinessFunction),
						b"deliveryLeadTime" => Ok(Field::DeliveryLeadTime),
						b"description" => Ok(Field::Description),
						b"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						b"eligibleCustomerType" => Ok(Field::EligibleCustomerType),
						b"eligibleDuration" => Ok(Field::EligibleDuration),
						b"eligibleQuantity" => Ok(Field::EligibleQuantity),
						b"eligibleRegion" => Ok(Field::EligibleRegion),
						b"eligibleTransactionVolume" => Ok(Field::EligibleTransactionVolume),
						b"gtin" => Ok(Field::Gtin),
						b"gtin12" => Ok(Field::Gtin12),
						b"gtin13" => Ok(Field::Gtin13),
						b"gtin14" => Ok(Field::Gtin14),
						b"gtin8" => Ok(Field::Gtin8),
						b"identifier" => Ok(Field::Identifier),
						b"image" => Ok(Field::Image),
						b"includesObject" => Ok(Field::IncludesObject),
						b"ineligibleRegion" => Ok(Field::IneligibleRegion),
						b"inventoryLevel" => Ok(Field::InventoryLevel),
						b"itemCondition" => Ok(Field::ItemCondition),
						b"itemOffered" => Ok(Field::ItemOffered),
						b"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						b"mpn" => Ok(Field::Mpn),
						b"name" => Ok(Field::Name),
						b"potentialAction" => Ok(Field::PotentialAction),
						b"priceSpecification" => Ok(Field::PriceSpecification),
						b"sameAs" => Ok(Field::SameAs),
						b"seller" => Ok(Field::Seller),
						b"serialNumber" => Ok(Field::SerialNumber),
						b"sku" => Ok(Field::Sku),
						b"subjectOf" => Ok(Field::SubjectOf),
						b"url" => Ok(Field::Url),
						b"validFrom" => Ok(Field::ValidFrom),
						b"validThrough" => Ok(Field::ValidThrough),
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
					let mut r#accepted_payment_method_property = None;
					let mut r#additional_type_property = None;
					let mut r#advance_booking_requirement_property = None;
					let mut r#alternate_name_property = None;
					let mut r#area_served_property = None;
					let mut r#asin_property = None;
					let mut r#availability_property = None;
					let mut r#availability_ends_property = None;
					let mut r#availability_starts_property = None;
					let mut r#available_at_or_from_property = None;
					let mut r#available_delivery_method_property = None;
					let mut r#business_function_property = None;
					let mut r#delivery_lead_time_property = None;
					let mut r#description_property = None;
					let mut r#disambiguating_description_property = None;
					let mut r#eligible_customer_type_property = None;
					let mut r#eligible_duration_property = None;
					let mut r#eligible_quantity_property = None;
					let mut r#eligible_region_property = None;
					let mut r#eligible_transaction_volume_property = None;
					let mut r#gtin_property = None;
					let mut r#gtin_12_property = None;
					let mut r#gtin_13_property = None;
					let mut r#gtin_14_property = None;
					let mut r#gtin_8_property = None;
					let mut r#identifier_property = None;
					let mut r#image_property = None;
					let mut r#includes_object_property = None;
					let mut r#ineligible_region_property = None;
					let mut r#inventory_level_property = None;
					let mut r#item_condition_property = None;
					let mut r#item_offered_property = None;
					let mut r#main_entity_of_page_property = None;
					let mut r#mpn_property = None;
					let mut r#name_property = None;
					let mut r#potential_action_property = None;
					let mut r#price_specification_property = None;
					let mut r#same_as_property = None;
					let mut r#seller_property = None;
					let mut r#serial_number_property = None;
					let mut r#sku_property = None;
					let mut r#subject_of_property = None;
					let mut r#url_property = None;
					let mut r#valid_from_property = None;
					let mut r#valid_through_property = None;
					let mut r#warranty_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
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
						r#accepted_payment_method: r#accepted_payment_method_property
							.unwrap_or_default(),
						r#additional_type: r#additional_type_property.unwrap_or_default(),
						r#advance_booking_requirement: r#advance_booking_requirement_property
							.unwrap_or_default(),
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
						r#area_served: r#area_served_property.unwrap_or_default(),
						r#asin: r#asin_property.unwrap_or_default(),
						r#availability: r#availability_property.unwrap_or_default(),
						r#availability_ends: r#availability_ends_property.unwrap_or_default(),
						r#availability_starts: r#availability_starts_property.unwrap_or_default(),
						r#available_at_or_from: r#available_at_or_from_property.unwrap_or_default(),
						r#available_delivery_method: r#available_delivery_method_property
							.unwrap_or_default(),
						r#business_function: r#business_function_property.unwrap_or_default(),
						r#delivery_lead_time: r#delivery_lead_time_property.unwrap_or_default(),
						r#description: r#description_property.unwrap_or_default(),
						r#disambiguating_description: r#disambiguating_description_property
							.unwrap_or_default(),
						r#eligible_customer_type: r#eligible_customer_type_property
							.unwrap_or_default(),
						r#eligible_duration: r#eligible_duration_property.unwrap_or_default(),
						r#eligible_quantity: r#eligible_quantity_property.unwrap_or_default(),
						r#eligible_region: r#eligible_region_property.unwrap_or_default(),
						r#eligible_transaction_volume: r#eligible_transaction_volume_property
							.unwrap_or_default(),
						r#gtin: r#gtin_property.unwrap_or_default(),
						r#gtin_12: r#gtin_12_property.unwrap_or_default(),
						r#gtin_13: r#gtin_13_property.unwrap_or_default(),
						r#gtin_14: r#gtin_14_property.unwrap_or_default(),
						r#gtin_8: r#gtin_8_property.unwrap_or_default(),
						r#identifier: r#identifier_property.unwrap_or_default(),
						r#image: r#image_property.unwrap_or_default(),
						r#includes_object: r#includes_object_property.unwrap_or_default(),
						r#ineligible_region: r#ineligible_region_property.unwrap_or_default(),
						r#inventory_level: r#inventory_level_property.unwrap_or_default(),
						r#item_condition: r#item_condition_property.unwrap_or_default(),
						r#item_offered: r#item_offered_property.unwrap_or_default(),
						r#main_entity_of_page: r#main_entity_of_page_property.unwrap_or_default(),
						r#mpn: r#mpn_property.unwrap_or_default(),
						r#name: r#name_property.unwrap_or_default(),
						r#potential_action: r#potential_action_property.unwrap_or_default(),
						r#price_specification: r#price_specification_property.unwrap_or_default(),
						r#same_as: r#same_as_property.unwrap_or_default(),
						r#seller: r#seller_property.unwrap_or_default(),
						r#serial_number: r#serial_number_property.unwrap_or_default(),
						r#sku: r#sku_property.unwrap_or_default(),
						r#subject_of: r#subject_of_property.unwrap_or_default(),
						r#url: r#url_property.unwrap_or_default(),
						r#valid_from: r#valid_from_property.unwrap_or_default(),
						r#valid_through: r#valid_through_property.unwrap_or_default(),
						r#warranty: r#warranty_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				"acceptedPaymentMethod",
				"additionalType",
				"advanceBookingRequirement",
				"alternateName",
				"areaServed",
				"asin",
				"availability",
				"availabilityEnds",
				"availabilityStarts",
				"availableAtOrFrom",
				"availableDeliveryMethod",
				"businessFunction",
				"deliveryLeadTime",
				"description",
				"disambiguatingDescription",
				"eligibleCustomerType",
				"eligibleDuration",
				"eligibleQuantity",
				"eligibleRegion",
				"eligibleTransactionVolume",
				"gtin",
				"gtin12",
				"gtin13",
				"gtin14",
				"gtin8",
				"identifier",
				"image",
				"includesObject",
				"ineligibleRegion",
				"inventoryLevel",
				"itemCondition",
				"itemOffered",
				"mainEntityOfPage",
				"mpn",
				"name",
				"potentialAction",
				"priceSpecification",
				"sameAs",
				"seller",
				"serialNumber",
				"sku",
				"subjectOf",
				"url",
				"validFrom",
				"validThrough",
				"warranty",
			];
			deserializer.deserialize_struct("Demand", FIELDS, ClassVisitor)
		}
	}
}
