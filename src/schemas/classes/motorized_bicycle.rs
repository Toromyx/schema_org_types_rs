use super::*;
/// <https://schema.org/MotorizedBicycle>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct MotorizedBicycle {
	pub r#acceleration_time: Vec<AccelerationTimeProperty>,
	pub r#additional_property: Vec<AdditionalPropertyProperty>,
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	pub r#aggregate_rating: Vec<AggregateRatingProperty>,
	pub r#alternate_name: Vec<AlternateNameProperty>,
	pub r#asin: Vec<AsinProperty>,
	pub r#audience: Vec<AudienceProperty>,
	pub r#award: Vec<AwardProperty>,
	pub r#awards: Vec<AwardsProperty>,
	pub r#body_type: Vec<BodyTypeProperty>,
	pub r#brand: Vec<BrandProperty>,
	pub r#call_sign: Vec<CallSignProperty>,
	pub r#cargo_volume: Vec<CargoVolumeProperty>,
	pub r#category: Vec<CategoryProperty>,
	pub r#color: Vec<ColorProperty>,
	pub r#country_of_assembly: Vec<CountryOfAssemblyProperty>,
	pub r#country_of_last_processing: Vec<CountryOfLastProcessingProperty>,
	pub r#country_of_origin: Vec<CountryOfOriginProperty>,
	pub r#date_vehicle_first_registered: Vec<DateVehicleFirstRegisteredProperty>,
	pub r#depth: Vec<DepthProperty>,
	pub r#description: Vec<DescriptionProperty>,
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	pub r#drive_wheel_configuration: Vec<DriveWheelConfigurationProperty>,
	pub r#emissions_co_2: Vec<EmissionsCo2Property>,
	pub r#fuel_capacity: Vec<FuelCapacityProperty>,
	pub r#fuel_consumption: Vec<FuelConsumptionProperty>,
	pub r#fuel_efficiency: Vec<FuelEfficiencyProperty>,
	pub r#fuel_type: Vec<FuelTypeProperty>,
	pub r#funding: Vec<FundingProperty>,
	pub r#gtin: Vec<GtinProperty>,
	pub r#gtin_12: Vec<Gtin12Property>,
	pub r#gtin_13: Vec<Gtin13Property>,
	pub r#gtin_14: Vec<Gtin14Property>,
	pub r#gtin_8: Vec<Gtin8Property>,
	pub r#has_adult_consideration: Vec<HasAdultConsiderationProperty>,
	pub r#has_energy_consumption_details: Vec<HasEnergyConsumptionDetailsProperty>,
	pub r#has_measurement: Vec<HasMeasurementProperty>,
	pub r#has_merchant_return_policy: Vec<HasMerchantReturnPolicyProperty>,
	pub r#has_product_return_policy: Vec<HasProductReturnPolicyProperty>,
	pub r#height: Vec<HeightProperty>,
	pub r#identifier: Vec<IdentifierProperty>,
	pub r#image: Vec<ImageProperty>,
	pub r#in_product_group_with_id: Vec<InProductGroupWithIdProperty>,
	pub r#is_accessory_or_spare_part_for: Vec<IsAccessoryOrSparePartForProperty>,
	pub r#is_consumable_for: Vec<IsConsumableForProperty>,
	pub r#is_family_friendly: Vec<IsFamilyFriendlyProperty>,
	pub r#is_related_to: Vec<IsRelatedToProperty>,
	pub r#is_similar_to: Vec<IsSimilarToProperty>,
	pub r#is_variant_of: Vec<IsVariantOfProperty>,
	pub r#item_condition: Vec<ItemConditionProperty>,
	pub r#keywords: Vec<KeywordsProperty>,
	pub r#known_vehicle_damages: Vec<KnownVehicleDamagesProperty>,
	pub r#logo: Vec<LogoProperty>,
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	pub r#manufacturer: Vec<ManufacturerProperty>,
	pub r#material: Vec<MaterialProperty>,
	pub r#meets_emission_standard: Vec<MeetsEmissionStandardProperty>,
	pub r#mileage_from_odometer: Vec<MileageFromOdometerProperty>,
	pub r#mobile_url: Vec<MobileUrlProperty>,
	pub r#model: Vec<ModelProperty>,
	pub r#model_date: Vec<ModelDateProperty>,
	pub r#mpn: Vec<MpnProperty>,
	pub r#name: Vec<NameProperty>,
	pub r#negative_notes: Vec<NegativeNotesProperty>,
	pub r#nsn: Vec<NsnProperty>,
	pub r#number_of_airbags: Vec<NumberOfAirbagsProperty>,
	pub r#number_of_axles: Vec<NumberOfAxlesProperty>,
	pub r#number_of_doors: Vec<NumberOfDoorsProperty>,
	pub r#number_of_forward_gears: Vec<NumberOfForwardGearsProperty>,
	pub r#number_of_previous_owners: Vec<NumberOfPreviousOwnersProperty>,
	pub r#offers: Vec<OffersProperty>,
	pub r#pattern: Vec<PatternProperty>,
	pub r#payload: Vec<PayloadProperty>,
	pub r#positive_notes: Vec<PositiveNotesProperty>,
	pub r#potential_action: Vec<PotentialActionProperty>,
	pub r#product_id: Vec<ProductIdProperty>,
	pub r#production_date: Vec<ProductionDateProperty>,
	pub r#purchase_date: Vec<PurchaseDateProperty>,
	pub r#release_date: Vec<ReleaseDateProperty>,
	pub r#review: Vec<ReviewProperty>,
	pub r#reviews: Vec<ReviewsProperty>,
	pub r#same_as: Vec<SameAsProperty>,
	pub r#seating_capacity: Vec<SeatingCapacityProperty>,
	pub r#size: Vec<SizeProperty>,
	pub r#sku: Vec<SkuProperty>,
	pub r#slogan: Vec<SloganProperty>,
	pub r#speed: Vec<SpeedProperty>,
	pub r#steering_position: Vec<SteeringPositionProperty>,
	pub r#stupid_property: Vec<StupidPropertyProperty>,
	pub r#subject_of: Vec<SubjectOfProperty>,
	pub r#tongue_weight: Vec<TongueWeightProperty>,
	pub r#trailer_weight: Vec<TrailerWeightProperty>,
	pub r#url: Vec<UrlProperty>,
	pub r#vehicle_configuration: Vec<VehicleConfigurationProperty>,
	pub r#vehicle_engine: Vec<VehicleEngineProperty>,
	pub r#vehicle_identification_number: Vec<VehicleIdentificationNumberProperty>,
	pub r#vehicle_interior_color: Vec<VehicleInteriorColorProperty>,
	pub r#vehicle_interior_type: Vec<VehicleInteriorTypeProperty>,
	pub r#vehicle_model_date: Vec<VehicleModelDateProperty>,
	pub r#vehicle_seating_capacity: Vec<VehicleSeatingCapacityProperty>,
	pub r#vehicle_special_usage: Vec<VehicleSpecialUsageProperty>,
	pub r#vehicle_transmission: Vec<VehicleTransmissionProperty>,
	pub r#weight: Vec<WeightProperty>,
	pub r#weight_total: Vec<WeightTotalProperty>,
	pub r#wheelbase: Vec<WheelbaseProperty>,
	pub r#width: Vec<WidthProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MotorizedBicycle {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#acceleration_time) as usize,
				!Vec::is_empty(&self.r#additional_property) as usize,
				!Vec::is_empty(&self.r#additional_type) as usize,
				!Vec::is_empty(&self.r#aggregate_rating) as usize,
				!Vec::is_empty(&self.r#alternate_name) as usize,
				!Vec::is_empty(&self.r#asin) as usize,
				!Vec::is_empty(&self.r#audience) as usize,
				!Vec::is_empty(&self.r#award) as usize,
				!Vec::is_empty(&self.r#awards) as usize,
				!Vec::is_empty(&self.r#body_type) as usize,
				!Vec::is_empty(&self.r#brand) as usize,
				!Vec::is_empty(&self.r#call_sign) as usize,
				!Vec::is_empty(&self.r#cargo_volume) as usize,
				!Vec::is_empty(&self.r#category) as usize,
				!Vec::is_empty(&self.r#color) as usize,
				!Vec::is_empty(&self.r#country_of_assembly) as usize,
				!Vec::is_empty(&self.r#country_of_last_processing) as usize,
				!Vec::is_empty(&self.r#country_of_origin) as usize,
				!Vec::is_empty(&self.r#date_vehicle_first_registered) as usize,
				!Vec::is_empty(&self.r#depth) as usize,
				!Vec::is_empty(&self.r#description) as usize,
				!Vec::is_empty(&self.r#disambiguating_description) as usize,
				!Vec::is_empty(&self.r#drive_wheel_configuration) as usize,
				!Vec::is_empty(&self.r#emissions_co_2) as usize,
				!Vec::is_empty(&self.r#fuel_capacity) as usize,
				!Vec::is_empty(&self.r#fuel_consumption) as usize,
				!Vec::is_empty(&self.r#fuel_efficiency) as usize,
				!Vec::is_empty(&self.r#fuel_type) as usize,
				!Vec::is_empty(&self.r#funding) as usize,
				!Vec::is_empty(&self.r#gtin) as usize,
				!Vec::is_empty(&self.r#gtin_12) as usize,
				!Vec::is_empty(&self.r#gtin_13) as usize,
				!Vec::is_empty(&self.r#gtin_14) as usize,
				!Vec::is_empty(&self.r#gtin_8) as usize,
				!Vec::is_empty(&self.r#has_adult_consideration) as usize,
				!Vec::is_empty(&self.r#has_energy_consumption_details) as usize,
				!Vec::is_empty(&self.r#has_measurement) as usize,
				!Vec::is_empty(&self.r#has_merchant_return_policy) as usize,
				!Vec::is_empty(&self.r#has_product_return_policy) as usize,
				!Vec::is_empty(&self.r#height) as usize,
				!Vec::is_empty(&self.r#identifier) as usize,
				!Vec::is_empty(&self.r#image) as usize,
				!Vec::is_empty(&self.r#in_product_group_with_id) as usize,
				!Vec::is_empty(&self.r#is_accessory_or_spare_part_for) as usize,
				!Vec::is_empty(&self.r#is_consumable_for) as usize,
				!Vec::is_empty(&self.r#is_family_friendly) as usize,
				!Vec::is_empty(&self.r#is_related_to) as usize,
				!Vec::is_empty(&self.r#is_similar_to) as usize,
				!Vec::is_empty(&self.r#is_variant_of) as usize,
				!Vec::is_empty(&self.r#item_condition) as usize,
				!Vec::is_empty(&self.r#keywords) as usize,
				!Vec::is_empty(&self.r#known_vehicle_damages) as usize,
				!Vec::is_empty(&self.r#logo) as usize,
				!Vec::is_empty(&self.r#main_entity_of_page) as usize,
				!Vec::is_empty(&self.r#manufacturer) as usize,
				!Vec::is_empty(&self.r#material) as usize,
				!Vec::is_empty(&self.r#meets_emission_standard) as usize,
				!Vec::is_empty(&self.r#mileage_from_odometer) as usize,
				!Vec::is_empty(&self.r#mobile_url) as usize,
				!Vec::is_empty(&self.r#model) as usize,
				!Vec::is_empty(&self.r#model_date) as usize,
				!Vec::is_empty(&self.r#mpn) as usize,
				!Vec::is_empty(&self.r#name) as usize,
				!Vec::is_empty(&self.r#negative_notes) as usize,
				!Vec::is_empty(&self.r#nsn) as usize,
				!Vec::is_empty(&self.r#number_of_airbags) as usize,
				!Vec::is_empty(&self.r#number_of_axles) as usize,
				!Vec::is_empty(&self.r#number_of_doors) as usize,
				!Vec::is_empty(&self.r#number_of_forward_gears) as usize,
				!Vec::is_empty(&self.r#number_of_previous_owners) as usize,
				!Vec::is_empty(&self.r#offers) as usize,
				!Vec::is_empty(&self.r#pattern) as usize,
				!Vec::is_empty(&self.r#payload) as usize,
				!Vec::is_empty(&self.r#positive_notes) as usize,
				!Vec::is_empty(&self.r#potential_action) as usize,
				!Vec::is_empty(&self.r#product_id) as usize,
				!Vec::is_empty(&self.r#production_date) as usize,
				!Vec::is_empty(&self.r#purchase_date) as usize,
				!Vec::is_empty(&self.r#release_date) as usize,
				!Vec::is_empty(&self.r#review) as usize,
				!Vec::is_empty(&self.r#reviews) as usize,
				!Vec::is_empty(&self.r#same_as) as usize,
				!Vec::is_empty(&self.r#seating_capacity) as usize,
				!Vec::is_empty(&self.r#size) as usize,
				!Vec::is_empty(&self.r#sku) as usize,
				!Vec::is_empty(&self.r#slogan) as usize,
				!Vec::is_empty(&self.r#speed) as usize,
				!Vec::is_empty(&self.r#steering_position) as usize,
				!Vec::is_empty(&self.r#stupid_property) as usize,
				!Vec::is_empty(&self.r#subject_of) as usize,
				!Vec::is_empty(&self.r#tongue_weight) as usize,
				!Vec::is_empty(&self.r#trailer_weight) as usize,
				!Vec::is_empty(&self.r#url) as usize,
				!Vec::is_empty(&self.r#vehicle_configuration) as usize,
				!Vec::is_empty(&self.r#vehicle_engine) as usize,
				!Vec::is_empty(&self.r#vehicle_identification_number) as usize,
				!Vec::is_empty(&self.r#vehicle_interior_color) as usize,
				!Vec::is_empty(&self.r#vehicle_interior_type) as usize,
				!Vec::is_empty(&self.r#vehicle_model_date) as usize,
				!Vec::is_empty(&self.r#vehicle_seating_capacity) as usize,
				!Vec::is_empty(&self.r#vehicle_special_usage) as usize,
				!Vec::is_empty(&self.r#vehicle_transmission) as usize,
				!Vec::is_empty(&self.r#weight) as usize,
				!Vec::is_empty(&self.r#weight_total) as usize,
				!Vec::is_empty(&self.r#wheelbase) as usize,
				!Vec::is_empty(&self.r#width) as usize,
			]
			.iter()
			.sum();
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "MotorizedBicycle", len)?;
			if !Vec::is_empty(&self.r#acceleration_time) {
				serialize_struct.serialize_field("accelerationTime", {
					struct SerializeWith<'a>(&'a Vec<AccelerationTimeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#acceleration_time)
				})?;
			} else {
				serialize_struct.skip_field("accelerationTime")?;
			}
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
			if !Vec::is_empty(&self.r#aggregate_rating) {
				serialize_struct.serialize_field("aggregateRating", {
					struct SerializeWith<'a>(&'a Vec<AggregateRatingProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#aggregate_rating)
				})?;
			} else {
				serialize_struct.skip_field("aggregateRating")?;
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
			if !Vec::is_empty(&self.r#audience) {
				serialize_struct.serialize_field("audience", {
					struct SerializeWith<'a>(&'a Vec<AudienceProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#audience)
				})?;
			} else {
				serialize_struct.skip_field("audience")?;
			}
			if !Vec::is_empty(&self.r#award) {
				serialize_struct.serialize_field("award", {
					struct SerializeWith<'a>(&'a Vec<AwardProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#award)
				})?;
			} else {
				serialize_struct.skip_field("award")?;
			}
			if !Vec::is_empty(&self.r#awards) {
				serialize_struct.serialize_field("awards", {
					struct SerializeWith<'a>(&'a Vec<AwardsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#awards)
				})?;
			} else {
				serialize_struct.skip_field("awards")?;
			}
			if !Vec::is_empty(&self.r#body_type) {
				serialize_struct.serialize_field("bodyType", {
					struct SerializeWith<'a>(&'a Vec<BodyTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#body_type)
				})?;
			} else {
				serialize_struct.skip_field("bodyType")?;
			}
			if !Vec::is_empty(&self.r#brand) {
				serialize_struct.serialize_field("brand", {
					struct SerializeWith<'a>(&'a Vec<BrandProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#brand)
				})?;
			} else {
				serialize_struct.skip_field("brand")?;
			}
			if !Vec::is_empty(&self.r#call_sign) {
				serialize_struct.serialize_field("callSign", {
					struct SerializeWith<'a>(&'a Vec<CallSignProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#call_sign)
				})?;
			} else {
				serialize_struct.skip_field("callSign")?;
			}
			if !Vec::is_empty(&self.r#cargo_volume) {
				serialize_struct.serialize_field("cargoVolume", {
					struct SerializeWith<'a>(&'a Vec<CargoVolumeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cargo_volume)
				})?;
			} else {
				serialize_struct.skip_field("cargoVolume")?;
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
			if !Vec::is_empty(&self.r#color) {
				serialize_struct.serialize_field("color", {
					struct SerializeWith<'a>(&'a Vec<ColorProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#color)
				})?;
			} else {
				serialize_struct.skip_field("color")?;
			}
			if !Vec::is_empty(&self.r#country_of_assembly) {
				serialize_struct.serialize_field("countryOfAssembly", {
					struct SerializeWith<'a>(&'a Vec<CountryOfAssemblyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#country_of_assembly)
				})?;
			} else {
				serialize_struct.skip_field("countryOfAssembly")?;
			}
			if !Vec::is_empty(&self.r#country_of_last_processing) {
				serialize_struct.serialize_field("countryOfLastProcessing", {
					struct SerializeWith<'a>(&'a Vec<CountryOfLastProcessingProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#country_of_last_processing)
				})?;
			} else {
				serialize_struct.skip_field("countryOfLastProcessing")?;
			}
			if !Vec::is_empty(&self.r#country_of_origin) {
				serialize_struct.serialize_field("countryOfOrigin", {
					struct SerializeWith<'a>(&'a Vec<CountryOfOriginProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#country_of_origin)
				})?;
			} else {
				serialize_struct.skip_field("countryOfOrigin")?;
			}
			if !Vec::is_empty(&self.r#date_vehicle_first_registered) {
				serialize_struct.serialize_field("dateVehicleFirstRegistered", {
					struct SerializeWith<'a>(&'a Vec<DateVehicleFirstRegisteredProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#date_vehicle_first_registered)
				})?;
			} else {
				serialize_struct.skip_field("dateVehicleFirstRegistered")?;
			}
			if !Vec::is_empty(&self.r#depth) {
				serialize_struct.serialize_field("depth", {
					struct SerializeWith<'a>(&'a Vec<DepthProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#depth)
				})?;
			} else {
				serialize_struct.skip_field("depth")?;
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
			if !Vec::is_empty(&self.r#drive_wheel_configuration) {
				serialize_struct.serialize_field("driveWheelConfiguration", {
					struct SerializeWith<'a>(&'a Vec<DriveWheelConfigurationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#drive_wheel_configuration)
				})?;
			} else {
				serialize_struct.skip_field("driveWheelConfiguration")?;
			}
			if !Vec::is_empty(&self.r#emissions_co_2) {
				serialize_struct.serialize_field("emissionsCO2", {
					struct SerializeWith<'a>(&'a Vec<EmissionsCo2Property>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#emissions_co_2)
				})?;
			} else {
				serialize_struct.skip_field("emissionsCO2")?;
			}
			if !Vec::is_empty(&self.r#fuel_capacity) {
				serialize_struct.serialize_field("fuelCapacity", {
					struct SerializeWith<'a>(&'a Vec<FuelCapacityProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#fuel_capacity)
				})?;
			} else {
				serialize_struct.skip_field("fuelCapacity")?;
			}
			if !Vec::is_empty(&self.r#fuel_consumption) {
				serialize_struct.serialize_field("fuelConsumption", {
					struct SerializeWith<'a>(&'a Vec<FuelConsumptionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#fuel_consumption)
				})?;
			} else {
				serialize_struct.skip_field("fuelConsumption")?;
			}
			if !Vec::is_empty(&self.r#fuel_efficiency) {
				serialize_struct.serialize_field("fuelEfficiency", {
					struct SerializeWith<'a>(&'a Vec<FuelEfficiencyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#fuel_efficiency)
				})?;
			} else {
				serialize_struct.skip_field("fuelEfficiency")?;
			}
			if !Vec::is_empty(&self.r#fuel_type) {
				serialize_struct.serialize_field("fuelType", {
					struct SerializeWith<'a>(&'a Vec<FuelTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#fuel_type)
				})?;
			} else {
				serialize_struct.skip_field("fuelType")?;
			}
			if !Vec::is_empty(&self.r#funding) {
				serialize_struct.serialize_field("funding", {
					struct SerializeWith<'a>(&'a Vec<FundingProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#funding)
				})?;
			} else {
				serialize_struct.skip_field("funding")?;
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
			if !Vec::is_empty(&self.r#has_adult_consideration) {
				serialize_struct.serialize_field("hasAdultConsideration", {
					struct SerializeWith<'a>(&'a Vec<HasAdultConsiderationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_adult_consideration)
				})?;
			} else {
				serialize_struct.skip_field("hasAdultConsideration")?;
			}
			if !Vec::is_empty(&self.r#has_energy_consumption_details) {
				serialize_struct.serialize_field("hasEnergyConsumptionDetails", {
					struct SerializeWith<'a>(&'a Vec<HasEnergyConsumptionDetailsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_energy_consumption_details)
				})?;
			} else {
				serialize_struct.skip_field("hasEnergyConsumptionDetails")?;
			}
			if !Vec::is_empty(&self.r#has_measurement) {
				serialize_struct.serialize_field("hasMeasurement", {
					struct SerializeWith<'a>(&'a Vec<HasMeasurementProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_measurement)
				})?;
			} else {
				serialize_struct.skip_field("hasMeasurement")?;
			}
			if !Vec::is_empty(&self.r#has_merchant_return_policy) {
				serialize_struct.serialize_field("hasMerchantReturnPolicy", {
					struct SerializeWith<'a>(&'a Vec<HasMerchantReturnPolicyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_merchant_return_policy)
				})?;
			} else {
				serialize_struct.skip_field("hasMerchantReturnPolicy")?;
			}
			if !Vec::is_empty(&self.r#has_product_return_policy) {
				serialize_struct.serialize_field("hasProductReturnPolicy", {
					struct SerializeWith<'a>(&'a Vec<HasProductReturnPolicyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_product_return_policy)
				})?;
			} else {
				serialize_struct.skip_field("hasProductReturnPolicy")?;
			}
			if !Vec::is_empty(&self.r#height) {
				serialize_struct.serialize_field("height", {
					struct SerializeWith<'a>(&'a Vec<HeightProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#height)
				})?;
			} else {
				serialize_struct.skip_field("height")?;
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
			if !Vec::is_empty(&self.r#in_product_group_with_id) {
				serialize_struct.serialize_field("inProductGroupWithID", {
					struct SerializeWith<'a>(&'a Vec<InProductGroupWithIdProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#in_product_group_with_id)
				})?;
			} else {
				serialize_struct.skip_field("inProductGroupWithID")?;
			}
			if !Vec::is_empty(&self.r#is_accessory_or_spare_part_for) {
				serialize_struct.serialize_field("isAccessoryOrSparePartFor", {
					struct SerializeWith<'a>(&'a Vec<IsAccessoryOrSparePartForProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_accessory_or_spare_part_for)
				})?;
			} else {
				serialize_struct.skip_field("isAccessoryOrSparePartFor")?;
			}
			if !Vec::is_empty(&self.r#is_consumable_for) {
				serialize_struct.serialize_field("isConsumableFor", {
					struct SerializeWith<'a>(&'a Vec<IsConsumableForProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_consumable_for)
				})?;
			} else {
				serialize_struct.skip_field("isConsumableFor")?;
			}
			if !Vec::is_empty(&self.r#is_family_friendly) {
				serialize_struct.serialize_field("isFamilyFriendly", {
					struct SerializeWith<'a>(&'a Vec<IsFamilyFriendlyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_family_friendly)
				})?;
			} else {
				serialize_struct.skip_field("isFamilyFriendly")?;
			}
			if !Vec::is_empty(&self.r#is_related_to) {
				serialize_struct.serialize_field("isRelatedTo", {
					struct SerializeWith<'a>(&'a Vec<IsRelatedToProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_related_to)
				})?;
			} else {
				serialize_struct.skip_field("isRelatedTo")?;
			}
			if !Vec::is_empty(&self.r#is_similar_to) {
				serialize_struct.serialize_field("isSimilarTo", {
					struct SerializeWith<'a>(&'a Vec<IsSimilarToProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_similar_to)
				})?;
			} else {
				serialize_struct.skip_field("isSimilarTo")?;
			}
			if !Vec::is_empty(&self.r#is_variant_of) {
				serialize_struct.serialize_field("isVariantOf", {
					struct SerializeWith<'a>(&'a Vec<IsVariantOfProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_variant_of)
				})?;
			} else {
				serialize_struct.skip_field("isVariantOf")?;
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
			if !Vec::is_empty(&self.r#keywords) {
				serialize_struct.serialize_field("keywords", {
					struct SerializeWith<'a>(&'a Vec<KeywordsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#keywords)
				})?;
			} else {
				serialize_struct.skip_field("keywords")?;
			}
			if !Vec::is_empty(&self.r#known_vehicle_damages) {
				serialize_struct.serialize_field("knownVehicleDamages", {
					struct SerializeWith<'a>(&'a Vec<KnownVehicleDamagesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#known_vehicle_damages)
				})?;
			} else {
				serialize_struct.skip_field("knownVehicleDamages")?;
			}
			if !Vec::is_empty(&self.r#logo) {
				serialize_struct.serialize_field("logo", {
					struct SerializeWith<'a>(&'a Vec<LogoProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#logo)
				})?;
			} else {
				serialize_struct.skip_field("logo")?;
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
			if !Vec::is_empty(&self.r#manufacturer) {
				serialize_struct.serialize_field("manufacturer", {
					struct SerializeWith<'a>(&'a Vec<ManufacturerProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#manufacturer)
				})?;
			} else {
				serialize_struct.skip_field("manufacturer")?;
			}
			if !Vec::is_empty(&self.r#material) {
				serialize_struct.serialize_field("material", {
					struct SerializeWith<'a>(&'a Vec<MaterialProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#material)
				})?;
			} else {
				serialize_struct.skip_field("material")?;
			}
			if !Vec::is_empty(&self.r#meets_emission_standard) {
				serialize_struct.serialize_field("meetsEmissionStandard", {
					struct SerializeWith<'a>(&'a Vec<MeetsEmissionStandardProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#meets_emission_standard)
				})?;
			} else {
				serialize_struct.skip_field("meetsEmissionStandard")?;
			}
			if !Vec::is_empty(&self.r#mileage_from_odometer) {
				serialize_struct.serialize_field("mileageFromOdometer", {
					struct SerializeWith<'a>(&'a Vec<MileageFromOdometerProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#mileage_from_odometer)
				})?;
			} else {
				serialize_struct.skip_field("mileageFromOdometer")?;
			}
			if !Vec::is_empty(&self.r#mobile_url) {
				serialize_struct.serialize_field("mobileUrl", {
					struct SerializeWith<'a>(&'a Vec<MobileUrlProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#mobile_url)
				})?;
			} else {
				serialize_struct.skip_field("mobileUrl")?;
			}
			if !Vec::is_empty(&self.r#model) {
				serialize_struct.serialize_field("model", {
					struct SerializeWith<'a>(&'a Vec<ModelProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#model)
				})?;
			} else {
				serialize_struct.skip_field("model")?;
			}
			if !Vec::is_empty(&self.r#model_date) {
				serialize_struct.serialize_field("modelDate", {
					struct SerializeWith<'a>(&'a Vec<ModelDateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#model_date)
				})?;
			} else {
				serialize_struct.skip_field("modelDate")?;
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
			if !Vec::is_empty(&self.r#negative_notes) {
				serialize_struct.serialize_field("negativeNotes", {
					struct SerializeWith<'a>(&'a Vec<NegativeNotesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#negative_notes)
				})?;
			} else {
				serialize_struct.skip_field("negativeNotes")?;
			}
			if !Vec::is_empty(&self.r#nsn) {
				serialize_struct.serialize_field("nsn", {
					struct SerializeWith<'a>(&'a Vec<NsnProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#nsn)
				})?;
			} else {
				serialize_struct.skip_field("nsn")?;
			}
			if !Vec::is_empty(&self.r#number_of_airbags) {
				serialize_struct.serialize_field("numberOfAirbags", {
					struct SerializeWith<'a>(&'a Vec<NumberOfAirbagsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#number_of_airbags)
				})?;
			} else {
				serialize_struct.skip_field("numberOfAirbags")?;
			}
			if !Vec::is_empty(&self.r#number_of_axles) {
				serialize_struct.serialize_field("numberOfAxles", {
					struct SerializeWith<'a>(&'a Vec<NumberOfAxlesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#number_of_axles)
				})?;
			} else {
				serialize_struct.skip_field("numberOfAxles")?;
			}
			if !Vec::is_empty(&self.r#number_of_doors) {
				serialize_struct.serialize_field("numberOfDoors", {
					struct SerializeWith<'a>(&'a Vec<NumberOfDoorsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#number_of_doors)
				})?;
			} else {
				serialize_struct.skip_field("numberOfDoors")?;
			}
			if !Vec::is_empty(&self.r#number_of_forward_gears) {
				serialize_struct.serialize_field("numberOfForwardGears", {
					struct SerializeWith<'a>(&'a Vec<NumberOfForwardGearsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#number_of_forward_gears)
				})?;
			} else {
				serialize_struct.skip_field("numberOfForwardGears")?;
			}
			if !Vec::is_empty(&self.r#number_of_previous_owners) {
				serialize_struct.serialize_field("numberOfPreviousOwners", {
					struct SerializeWith<'a>(&'a Vec<NumberOfPreviousOwnersProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#number_of_previous_owners)
				})?;
			} else {
				serialize_struct.skip_field("numberOfPreviousOwners")?;
			}
			if !Vec::is_empty(&self.r#offers) {
				serialize_struct.serialize_field("offers", {
					struct SerializeWith<'a>(&'a Vec<OffersProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#offers)
				})?;
			} else {
				serialize_struct.skip_field("offers")?;
			}
			if !Vec::is_empty(&self.r#pattern) {
				serialize_struct.serialize_field("pattern", {
					struct SerializeWith<'a>(&'a Vec<PatternProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#pattern)
				})?;
			} else {
				serialize_struct.skip_field("pattern")?;
			}
			if !Vec::is_empty(&self.r#payload) {
				serialize_struct.serialize_field("payload", {
					struct SerializeWith<'a>(&'a Vec<PayloadProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#payload)
				})?;
			} else {
				serialize_struct.skip_field("payload")?;
			}
			if !Vec::is_empty(&self.r#positive_notes) {
				serialize_struct.serialize_field("positiveNotes", {
					struct SerializeWith<'a>(&'a Vec<PositiveNotesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#positive_notes)
				})?;
			} else {
				serialize_struct.skip_field("positiveNotes")?;
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
			if !Vec::is_empty(&self.r#product_id) {
				serialize_struct.serialize_field("productID", {
					struct SerializeWith<'a>(&'a Vec<ProductIdProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#product_id)
				})?;
			} else {
				serialize_struct.skip_field("productID")?;
			}
			if !Vec::is_empty(&self.r#production_date) {
				serialize_struct.serialize_field("productionDate", {
					struct SerializeWith<'a>(&'a Vec<ProductionDateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#production_date)
				})?;
			} else {
				serialize_struct.skip_field("productionDate")?;
			}
			if !Vec::is_empty(&self.r#purchase_date) {
				serialize_struct.serialize_field("purchaseDate", {
					struct SerializeWith<'a>(&'a Vec<PurchaseDateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#purchase_date)
				})?;
			} else {
				serialize_struct.skip_field("purchaseDate")?;
			}
			if !Vec::is_empty(&self.r#release_date) {
				serialize_struct.serialize_field("releaseDate", {
					struct SerializeWith<'a>(&'a Vec<ReleaseDateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#release_date)
				})?;
			} else {
				serialize_struct.skip_field("releaseDate")?;
			}
			if !Vec::is_empty(&self.r#review) {
				serialize_struct.serialize_field("review", {
					struct SerializeWith<'a>(&'a Vec<ReviewProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#review)
				})?;
			} else {
				serialize_struct.skip_field("review")?;
			}
			if !Vec::is_empty(&self.r#reviews) {
				serialize_struct.serialize_field("reviews", {
					struct SerializeWith<'a>(&'a Vec<ReviewsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#reviews)
				})?;
			} else {
				serialize_struct.skip_field("reviews")?;
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
			if !Vec::is_empty(&self.r#seating_capacity) {
				serialize_struct.serialize_field("seatingCapacity", {
					struct SerializeWith<'a>(&'a Vec<SeatingCapacityProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#seating_capacity)
				})?;
			} else {
				serialize_struct.skip_field("seatingCapacity")?;
			}
			if !Vec::is_empty(&self.r#size) {
				serialize_struct.serialize_field("size", {
					struct SerializeWith<'a>(&'a Vec<SizeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#size)
				})?;
			} else {
				serialize_struct.skip_field("size")?;
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
			if !Vec::is_empty(&self.r#slogan) {
				serialize_struct.serialize_field("slogan", {
					struct SerializeWith<'a>(&'a Vec<SloganProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#slogan)
				})?;
			} else {
				serialize_struct.skip_field("slogan")?;
			}
			if !Vec::is_empty(&self.r#speed) {
				serialize_struct.serialize_field("speed", {
					struct SerializeWith<'a>(&'a Vec<SpeedProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#speed)
				})?;
			} else {
				serialize_struct.skip_field("speed")?;
			}
			if !Vec::is_empty(&self.r#steering_position) {
				serialize_struct.serialize_field("steeringPosition", {
					struct SerializeWith<'a>(&'a Vec<SteeringPositionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#steering_position)
				})?;
			} else {
				serialize_struct.skip_field("steeringPosition")?;
			}
			if !Vec::is_empty(&self.r#stupid_property) {
				serialize_struct.serialize_field("stupidProperty", {
					struct SerializeWith<'a>(&'a Vec<StupidPropertyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#stupid_property)
				})?;
			} else {
				serialize_struct.skip_field("stupidProperty")?;
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
			if !Vec::is_empty(&self.r#tongue_weight) {
				serialize_struct.serialize_field("tongueWeight", {
					struct SerializeWith<'a>(&'a Vec<TongueWeightProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#tongue_weight)
				})?;
			} else {
				serialize_struct.skip_field("tongueWeight")?;
			}
			if !Vec::is_empty(&self.r#trailer_weight) {
				serialize_struct.serialize_field("trailerWeight", {
					struct SerializeWith<'a>(&'a Vec<TrailerWeightProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#trailer_weight)
				})?;
			} else {
				serialize_struct.skip_field("trailerWeight")?;
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
			if !Vec::is_empty(&self.r#vehicle_configuration) {
				serialize_struct.serialize_field("vehicleConfiguration", {
					struct SerializeWith<'a>(&'a Vec<VehicleConfigurationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_configuration)
				})?;
			} else {
				serialize_struct.skip_field("vehicleConfiguration")?;
			}
			if !Vec::is_empty(&self.r#vehicle_engine) {
				serialize_struct.serialize_field("vehicleEngine", {
					struct SerializeWith<'a>(&'a Vec<VehicleEngineProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_engine)
				})?;
			} else {
				serialize_struct.skip_field("vehicleEngine")?;
			}
			if !Vec::is_empty(&self.r#vehicle_identification_number) {
				serialize_struct.serialize_field("vehicleIdentificationNumber", {
					struct SerializeWith<'a>(&'a Vec<VehicleIdentificationNumberProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_identification_number)
				})?;
			} else {
				serialize_struct.skip_field("vehicleIdentificationNumber")?;
			}
			if !Vec::is_empty(&self.r#vehicle_interior_color) {
				serialize_struct.serialize_field("vehicleInteriorColor", {
					struct SerializeWith<'a>(&'a Vec<VehicleInteriorColorProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_interior_color)
				})?;
			} else {
				serialize_struct.skip_field("vehicleInteriorColor")?;
			}
			if !Vec::is_empty(&self.r#vehicle_interior_type) {
				serialize_struct.serialize_field("vehicleInteriorType", {
					struct SerializeWith<'a>(&'a Vec<VehicleInteriorTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_interior_type)
				})?;
			} else {
				serialize_struct.skip_field("vehicleInteriorType")?;
			}
			if !Vec::is_empty(&self.r#vehicle_model_date) {
				serialize_struct.serialize_field("vehicleModelDate", {
					struct SerializeWith<'a>(&'a Vec<VehicleModelDateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_model_date)
				})?;
			} else {
				serialize_struct.skip_field("vehicleModelDate")?;
			}
			if !Vec::is_empty(&self.r#vehicle_seating_capacity) {
				serialize_struct.serialize_field("vehicleSeatingCapacity", {
					struct SerializeWith<'a>(&'a Vec<VehicleSeatingCapacityProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_seating_capacity)
				})?;
			} else {
				serialize_struct.skip_field("vehicleSeatingCapacity")?;
			}
			if !Vec::is_empty(&self.r#vehicle_special_usage) {
				serialize_struct.serialize_field("vehicleSpecialUsage", {
					struct SerializeWith<'a>(&'a Vec<VehicleSpecialUsageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_special_usage)
				})?;
			} else {
				serialize_struct.skip_field("vehicleSpecialUsage")?;
			}
			if !Vec::is_empty(&self.r#vehicle_transmission) {
				serialize_struct.serialize_field("vehicleTransmission", {
					struct SerializeWith<'a>(&'a Vec<VehicleTransmissionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_transmission)
				})?;
			} else {
				serialize_struct.skip_field("vehicleTransmission")?;
			}
			if !Vec::is_empty(&self.r#weight) {
				serialize_struct.serialize_field("weight", {
					struct SerializeWith<'a>(&'a Vec<WeightProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#weight)
				})?;
			} else {
				serialize_struct.skip_field("weight")?;
			}
			if !Vec::is_empty(&self.r#weight_total) {
				serialize_struct.serialize_field("weightTotal", {
					struct SerializeWith<'a>(&'a Vec<WeightTotalProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#weight_total)
				})?;
			} else {
				serialize_struct.skip_field("weightTotal")?;
			}
			if !Vec::is_empty(&self.r#wheelbase) {
				serialize_struct.serialize_field("wheelbase", {
					struct SerializeWith<'a>(&'a Vec<WheelbaseProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#wheelbase)
				})?;
			} else {
				serialize_struct.skip_field("wheelbase")?;
			}
			if !Vec::is_empty(&self.r#width) {
				serialize_struct.serialize_field("width", {
					struct SerializeWith<'a>(&'a Vec<WidthProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#width)
				})?;
			} else {
				serialize_struct.skip_field("width")?;
			}
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for MotorizedBicycle {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AccelerationTime,
				AdditionalProperty,
				AdditionalType,
				AggregateRating,
				AlternateName,
				Asin,
				Audience,
				Award,
				Awards,
				BodyType,
				Brand,
				CallSign,
				CargoVolume,
				Category,
				Color,
				CountryOfAssembly,
				CountryOfLastProcessing,
				CountryOfOrigin,
				DateVehicleFirstRegistered,
				Depth,
				Description,
				DisambiguatingDescription,
				DriveWheelConfiguration,
				EmissionsCo2,
				FuelCapacity,
				FuelConsumption,
				FuelEfficiency,
				FuelType,
				Funding,
				Gtin,
				Gtin12,
				Gtin13,
				Gtin14,
				Gtin8,
				HasAdultConsideration,
				HasEnergyConsumptionDetails,
				HasMeasurement,
				HasMerchantReturnPolicy,
				HasProductReturnPolicy,
				Height,
				Identifier,
				Image,
				InProductGroupWithId,
				IsAccessoryOrSparePartFor,
				IsConsumableFor,
				IsFamilyFriendly,
				IsRelatedTo,
				IsSimilarTo,
				IsVariantOf,
				ItemCondition,
				Keywords,
				KnownVehicleDamages,
				Logo,
				MainEntityOfPage,
				Manufacturer,
				Material,
				MeetsEmissionStandard,
				MileageFromOdometer,
				MobileUrl,
				Model,
				ModelDate,
				Mpn,
				Name,
				NegativeNotes,
				Nsn,
				NumberOfAirbags,
				NumberOfAxles,
				NumberOfDoors,
				NumberOfForwardGears,
				NumberOfPreviousOwners,
				Offers,
				Pattern,
				Payload,
				PositiveNotes,
				PotentialAction,
				ProductId,
				ProductionDate,
				PurchaseDate,
				ReleaseDate,
				Review,
				Reviews,
				SameAs,
				SeatingCapacity,
				Size,
				Sku,
				Slogan,
				Speed,
				SteeringPosition,
				StupidProperty,
				SubjectOf,
				TongueWeight,
				TrailerWeight,
				Url,
				VehicleConfiguration,
				VehicleEngine,
				VehicleIdentificationNumber,
				VehicleInteriorColor,
				VehicleInteriorType,
				VehicleModelDate,
				VehicleSeatingCapacity,
				VehicleSpecialUsage,
				VehicleTransmission,
				Weight,
				WeightTotal,
				Wheelbase,
				Width,
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
						"accelerationTime" => Ok(Field::AccelerationTime),
						"additionalProperty" => Ok(Field::AdditionalProperty),
						"additionalType" => Ok(Field::AdditionalType),
						"aggregateRating" => Ok(Field::AggregateRating),
						"alternateName" => Ok(Field::AlternateName),
						"asin" => Ok(Field::Asin),
						"audience" => Ok(Field::Audience),
						"award" => Ok(Field::Award),
						"awards" => Ok(Field::Awards),
						"bodyType" => Ok(Field::BodyType),
						"brand" => Ok(Field::Brand),
						"callSign" => Ok(Field::CallSign),
						"cargoVolume" => Ok(Field::CargoVolume),
						"category" => Ok(Field::Category),
						"color" => Ok(Field::Color),
						"countryOfAssembly" => Ok(Field::CountryOfAssembly),
						"countryOfLastProcessing" => Ok(Field::CountryOfLastProcessing),
						"countryOfOrigin" => Ok(Field::CountryOfOrigin),
						"dateVehicleFirstRegistered" => Ok(Field::DateVehicleFirstRegistered),
						"depth" => Ok(Field::Depth),
						"description" => Ok(Field::Description),
						"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						"driveWheelConfiguration" => Ok(Field::DriveWheelConfiguration),
						"emissionsCO2" => Ok(Field::EmissionsCo2),
						"fuelCapacity" => Ok(Field::FuelCapacity),
						"fuelConsumption" => Ok(Field::FuelConsumption),
						"fuelEfficiency" => Ok(Field::FuelEfficiency),
						"fuelType" => Ok(Field::FuelType),
						"funding" => Ok(Field::Funding),
						"gtin" => Ok(Field::Gtin),
						"gtin12" => Ok(Field::Gtin12),
						"gtin13" => Ok(Field::Gtin13),
						"gtin14" => Ok(Field::Gtin14),
						"gtin8" => Ok(Field::Gtin8),
						"hasAdultConsideration" => Ok(Field::HasAdultConsideration),
						"hasEnergyConsumptionDetails" => Ok(Field::HasEnergyConsumptionDetails),
						"hasMeasurement" => Ok(Field::HasMeasurement),
						"hasMerchantReturnPolicy" => Ok(Field::HasMerchantReturnPolicy),
						"hasProductReturnPolicy" => Ok(Field::HasProductReturnPolicy),
						"height" => Ok(Field::Height),
						"identifier" => Ok(Field::Identifier),
						"image" => Ok(Field::Image),
						"inProductGroupWithID" => Ok(Field::InProductGroupWithId),
						"isAccessoryOrSparePartFor" => Ok(Field::IsAccessoryOrSparePartFor),
						"isConsumableFor" => Ok(Field::IsConsumableFor),
						"isFamilyFriendly" => Ok(Field::IsFamilyFriendly),
						"isRelatedTo" => Ok(Field::IsRelatedTo),
						"isSimilarTo" => Ok(Field::IsSimilarTo),
						"isVariantOf" => Ok(Field::IsVariantOf),
						"itemCondition" => Ok(Field::ItemCondition),
						"keywords" => Ok(Field::Keywords),
						"knownVehicleDamages" => Ok(Field::KnownVehicleDamages),
						"logo" => Ok(Field::Logo),
						"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						"manufacturer" => Ok(Field::Manufacturer),
						"material" => Ok(Field::Material),
						"meetsEmissionStandard" => Ok(Field::MeetsEmissionStandard),
						"mileageFromOdometer" => Ok(Field::MileageFromOdometer),
						"mobileUrl" => Ok(Field::MobileUrl),
						"model" => Ok(Field::Model),
						"modelDate" => Ok(Field::ModelDate),
						"mpn" => Ok(Field::Mpn),
						"name" => Ok(Field::Name),
						"negativeNotes" => Ok(Field::NegativeNotes),
						"nsn" => Ok(Field::Nsn),
						"numberOfAirbags" => Ok(Field::NumberOfAirbags),
						"numberOfAxles" => Ok(Field::NumberOfAxles),
						"numberOfDoors" => Ok(Field::NumberOfDoors),
						"numberOfForwardGears" => Ok(Field::NumberOfForwardGears),
						"numberOfPreviousOwners" => Ok(Field::NumberOfPreviousOwners),
						"offers" => Ok(Field::Offers),
						"pattern" => Ok(Field::Pattern),
						"payload" => Ok(Field::Payload),
						"positiveNotes" => Ok(Field::PositiveNotes),
						"potentialAction" => Ok(Field::PotentialAction),
						"productID" => Ok(Field::ProductId),
						"productionDate" => Ok(Field::ProductionDate),
						"purchaseDate" => Ok(Field::PurchaseDate),
						"releaseDate" => Ok(Field::ReleaseDate),
						"review" => Ok(Field::Review),
						"reviews" => Ok(Field::Reviews),
						"sameAs" => Ok(Field::SameAs),
						"seatingCapacity" => Ok(Field::SeatingCapacity),
						"size" => Ok(Field::Size),
						"sku" => Ok(Field::Sku),
						"slogan" => Ok(Field::Slogan),
						"speed" => Ok(Field::Speed),
						"steeringPosition" => Ok(Field::SteeringPosition),
						"stupidProperty" => Ok(Field::StupidProperty),
						"subjectOf" => Ok(Field::SubjectOf),
						"tongueWeight" => Ok(Field::TongueWeight),
						"trailerWeight" => Ok(Field::TrailerWeight),
						"url" => Ok(Field::Url),
						"vehicleConfiguration" => Ok(Field::VehicleConfiguration),
						"vehicleEngine" => Ok(Field::VehicleEngine),
						"vehicleIdentificationNumber" => Ok(Field::VehicleIdentificationNumber),
						"vehicleInteriorColor" => Ok(Field::VehicleInteriorColor),
						"vehicleInteriorType" => Ok(Field::VehicleInteriorType),
						"vehicleModelDate" => Ok(Field::VehicleModelDate),
						"vehicleSeatingCapacity" => Ok(Field::VehicleSeatingCapacity),
						"vehicleSpecialUsage" => Ok(Field::VehicleSpecialUsage),
						"vehicleTransmission" => Ok(Field::VehicleTransmission),
						"weight" => Ok(Field::Weight),
						"weightTotal" => Ok(Field::WeightTotal),
						"wheelbase" => Ok(Field::Wheelbase),
						"width" => Ok(Field::Width),
						_ => Ok(Field::Ignore),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"accelerationTime" => Ok(Field::AccelerationTime),
						b"additionalProperty" => Ok(Field::AdditionalProperty),
						b"additionalType" => Ok(Field::AdditionalType),
						b"aggregateRating" => Ok(Field::AggregateRating),
						b"alternateName" => Ok(Field::AlternateName),
						b"asin" => Ok(Field::Asin),
						b"audience" => Ok(Field::Audience),
						b"award" => Ok(Field::Award),
						b"awards" => Ok(Field::Awards),
						b"bodyType" => Ok(Field::BodyType),
						b"brand" => Ok(Field::Brand),
						b"callSign" => Ok(Field::CallSign),
						b"cargoVolume" => Ok(Field::CargoVolume),
						b"category" => Ok(Field::Category),
						b"color" => Ok(Field::Color),
						b"countryOfAssembly" => Ok(Field::CountryOfAssembly),
						b"countryOfLastProcessing" => Ok(Field::CountryOfLastProcessing),
						b"countryOfOrigin" => Ok(Field::CountryOfOrigin),
						b"dateVehicleFirstRegistered" => Ok(Field::DateVehicleFirstRegistered),
						b"depth" => Ok(Field::Depth),
						b"description" => Ok(Field::Description),
						b"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						b"driveWheelConfiguration" => Ok(Field::DriveWheelConfiguration),
						b"emissionsCO2" => Ok(Field::EmissionsCo2),
						b"fuelCapacity" => Ok(Field::FuelCapacity),
						b"fuelConsumption" => Ok(Field::FuelConsumption),
						b"fuelEfficiency" => Ok(Field::FuelEfficiency),
						b"fuelType" => Ok(Field::FuelType),
						b"funding" => Ok(Field::Funding),
						b"gtin" => Ok(Field::Gtin),
						b"gtin12" => Ok(Field::Gtin12),
						b"gtin13" => Ok(Field::Gtin13),
						b"gtin14" => Ok(Field::Gtin14),
						b"gtin8" => Ok(Field::Gtin8),
						b"hasAdultConsideration" => Ok(Field::HasAdultConsideration),
						b"hasEnergyConsumptionDetails" => Ok(Field::HasEnergyConsumptionDetails),
						b"hasMeasurement" => Ok(Field::HasMeasurement),
						b"hasMerchantReturnPolicy" => Ok(Field::HasMerchantReturnPolicy),
						b"hasProductReturnPolicy" => Ok(Field::HasProductReturnPolicy),
						b"height" => Ok(Field::Height),
						b"identifier" => Ok(Field::Identifier),
						b"image" => Ok(Field::Image),
						b"inProductGroupWithID" => Ok(Field::InProductGroupWithId),
						b"isAccessoryOrSparePartFor" => Ok(Field::IsAccessoryOrSparePartFor),
						b"isConsumableFor" => Ok(Field::IsConsumableFor),
						b"isFamilyFriendly" => Ok(Field::IsFamilyFriendly),
						b"isRelatedTo" => Ok(Field::IsRelatedTo),
						b"isSimilarTo" => Ok(Field::IsSimilarTo),
						b"isVariantOf" => Ok(Field::IsVariantOf),
						b"itemCondition" => Ok(Field::ItemCondition),
						b"keywords" => Ok(Field::Keywords),
						b"knownVehicleDamages" => Ok(Field::KnownVehicleDamages),
						b"logo" => Ok(Field::Logo),
						b"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						b"manufacturer" => Ok(Field::Manufacturer),
						b"material" => Ok(Field::Material),
						b"meetsEmissionStandard" => Ok(Field::MeetsEmissionStandard),
						b"mileageFromOdometer" => Ok(Field::MileageFromOdometer),
						b"mobileUrl" => Ok(Field::MobileUrl),
						b"model" => Ok(Field::Model),
						b"modelDate" => Ok(Field::ModelDate),
						b"mpn" => Ok(Field::Mpn),
						b"name" => Ok(Field::Name),
						b"negativeNotes" => Ok(Field::NegativeNotes),
						b"nsn" => Ok(Field::Nsn),
						b"numberOfAirbags" => Ok(Field::NumberOfAirbags),
						b"numberOfAxles" => Ok(Field::NumberOfAxles),
						b"numberOfDoors" => Ok(Field::NumberOfDoors),
						b"numberOfForwardGears" => Ok(Field::NumberOfForwardGears),
						b"numberOfPreviousOwners" => Ok(Field::NumberOfPreviousOwners),
						b"offers" => Ok(Field::Offers),
						b"pattern" => Ok(Field::Pattern),
						b"payload" => Ok(Field::Payload),
						b"positiveNotes" => Ok(Field::PositiveNotes),
						b"potentialAction" => Ok(Field::PotentialAction),
						b"productID" => Ok(Field::ProductId),
						b"productionDate" => Ok(Field::ProductionDate),
						b"purchaseDate" => Ok(Field::PurchaseDate),
						b"releaseDate" => Ok(Field::ReleaseDate),
						b"review" => Ok(Field::Review),
						b"reviews" => Ok(Field::Reviews),
						b"sameAs" => Ok(Field::SameAs),
						b"seatingCapacity" => Ok(Field::SeatingCapacity),
						b"size" => Ok(Field::Size),
						b"sku" => Ok(Field::Sku),
						b"slogan" => Ok(Field::Slogan),
						b"speed" => Ok(Field::Speed),
						b"steeringPosition" => Ok(Field::SteeringPosition),
						b"stupidProperty" => Ok(Field::StupidProperty),
						b"subjectOf" => Ok(Field::SubjectOf),
						b"tongueWeight" => Ok(Field::TongueWeight),
						b"trailerWeight" => Ok(Field::TrailerWeight),
						b"url" => Ok(Field::Url),
						b"vehicleConfiguration" => Ok(Field::VehicleConfiguration),
						b"vehicleEngine" => Ok(Field::VehicleEngine),
						b"vehicleIdentificationNumber" => Ok(Field::VehicleIdentificationNumber),
						b"vehicleInteriorColor" => Ok(Field::VehicleInteriorColor),
						b"vehicleInteriorType" => Ok(Field::VehicleInteriorType),
						b"vehicleModelDate" => Ok(Field::VehicleModelDate),
						b"vehicleSeatingCapacity" => Ok(Field::VehicleSeatingCapacity),
						b"vehicleSpecialUsage" => Ok(Field::VehicleSpecialUsage),
						b"vehicleTransmission" => Ok(Field::VehicleTransmission),
						b"weight" => Ok(Field::Weight),
						b"weightTotal" => Ok(Field::WeightTotal),
						b"wheelbase" => Ok(Field::Wheelbase),
						b"width" => Ok(Field::Width),
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
				type Value = MotorizedBicycle;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MotorizedBicycle")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#acceleration_time_property = None;
					let mut r#additional_property_property = None;
					let mut r#additional_type_property = None;
					let mut r#aggregate_rating_property = None;
					let mut r#alternate_name_property = None;
					let mut r#asin_property = None;
					let mut r#audience_property = None;
					let mut r#award_property = None;
					let mut r#awards_property = None;
					let mut r#body_type_property = None;
					let mut r#brand_property = None;
					let mut r#call_sign_property = None;
					let mut r#cargo_volume_property = None;
					let mut r#category_property = None;
					let mut r#color_property = None;
					let mut r#country_of_assembly_property = None;
					let mut r#country_of_last_processing_property = None;
					let mut r#country_of_origin_property = None;
					let mut r#date_vehicle_first_registered_property = None;
					let mut r#depth_property = None;
					let mut r#description_property = None;
					let mut r#disambiguating_description_property = None;
					let mut r#drive_wheel_configuration_property = None;
					let mut r#emissions_co_2_property = None;
					let mut r#fuel_capacity_property = None;
					let mut r#fuel_consumption_property = None;
					let mut r#fuel_efficiency_property = None;
					let mut r#fuel_type_property = None;
					let mut r#funding_property = None;
					let mut r#gtin_property = None;
					let mut r#gtin_12_property = None;
					let mut r#gtin_13_property = None;
					let mut r#gtin_14_property = None;
					let mut r#gtin_8_property = None;
					let mut r#has_adult_consideration_property = None;
					let mut r#has_energy_consumption_details_property = None;
					let mut r#has_measurement_property = None;
					let mut r#has_merchant_return_policy_property = None;
					let mut r#has_product_return_policy_property = None;
					let mut r#height_property = None;
					let mut r#identifier_property = None;
					let mut r#image_property = None;
					let mut r#in_product_group_with_id_property = None;
					let mut r#is_accessory_or_spare_part_for_property = None;
					let mut r#is_consumable_for_property = None;
					let mut r#is_family_friendly_property = None;
					let mut r#is_related_to_property = None;
					let mut r#is_similar_to_property = None;
					let mut r#is_variant_of_property = None;
					let mut r#item_condition_property = None;
					let mut r#keywords_property = None;
					let mut r#known_vehicle_damages_property = None;
					let mut r#logo_property = None;
					let mut r#main_entity_of_page_property = None;
					let mut r#manufacturer_property = None;
					let mut r#material_property = None;
					let mut r#meets_emission_standard_property = None;
					let mut r#mileage_from_odometer_property = None;
					let mut r#mobile_url_property = None;
					let mut r#model_property = None;
					let mut r#model_date_property = None;
					let mut r#mpn_property = None;
					let mut r#name_property = None;
					let mut r#negative_notes_property = None;
					let mut r#nsn_property = None;
					let mut r#number_of_airbags_property = None;
					let mut r#number_of_axles_property = None;
					let mut r#number_of_doors_property = None;
					let mut r#number_of_forward_gears_property = None;
					let mut r#number_of_previous_owners_property = None;
					let mut r#offers_property = None;
					let mut r#pattern_property = None;
					let mut r#payload_property = None;
					let mut r#positive_notes_property = None;
					let mut r#potential_action_property = None;
					let mut r#product_id_property = None;
					let mut r#production_date_property = None;
					let mut r#purchase_date_property = None;
					let mut r#release_date_property = None;
					let mut r#review_property = None;
					let mut r#reviews_property = None;
					let mut r#same_as_property = None;
					let mut r#seating_capacity_property = None;
					let mut r#size_property = None;
					let mut r#sku_property = None;
					let mut r#slogan_property = None;
					let mut r#speed_property = None;
					let mut r#steering_position_property = None;
					let mut r#stupid_property_property = None;
					let mut r#subject_of_property = None;
					let mut r#tongue_weight_property = None;
					let mut r#trailer_weight_property = None;
					let mut r#url_property = None;
					let mut r#vehicle_configuration_property = None;
					let mut r#vehicle_engine_property = None;
					let mut r#vehicle_identification_number_property = None;
					let mut r#vehicle_interior_color_property = None;
					let mut r#vehicle_interior_type_property = None;
					let mut r#vehicle_model_date_property = None;
					let mut r#vehicle_seating_capacity_property = None;
					let mut r#vehicle_special_usage_property = None;
					let mut r#vehicle_transmission_property = None;
					let mut r#weight_property = None;
					let mut r#weight_total_property = None;
					let mut r#wheelbase_property = None;
					let mut r#width_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
							Field::AccelerationTime => {
								if r#acceleration_time_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"accelerationTime",
									));
								}
								r#acceleration_time_property = Some({
									struct DeserializeWith(Vec<AccelerationTimeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
							Field::AggregateRating => {
								if r#aggregate_rating_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"aggregateRating",
									));
								}
								r#aggregate_rating_property = Some({
									struct DeserializeWith(Vec<AggregateRatingProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::Audience => {
								if r#audience_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"audience",
									));
								}
								r#audience_property = Some({
									struct DeserializeWith(Vec<AudienceProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Award => {
								if r#award_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("award"));
								}
								r#award_property = Some({
									struct DeserializeWith(Vec<AwardProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Awards => {
								if r#awards_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("awards"));
								}
								r#awards_property = Some({
									struct DeserializeWith(Vec<AwardsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::BodyType => {
								if r#body_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"bodyType",
									));
								}
								r#body_type_property = Some({
									struct DeserializeWith(Vec<BodyTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Brand => {
								if r#brand_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("brand"));
								}
								r#brand_property = Some({
									struct DeserializeWith(Vec<BrandProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CallSign => {
								if r#call_sign_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"callSign",
									));
								}
								r#call_sign_property = Some({
									struct DeserializeWith(Vec<CallSignProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CargoVolume => {
								if r#cargo_volume_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cargoVolume",
									));
								}
								r#cargo_volume_property = Some({
									struct DeserializeWith(Vec<CargoVolumeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::Color => {
								if r#color_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("color"));
								}
								r#color_property = Some({
									struct DeserializeWith(Vec<ColorProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CountryOfAssembly => {
								if r#country_of_assembly_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"countryOfAssembly",
									));
								}
								r#country_of_assembly_property = Some({
									struct DeserializeWith(Vec<CountryOfAssemblyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CountryOfLastProcessing => {
								if r#country_of_last_processing_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"countryOfLastProcessing",
									));
								}
								r#country_of_last_processing_property = Some({
									struct DeserializeWith(Vec<CountryOfLastProcessingProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CountryOfOrigin => {
								if r#country_of_origin_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"countryOfOrigin",
									));
								}
								r#country_of_origin_property = Some({
									struct DeserializeWith(Vec<CountryOfOriginProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::DateVehicleFirstRegistered => {
								if r#date_vehicle_first_registered_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"dateVehicleFirstRegistered",
									));
								}
								r#date_vehicle_first_registered_property = Some({
									struct DeserializeWith(Vec<DateVehicleFirstRegisteredProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Depth => {
								if r#depth_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("depth"));
								}
								r#depth_property = Some({
									struct DeserializeWith(Vec<DepthProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::DriveWheelConfiguration => {
								if r#drive_wheel_configuration_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"driveWheelConfiguration",
									));
								}
								r#drive_wheel_configuration_property = Some({
									struct DeserializeWith(Vec<DriveWheelConfigurationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::EmissionsCo2 => {
								if r#emissions_co_2_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"emissionsCO2",
									));
								}
								r#emissions_co_2_property = Some({
									struct DeserializeWith(Vec<EmissionsCo2Property>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::FuelCapacity => {
								if r#fuel_capacity_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"fuelCapacity",
									));
								}
								r#fuel_capacity_property = Some({
									struct DeserializeWith(Vec<FuelCapacityProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::FuelConsumption => {
								if r#fuel_consumption_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"fuelConsumption",
									));
								}
								r#fuel_consumption_property = Some({
									struct DeserializeWith(Vec<FuelConsumptionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::FuelEfficiency => {
								if r#fuel_efficiency_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"fuelEfficiency",
									));
								}
								r#fuel_efficiency_property = Some({
									struct DeserializeWith(Vec<FuelEfficiencyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::FuelType => {
								if r#fuel_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"fuelType",
									));
								}
								r#fuel_type_property = Some({
									struct DeserializeWith(Vec<FuelTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Funding => {
								if r#funding_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"funding",
									));
								}
								r#funding_property = Some({
									struct DeserializeWith(Vec<FundingProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::HasAdultConsideration => {
								if r#has_adult_consideration_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasAdultConsideration",
									));
								}
								r#has_adult_consideration_property = Some({
									struct DeserializeWith(Vec<HasAdultConsiderationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::HasEnergyConsumptionDetails => {
								if r#has_energy_consumption_details_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasEnergyConsumptionDetails",
									));
								}
								r#has_energy_consumption_details_property = Some({
									struct DeserializeWith(
										Vec<HasEnergyConsumptionDetailsProperty>,
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
							Field::HasMeasurement => {
								if r#has_measurement_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasMeasurement",
									));
								}
								r#has_measurement_property = Some({
									struct DeserializeWith(Vec<HasMeasurementProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::HasMerchantReturnPolicy => {
								if r#has_merchant_return_policy_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasMerchantReturnPolicy",
									));
								}
								r#has_merchant_return_policy_property = Some({
									struct DeserializeWith(Vec<HasMerchantReturnPolicyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::HasProductReturnPolicy => {
								if r#has_product_return_policy_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasProductReturnPolicy",
									));
								}
								r#has_product_return_policy_property = Some({
									struct DeserializeWith(Vec<HasProductReturnPolicyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Height => {
								if r#height_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("height"));
								}
								r#height_property = Some({
									struct DeserializeWith(Vec<HeightProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::InProductGroupWithId => {
								if r#in_product_group_with_id_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"inProductGroupWithID",
									));
								}
								r#in_product_group_with_id_property = Some({
									struct DeserializeWith(Vec<InProductGroupWithIdProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::IsAccessoryOrSparePartFor => {
								if r#is_accessory_or_spare_part_for_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isAccessoryOrSparePartFor",
									));
								}
								r#is_accessory_or_spare_part_for_property = Some({
									struct DeserializeWith(Vec<IsAccessoryOrSparePartForProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::IsConsumableFor => {
								if r#is_consumable_for_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isConsumableFor",
									));
								}
								r#is_consumable_for_property = Some({
									struct DeserializeWith(Vec<IsConsumableForProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::IsFamilyFriendly => {
								if r#is_family_friendly_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isFamilyFriendly",
									));
								}
								r#is_family_friendly_property = Some({
									struct DeserializeWith(Vec<IsFamilyFriendlyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::IsRelatedTo => {
								if r#is_related_to_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isRelatedTo",
									));
								}
								r#is_related_to_property = Some({
									struct DeserializeWith(Vec<IsRelatedToProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::IsSimilarTo => {
								if r#is_similar_to_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isSimilarTo",
									));
								}
								r#is_similar_to_property = Some({
									struct DeserializeWith(Vec<IsSimilarToProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::IsVariantOf => {
								if r#is_variant_of_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isVariantOf",
									));
								}
								r#is_variant_of_property = Some({
									struct DeserializeWith(Vec<IsVariantOfProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::Keywords => {
								if r#keywords_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"keywords",
									));
								}
								r#keywords_property = Some({
									struct DeserializeWith(Vec<KeywordsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::KnownVehicleDamages => {
								if r#known_vehicle_damages_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"knownVehicleDamages",
									));
								}
								r#known_vehicle_damages_property = Some({
									struct DeserializeWith(Vec<KnownVehicleDamagesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Logo => {
								if r#logo_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("logo"));
								}
								r#logo_property = Some({
									struct DeserializeWith(Vec<LogoProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::Manufacturer => {
								if r#manufacturer_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"manufacturer",
									));
								}
								r#manufacturer_property = Some({
									struct DeserializeWith(Vec<ManufacturerProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Material => {
								if r#material_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"material",
									));
								}
								r#material_property = Some({
									struct DeserializeWith(Vec<MaterialProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::MeetsEmissionStandard => {
								if r#meets_emission_standard_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"meetsEmissionStandard",
									));
								}
								r#meets_emission_standard_property = Some({
									struct DeserializeWith(Vec<MeetsEmissionStandardProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::MileageFromOdometer => {
								if r#mileage_from_odometer_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"mileageFromOdometer",
									));
								}
								r#mileage_from_odometer_property = Some({
									struct DeserializeWith(Vec<MileageFromOdometerProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::MobileUrl => {
								if r#mobile_url_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"mobileUrl",
									));
								}
								r#mobile_url_property = Some({
									struct DeserializeWith(Vec<MobileUrlProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Model => {
								if r#model_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("model"));
								}
								r#model_property = Some({
									struct DeserializeWith(Vec<ModelProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ModelDate => {
								if r#model_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"modelDate",
									));
								}
								r#model_date_property = Some({
									struct DeserializeWith(Vec<ModelDateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::NegativeNotes => {
								if r#negative_notes_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"negativeNotes",
									));
								}
								r#negative_notes_property = Some({
									struct DeserializeWith(Vec<NegativeNotesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Nsn => {
								if r#nsn_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("nsn"));
								}
								r#nsn_property = Some({
									struct DeserializeWith(Vec<NsnProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::NumberOfAirbags => {
								if r#number_of_airbags_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfAirbags",
									));
								}
								r#number_of_airbags_property = Some({
									struct DeserializeWith(Vec<NumberOfAirbagsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::NumberOfAxles => {
								if r#number_of_axles_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfAxles",
									));
								}
								r#number_of_axles_property = Some({
									struct DeserializeWith(Vec<NumberOfAxlesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::NumberOfDoors => {
								if r#number_of_doors_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfDoors",
									));
								}
								r#number_of_doors_property = Some({
									struct DeserializeWith(Vec<NumberOfDoorsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::NumberOfForwardGears => {
								if r#number_of_forward_gears_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfForwardGears",
									));
								}
								r#number_of_forward_gears_property = Some({
									struct DeserializeWith(Vec<NumberOfForwardGearsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::NumberOfPreviousOwners => {
								if r#number_of_previous_owners_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfPreviousOwners",
									));
								}
								r#number_of_previous_owners_property = Some({
									struct DeserializeWith(Vec<NumberOfPreviousOwnersProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Offers => {
								if r#offers_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("offers"));
								}
								r#offers_property = Some({
									struct DeserializeWith(Vec<OffersProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Pattern => {
								if r#pattern_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"pattern",
									));
								}
								r#pattern_property = Some({
									struct DeserializeWith(Vec<PatternProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Payload => {
								if r#payload_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"payload",
									));
								}
								r#payload_property = Some({
									struct DeserializeWith(Vec<PayloadProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::PositiveNotes => {
								if r#positive_notes_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"positiveNotes",
									));
								}
								r#positive_notes_property = Some({
									struct DeserializeWith(Vec<PositiveNotesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::ProductId => {
								if r#product_id_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"productID",
									));
								}
								r#product_id_property = Some({
									struct DeserializeWith(Vec<ProductIdProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ProductionDate => {
								if r#production_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"productionDate",
									));
								}
								r#production_date_property = Some({
									struct DeserializeWith(Vec<ProductionDateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::PurchaseDate => {
								if r#purchase_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"purchaseDate",
									));
								}
								r#purchase_date_property = Some({
									struct DeserializeWith(Vec<PurchaseDateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ReleaseDate => {
								if r#release_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"releaseDate",
									));
								}
								r#release_date_property = Some({
									struct DeserializeWith(Vec<ReleaseDateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Review => {
								if r#review_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("review"));
								}
								r#review_property = Some({
									struct DeserializeWith(Vec<ReviewProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Reviews => {
								if r#reviews_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"reviews",
									));
								}
								r#reviews_property = Some({
									struct DeserializeWith(Vec<ReviewsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::SeatingCapacity => {
								if r#seating_capacity_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"seatingCapacity",
									));
								}
								r#seating_capacity_property = Some({
									struct DeserializeWith(Vec<SeatingCapacityProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Size => {
								if r#size_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("size"));
								}
								r#size_property = Some({
									struct DeserializeWith(Vec<SizeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::Slogan => {
								if r#slogan_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("slogan"));
								}
								r#slogan_property = Some({
									struct DeserializeWith(Vec<SloganProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Speed => {
								if r#speed_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("speed"));
								}
								r#speed_property = Some({
									struct DeserializeWith(Vec<SpeedProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::SteeringPosition => {
								if r#steering_position_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"steeringPosition",
									));
								}
								r#steering_position_property = Some({
									struct DeserializeWith(Vec<SteeringPositionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::StupidProperty => {
								if r#stupid_property_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"stupidProperty",
									));
								}
								r#stupid_property_property = Some({
									struct DeserializeWith(Vec<StupidPropertyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::TongueWeight => {
								if r#tongue_weight_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"tongueWeight",
									));
								}
								r#tongue_weight_property = Some({
									struct DeserializeWith(Vec<TongueWeightProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::TrailerWeight => {
								if r#trailer_weight_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"trailerWeight",
									));
								}
								r#trailer_weight_property = Some({
									struct DeserializeWith(Vec<TrailerWeightProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::VehicleConfiguration => {
								if r#vehicle_configuration_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleConfiguration",
									));
								}
								r#vehicle_configuration_property = Some({
									struct DeserializeWith(Vec<VehicleConfigurationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::VehicleEngine => {
								if r#vehicle_engine_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleEngine",
									));
								}
								r#vehicle_engine_property = Some({
									struct DeserializeWith(Vec<VehicleEngineProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::VehicleIdentificationNumber => {
								if r#vehicle_identification_number_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleIdentificationNumber",
									));
								}
								r#vehicle_identification_number_property = Some({
									struct DeserializeWith(
										Vec<VehicleIdentificationNumberProperty>,
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
							Field::VehicleInteriorColor => {
								if r#vehicle_interior_color_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleInteriorColor",
									));
								}
								r#vehicle_interior_color_property = Some({
									struct DeserializeWith(Vec<VehicleInteriorColorProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::VehicleInteriorType => {
								if r#vehicle_interior_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleInteriorType",
									));
								}
								r#vehicle_interior_type_property = Some({
									struct DeserializeWith(Vec<VehicleInteriorTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::VehicleModelDate => {
								if r#vehicle_model_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleModelDate",
									));
								}
								r#vehicle_model_date_property = Some({
									struct DeserializeWith(Vec<VehicleModelDateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::VehicleSeatingCapacity => {
								if r#vehicle_seating_capacity_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleSeatingCapacity",
									));
								}
								r#vehicle_seating_capacity_property = Some({
									struct DeserializeWith(Vec<VehicleSeatingCapacityProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::VehicleSpecialUsage => {
								if r#vehicle_special_usage_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleSpecialUsage",
									));
								}
								r#vehicle_special_usage_property = Some({
									struct DeserializeWith(Vec<VehicleSpecialUsageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::VehicleTransmission => {
								if r#vehicle_transmission_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleTransmission",
									));
								}
								r#vehicle_transmission_property = Some({
									struct DeserializeWith(Vec<VehicleTransmissionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Weight => {
								if r#weight_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("weight"));
								}
								r#weight_property = Some({
									struct DeserializeWith(Vec<WeightProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::WeightTotal => {
								if r#weight_total_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"weightTotal",
									));
								}
								r#weight_total_property = Some({
									struct DeserializeWith(Vec<WeightTotalProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Wheelbase => {
								if r#wheelbase_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"wheelbase",
									));
								}
								r#wheelbase_property = Some({
									struct DeserializeWith(Vec<WheelbaseProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Width => {
								if r#width_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("width"));
								}
								r#width_property = Some({
									struct DeserializeWith(Vec<WidthProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
					Ok(MotorizedBicycle {
						r#acceleration_time: r#acceleration_time_property.unwrap_or_default(),
						r#additional_property: r#additional_property_property.unwrap_or_default(),
						r#additional_type: r#additional_type_property.unwrap_or_default(),
						r#aggregate_rating: r#aggregate_rating_property.unwrap_or_default(),
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
						r#asin: r#asin_property.unwrap_or_default(),
						r#audience: r#audience_property.unwrap_or_default(),
						r#award: r#award_property.unwrap_or_default(),
						r#awards: r#awards_property.unwrap_or_default(),
						r#body_type: r#body_type_property.unwrap_or_default(),
						r#brand: r#brand_property.unwrap_or_default(),
						r#call_sign: r#call_sign_property.unwrap_or_default(),
						r#cargo_volume: r#cargo_volume_property.unwrap_or_default(),
						r#category: r#category_property.unwrap_or_default(),
						r#color: r#color_property.unwrap_or_default(),
						r#country_of_assembly: r#country_of_assembly_property.unwrap_or_default(),
						r#country_of_last_processing: r#country_of_last_processing_property
							.unwrap_or_default(),
						r#country_of_origin: r#country_of_origin_property.unwrap_or_default(),
						r#date_vehicle_first_registered: r#date_vehicle_first_registered_property
							.unwrap_or_default(),
						r#depth: r#depth_property.unwrap_or_default(),
						r#description: r#description_property.unwrap_or_default(),
						r#disambiguating_description: r#disambiguating_description_property
							.unwrap_or_default(),
						r#drive_wheel_configuration: r#drive_wheel_configuration_property
							.unwrap_or_default(),
						r#emissions_co_2: r#emissions_co_2_property.unwrap_or_default(),
						r#fuel_capacity: r#fuel_capacity_property.unwrap_or_default(),
						r#fuel_consumption: r#fuel_consumption_property.unwrap_or_default(),
						r#fuel_efficiency: r#fuel_efficiency_property.unwrap_or_default(),
						r#fuel_type: r#fuel_type_property.unwrap_or_default(),
						r#funding: r#funding_property.unwrap_or_default(),
						r#gtin: r#gtin_property.unwrap_or_default(),
						r#gtin_12: r#gtin_12_property.unwrap_or_default(),
						r#gtin_13: r#gtin_13_property.unwrap_or_default(),
						r#gtin_14: r#gtin_14_property.unwrap_or_default(),
						r#gtin_8: r#gtin_8_property.unwrap_or_default(),
						r#has_adult_consideration: r#has_adult_consideration_property
							.unwrap_or_default(),
						r#has_energy_consumption_details: r#has_energy_consumption_details_property
							.unwrap_or_default(),
						r#has_measurement: r#has_measurement_property.unwrap_or_default(),
						r#has_merchant_return_policy: r#has_merchant_return_policy_property
							.unwrap_or_default(),
						r#has_product_return_policy: r#has_product_return_policy_property
							.unwrap_or_default(),
						r#height: r#height_property.unwrap_or_default(),
						r#identifier: r#identifier_property.unwrap_or_default(),
						r#image: r#image_property.unwrap_or_default(),
						r#in_product_group_with_id: r#in_product_group_with_id_property
							.unwrap_or_default(),
						r#is_accessory_or_spare_part_for: r#is_accessory_or_spare_part_for_property
							.unwrap_or_default(),
						r#is_consumable_for: r#is_consumable_for_property.unwrap_or_default(),
						r#is_family_friendly: r#is_family_friendly_property.unwrap_or_default(),
						r#is_related_to: r#is_related_to_property.unwrap_or_default(),
						r#is_similar_to: r#is_similar_to_property.unwrap_or_default(),
						r#is_variant_of: r#is_variant_of_property.unwrap_or_default(),
						r#item_condition: r#item_condition_property.unwrap_or_default(),
						r#keywords: r#keywords_property.unwrap_or_default(),
						r#known_vehicle_damages: r#known_vehicle_damages_property
							.unwrap_or_default(),
						r#logo: r#logo_property.unwrap_or_default(),
						r#main_entity_of_page: r#main_entity_of_page_property.unwrap_or_default(),
						r#manufacturer: r#manufacturer_property.unwrap_or_default(),
						r#material: r#material_property.unwrap_or_default(),
						r#meets_emission_standard: r#meets_emission_standard_property
							.unwrap_or_default(),
						r#mileage_from_odometer: r#mileage_from_odometer_property
							.unwrap_or_default(),
						r#mobile_url: r#mobile_url_property.unwrap_or_default(),
						r#model: r#model_property.unwrap_or_default(),
						r#model_date: r#model_date_property.unwrap_or_default(),
						r#mpn: r#mpn_property.unwrap_or_default(),
						r#name: r#name_property.unwrap_or_default(),
						r#negative_notes: r#negative_notes_property.unwrap_or_default(),
						r#nsn: r#nsn_property.unwrap_or_default(),
						r#number_of_airbags: r#number_of_airbags_property.unwrap_or_default(),
						r#number_of_axles: r#number_of_axles_property.unwrap_or_default(),
						r#number_of_doors: r#number_of_doors_property.unwrap_or_default(),
						r#number_of_forward_gears: r#number_of_forward_gears_property
							.unwrap_or_default(),
						r#number_of_previous_owners: r#number_of_previous_owners_property
							.unwrap_or_default(),
						r#offers: r#offers_property.unwrap_or_default(),
						r#pattern: r#pattern_property.unwrap_or_default(),
						r#payload: r#payload_property.unwrap_or_default(),
						r#positive_notes: r#positive_notes_property.unwrap_or_default(),
						r#potential_action: r#potential_action_property.unwrap_or_default(),
						r#product_id: r#product_id_property.unwrap_or_default(),
						r#production_date: r#production_date_property.unwrap_or_default(),
						r#purchase_date: r#purchase_date_property.unwrap_or_default(),
						r#release_date: r#release_date_property.unwrap_or_default(),
						r#review: r#review_property.unwrap_or_default(),
						r#reviews: r#reviews_property.unwrap_or_default(),
						r#same_as: r#same_as_property.unwrap_or_default(),
						r#seating_capacity: r#seating_capacity_property.unwrap_or_default(),
						r#size: r#size_property.unwrap_or_default(),
						r#sku: r#sku_property.unwrap_or_default(),
						r#slogan: r#slogan_property.unwrap_or_default(),
						r#speed: r#speed_property.unwrap_or_default(),
						r#steering_position: r#steering_position_property.unwrap_or_default(),
						r#stupid_property: r#stupid_property_property.unwrap_or_default(),
						r#subject_of: r#subject_of_property.unwrap_or_default(),
						r#tongue_weight: r#tongue_weight_property.unwrap_or_default(),
						r#trailer_weight: r#trailer_weight_property.unwrap_or_default(),
						r#url: r#url_property.unwrap_or_default(),
						r#vehicle_configuration: r#vehicle_configuration_property
							.unwrap_or_default(),
						r#vehicle_engine: r#vehicle_engine_property.unwrap_or_default(),
						r#vehicle_identification_number: r#vehicle_identification_number_property
							.unwrap_or_default(),
						r#vehicle_interior_color: r#vehicle_interior_color_property
							.unwrap_or_default(),
						r#vehicle_interior_type: r#vehicle_interior_type_property
							.unwrap_or_default(),
						r#vehicle_model_date: r#vehicle_model_date_property.unwrap_or_default(),
						r#vehicle_seating_capacity: r#vehicle_seating_capacity_property
							.unwrap_or_default(),
						r#vehicle_special_usage: r#vehicle_special_usage_property
							.unwrap_or_default(),
						r#vehicle_transmission: r#vehicle_transmission_property.unwrap_or_default(),
						r#weight: r#weight_property.unwrap_or_default(),
						r#weight_total: r#weight_total_property.unwrap_or_default(),
						r#wheelbase: r#wheelbase_property.unwrap_or_default(),
						r#width: r#width_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				"accelerationTime",
				"additionalProperty",
				"additionalType",
				"aggregateRating",
				"alternateName",
				"asin",
				"audience",
				"award",
				"awards",
				"bodyType",
				"brand",
				"callSign",
				"cargoVolume",
				"category",
				"color",
				"countryOfAssembly",
				"countryOfLastProcessing",
				"countryOfOrigin",
				"dateVehicleFirstRegistered",
				"depth",
				"description",
				"disambiguatingDescription",
				"driveWheelConfiguration",
				"emissionsCO2",
				"fuelCapacity",
				"fuelConsumption",
				"fuelEfficiency",
				"fuelType",
				"funding",
				"gtin",
				"gtin12",
				"gtin13",
				"gtin14",
				"gtin8",
				"hasAdultConsideration",
				"hasEnergyConsumptionDetails",
				"hasMeasurement",
				"hasMerchantReturnPolicy",
				"hasProductReturnPolicy",
				"height",
				"identifier",
				"image",
				"inProductGroupWithID",
				"isAccessoryOrSparePartFor",
				"isConsumableFor",
				"isFamilyFriendly",
				"isRelatedTo",
				"isSimilarTo",
				"isVariantOf",
				"itemCondition",
				"keywords",
				"knownVehicleDamages",
				"logo",
				"mainEntityOfPage",
				"manufacturer",
				"material",
				"meetsEmissionStandard",
				"mileageFromOdometer",
				"mobileUrl",
				"model",
				"modelDate",
				"mpn",
				"name",
				"negativeNotes",
				"nsn",
				"numberOfAirbags",
				"numberOfAxles",
				"numberOfDoors",
				"numberOfForwardGears",
				"numberOfPreviousOwners",
				"offers",
				"pattern",
				"payload",
				"positiveNotes",
				"potentialAction",
				"productID",
				"productionDate",
				"purchaseDate",
				"releaseDate",
				"review",
				"reviews",
				"sameAs",
				"seatingCapacity",
				"size",
				"sku",
				"slogan",
				"speed",
				"steeringPosition",
				"stupidProperty",
				"subjectOf",
				"tongueWeight",
				"trailerWeight",
				"url",
				"vehicleConfiguration",
				"vehicleEngine",
				"vehicleIdentificationNumber",
				"vehicleInteriorColor",
				"vehicleInteriorType",
				"vehicleModelDate",
				"vehicleSeatingCapacity",
				"vehicleSpecialUsage",
				"vehicleTransmission",
				"weight",
				"weightTotal",
				"wheelbase",
				"width",
			];
			deserializer.deserialize_struct("MotorizedBicycle", FIELDS, ClassVisitor)
		}
	}
}
