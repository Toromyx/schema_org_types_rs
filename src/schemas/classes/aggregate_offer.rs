use super::*;
/// <https://schema.org/AggregateOffer>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct AggregateOffer {
    #[cfg(any(
        any(
            feature = "accepted-payment-method-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "acceptedPaymentMethod"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#accepted_payment_method: Vec<AcceptedPaymentMethodProperty>,
    #[cfg(any(
        any(feature = "add-on-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "addOn"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#add_on: Vec<AddOnProperty>,
    #[cfg(any(
        any(
            feature = "additional-type-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "additionalType"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#additional_type: Vec<AdditionalTypeProperty>,
    #[cfg(any(
        any(
            feature = "advance-booking-requirement-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(rename = "advanceBookingRequirement")
    )]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#advance_booking_requirement: Vec<AdvanceBookingRequirementProperty>,
    #[cfg(any(
        any(
            feature = "aggregate-rating-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "aggregateRating"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#aggregate_rating: Vec<AggregateRatingProperty>,
    #[cfg(any(
        any(
            feature = "alternate-name-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "alternateName"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#alternate_name: Vec<AlternateNameProperty>,
    #[cfg(any(
        any(
            feature = "area-served-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "areaServed"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#area_served: Vec<AreaServedProperty>,
    #[cfg(any(
        any(feature = "asin-property-schema", feature = "pending-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "asin"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#asin: Vec<AsinProperty>,
    #[cfg(any(
        any(
            feature = "availability-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "availability"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#availability: Vec<AvailabilityProperty>,
    #[cfg(any(
        any(
            feature = "availability-ends-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "availabilityEnds"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#availability_ends: Vec<AvailabilityEndsProperty>,
    #[cfg(any(
        any(
            feature = "availability-starts-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "availabilityStarts"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#availability_starts: Vec<AvailabilityStartsProperty>,
    #[cfg(any(
        any(
            feature = "available-at-or-from-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "availableAtOrFrom"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#available_at_or_from: Vec<AvailableAtOrFromProperty>,
    #[cfg(any(
        any(
            feature = "available-delivery-method-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "availableDeliveryMethod"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#available_delivery_method: Vec<AvailableDeliveryMethodProperty>,
    #[cfg(any(
        any(
            feature = "business-function-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "businessFunction"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#business_function: Vec<BusinessFunctionProperty>,
    #[cfg(any(
        any(
            feature = "category-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "category"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#category: Vec<CategoryProperty>,
    #[cfg(any(
        any(
            feature = "checkout-page-url-template-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "checkoutPageURLTemplate"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#checkout_page_url_template: Vec<CheckoutPageUrlTemplateProperty>,
    #[cfg(any(
        any(
            feature = "delivery-lead-time-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "deliveryLeadTime"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#delivery_lead_time: Vec<DeliveryLeadTimeProperty>,
    #[cfg(any(
        any(
            feature = "description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "description"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#description: Vec<DescriptionProperty>,
    #[cfg(any(
        any(
            feature = "disambiguating-description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(rename = "disambiguatingDescription")
    )]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
    #[cfg(any(
        any(
            feature = "eligible-customer-type-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "eligibleCustomerType"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#eligible_customer_type: Vec<EligibleCustomerTypeProperty>,
    #[cfg(any(
        any(
            feature = "eligible-duration-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "eligibleDuration"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#eligible_duration: Vec<EligibleDurationProperty>,
    #[cfg(any(
        any(
            feature = "eligible-quantity-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "eligibleQuantity"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#eligible_quantity: Vec<EligibleQuantityProperty>,
    #[cfg(any(
        any(
            feature = "eligible-region-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "eligibleRegion"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#eligible_region: Vec<EligibleRegionProperty>,
    #[cfg(any(
        any(
            feature = "eligible-transaction-volume-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(rename = "eligibleTransactionVolume")
    )]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#eligible_transaction_volume: Vec<EligibleTransactionVolumeProperty>,
    #[cfg(any(
        any(feature = "gtin-property-schema", feature = "pending-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "gtin"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#gtin: Vec<GtinProperty>,
    #[cfg(any(
        any(
            feature = "gtin-12-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "gtin12"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#gtin_12: Vec<Gtin12Property>,
    #[cfg(any(
        any(
            feature = "gtin-13-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "gtin13"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#gtin_13: Vec<Gtin13Property>,
    #[cfg(any(
        any(
            feature = "gtin-14-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "gtin14"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#gtin_14: Vec<Gtin14Property>,
    #[cfg(any(
        any(feature = "gtin-8-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "gtin8"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#gtin_8: Vec<Gtin8Property>,
    #[cfg(any(
        any(
            feature = "has-adult-consideration-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "hasAdultConsideration"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#has_adult_consideration: Vec<HasAdultConsiderationProperty>,
    #[cfg(any(
        any(
            feature = "has-measurement-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "hasMeasurement"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#has_measurement: Vec<HasMeasurementProperty>,
    #[cfg(any(
        any(
            feature = "has-merchant-return-policy-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "hasMerchantReturnPolicy"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#has_merchant_return_policy: Vec<HasMerchantReturnPolicyProperty>,
    #[cfg(any(
        any(
            feature = "high-price-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "highPrice"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#high_price: Vec<HighPriceProperty>,
    #[cfg(any(
        any(
            feature = "identifier-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "identifier"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#identifier: Vec<IdentifierProperty>,
    #[cfg(any(
        any(feature = "image-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "image"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#image: Vec<ImageProperty>,
    #[cfg(any(
        any(
            feature = "includes-object-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "includesObject"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#includes_object: Vec<IncludesObjectProperty>,
    #[cfg(any(
        any(
            feature = "ineligible-region-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "ineligibleRegion"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#ineligible_region: Vec<IneligibleRegionProperty>,
    #[cfg(any(
        any(
            feature = "inventory-level-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "inventoryLevel"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#inventory_level: Vec<InventoryLevelProperty>,
    #[cfg(any(
        any(
            feature = "is-family-friendly-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "isFamilyFriendly"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#is_family_friendly: Vec<IsFamilyFriendlyProperty>,
    #[cfg(any(
        any(
            feature = "item-condition-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "itemCondition"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#item_condition: Vec<ItemConditionProperty>,
    #[cfg(any(
        any(
            feature = "item-offered-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "itemOffered"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#item_offered: Vec<ItemOfferedProperty>,
    #[cfg(any(
        any(
            feature = "lease-length-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "leaseLength"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#lease_length: Vec<LeaseLengthProperty>,
    #[cfg(any(
        any(
            feature = "low-price-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "lowPrice"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#low_price: Vec<LowPriceProperty>,
    #[cfg(any(
        any(
            feature = "main-entity-of-page-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "mainEntityOfPage"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
    #[cfg(any(
        any(
            feature = "mobile-url-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "mobileUrl"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#mobile_url: Vec<MobileUrlProperty>,
    #[cfg(any(
        any(feature = "mpn-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "mpn"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#mpn: Vec<MpnProperty>,
    #[cfg(any(
        any(feature = "name-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "name"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#name: Vec<NameProperty>,
    #[cfg(any(
        any(
            feature = "offer-count-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "offerCount"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#offer_count: Vec<OfferCountProperty>,
    #[cfg(any(
        any(
            feature = "offered-by-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "offeredBy"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#offered_by: Vec<OfferedByProperty>,
    #[cfg(any(
        any(feature = "offers-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "offers"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#offers: Vec<OffersProperty>,
    #[cfg(any(
        any(
            feature = "potential-action-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "potentialAction"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#potential_action: Vec<PotentialActionProperty>,
    #[cfg(any(
        any(feature = "price-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "price"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#price: Vec<PriceProperty>,
    #[cfg(any(
        any(
            feature = "price-currency-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "priceCurrency"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#price_currency: Vec<PriceCurrencyProperty>,
    #[cfg(any(
        any(
            feature = "price-specification-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "priceSpecification"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#price_specification: Vec<PriceSpecificationProperty>,
    #[cfg(any(
        any(
            feature = "price-valid-until-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "priceValidUntil"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#price_valid_until: Vec<PriceValidUntilProperty>,
    #[cfg(any(
        any(feature = "review-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "review"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#review: Vec<ReviewProperty>,
    #[cfg(any(
        any(
            feature = "reviews-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "reviews"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#reviews: Vec<ReviewsProperty>,
    #[cfg(any(
        any(
            feature = "same-as-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "sameAs"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#same_as: Vec<SameAsProperty>,
    #[cfg(any(
        any(feature = "seller-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "seller"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#seller: Vec<SellerProperty>,
    #[cfg(any(
        any(
            feature = "serial-number-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "serialNumber"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#serial_number: Vec<SerialNumberProperty>,
    #[cfg(any(
        any(
            feature = "shipping-details-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "shippingDetails"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#shipping_details: Vec<ShippingDetailsProperty>,
    #[cfg(any(
        any(feature = "sku-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "sku"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#sku: Vec<SkuProperty>,
    #[cfg(any(
        any(
            feature = "subject-of-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "subjectOf"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#subject_of: Vec<SubjectOfProperty>,
    #[cfg(any(
        any(feature = "url-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "url"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#url: Vec<UrlProperty>,
    #[cfg(any(
        any(
            feature = "valid-from-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "validFrom"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#valid_from: Vec<ValidFromProperty>,
    #[cfg(any(
        any(
            feature = "valid-through-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "validThrough"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#valid_through: Vec<ValidThroughProperty>,
    #[cfg(any(
        any(
            feature = "warranty-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "warranty"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#warranty: Vec<WarrantyProperty>,
}
