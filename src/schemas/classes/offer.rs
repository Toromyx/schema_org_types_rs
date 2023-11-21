use super::*;
/// <https://schema.org/Offer>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Offer {
	/// <https://schema.org/acceptedPaymentMethod>
	pub r#accepted_payment_method: Vec<AcceptedPaymentMethodProperty>,
	/// <https://schema.org/addOn>
	pub r#add_on: Vec<AddOnProperty>,
	/// <https://schema.org/advanceBookingRequirement>
	pub r#advance_booking_requirement: Vec<AdvanceBookingRequirementProperty>,
	/// <https://schema.org/aggregateRating>
	pub r#aggregate_rating: Vec<AggregateRatingProperty>,
	/// <https://schema.org/areaServed>
	pub r#area_served: Vec<AreaServedProperty>,
	/// <https://schema.org/asin>
	pub r#asin: Vec<AsinProperty>,
	/// <https://schema.org/availability>
	pub r#availability: Vec<AvailabilityProperty>,
	/// <https://schema.org/availabilityEnds>
	pub r#availability_ends: Vec<AvailabilityEndsProperty>,
	/// <https://schema.org/availabilityStarts>
	pub r#availability_starts: Vec<AvailabilityStartsProperty>,
	/// <https://schema.org/availableAtOrFrom>
	pub r#available_at_or_from: Vec<AvailableAtOrFromProperty>,
	/// <https://schema.org/availableDeliveryMethod>
	pub r#available_delivery_method: Vec<AvailableDeliveryMethodProperty>,
	/// <https://schema.org/businessFunction>
	pub r#business_function: Vec<BusinessFunctionProperty>,
	/// <https://schema.org/category>
	pub r#category: Vec<CategoryProperty>,
	/// <https://schema.org/checkoutPageURLTemplate>
	pub r#checkout_page_url_template: Vec<CheckoutPageUrlTemplateProperty>,
	/// <https://schema.org/deliveryLeadTime>
	pub r#delivery_lead_time: Vec<DeliveryLeadTimeProperty>,
	/// <https://schema.org/eligibleCustomerType>
	pub r#eligible_customer_type: Vec<EligibleCustomerTypeProperty>,
	/// <https://schema.org/eligibleDuration>
	pub r#eligible_duration: Vec<EligibleDurationProperty>,
	/// <https://schema.org/eligibleQuantity>
	pub r#eligible_quantity: Vec<EligibleQuantityProperty>,
	/// <https://schema.org/eligibleRegion>
	pub r#eligible_region: Vec<EligibleRegionProperty>,
	/// <https://schema.org/eligibleTransactionVolume>
	pub r#eligible_transaction_volume: Vec<EligibleTransactionVolumeProperty>,
	/// <https://schema.org/gtin>
	pub r#gtin: Vec<GtinProperty>,
	/// <https://schema.org/gtin12>
	pub r#gtin_12: Vec<Gtin12Property>,
	/// <https://schema.org/gtin13>
	pub r#gtin_13: Vec<Gtin13Property>,
	/// <https://schema.org/gtin14>
	pub r#gtin_14: Vec<Gtin14Property>,
	/// <https://schema.org/gtin8>
	pub r#gtin_8: Vec<Gtin8Property>,
	/// <https://schema.org/hasAdultConsideration>
	pub r#has_adult_consideration: Vec<HasAdultConsiderationProperty>,
	/// <https://schema.org/hasMeasurement>
	pub r#has_measurement: Vec<HasMeasurementProperty>,
	/// <https://schema.org/hasMerchantReturnPolicy>
	pub r#has_merchant_return_policy: Vec<HasMerchantReturnPolicyProperty>,
	/// <https://schema.org/includesObject>
	pub r#includes_object: Vec<IncludesObjectProperty>,
	/// <https://schema.org/ineligibleRegion>
	pub r#ineligible_region: Vec<IneligibleRegionProperty>,
	/// <https://schema.org/inventoryLevel>
	pub r#inventory_level: Vec<InventoryLevelProperty>,
	/// <https://schema.org/isFamilyFriendly>
	pub r#is_family_friendly: Vec<IsFamilyFriendlyProperty>,
	/// <https://schema.org/itemCondition>
	pub r#item_condition: Vec<ItemConditionProperty>,
	/// <https://schema.org/itemOffered>
	pub r#item_offered: Vec<ItemOfferedProperty>,
	/// <https://schema.org/leaseLength>
	pub r#lease_length: Vec<LeaseLengthProperty>,
	/// <https://schema.org/mobileUrl>
	pub r#mobile_url: Vec<MobileUrlProperty>,
	/// <https://schema.org/mpn>
	pub r#mpn: Vec<MpnProperty>,
	/// <https://schema.org/offeredBy>
	pub r#offered_by: Vec<OfferedByProperty>,
	/// <https://schema.org/price>
	pub r#price: Vec<PriceProperty>,
	/// <https://schema.org/priceCurrency>
	pub r#price_currency: Vec<PriceCurrencyProperty>,
	/// <https://schema.org/priceSpecification>
	pub r#price_specification: Vec<PriceSpecificationProperty>,
	/// <https://schema.org/priceValidUntil>
	pub r#price_valid_until: Vec<PriceValidUntilProperty>,
	/// <https://schema.org/review>
	pub r#review: Vec<ReviewProperty>,
	/// <https://schema.org/reviews>
	#[deprecated = "This schema is superseded by <https://schema.org/review>."]
	pub r#reviews: Vec<ReviewsProperty>,
	/// <https://schema.org/seller>
	pub r#seller: Vec<SellerProperty>,
	/// <https://schema.org/serialNumber>
	pub r#serial_number: Vec<SerialNumberProperty>,
	/// <https://schema.org/shippingDetails>
	pub r#shipping_details: Vec<ShippingDetailsProperty>,
	/// <https://schema.org/sku>
	pub r#sku: Vec<SkuProperty>,
	/// <https://schema.org/validFrom>
	pub r#valid_from: Vec<ValidFromProperty>,
	/// <https://schema.org/validThrough>
	pub r#valid_through: Vec<ValidThroughProperty>,
	/// <https://schema.org/warranty>
	pub r#warranty: Vec<WarrantyProperty>,
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
/// This trait is for properties from <https://schema.org/Offer>.
pub trait OfferTrait {
	/// Get <https://schema.org/acceptedPaymentMethod> from [`Self`] as borrowed slice.
	fn get_accepted_payment_method(&self) -> &[AcceptedPaymentMethodProperty];
	/// Take <https://schema.org/acceptedPaymentMethod> from [`Self`] as owned vector.
	fn take_accepted_payment_method(&mut self) -> Vec<AcceptedPaymentMethodProperty>;
	/// Get <https://schema.org/addOn> from [`Self`] as borrowed slice.
	fn get_add_on(&self) -> &[AddOnProperty];
	/// Take <https://schema.org/addOn> from [`Self`] as owned vector.
	fn take_add_on(&mut self) -> Vec<AddOnProperty>;
	/// Get <https://schema.org/advanceBookingRequirement> from [`Self`] as borrowed slice.
	fn get_advance_booking_requirement(&self) -> &[AdvanceBookingRequirementProperty];
	/// Take <https://schema.org/advanceBookingRequirement> from [`Self`] as owned vector.
	fn take_advance_booking_requirement(&mut self) -> Vec<AdvanceBookingRequirementProperty>;
	/// Get <https://schema.org/aggregateRating> from [`Self`] as borrowed slice.
	fn get_aggregate_rating(&self) -> &[AggregateRatingProperty];
	/// Take <https://schema.org/aggregateRating> from [`Self`] as owned vector.
	fn take_aggregate_rating(&mut self) -> Vec<AggregateRatingProperty>;
	/// Get <https://schema.org/areaServed> from [`Self`] as borrowed slice.
	fn get_area_served(&self) -> &[AreaServedProperty];
	/// Take <https://schema.org/areaServed> from [`Self`] as owned vector.
	fn take_area_served(&mut self) -> Vec<AreaServedProperty>;
	/// Get <https://schema.org/asin> from [`Self`] as borrowed slice.
	fn get_asin(&self) -> &[AsinProperty];
	/// Take <https://schema.org/asin> from [`Self`] as owned vector.
	fn take_asin(&mut self) -> Vec<AsinProperty>;
	/// Get <https://schema.org/availability> from [`Self`] as borrowed slice.
	fn get_availability(&self) -> &[AvailabilityProperty];
	/// Take <https://schema.org/availability> from [`Self`] as owned vector.
	fn take_availability(&mut self) -> Vec<AvailabilityProperty>;
	/// Get <https://schema.org/availabilityEnds> from [`Self`] as borrowed slice.
	fn get_availability_ends(&self) -> &[AvailabilityEndsProperty];
	/// Take <https://schema.org/availabilityEnds> from [`Self`] as owned vector.
	fn take_availability_ends(&mut self) -> Vec<AvailabilityEndsProperty>;
	/// Get <https://schema.org/availabilityStarts> from [`Self`] as borrowed slice.
	fn get_availability_starts(&self) -> &[AvailabilityStartsProperty];
	/// Take <https://schema.org/availabilityStarts> from [`Self`] as owned vector.
	fn take_availability_starts(&mut self) -> Vec<AvailabilityStartsProperty>;
	/// Get <https://schema.org/availableAtOrFrom> from [`Self`] as borrowed slice.
	fn get_available_at_or_from(&self) -> &[AvailableAtOrFromProperty];
	/// Take <https://schema.org/availableAtOrFrom> from [`Self`] as owned vector.
	fn take_available_at_or_from(&mut self) -> Vec<AvailableAtOrFromProperty>;
	/// Get <https://schema.org/availableDeliveryMethod> from [`Self`] as borrowed slice.
	fn get_available_delivery_method(&self) -> &[AvailableDeliveryMethodProperty];
	/// Take <https://schema.org/availableDeliveryMethod> from [`Self`] as owned vector.
	fn take_available_delivery_method(&mut self) -> Vec<AvailableDeliveryMethodProperty>;
	/// Get <https://schema.org/businessFunction> from [`Self`] as borrowed slice.
	fn get_business_function(&self) -> &[BusinessFunctionProperty];
	/// Take <https://schema.org/businessFunction> from [`Self`] as owned vector.
	fn take_business_function(&mut self) -> Vec<BusinessFunctionProperty>;
	/// Get <https://schema.org/category> from [`Self`] as borrowed slice.
	fn get_category(&self) -> &[CategoryProperty];
	/// Take <https://schema.org/category> from [`Self`] as owned vector.
	fn take_category(&mut self) -> Vec<CategoryProperty>;
	/// Get <https://schema.org/checkoutPageURLTemplate> from [`Self`] as borrowed slice.
	fn get_checkout_page_url_template(&self) -> &[CheckoutPageUrlTemplateProperty];
	/// Take <https://schema.org/checkoutPageURLTemplate> from [`Self`] as owned vector.
	fn take_checkout_page_url_template(&mut self) -> Vec<CheckoutPageUrlTemplateProperty>;
	/// Get <https://schema.org/deliveryLeadTime> from [`Self`] as borrowed slice.
	fn get_delivery_lead_time(&self) -> &[DeliveryLeadTimeProperty];
	/// Take <https://schema.org/deliveryLeadTime> from [`Self`] as owned vector.
	fn take_delivery_lead_time(&mut self) -> Vec<DeliveryLeadTimeProperty>;
	/// Get <https://schema.org/eligibleCustomerType> from [`Self`] as borrowed slice.
	fn get_eligible_customer_type(&self) -> &[EligibleCustomerTypeProperty];
	/// Take <https://schema.org/eligibleCustomerType> from [`Self`] as owned vector.
	fn take_eligible_customer_type(&mut self) -> Vec<EligibleCustomerTypeProperty>;
	/// Get <https://schema.org/eligibleDuration> from [`Self`] as borrowed slice.
	fn get_eligible_duration(&self) -> &[EligibleDurationProperty];
	/// Take <https://schema.org/eligibleDuration> from [`Self`] as owned vector.
	fn take_eligible_duration(&mut self) -> Vec<EligibleDurationProperty>;
	/// Get <https://schema.org/eligibleQuantity> from [`Self`] as borrowed slice.
	fn get_eligible_quantity(&self) -> &[EligibleQuantityProperty];
	/// Take <https://schema.org/eligibleQuantity> from [`Self`] as owned vector.
	fn take_eligible_quantity(&mut self) -> Vec<EligibleQuantityProperty>;
	/// Get <https://schema.org/eligibleRegion> from [`Self`] as borrowed slice.
	fn get_eligible_region(&self) -> &[EligibleRegionProperty];
	/// Take <https://schema.org/eligibleRegion> from [`Self`] as owned vector.
	fn take_eligible_region(&mut self) -> Vec<EligibleRegionProperty>;
	/// Get <https://schema.org/eligibleTransactionVolume> from [`Self`] as borrowed slice.
	fn get_eligible_transaction_volume(&self) -> &[EligibleTransactionVolumeProperty];
	/// Take <https://schema.org/eligibleTransactionVolume> from [`Self`] as owned vector.
	fn take_eligible_transaction_volume(&mut self) -> Vec<EligibleTransactionVolumeProperty>;
	/// Get <https://schema.org/gtin> from [`Self`] as borrowed slice.
	fn get_gtin(&self) -> &[GtinProperty];
	/// Take <https://schema.org/gtin> from [`Self`] as owned vector.
	fn take_gtin(&mut self) -> Vec<GtinProperty>;
	/// Get <https://schema.org/gtin12> from [`Self`] as borrowed slice.
	fn get_gtin_12(&self) -> &[Gtin12Property];
	/// Take <https://schema.org/gtin12> from [`Self`] as owned vector.
	fn take_gtin_12(&mut self) -> Vec<Gtin12Property>;
	/// Get <https://schema.org/gtin13> from [`Self`] as borrowed slice.
	fn get_gtin_13(&self) -> &[Gtin13Property];
	/// Take <https://schema.org/gtin13> from [`Self`] as owned vector.
	fn take_gtin_13(&mut self) -> Vec<Gtin13Property>;
	/// Get <https://schema.org/gtin14> from [`Self`] as borrowed slice.
	fn get_gtin_14(&self) -> &[Gtin14Property];
	/// Take <https://schema.org/gtin14> from [`Self`] as owned vector.
	fn take_gtin_14(&mut self) -> Vec<Gtin14Property>;
	/// Get <https://schema.org/gtin8> from [`Self`] as borrowed slice.
	fn get_gtin_8(&self) -> &[Gtin8Property];
	/// Take <https://schema.org/gtin8> from [`Self`] as owned vector.
	fn take_gtin_8(&mut self) -> Vec<Gtin8Property>;
	/// Get <https://schema.org/hasAdultConsideration> from [`Self`] as borrowed slice.
	fn get_has_adult_consideration(&self) -> &[HasAdultConsiderationProperty];
	/// Take <https://schema.org/hasAdultConsideration> from [`Self`] as owned vector.
	fn take_has_adult_consideration(&mut self) -> Vec<HasAdultConsiderationProperty>;
	/// Get <https://schema.org/hasMeasurement> from [`Self`] as borrowed slice.
	fn get_has_measurement(&self) -> &[HasMeasurementProperty];
	/// Take <https://schema.org/hasMeasurement> from [`Self`] as owned vector.
	fn take_has_measurement(&mut self) -> Vec<HasMeasurementProperty>;
	/// Get <https://schema.org/hasMerchantReturnPolicy> from [`Self`] as borrowed slice.
	fn get_has_merchant_return_policy(&self) -> &[HasMerchantReturnPolicyProperty];
	/// Take <https://schema.org/hasMerchantReturnPolicy> from [`Self`] as owned vector.
	fn take_has_merchant_return_policy(&mut self) -> Vec<HasMerchantReturnPolicyProperty>;
	/// Get <https://schema.org/includesObject> from [`Self`] as borrowed slice.
	fn get_includes_object(&self) -> &[IncludesObjectProperty];
	/// Take <https://schema.org/includesObject> from [`Self`] as owned vector.
	fn take_includes_object(&mut self) -> Vec<IncludesObjectProperty>;
	/// Get <https://schema.org/ineligibleRegion> from [`Self`] as borrowed slice.
	fn get_ineligible_region(&self) -> &[IneligibleRegionProperty];
	/// Take <https://schema.org/ineligibleRegion> from [`Self`] as owned vector.
	fn take_ineligible_region(&mut self) -> Vec<IneligibleRegionProperty>;
	/// Get <https://schema.org/inventoryLevel> from [`Self`] as borrowed slice.
	fn get_inventory_level(&self) -> &[InventoryLevelProperty];
	/// Take <https://schema.org/inventoryLevel> from [`Self`] as owned vector.
	fn take_inventory_level(&mut self) -> Vec<InventoryLevelProperty>;
	/// Get <https://schema.org/isFamilyFriendly> from [`Self`] as borrowed slice.
	fn get_is_family_friendly(&self) -> &[IsFamilyFriendlyProperty];
	/// Take <https://schema.org/isFamilyFriendly> from [`Self`] as owned vector.
	fn take_is_family_friendly(&mut self) -> Vec<IsFamilyFriendlyProperty>;
	/// Get <https://schema.org/itemCondition> from [`Self`] as borrowed slice.
	fn get_item_condition(&self) -> &[ItemConditionProperty];
	/// Take <https://schema.org/itemCondition> from [`Self`] as owned vector.
	fn take_item_condition(&mut self) -> Vec<ItemConditionProperty>;
	/// Get <https://schema.org/itemOffered> from [`Self`] as borrowed slice.
	fn get_item_offered(&self) -> &[ItemOfferedProperty];
	/// Take <https://schema.org/itemOffered> from [`Self`] as owned vector.
	fn take_item_offered(&mut self) -> Vec<ItemOfferedProperty>;
	/// Get <https://schema.org/leaseLength> from [`Self`] as borrowed slice.
	fn get_lease_length(&self) -> &[LeaseLengthProperty];
	/// Take <https://schema.org/leaseLength> from [`Self`] as owned vector.
	fn take_lease_length(&mut self) -> Vec<LeaseLengthProperty>;
	/// Get <https://schema.org/mobileUrl> from [`Self`] as borrowed slice.
	fn get_mobile_url(&self) -> &[MobileUrlProperty];
	/// Take <https://schema.org/mobileUrl> from [`Self`] as owned vector.
	fn take_mobile_url(&mut self) -> Vec<MobileUrlProperty>;
	/// Get <https://schema.org/mpn> from [`Self`] as borrowed slice.
	fn get_mpn(&self) -> &[MpnProperty];
	/// Take <https://schema.org/mpn> from [`Self`] as owned vector.
	fn take_mpn(&mut self) -> Vec<MpnProperty>;
	/// Get <https://schema.org/offeredBy> from [`Self`] as borrowed slice.
	fn get_offered_by(&self) -> &[OfferedByProperty];
	/// Take <https://schema.org/offeredBy> from [`Self`] as owned vector.
	fn take_offered_by(&mut self) -> Vec<OfferedByProperty>;
	/// Get <https://schema.org/price> from [`Self`] as borrowed slice.
	fn get_price(&self) -> &[PriceProperty];
	/// Take <https://schema.org/price> from [`Self`] as owned vector.
	fn take_price(&mut self) -> Vec<PriceProperty>;
	/// Get <https://schema.org/priceCurrency> from [`Self`] as borrowed slice.
	fn get_price_currency(&self) -> &[PriceCurrencyProperty];
	/// Take <https://schema.org/priceCurrency> from [`Self`] as owned vector.
	fn take_price_currency(&mut self) -> Vec<PriceCurrencyProperty>;
	/// Get <https://schema.org/priceSpecification> from [`Self`] as borrowed slice.
	fn get_price_specification(&self) -> &[PriceSpecificationProperty];
	/// Take <https://schema.org/priceSpecification> from [`Self`] as owned vector.
	fn take_price_specification(&mut self) -> Vec<PriceSpecificationProperty>;
	/// Get <https://schema.org/priceValidUntil> from [`Self`] as borrowed slice.
	fn get_price_valid_until(&self) -> &[PriceValidUntilProperty];
	/// Take <https://schema.org/priceValidUntil> from [`Self`] as owned vector.
	fn take_price_valid_until(&mut self) -> Vec<PriceValidUntilProperty>;
	/// Get <https://schema.org/review> from [`Self`] as borrowed slice.
	fn get_review(&self) -> &[ReviewProperty];
	/// Take <https://schema.org/review> from [`Self`] as owned vector.
	fn take_review(&mut self) -> Vec<ReviewProperty>;
	/// Get <https://schema.org/reviews> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/review>."]
	fn get_reviews(&self) -> &[ReviewsProperty];
	/// Take <https://schema.org/reviews> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/review>."]
	fn take_reviews(&mut self) -> Vec<ReviewsProperty>;
	/// Get <https://schema.org/seller> from [`Self`] as borrowed slice.
	fn get_seller(&self) -> &[SellerProperty];
	/// Take <https://schema.org/seller> from [`Self`] as owned vector.
	fn take_seller(&mut self) -> Vec<SellerProperty>;
	/// Get <https://schema.org/serialNumber> from [`Self`] as borrowed slice.
	fn get_serial_number(&self) -> &[SerialNumberProperty];
	/// Take <https://schema.org/serialNumber> from [`Self`] as owned vector.
	fn take_serial_number(&mut self) -> Vec<SerialNumberProperty>;
	/// Get <https://schema.org/shippingDetails> from [`Self`] as borrowed slice.
	fn get_shipping_details(&self) -> &[ShippingDetailsProperty];
	/// Take <https://schema.org/shippingDetails> from [`Self`] as owned vector.
	fn take_shipping_details(&mut self) -> Vec<ShippingDetailsProperty>;
	/// Get <https://schema.org/sku> from [`Self`] as borrowed slice.
	fn get_sku(&self) -> &[SkuProperty];
	/// Take <https://schema.org/sku> from [`Self`] as owned vector.
	fn take_sku(&mut self) -> Vec<SkuProperty>;
	/// Get <https://schema.org/validFrom> from [`Self`] as borrowed slice.
	fn get_valid_from(&self) -> &[ValidFromProperty];
	/// Take <https://schema.org/validFrom> from [`Self`] as owned vector.
	fn take_valid_from(&mut self) -> Vec<ValidFromProperty>;
	/// Get <https://schema.org/validThrough> from [`Self`] as borrowed slice.
	fn get_valid_through(&self) -> &[ValidThroughProperty];
	/// Take <https://schema.org/validThrough> from [`Self`] as owned vector.
	fn take_valid_through(&mut self) -> Vec<ValidThroughProperty>;
	/// Get <https://schema.org/warranty> from [`Self`] as borrowed slice.
	fn get_warranty(&self) -> &[WarrantyProperty];
	/// Take <https://schema.org/warranty> from [`Self`] as owned vector.
	fn take_warranty(&mut self) -> Vec<WarrantyProperty>;
}
impl OfferTrait for Offer {
	fn get_accepted_payment_method(&self) -> &[AcceptedPaymentMethodProperty] {
		self.r#accepted_payment_method.as_slice()
	}
	fn take_accepted_payment_method(&mut self) -> Vec<AcceptedPaymentMethodProperty> {
		std::mem::take(&mut self.r#accepted_payment_method)
	}
	fn get_add_on(&self) -> &[AddOnProperty] {
		self.r#add_on.as_slice()
	}
	fn take_add_on(&mut self) -> Vec<AddOnProperty> {
		std::mem::take(&mut self.r#add_on)
	}
	fn get_advance_booking_requirement(&self) -> &[AdvanceBookingRequirementProperty] {
		self.r#advance_booking_requirement.as_slice()
	}
	fn take_advance_booking_requirement(&mut self) -> Vec<AdvanceBookingRequirementProperty> {
		std::mem::take(&mut self.r#advance_booking_requirement)
	}
	fn get_aggregate_rating(&self) -> &[AggregateRatingProperty] {
		self.r#aggregate_rating.as_slice()
	}
	fn take_aggregate_rating(&mut self) -> Vec<AggregateRatingProperty> {
		std::mem::take(&mut self.r#aggregate_rating)
	}
	fn get_area_served(&self) -> &[AreaServedProperty] {
		self.r#area_served.as_slice()
	}
	fn take_area_served(&mut self) -> Vec<AreaServedProperty> {
		std::mem::take(&mut self.r#area_served)
	}
	fn get_asin(&self) -> &[AsinProperty] {
		self.r#asin.as_slice()
	}
	fn take_asin(&mut self) -> Vec<AsinProperty> {
		std::mem::take(&mut self.r#asin)
	}
	fn get_availability(&self) -> &[AvailabilityProperty] {
		self.r#availability.as_slice()
	}
	fn take_availability(&mut self) -> Vec<AvailabilityProperty> {
		std::mem::take(&mut self.r#availability)
	}
	fn get_availability_ends(&self) -> &[AvailabilityEndsProperty] {
		self.r#availability_ends.as_slice()
	}
	fn take_availability_ends(&mut self) -> Vec<AvailabilityEndsProperty> {
		std::mem::take(&mut self.r#availability_ends)
	}
	fn get_availability_starts(&self) -> &[AvailabilityStartsProperty] {
		self.r#availability_starts.as_slice()
	}
	fn take_availability_starts(&mut self) -> Vec<AvailabilityStartsProperty> {
		std::mem::take(&mut self.r#availability_starts)
	}
	fn get_available_at_or_from(&self) -> &[AvailableAtOrFromProperty] {
		self.r#available_at_or_from.as_slice()
	}
	fn take_available_at_or_from(&mut self) -> Vec<AvailableAtOrFromProperty> {
		std::mem::take(&mut self.r#available_at_or_from)
	}
	fn get_available_delivery_method(&self) -> &[AvailableDeliveryMethodProperty] {
		self.r#available_delivery_method.as_slice()
	}
	fn take_available_delivery_method(&mut self) -> Vec<AvailableDeliveryMethodProperty> {
		std::mem::take(&mut self.r#available_delivery_method)
	}
	fn get_business_function(&self) -> &[BusinessFunctionProperty] {
		self.r#business_function.as_slice()
	}
	fn take_business_function(&mut self) -> Vec<BusinessFunctionProperty> {
		std::mem::take(&mut self.r#business_function)
	}
	fn get_category(&self) -> &[CategoryProperty] {
		self.r#category.as_slice()
	}
	fn take_category(&mut self) -> Vec<CategoryProperty> {
		std::mem::take(&mut self.r#category)
	}
	fn get_checkout_page_url_template(&self) -> &[CheckoutPageUrlTemplateProperty] {
		self.r#checkout_page_url_template.as_slice()
	}
	fn take_checkout_page_url_template(&mut self) -> Vec<CheckoutPageUrlTemplateProperty> {
		std::mem::take(&mut self.r#checkout_page_url_template)
	}
	fn get_delivery_lead_time(&self) -> &[DeliveryLeadTimeProperty] {
		self.r#delivery_lead_time.as_slice()
	}
	fn take_delivery_lead_time(&mut self) -> Vec<DeliveryLeadTimeProperty> {
		std::mem::take(&mut self.r#delivery_lead_time)
	}
	fn get_eligible_customer_type(&self) -> &[EligibleCustomerTypeProperty] {
		self.r#eligible_customer_type.as_slice()
	}
	fn take_eligible_customer_type(&mut self) -> Vec<EligibleCustomerTypeProperty> {
		std::mem::take(&mut self.r#eligible_customer_type)
	}
	fn get_eligible_duration(&self) -> &[EligibleDurationProperty] {
		self.r#eligible_duration.as_slice()
	}
	fn take_eligible_duration(&mut self) -> Vec<EligibleDurationProperty> {
		std::mem::take(&mut self.r#eligible_duration)
	}
	fn get_eligible_quantity(&self) -> &[EligibleQuantityProperty] {
		self.r#eligible_quantity.as_slice()
	}
	fn take_eligible_quantity(&mut self) -> Vec<EligibleQuantityProperty> {
		std::mem::take(&mut self.r#eligible_quantity)
	}
	fn get_eligible_region(&self) -> &[EligibleRegionProperty] {
		self.r#eligible_region.as_slice()
	}
	fn take_eligible_region(&mut self) -> Vec<EligibleRegionProperty> {
		std::mem::take(&mut self.r#eligible_region)
	}
	fn get_eligible_transaction_volume(&self) -> &[EligibleTransactionVolumeProperty] {
		self.r#eligible_transaction_volume.as_slice()
	}
	fn take_eligible_transaction_volume(&mut self) -> Vec<EligibleTransactionVolumeProperty> {
		std::mem::take(&mut self.r#eligible_transaction_volume)
	}
	fn get_gtin(&self) -> &[GtinProperty] {
		self.r#gtin.as_slice()
	}
	fn take_gtin(&mut self) -> Vec<GtinProperty> {
		std::mem::take(&mut self.r#gtin)
	}
	fn get_gtin_12(&self) -> &[Gtin12Property] {
		self.r#gtin_12.as_slice()
	}
	fn take_gtin_12(&mut self) -> Vec<Gtin12Property> {
		std::mem::take(&mut self.r#gtin_12)
	}
	fn get_gtin_13(&self) -> &[Gtin13Property] {
		self.r#gtin_13.as_slice()
	}
	fn take_gtin_13(&mut self) -> Vec<Gtin13Property> {
		std::mem::take(&mut self.r#gtin_13)
	}
	fn get_gtin_14(&self) -> &[Gtin14Property] {
		self.r#gtin_14.as_slice()
	}
	fn take_gtin_14(&mut self) -> Vec<Gtin14Property> {
		std::mem::take(&mut self.r#gtin_14)
	}
	fn get_gtin_8(&self) -> &[Gtin8Property] {
		self.r#gtin_8.as_slice()
	}
	fn take_gtin_8(&mut self) -> Vec<Gtin8Property> {
		std::mem::take(&mut self.r#gtin_8)
	}
	fn get_has_adult_consideration(&self) -> &[HasAdultConsiderationProperty] {
		self.r#has_adult_consideration.as_slice()
	}
	fn take_has_adult_consideration(&mut self) -> Vec<HasAdultConsiderationProperty> {
		std::mem::take(&mut self.r#has_adult_consideration)
	}
	fn get_has_measurement(&self) -> &[HasMeasurementProperty] {
		self.r#has_measurement.as_slice()
	}
	fn take_has_measurement(&mut self) -> Vec<HasMeasurementProperty> {
		std::mem::take(&mut self.r#has_measurement)
	}
	fn get_has_merchant_return_policy(&self) -> &[HasMerchantReturnPolicyProperty] {
		self.r#has_merchant_return_policy.as_slice()
	}
	fn take_has_merchant_return_policy(&mut self) -> Vec<HasMerchantReturnPolicyProperty> {
		std::mem::take(&mut self.r#has_merchant_return_policy)
	}
	fn get_includes_object(&self) -> &[IncludesObjectProperty] {
		self.r#includes_object.as_slice()
	}
	fn take_includes_object(&mut self) -> Vec<IncludesObjectProperty> {
		std::mem::take(&mut self.r#includes_object)
	}
	fn get_ineligible_region(&self) -> &[IneligibleRegionProperty] {
		self.r#ineligible_region.as_slice()
	}
	fn take_ineligible_region(&mut self) -> Vec<IneligibleRegionProperty> {
		std::mem::take(&mut self.r#ineligible_region)
	}
	fn get_inventory_level(&self) -> &[InventoryLevelProperty] {
		self.r#inventory_level.as_slice()
	}
	fn take_inventory_level(&mut self) -> Vec<InventoryLevelProperty> {
		std::mem::take(&mut self.r#inventory_level)
	}
	fn get_is_family_friendly(&self) -> &[IsFamilyFriendlyProperty] {
		self.r#is_family_friendly.as_slice()
	}
	fn take_is_family_friendly(&mut self) -> Vec<IsFamilyFriendlyProperty> {
		std::mem::take(&mut self.r#is_family_friendly)
	}
	fn get_item_condition(&self) -> &[ItemConditionProperty] {
		self.r#item_condition.as_slice()
	}
	fn take_item_condition(&mut self) -> Vec<ItemConditionProperty> {
		std::mem::take(&mut self.r#item_condition)
	}
	fn get_item_offered(&self) -> &[ItemOfferedProperty] {
		self.r#item_offered.as_slice()
	}
	fn take_item_offered(&mut self) -> Vec<ItemOfferedProperty> {
		std::mem::take(&mut self.r#item_offered)
	}
	fn get_lease_length(&self) -> &[LeaseLengthProperty] {
		self.r#lease_length.as_slice()
	}
	fn take_lease_length(&mut self) -> Vec<LeaseLengthProperty> {
		std::mem::take(&mut self.r#lease_length)
	}
	fn get_mobile_url(&self) -> &[MobileUrlProperty] {
		self.r#mobile_url.as_slice()
	}
	fn take_mobile_url(&mut self) -> Vec<MobileUrlProperty> {
		std::mem::take(&mut self.r#mobile_url)
	}
	fn get_mpn(&self) -> &[MpnProperty] {
		self.r#mpn.as_slice()
	}
	fn take_mpn(&mut self) -> Vec<MpnProperty> {
		std::mem::take(&mut self.r#mpn)
	}
	fn get_offered_by(&self) -> &[OfferedByProperty] {
		self.r#offered_by.as_slice()
	}
	fn take_offered_by(&mut self) -> Vec<OfferedByProperty> {
		std::mem::take(&mut self.r#offered_by)
	}
	fn get_price(&self) -> &[PriceProperty] {
		self.r#price.as_slice()
	}
	fn take_price(&mut self) -> Vec<PriceProperty> {
		std::mem::take(&mut self.r#price)
	}
	fn get_price_currency(&self) -> &[PriceCurrencyProperty] {
		self.r#price_currency.as_slice()
	}
	fn take_price_currency(&mut self) -> Vec<PriceCurrencyProperty> {
		std::mem::take(&mut self.r#price_currency)
	}
	fn get_price_specification(&self) -> &[PriceSpecificationProperty] {
		self.r#price_specification.as_slice()
	}
	fn take_price_specification(&mut self) -> Vec<PriceSpecificationProperty> {
		std::mem::take(&mut self.r#price_specification)
	}
	fn get_price_valid_until(&self) -> &[PriceValidUntilProperty] {
		self.r#price_valid_until.as_slice()
	}
	fn take_price_valid_until(&mut self) -> Vec<PriceValidUntilProperty> {
		std::mem::take(&mut self.r#price_valid_until)
	}
	fn get_review(&self) -> &[ReviewProperty] {
		self.r#review.as_slice()
	}
	fn take_review(&mut self) -> Vec<ReviewProperty> {
		std::mem::take(&mut self.r#review)
	}
	fn get_reviews(&self) -> &[ReviewsProperty] {
		self.r#reviews.as_slice()
	}
	fn take_reviews(&mut self) -> Vec<ReviewsProperty> {
		std::mem::take(&mut self.r#reviews)
	}
	fn get_seller(&self) -> &[SellerProperty] {
		self.r#seller.as_slice()
	}
	fn take_seller(&mut self) -> Vec<SellerProperty> {
		std::mem::take(&mut self.r#seller)
	}
	fn get_serial_number(&self) -> &[SerialNumberProperty] {
		self.r#serial_number.as_slice()
	}
	fn take_serial_number(&mut self) -> Vec<SerialNumberProperty> {
		std::mem::take(&mut self.r#serial_number)
	}
	fn get_shipping_details(&self) -> &[ShippingDetailsProperty] {
		self.r#shipping_details.as_slice()
	}
	fn take_shipping_details(&mut self) -> Vec<ShippingDetailsProperty> {
		std::mem::take(&mut self.r#shipping_details)
	}
	fn get_sku(&self) -> &[SkuProperty] {
		self.r#sku.as_slice()
	}
	fn take_sku(&mut self) -> Vec<SkuProperty> {
		std::mem::take(&mut self.r#sku)
	}
	fn get_valid_from(&self) -> &[ValidFromProperty] {
		self.r#valid_from.as_slice()
	}
	fn take_valid_from(&mut self) -> Vec<ValidFromProperty> {
		std::mem::take(&mut self.r#valid_from)
	}
	fn get_valid_through(&self) -> &[ValidThroughProperty] {
		self.r#valid_through.as_slice()
	}
	fn take_valid_through(&mut self) -> Vec<ValidThroughProperty> {
		std::mem::take(&mut self.r#valid_through)
	}
	fn get_warranty(&self) -> &[WarrantyProperty] {
		self.r#warranty.as_slice()
	}
	fn take_warranty(&mut self) -> Vec<WarrantyProperty> {
		std::mem::take(&mut self.r#warranty)
	}
}
impl ThingTrait for Offer {
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
	impl Serialize for Offer {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#accepted_payment_method) as usize,
				!Vec::is_empty(&self.r#add_on) as usize,
				!Vec::is_empty(&self.r#advance_booking_requirement) as usize,
				!Vec::is_empty(&self.r#aggregate_rating) as usize,
				!Vec::is_empty(&self.r#area_served) as usize,
				!Vec::is_empty(&self.r#asin) as usize,
				!Vec::is_empty(&self.r#availability) as usize,
				!Vec::is_empty(&self.r#availability_ends) as usize,
				!Vec::is_empty(&self.r#availability_starts) as usize,
				!Vec::is_empty(&self.r#available_at_or_from) as usize,
				!Vec::is_empty(&self.r#available_delivery_method) as usize,
				!Vec::is_empty(&self.r#business_function) as usize,
				!Vec::is_empty(&self.r#category) as usize,
				!Vec::is_empty(&self.r#checkout_page_url_template) as usize,
				!Vec::is_empty(&self.r#delivery_lead_time) as usize,
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
				!Vec::is_empty(&self.r#has_adult_consideration) as usize,
				!Vec::is_empty(&self.r#has_measurement) as usize,
				!Vec::is_empty(&self.r#has_merchant_return_policy) as usize,
				!Vec::is_empty(&self.r#includes_object) as usize,
				!Vec::is_empty(&self.r#ineligible_region) as usize,
				!Vec::is_empty(&self.r#inventory_level) as usize,
				!Vec::is_empty(&self.r#is_family_friendly) as usize,
				!Vec::is_empty(&self.r#item_condition) as usize,
				!Vec::is_empty(&self.r#item_offered) as usize,
				!Vec::is_empty(&self.r#lease_length) as usize,
				!Vec::is_empty(&self.r#mobile_url) as usize,
				!Vec::is_empty(&self.r#mpn) as usize,
				!Vec::is_empty(&self.r#offered_by) as usize,
				!Vec::is_empty(&self.r#price) as usize,
				!Vec::is_empty(&self.r#price_currency) as usize,
				!Vec::is_empty(&self.r#price_specification) as usize,
				!Vec::is_empty(&self.r#price_valid_until) as usize,
				!Vec::is_empty(&self.r#review) as usize,
				!Vec::is_empty(&self.r#reviews) as usize,
				!Vec::is_empty(&self.r#seller) as usize,
				!Vec::is_empty(&self.r#serial_number) as usize,
				!Vec::is_empty(&self.r#shipping_details) as usize,
				!Vec::is_empty(&self.r#sku) as usize,
				!Vec::is_empty(&self.r#valid_from) as usize,
				!Vec::is_empty(&self.r#valid_through) as usize,
				!Vec::is_empty(&self.r#warranty) as usize,
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
			let mut serialize_struct = Serializer::serialize_struct(serializer, "Offer", len)?;
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
			if !Vec::is_empty(&self.r#add_on) {
				serialize_struct.serialize_field("addOn", {
					struct SerializeWith<'a>(&'a Vec<AddOnProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#add_on)
				})?;
			} else {
				serialize_struct.skip_field("addOn")?;
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
			if !Vec::is_empty(&self.r#checkout_page_url_template) {
				serialize_struct.serialize_field("checkoutPageURLTemplate", {
					struct SerializeWith<'a>(&'a Vec<CheckoutPageUrlTemplateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#checkout_page_url_template)
				})?;
			} else {
				serialize_struct.skip_field("checkoutPageURLTemplate")?;
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
			if !Vec::is_empty(&self.r#lease_length) {
				serialize_struct.serialize_field("leaseLength", {
					struct SerializeWith<'a>(&'a Vec<LeaseLengthProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#lease_length)
				})?;
			} else {
				serialize_struct.skip_field("leaseLength")?;
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
			if !Vec::is_empty(&self.r#offered_by) {
				serialize_struct.serialize_field("offeredBy", {
					struct SerializeWith<'a>(&'a Vec<OfferedByProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#offered_by)
				})?;
			} else {
				serialize_struct.skip_field("offeredBy")?;
			}
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
			if !Vec::is_empty(&self.r#price_valid_until) {
				serialize_struct.serialize_field("priceValidUntil", {
					struct SerializeWith<'a>(&'a Vec<PriceValidUntilProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#price_valid_until)
				})?;
			} else {
				serialize_struct.skip_field("priceValidUntil")?;
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
			if !Vec::is_empty(&self.r#shipping_details) {
				serialize_struct.serialize_field("shippingDetails", {
					struct SerializeWith<'a>(&'a Vec<ShippingDetailsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#shipping_details)
				})?;
			} else {
				serialize_struct.skip_field("shippingDetails")?;
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
	impl<'de> Deserialize<'de> for Offer {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AcceptedPaymentMethod,
				AddOn,
				AdvanceBookingRequirement,
				AggregateRating,
				AreaServed,
				Asin,
				Availability,
				AvailabilityEnds,
				AvailabilityStarts,
				AvailableAtOrFrom,
				AvailableDeliveryMethod,
				BusinessFunction,
				Category,
				CheckoutPageUrlTemplate,
				DeliveryLeadTime,
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
				HasAdultConsideration,
				HasMeasurement,
				HasMerchantReturnPolicy,
				IncludesObject,
				IneligibleRegion,
				InventoryLevel,
				IsFamilyFriendly,
				ItemCondition,
				ItemOffered,
				LeaseLength,
				MobileUrl,
				Mpn,
				OfferedBy,
				Price,
				PriceCurrency,
				PriceSpecification,
				PriceValidUntil,
				Review,
				Reviews,
				Seller,
				SerialNumber,
				ShippingDetails,
				Sku,
				ValidFrom,
				ValidThrough,
				Warranty,
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
						"acceptedPaymentMethod" => Ok(Field::AcceptedPaymentMethod),
						"addOn" => Ok(Field::AddOn),
						"advanceBookingRequirement" => Ok(Field::AdvanceBookingRequirement),
						"aggregateRating" => Ok(Field::AggregateRating),
						"areaServed" => Ok(Field::AreaServed),
						"asin" => Ok(Field::Asin),
						"availability" => Ok(Field::Availability),
						"availabilityEnds" => Ok(Field::AvailabilityEnds),
						"availabilityStarts" => Ok(Field::AvailabilityStarts),
						"availableAtOrFrom" => Ok(Field::AvailableAtOrFrom),
						"availableDeliveryMethod" => Ok(Field::AvailableDeliveryMethod),
						"businessFunction" => Ok(Field::BusinessFunction),
						"category" => Ok(Field::Category),
						"checkoutPageURLTemplate" => Ok(Field::CheckoutPageUrlTemplate),
						"deliveryLeadTime" => Ok(Field::DeliveryLeadTime),
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
						"hasAdultConsideration" => Ok(Field::HasAdultConsideration),
						"hasMeasurement" => Ok(Field::HasMeasurement),
						"hasMerchantReturnPolicy" => Ok(Field::HasMerchantReturnPolicy),
						"includesObject" => Ok(Field::IncludesObject),
						"ineligibleRegion" => Ok(Field::IneligibleRegion),
						"inventoryLevel" => Ok(Field::InventoryLevel),
						"isFamilyFriendly" => Ok(Field::IsFamilyFriendly),
						"itemCondition" => Ok(Field::ItemCondition),
						"itemOffered" => Ok(Field::ItemOffered),
						"leaseLength" => Ok(Field::LeaseLength),
						"mobileUrl" => Ok(Field::MobileUrl),
						"mpn" => Ok(Field::Mpn),
						"offeredBy" => Ok(Field::OfferedBy),
						"price" => Ok(Field::Price),
						"priceCurrency" => Ok(Field::PriceCurrency),
						"priceSpecification" => Ok(Field::PriceSpecification),
						"priceValidUntil" => Ok(Field::PriceValidUntil),
						"review" => Ok(Field::Review),
						"reviews" => Ok(Field::Reviews),
						"seller" => Ok(Field::Seller),
						"serialNumber" => Ok(Field::SerialNumber),
						"shippingDetails" => Ok(Field::ShippingDetails),
						"sku" => Ok(Field::Sku),
						"validFrom" => Ok(Field::ValidFrom),
						"validThrough" => Ok(Field::ValidThrough),
						"warranty" => Ok(Field::Warranty),
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
						b"acceptedPaymentMethod" => Ok(Field::AcceptedPaymentMethod),
						b"addOn" => Ok(Field::AddOn),
						b"advanceBookingRequirement" => Ok(Field::AdvanceBookingRequirement),
						b"aggregateRating" => Ok(Field::AggregateRating),
						b"areaServed" => Ok(Field::AreaServed),
						b"asin" => Ok(Field::Asin),
						b"availability" => Ok(Field::Availability),
						b"availabilityEnds" => Ok(Field::AvailabilityEnds),
						b"availabilityStarts" => Ok(Field::AvailabilityStarts),
						b"availableAtOrFrom" => Ok(Field::AvailableAtOrFrom),
						b"availableDeliveryMethod" => Ok(Field::AvailableDeliveryMethod),
						b"businessFunction" => Ok(Field::BusinessFunction),
						b"category" => Ok(Field::Category),
						b"checkoutPageURLTemplate" => Ok(Field::CheckoutPageUrlTemplate),
						b"deliveryLeadTime" => Ok(Field::DeliveryLeadTime),
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
						b"hasAdultConsideration" => Ok(Field::HasAdultConsideration),
						b"hasMeasurement" => Ok(Field::HasMeasurement),
						b"hasMerchantReturnPolicy" => Ok(Field::HasMerchantReturnPolicy),
						b"includesObject" => Ok(Field::IncludesObject),
						b"ineligibleRegion" => Ok(Field::IneligibleRegion),
						b"inventoryLevel" => Ok(Field::InventoryLevel),
						b"isFamilyFriendly" => Ok(Field::IsFamilyFriendly),
						b"itemCondition" => Ok(Field::ItemCondition),
						b"itemOffered" => Ok(Field::ItemOffered),
						b"leaseLength" => Ok(Field::LeaseLength),
						b"mobileUrl" => Ok(Field::MobileUrl),
						b"mpn" => Ok(Field::Mpn),
						b"offeredBy" => Ok(Field::OfferedBy),
						b"price" => Ok(Field::Price),
						b"priceCurrency" => Ok(Field::PriceCurrency),
						b"priceSpecification" => Ok(Field::PriceSpecification),
						b"priceValidUntil" => Ok(Field::PriceValidUntil),
						b"review" => Ok(Field::Review),
						b"reviews" => Ok(Field::Reviews),
						b"seller" => Ok(Field::Seller),
						b"serialNumber" => Ok(Field::SerialNumber),
						b"shippingDetails" => Ok(Field::ShippingDetails),
						b"sku" => Ok(Field::Sku),
						b"validFrom" => Ok(Field::ValidFrom),
						b"validThrough" => Ok(Field::ValidThrough),
						b"warranty" => Ok(Field::Warranty),
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
				type Value = Offer;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Offer")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#accepted_payment_method_property = None;
					let mut r#add_on_property = None;
					let mut r#advance_booking_requirement_property = None;
					let mut r#aggregate_rating_property = None;
					let mut r#area_served_property = None;
					let mut r#asin_property = None;
					let mut r#availability_property = None;
					let mut r#availability_ends_property = None;
					let mut r#availability_starts_property = None;
					let mut r#available_at_or_from_property = None;
					let mut r#available_delivery_method_property = None;
					let mut r#business_function_property = None;
					let mut r#category_property = None;
					let mut r#checkout_page_url_template_property = None;
					let mut r#delivery_lead_time_property = None;
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
					let mut r#has_adult_consideration_property = None;
					let mut r#has_measurement_property = None;
					let mut r#has_merchant_return_policy_property = None;
					let mut r#includes_object_property = None;
					let mut r#ineligible_region_property = None;
					let mut r#inventory_level_property = None;
					let mut r#is_family_friendly_property = None;
					let mut r#item_condition_property = None;
					let mut r#item_offered_property = None;
					let mut r#lease_length_property = None;
					let mut r#mobile_url_property = None;
					let mut r#mpn_property = None;
					let mut r#offered_by_property = None;
					let mut r#price_property = None;
					let mut r#price_currency_property = None;
					let mut r#price_specification_property = None;
					let mut r#price_valid_until_property = None;
					let mut r#review_property = None;
					let mut r#reviews_property = None;
					let mut r#seller_property = None;
					let mut r#serial_number_property = None;
					let mut r#shipping_details_property = None;
					let mut r#sku_property = None;
					let mut r#valid_from_property = None;
					let mut r#valid_through_property = None;
					let mut r#warranty_property = None;
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
							Field::AddOn => {
								if r#add_on_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("addOn"));
								}
								r#add_on_property = Some({
									struct DeserializeWith(Vec<AddOnProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::CheckoutPageUrlTemplate => {
								if r#checkout_page_url_template_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"checkoutPageURLTemplate",
									));
								}
								r#checkout_page_url_template_property = Some({
									struct DeserializeWith(Vec<CheckoutPageUrlTemplateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::LeaseLength => {
								if r#lease_length_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"leaseLength",
									));
								}
								r#lease_length_property = Some({
									struct DeserializeWith(Vec<LeaseLengthProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::OfferedBy => {
								if r#offered_by_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"offeredBy",
									));
								}
								r#offered_by_property = Some({
									struct DeserializeWith(Vec<OfferedByProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
							Field::PriceValidUntil => {
								if r#price_valid_until_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"priceValidUntil",
									));
								}
								r#price_valid_until_property = Some({
									struct DeserializeWith(Vec<PriceValidUntilProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::ShippingDetails => {
								if r#shipping_details_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"shippingDetails",
									));
								}
								r#shipping_details_property = Some({
									struct DeserializeWith(Vec<ShippingDetailsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
					Ok(Offer {
						r#accepted_payment_method: r#accepted_payment_method_property
							.unwrap_or_default(),
						r#add_on: r#add_on_property.unwrap_or_default(),
						r#advance_booking_requirement: r#advance_booking_requirement_property
							.unwrap_or_default(),
						r#aggregate_rating: r#aggregate_rating_property.unwrap_or_default(),
						r#area_served: r#area_served_property.unwrap_or_default(),
						r#asin: r#asin_property.unwrap_or_default(),
						r#availability: r#availability_property.unwrap_or_default(),
						r#availability_ends: r#availability_ends_property.unwrap_or_default(),
						r#availability_starts: r#availability_starts_property.unwrap_or_default(),
						r#available_at_or_from: r#available_at_or_from_property.unwrap_or_default(),
						r#available_delivery_method: r#available_delivery_method_property
							.unwrap_or_default(),
						r#business_function: r#business_function_property.unwrap_or_default(),
						r#category: r#category_property.unwrap_or_default(),
						r#checkout_page_url_template: r#checkout_page_url_template_property
							.unwrap_or_default(),
						r#delivery_lead_time: r#delivery_lead_time_property.unwrap_or_default(),
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
						r#has_adult_consideration: r#has_adult_consideration_property
							.unwrap_or_default(),
						r#has_measurement: r#has_measurement_property.unwrap_or_default(),
						r#has_merchant_return_policy: r#has_merchant_return_policy_property
							.unwrap_or_default(),
						r#includes_object: r#includes_object_property.unwrap_or_default(),
						r#ineligible_region: r#ineligible_region_property.unwrap_or_default(),
						r#inventory_level: r#inventory_level_property.unwrap_or_default(),
						r#is_family_friendly: r#is_family_friendly_property.unwrap_or_default(),
						r#item_condition: r#item_condition_property.unwrap_or_default(),
						r#item_offered: r#item_offered_property.unwrap_or_default(),
						r#lease_length: r#lease_length_property.unwrap_or_default(),
						r#mobile_url: r#mobile_url_property.unwrap_or_default(),
						r#mpn: r#mpn_property.unwrap_or_default(),
						r#offered_by: r#offered_by_property.unwrap_or_default(),
						r#price: r#price_property.unwrap_or_default(),
						r#price_currency: r#price_currency_property.unwrap_or_default(),
						r#price_specification: r#price_specification_property.unwrap_or_default(),
						r#price_valid_until: r#price_valid_until_property.unwrap_or_default(),
						r#review: r#review_property.unwrap_or_default(),
						r#reviews: r#reviews_property.unwrap_or_default(),
						r#seller: r#seller_property.unwrap_or_default(),
						r#serial_number: r#serial_number_property.unwrap_or_default(),
						r#shipping_details: r#shipping_details_property.unwrap_or_default(),
						r#sku: r#sku_property.unwrap_or_default(),
						r#valid_from: r#valid_from_property.unwrap_or_default(),
						r#valid_through: r#valid_through_property.unwrap_or_default(),
						r#warranty: r#warranty_property.unwrap_or_default(),
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
				"acceptedPaymentMethod",
				"addOn",
				"advanceBookingRequirement",
				"aggregateRating",
				"areaServed",
				"asin",
				"availability",
				"availabilityEnds",
				"availabilityStarts",
				"availableAtOrFrom",
				"availableDeliveryMethod",
				"businessFunction",
				"category",
				"checkoutPageURLTemplate",
				"deliveryLeadTime",
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
				"hasAdultConsideration",
				"hasMeasurement",
				"hasMerchantReturnPolicy",
				"includesObject",
				"ineligibleRegion",
				"inventoryLevel",
				"isFamilyFriendly",
				"itemCondition",
				"itemOffered",
				"leaseLength",
				"mobileUrl",
				"mpn",
				"offeredBy",
				"price",
				"priceCurrency",
				"priceSpecification",
				"priceValidUntil",
				"review",
				"reviews",
				"seller",
				"serialNumber",
				"shippingDetails",
				"sku",
				"validFrom",
				"validThrough",
				"warranty",
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
			deserializer.deserialize_struct("Offer", FIELDS, ClassVisitor)
		}
	}
}
