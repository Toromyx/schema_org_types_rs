use super::*;
/// A chemical or biologic substance, used as a medical therapy, that has a physiological effect on an organism. Here the term drug is used interchangeably with the term medicine although clinical knowledge makes a clear difference between them.
///
/// https://schema.org/Drug
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Drug {
    #[cfg(any(
        feature = "active-ingredient-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "activeIngredient"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#active_ingredient: Vec<ActiveIngredientProperty>,
    #[cfg(any(
        feature = "additional-property-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "additionalProperty"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#additional_property: Vec<AdditionalPropertyProperty>,
    #[cfg(any(
        feature = "additional-type-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "additionalType"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#additional_type: Vec<AdditionalTypeProperty>,
    #[cfg(any(
        feature = "administration-route-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "administrationRoute"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#administration_route: Vec<AdministrationRouteProperty>,
    #[cfg(any(
        feature = "aggregate-rating-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "aggregateRating"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#aggregate_rating: Vec<AggregateRatingProperty>,
    #[cfg(any(
        feature = "alcohol-warning-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "alcoholWarning"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#alcohol_warning: Vec<AlcoholWarningProperty>,
    #[cfg(any(
        feature = "alternate-name-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "alternateName"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#alternate_name: Vec<AlternateNameProperty>,
    #[cfg(any(feature = "asin-property-schema", feature = "pending-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "asin"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#asin: Vec<AsinProperty>,
    #[cfg(any(
        feature = "audience-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "audience"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#audience: Vec<AudienceProperty>,
    #[cfg(any(
        feature = "available-strength-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "availableStrength"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#available_strength: Vec<AvailableStrengthProperty>,
    #[cfg(any(feature = "award-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "award"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#award: Vec<AwardProperty>,
    #[cfg(any(feature = "awards-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "awards"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#awards: Vec<AwardsProperty>,
    #[cfg(any(feature = "brand-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "brand"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#brand: Vec<BrandProperty>,
    #[cfg(any(
        feature = "breastfeeding-warning-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "breastfeedingWarning"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#breastfeeding_warning: Vec<BreastfeedingWarningProperty>,
    #[cfg(any(
        feature = "category-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "category"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#category: Vec<CategoryProperty>,
    #[cfg(any(
        feature = "clincal-pharmacology-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "clincalPharmacology"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#clincal_pharmacology: Vec<ClincalPharmacologyProperty>,
    #[cfg(any(
        feature = "clinical-pharmacology-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "clinicalPharmacology"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#clinical_pharmacology: Vec<ClinicalPharmacologyProperty>,
    #[cfg(any(
        feature = "code-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "code"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#code: Vec<CodeProperty>,
    #[cfg(any(feature = "color-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "color"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#color: Vec<ColorProperty>,
    #[cfg(any(
        feature = "country-of-assembly-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "countryOfAssembly"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#country_of_assembly: Vec<CountryOfAssemblyProperty>,
    #[cfg(any(
        feature = "country-of-last-processing-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "countryOfLastProcessing"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#country_of_last_processing: Vec<CountryOfLastProcessingProperty>,
    #[cfg(any(
        feature = "country-of-origin-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "countryOfOrigin"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#country_of_origin: Vec<CountryOfOriginProperty>,
    #[cfg(any(feature = "depth-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "depth"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#depth: Vec<DepthProperty>,
    #[cfg(any(
        feature = "description-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "description"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#description: Vec<DescriptionProperty>,
    #[cfg(any(
        feature = "disambiguating-description-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "disambiguatingDescription"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
    #[cfg(any(
        feature = "dosage-form-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "dosageForm"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#dosage_form: Vec<DosageFormProperty>,
    #[cfg(any(
        feature = "dose-schedule-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "doseSchedule"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#dose_schedule: Vec<DoseScheduleProperty>,
    #[cfg(any(
        feature = "drug-class-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "drugClass"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#drug_class: Vec<DrugClassProperty>,
    #[cfg(any(
        feature = "drug-unit-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "drugUnit"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#drug_unit: Vec<DrugUnitProperty>,
    #[cfg(any(
        feature = "food-warning-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "foodWarning"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#food_warning: Vec<FoodWarningProperty>,
    #[cfg(any(
        feature = "funding-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "funding"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#funding: Vec<FundingProperty>,
    #[cfg(any(feature = "gtin-property-schema", feature = "pending-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "gtin"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#gtin: Vec<GtinProperty>,
    #[cfg(any(
        feature = "gtin-12-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "gtin12"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#gtin_12: Vec<Gtin12Property>,
    #[cfg(any(
        feature = "gtin-13-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "gtin13"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#gtin_13: Vec<Gtin13Property>,
    #[cfg(any(
        feature = "gtin-14-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "gtin14"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#gtin_14: Vec<Gtin14Property>,
    #[cfg(any(feature = "gtin-8-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "gtin8"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#gtin_8: Vec<Gtin8Property>,
    #[cfg(any(
        feature = "guideline-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "guideline"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#guideline: Vec<GuidelineProperty>,
    #[cfg(any(
        feature = "has-adult-consideration-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasAdultConsideration"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#has_adult_consideration: Vec<HasAdultConsiderationProperty>,
    #[cfg(any(
        feature = "has-energy-consumption-details-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasEnergyConsumptionDetails"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#has_energy_consumption_details: Vec<HasEnergyConsumptionDetailsProperty>,
    #[cfg(any(
        feature = "has-measurement-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasMeasurement"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#has_measurement: Vec<HasMeasurementProperty>,
    #[cfg(any(
        feature = "has-merchant-return-policy-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasMerchantReturnPolicy"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#has_merchant_return_policy: Vec<HasMerchantReturnPolicyProperty>,
    #[cfg(any(
        feature = "has-product-return-policy-property-schema",
        feature = "attic-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasProductReturnPolicy"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#has_product_return_policy: Vec<HasProductReturnPolicyProperty>,
    #[cfg(any(feature = "height-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "height"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#height: Vec<HeightProperty>,
    #[cfg(any(
        feature = "identifier-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "identifier"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#identifier: Vec<IdentifierProperty>,
    #[cfg(any(feature = "image-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "image"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#image: Vec<ImageProperty>,
    #[cfg(any(
        feature = "in-product-group-with-id-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "inProductGroupWithID"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#in_product_group_with_id: Vec<InProductGroupWithIdProperty>,
    #[cfg(any(
        feature = "included-in-health-insurance-plan-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "includedInHealthInsurancePlan"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#included_in_health_insurance_plan: Vec<IncludedInHealthInsurancePlanProperty>,
    #[cfg(any(
        feature = "interacting-drug-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "interactingDrug"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#interacting_drug: Vec<InteractingDrugProperty>,
    #[cfg(any(
        feature = "is-accessory-or-spare-part-for-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isAccessoryOrSparePartFor"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#is_accessory_or_spare_part_for: Vec<IsAccessoryOrSparePartForProperty>,
    #[cfg(any(
        feature = "is-available-generically-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isAvailableGenerically"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#is_available_generically: Vec<IsAvailableGenericallyProperty>,
    #[cfg(any(
        feature = "is-consumable-for-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isConsumableFor"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#is_consumable_for: Vec<IsConsumableForProperty>,
    #[cfg(any(
        feature = "is-family-friendly-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isFamilyFriendly"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#is_family_friendly: Vec<IsFamilyFriendlyProperty>,
    #[cfg(any(
        feature = "is-proprietary-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isProprietary"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#is_proprietary: Vec<IsProprietaryProperty>,
    #[cfg(any(
        feature = "is-related-to-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isRelatedTo"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#is_related_to: Vec<IsRelatedToProperty>,
    #[cfg(any(
        feature = "is-similar-to-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isSimilarTo"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#is_similar_to: Vec<IsSimilarToProperty>,
    #[cfg(any(
        feature = "is-variant-of-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isVariantOf"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#is_variant_of: Vec<IsVariantOfProperty>,
    #[cfg(any(
        feature = "item-condition-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "itemCondition"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#item_condition: Vec<ItemConditionProperty>,
    #[cfg(any(
        feature = "keywords-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "keywords"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#keywords: Vec<KeywordsProperty>,
    #[cfg(any(
        feature = "label-details-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "labelDetails"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#label_details: Vec<LabelDetailsProperty>,
    #[cfg(any(
        feature = "legal-status-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "legalStatus"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#legal_status: Vec<LegalStatusProperty>,
    #[cfg(any(feature = "logo-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "logo"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#logo: Vec<LogoProperty>,
    #[cfg(any(
        feature = "main-entity-of-page-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mainEntityOfPage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
    #[cfg(any(
        feature = "manufacturer-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "manufacturer"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#manufacturer: Vec<ManufacturerProperty>,
    #[cfg(any(
        feature = "material-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "material"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#material: Vec<MaterialProperty>,
    #[cfg(any(
        feature = "maximum-intake-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "maximumIntake"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#maximum_intake: Vec<MaximumIntakeProperty>,
    #[cfg(any(
        feature = "mechanism-of-action-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mechanismOfAction"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#mechanism_of_action: Vec<MechanismOfActionProperty>,
    #[cfg(any(
        feature = "medicine-system-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "medicineSystem"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#medicine_system: Vec<MedicineSystemProperty>,
    #[cfg(any(
        feature = "mobile-url-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mobileUrl"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#mobile_url: Vec<MobileUrlProperty>,
    #[cfg(any(feature = "model-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "model"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#model: Vec<ModelProperty>,
    #[cfg(any(feature = "mpn-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "mpn"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#mpn: Vec<MpnProperty>,
    #[cfg(any(feature = "name-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#name: Vec<NameProperty>,
    #[cfg(any(
        feature = "negative-notes-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "negativeNotes"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#negative_notes: Vec<NegativeNotesProperty>,
    #[cfg(any(
        feature = "non-proprietary-name-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "nonProprietaryName"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#non_proprietary_name: Vec<NonProprietaryNameProperty>,
    #[cfg(any(feature = "nsn-property-schema", feature = "pending-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "nsn"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#nsn: Vec<NsnProperty>,
    #[cfg(any(feature = "offers-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "offers"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#offers: Vec<OffersProperty>,
    #[cfg(any(
        feature = "overdosage-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "overdosage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#overdosage: Vec<OverdosageProperty>,
    #[cfg(any(
        feature = "pattern-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "pattern"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#pattern: Vec<PatternProperty>,
    #[cfg(any(
        feature = "positive-notes-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "positiveNotes"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#positive_notes: Vec<PositiveNotesProperty>,
    #[cfg(any(
        feature = "potential-action-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "potentialAction"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#potential_action: Vec<PotentialActionProperty>,
    #[cfg(any(
        feature = "pregnancy-category-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "pregnancyCategory"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#pregnancy_category: Vec<PregnancyCategoryProperty>,
    #[cfg(any(
        feature = "pregnancy-warning-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "pregnancyWarning"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#pregnancy_warning: Vec<PregnancyWarningProperty>,
    #[cfg(any(
        feature = "prescribing-info-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "prescribingInfo"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#prescribing_info: Vec<PrescribingInfoProperty>,
    #[cfg(any(
        feature = "prescription-status-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "prescriptionStatus"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#prescription_status: Vec<PrescriptionStatusProperty>,
    #[cfg(any(
        feature = "product-id-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "productID"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#product_id: Vec<ProductIdProperty>,
    #[cfg(any(
        feature = "production-date-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "productionDate"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#production_date: Vec<ProductionDateProperty>,
    #[cfg(any(
        feature = "proprietary-name-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "proprietaryName"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#proprietary_name: Vec<ProprietaryNameProperty>,
    #[cfg(any(
        feature = "purchase-date-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "purchaseDate"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#purchase_date: Vec<PurchaseDateProperty>,
    #[cfg(any(
        feature = "recognizing-authority-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "recognizingAuthority"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#recognizing_authority: Vec<RecognizingAuthorityProperty>,
    #[cfg(any(
        feature = "related-drug-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "relatedDrug"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#related_drug: Vec<RelatedDrugProperty>,
    #[cfg(any(
        feature = "release-date-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "releaseDate"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#release_date: Vec<ReleaseDateProperty>,
    #[cfg(any(
        feature = "relevant-specialty-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "relevantSpecialty"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#relevant_specialty: Vec<RelevantSpecialtyProperty>,
    #[cfg(any(feature = "review-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "review"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#review: Vec<ReviewProperty>,
    #[cfg(any(
        feature = "reviews-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "reviews"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#reviews: Vec<ReviewsProperty>,
    #[cfg(any(feature = "rxcui-property-schema", feature = "pending-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "rxcui"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#rxcui: Vec<RxcuiProperty>,
    #[cfg(any(
        feature = "same-as-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sameAs"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#same_as: Vec<SameAsProperty>,
    #[cfg(any(feature = "size-property-schema", feature = "pending-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "size"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#size: Vec<SizeProperty>,
    #[cfg(any(feature = "sku-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "sku"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#sku: Vec<SkuProperty>,
    #[cfg(any(feature = "slogan-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "slogan"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#slogan: Vec<SloganProperty>,
    #[cfg(any(
        feature = "study-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "study"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#study: Vec<StudyProperty>,
    #[cfg(any(
        feature = "subject-of-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "subjectOf"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#subject_of: Vec<SubjectOfProperty>,
    #[cfg(any(feature = "url-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "url"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#url: Vec<UrlProperty>,
    #[cfg(any(
        feature = "warning-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "warning"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#warning: Vec<WarningProperty>,
    #[cfg(any(feature = "weight-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "weight"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#weight: Vec<WeightProperty>,
    #[cfg(any(feature = "width-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "width"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#width: Vec<WidthProperty>,
}