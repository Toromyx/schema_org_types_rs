use super::*;
/// <https://schema.org/Drug>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Drug {
	#[cfg(any(
		any(
			feature = "active-ingredient-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#active_ingredient: Vec<ActiveIngredientProperty>,
	#[cfg(any(
		any(
			feature = "additional-property-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#additional_property: Vec<AdditionalPropertyProperty>,
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
			feature = "administration-route-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#administration_route: Vec<AdministrationRouteProperty>,
	#[cfg(any(
		any(
			feature = "aggregate-rating-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#aggregate_rating: Vec<AggregateRatingProperty>,
	#[cfg(any(
		any(
			feature = "alcohol-warning-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#alcohol_warning: Vec<AlcoholWarningProperty>,
	#[cfg(any(
		any(
			feature = "alternate-name-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#alternate_name: Vec<AlternateNameProperty>,
	#[cfg(any(
		any(feature = "asin-property-schema", feature = "pending-schema-section"),
		doc
	))]
	pub r#asin: Vec<AsinProperty>,
	#[cfg(any(
		any(
			feature = "audience-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#audience: Vec<AudienceProperty>,
	#[cfg(any(
		any(
			feature = "available-strength-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#available_strength: Vec<AvailableStrengthProperty>,
	#[cfg(any(
		any(feature = "award-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#award: Vec<AwardProperty>,
	#[cfg(any(
		any(feature = "awards-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#awards: Vec<AwardsProperty>,
	#[cfg(any(
		any(feature = "brand-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#brand: Vec<BrandProperty>,
	#[cfg(any(
		any(
			feature = "breastfeeding-warning-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#breastfeeding_warning: Vec<BreastfeedingWarningProperty>,
	#[cfg(any(
		any(
			feature = "category-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#category: Vec<CategoryProperty>,
	#[cfg(any(
		any(
			feature = "clincal-pharmacology-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#clincal_pharmacology: Vec<ClincalPharmacologyProperty>,
	#[cfg(any(
		any(
			feature = "clinical-pharmacology-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#clinical_pharmacology: Vec<ClinicalPharmacologyProperty>,
	#[cfg(any(
		any(
			feature = "code-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#code: Vec<CodeProperty>,
	#[cfg(any(
		any(feature = "color-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#color: Vec<ColorProperty>,
	#[cfg(any(
		any(
			feature = "country-of-assembly-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#country_of_assembly: Vec<CountryOfAssemblyProperty>,
	#[cfg(any(
		any(
			feature = "country-of-last-processing-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#country_of_last_processing: Vec<CountryOfLastProcessingProperty>,
	#[cfg(any(
		any(
			feature = "country-of-origin-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#country_of_origin: Vec<CountryOfOriginProperty>,
	#[cfg(any(
		any(feature = "depth-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#depth: Vec<DepthProperty>,
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
			feature = "dosage-form-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#dosage_form: Vec<DosageFormProperty>,
	#[cfg(any(
		any(
			feature = "dose-schedule-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#dose_schedule: Vec<DoseScheduleProperty>,
	#[cfg(any(
		any(
			feature = "drug-class-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#drug_class: Vec<DrugClassProperty>,
	#[cfg(any(
		any(
			feature = "drug-unit-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#drug_unit: Vec<DrugUnitProperty>,
	#[cfg(any(
		any(
			feature = "food-warning-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#food_warning: Vec<FoodWarningProperty>,
	#[cfg(any(
		any(
			feature = "funding-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#funding: Vec<FundingProperty>,
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
			feature = "guideline-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#guideline: Vec<GuidelineProperty>,
	#[cfg(any(
		any(
			feature = "has-adult-consideration-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#has_adult_consideration: Vec<HasAdultConsiderationProperty>,
	#[cfg(any(
		any(
			feature = "has-energy-consumption-details-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#has_energy_consumption_details: Vec<HasEnergyConsumptionDetailsProperty>,
	#[cfg(any(
		any(
			feature = "has-measurement-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#has_measurement: Vec<HasMeasurementProperty>,
	#[cfg(any(
		any(
			feature = "has-merchant-return-policy-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#has_merchant_return_policy: Vec<HasMerchantReturnPolicyProperty>,
	#[cfg(any(
		any(
			feature = "has-product-return-policy-property-schema",
			feature = "attic-schema-section"
		),
		doc
	))]
	pub r#has_product_return_policy: Vec<HasProductReturnPolicyProperty>,
	#[cfg(any(
		any(feature = "height-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#height: Vec<HeightProperty>,
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
			feature = "in-product-group-with-id-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#in_product_group_with_id: Vec<InProductGroupWithIdProperty>,
	#[cfg(any(
		any(
			feature = "included-in-health-insurance-plan-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#included_in_health_insurance_plan: Vec<IncludedInHealthInsurancePlanProperty>,
	#[cfg(any(
		any(
			feature = "interacting-drug-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#interacting_drug: Vec<InteractingDrugProperty>,
	#[cfg(any(
		any(
			feature = "is-accessory-or-spare-part-for-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_accessory_or_spare_part_for: Vec<IsAccessoryOrSparePartForProperty>,
	#[cfg(any(
		any(
			feature = "is-available-generically-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#is_available_generically: Vec<IsAvailableGenericallyProperty>,
	#[cfg(any(
		any(
			feature = "is-consumable-for-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_consumable_for: Vec<IsConsumableForProperty>,
	#[cfg(any(
		any(
			feature = "is-family-friendly-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_family_friendly: Vec<IsFamilyFriendlyProperty>,
	#[cfg(any(
		any(
			feature = "is-proprietary-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#is_proprietary: Vec<IsProprietaryProperty>,
	#[cfg(any(
		any(
			feature = "is-related-to-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_related_to: Vec<IsRelatedToProperty>,
	#[cfg(any(
		any(
			feature = "is-similar-to-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_similar_to: Vec<IsSimilarToProperty>,
	#[cfg(any(
		any(
			feature = "is-variant-of-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_variant_of: Vec<IsVariantOfProperty>,
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
			feature = "keywords-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#keywords: Vec<KeywordsProperty>,
	#[cfg(any(
		any(
			feature = "label-details-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#label_details: Vec<LabelDetailsProperty>,
	#[cfg(any(
		any(
			feature = "legal-status-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#legal_status: Vec<LegalStatusProperty>,
	#[cfg(any(
		any(feature = "logo-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#logo: Vec<LogoProperty>,
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
			feature = "manufacturer-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#manufacturer: Vec<ManufacturerProperty>,
	#[cfg(any(
		any(
			feature = "material-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#material: Vec<MaterialProperty>,
	#[cfg(any(
		any(
			feature = "maximum-intake-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#maximum_intake: Vec<MaximumIntakeProperty>,
	#[cfg(any(
		any(
			feature = "mechanism-of-action-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#mechanism_of_action: Vec<MechanismOfActionProperty>,
	#[cfg(any(
		any(
			feature = "medicine-system-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#medicine_system: Vec<MedicineSystemProperty>,
	#[cfg(any(
		any(
			feature = "mobile-url-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#mobile_url: Vec<MobileUrlProperty>,
	#[cfg(any(
		any(feature = "model-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#model: Vec<ModelProperty>,
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
			feature = "negative-notes-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#negative_notes: Vec<NegativeNotesProperty>,
	#[cfg(any(
		any(
			feature = "non-proprietary-name-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#non_proprietary_name: Vec<NonProprietaryNameProperty>,
	#[cfg(any(
		any(feature = "nsn-property-schema", feature = "pending-schema-section"),
		doc
	))]
	pub r#nsn: Vec<NsnProperty>,
	#[cfg(any(
		any(feature = "offers-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#offers: Vec<OffersProperty>,
	#[cfg(any(
		any(
			feature = "overdosage-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#overdosage: Vec<OverdosageProperty>,
	#[cfg(any(
		any(
			feature = "pattern-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#pattern: Vec<PatternProperty>,
	#[cfg(any(
		any(
			feature = "positive-notes-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#positive_notes: Vec<PositiveNotesProperty>,
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
			feature = "pregnancy-category-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#pregnancy_category: Vec<PregnancyCategoryProperty>,
	#[cfg(any(
		any(
			feature = "pregnancy-warning-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#pregnancy_warning: Vec<PregnancyWarningProperty>,
	#[cfg(any(
		any(
			feature = "prescribing-info-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#prescribing_info: Vec<PrescribingInfoProperty>,
	#[cfg(any(
		any(
			feature = "prescription-status-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#prescription_status: Vec<PrescriptionStatusProperty>,
	#[cfg(any(
		any(
			feature = "product-id-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#product_id: Vec<ProductIdProperty>,
	#[cfg(any(
		any(
			feature = "production-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#production_date: Vec<ProductionDateProperty>,
	#[cfg(any(
		any(
			feature = "proprietary-name-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#proprietary_name: Vec<ProprietaryNameProperty>,
	#[cfg(any(
		any(
			feature = "purchase-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#purchase_date: Vec<PurchaseDateProperty>,
	#[cfg(any(
		any(
			feature = "recognizing-authority-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#recognizing_authority: Vec<RecognizingAuthorityProperty>,
	#[cfg(any(
		any(
			feature = "related-drug-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#related_drug: Vec<RelatedDrugProperty>,
	#[cfg(any(
		any(
			feature = "release-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#release_date: Vec<ReleaseDateProperty>,
	#[cfg(any(
		any(
			feature = "relevant-specialty-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#relevant_specialty: Vec<RelevantSpecialtyProperty>,
	#[cfg(any(
		any(feature = "review-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#review: Vec<ReviewProperty>,
	#[cfg(any(
		any(
			feature = "reviews-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#reviews: Vec<ReviewsProperty>,
	#[cfg(any(
		any(feature = "rxcui-property-schema", feature = "pending-schema-section"),
		doc
	))]
	pub r#rxcui: Vec<RxcuiProperty>,
	#[cfg(any(
		any(
			feature = "same-as-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#same_as: Vec<SameAsProperty>,
	#[cfg(any(
		any(feature = "size-property-schema", feature = "pending-schema-section"),
		doc
	))]
	pub r#size: Vec<SizeProperty>,
	#[cfg(any(
		any(feature = "sku-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#sku: Vec<SkuProperty>,
	#[cfg(any(
		any(feature = "slogan-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#slogan: Vec<SloganProperty>,
	#[cfg(any(
		any(
			feature = "study-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#study: Vec<StudyProperty>,
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
			feature = "warning-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#warning: Vec<WarningProperty>,
	#[cfg(any(
		any(feature = "weight-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#weight: Vec<WeightProperty>,
	#[cfg(any(
		any(feature = "width-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#width: Vec<WidthProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for Drug {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				if cfg!(any(
					any(
						feature = "active-ingredient-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#active_ingredient) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "additional-property-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#additional_property) as usize
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
						feature = "administration-route-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#administration_route) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "aggregate-rating-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#aggregate_rating) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "alcohol-warning-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#alcohol_warning) as usize
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
						feature = "audience-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#audience) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "available-strength-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#available_strength) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "award-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#award) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "awards-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#awards) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "brand-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#brand) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "breastfeeding-warning-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#breastfeeding_warning) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "category-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#category) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "clincal-pharmacology-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#clincal_pharmacology) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "clinical-pharmacology-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#clinical_pharmacology) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "code-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#code) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "color-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#color) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "country-of-assembly-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#country_of_assembly) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "country-of-last-processing-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#country_of_last_processing) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "country-of-origin-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#country_of_origin) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "depth-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#depth) as usize
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
						feature = "dosage-form-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#dosage_form) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "dose-schedule-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#dose_schedule) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "drug-class-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#drug_class) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "drug-unit-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#drug_unit) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "food-warning-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#food_warning) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "funding-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#funding) as usize
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
						feature = "guideline-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#guideline) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-adult-consideration-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_adult_consideration) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-energy-consumption-details-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_energy_consumption_details) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-measurement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_measurement) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-merchant-return-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_merchant_return_policy) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-product-return-policy-property-schema",
						feature = "attic-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_product_return_policy) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "height-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#height) as usize
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
						feature = "in-product-group-with-id-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#in_product_group_with_id) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "included-in-health-insurance-plan-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#included_in_health_insurance_plan) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "interacting-drug-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#interacting_drug) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-accessory-or-spare-part-for-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_accessory_or_spare_part_for) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-available-generically-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_available_generically) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-consumable-for-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_consumable_for) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-family-friendly-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_family_friendly) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-proprietary-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_proprietary) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-related-to-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_related_to) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-similar-to-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_similar_to) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-variant-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_variant_of) as usize
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
						feature = "keywords-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#keywords) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "label-details-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#label_details) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "legal-status-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#legal_status) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "logo-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#logo) as usize
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
						feature = "manufacturer-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#manufacturer) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "material-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#material) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "maximum-intake-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#maximum_intake) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "mechanism-of-action-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#mechanism_of_action) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "medicine-system-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#medicine_system) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "mobile-url-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#mobile_url) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "model-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#model) as usize
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
						feature = "negative-notes-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#negative_notes) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "non-proprietary-name-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#non_proprietary_name) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "nsn-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#nsn) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "offers-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#offers) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "overdosage-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#overdosage) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "pattern-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#pattern) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "positive-notes-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#positive_notes) as usize
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
						feature = "pregnancy-category-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#pregnancy_category) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "pregnancy-warning-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#pregnancy_warning) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "prescribing-info-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#prescribing_info) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "prescription-status-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#prescription_status) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "product-id-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#product_id) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "production-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#production_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "proprietary-name-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#proprietary_name) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "purchase-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#purchase_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "recognizing-authority-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#recognizing_authority) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "related-drug-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#related_drug) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "release-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#release_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "relevant-specialty-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#relevant_specialty) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "review-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#review) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "reviews-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#reviews) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "rxcui-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#rxcui) as usize
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
						feature = "size-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#size) as usize
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
						feature = "slogan-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#slogan) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "study-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#study) as usize
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
						feature = "warning-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#warning) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "weight-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#weight) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "width-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#width) as usize
				} else {
					0
				},
			]
			.iter()
			.sum();
			let mut serialize_struct = Serializer::serialize_struct(serializer, "Drug", len)?;
			#[cfg(any(
				any(
					feature = "active-ingredient-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#active_ingredient) {
				serialize_struct.serialize_field("activeIngredient", {
					struct SerializeWith<'a>(&'a Vec<ActiveIngredientProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#active_ingredient)
				})?;
			} else {
				serialize_struct.skip_field("activeIngredient")?;
			}
			#[cfg(any(
				any(
					feature = "additional-property-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "administration-route-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#administration_route) {
				serialize_struct.serialize_field("administrationRoute", {
					struct SerializeWith<'a>(&'a Vec<AdministrationRouteProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#administration_route)
				})?;
			} else {
				serialize_struct.skip_field("administrationRoute")?;
			}
			#[cfg(any(
				any(
					feature = "aggregate-rating-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "alcohol-warning-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#alcohol_warning) {
				serialize_struct.serialize_field("alcoholWarning", {
					struct SerializeWith<'a>(&'a Vec<AlcoholWarningProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#alcohol_warning)
				})?;
			} else {
				serialize_struct.skip_field("alcoholWarning")?;
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
					feature = "audience-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "available-strength-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#available_strength) {
				serialize_struct.serialize_field("availableStrength", {
					struct SerializeWith<'a>(&'a Vec<AvailableStrengthProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#available_strength)
				})?;
			} else {
				serialize_struct.skip_field("availableStrength")?;
			}
			#[cfg(any(
				any(feature = "award-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(feature = "awards-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(feature = "brand-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "breastfeeding-warning-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#breastfeeding_warning) {
				serialize_struct.serialize_field("breastfeedingWarning", {
					struct SerializeWith<'a>(&'a Vec<BreastfeedingWarningProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#breastfeeding_warning)
				})?;
			} else {
				serialize_struct.skip_field("breastfeedingWarning")?;
			}
			#[cfg(any(
				any(
					feature = "category-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "clincal-pharmacology-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#clincal_pharmacology) {
				serialize_struct.serialize_field("clincalPharmacology", {
					struct SerializeWith<'a>(&'a Vec<ClincalPharmacologyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#clincal_pharmacology)
				})?;
			} else {
				serialize_struct.skip_field("clincalPharmacology")?;
			}
			#[cfg(any(
				any(
					feature = "clinical-pharmacology-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#clinical_pharmacology) {
				serialize_struct.serialize_field("clinicalPharmacology", {
					struct SerializeWith<'a>(&'a Vec<ClinicalPharmacologyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#clinical_pharmacology)
				})?;
			} else {
				serialize_struct.skip_field("clinicalPharmacology")?;
			}
			#[cfg(any(
				any(
					feature = "code-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#code) {
				serialize_struct.serialize_field("code", {
					struct SerializeWith<'a>(&'a Vec<CodeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#code)
				})?;
			} else {
				serialize_struct.skip_field("code")?;
			}
			#[cfg(any(
				any(feature = "color-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "country-of-assembly-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "country-of-last-processing-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "country-of-origin-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(feature = "depth-property-schema", feature = "general-schema-section"),
				doc
			))]
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
					feature = "dosage-form-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#dosage_form) {
				serialize_struct.serialize_field("dosageForm", {
					struct SerializeWith<'a>(&'a Vec<DosageFormProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#dosage_form)
				})?;
			} else {
				serialize_struct.skip_field("dosageForm")?;
			}
			#[cfg(any(
				any(
					feature = "dose-schedule-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#dose_schedule) {
				serialize_struct.serialize_field("doseSchedule", {
					struct SerializeWith<'a>(&'a Vec<DoseScheduleProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#dose_schedule)
				})?;
			} else {
				serialize_struct.skip_field("doseSchedule")?;
			}
			#[cfg(any(
				any(
					feature = "drug-class-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#drug_class) {
				serialize_struct.serialize_field("drugClass", {
					struct SerializeWith<'a>(&'a Vec<DrugClassProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#drug_class)
				})?;
			} else {
				serialize_struct.skip_field("drugClass")?;
			}
			#[cfg(any(
				any(
					feature = "drug-unit-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#drug_unit) {
				serialize_struct.serialize_field("drugUnit", {
					struct SerializeWith<'a>(&'a Vec<DrugUnitProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#drug_unit)
				})?;
			} else {
				serialize_struct.skip_field("drugUnit")?;
			}
			#[cfg(any(
				any(
					feature = "food-warning-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#food_warning) {
				serialize_struct.serialize_field("foodWarning", {
					struct SerializeWith<'a>(&'a Vec<FoodWarningProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#food_warning)
				})?;
			} else {
				serialize_struct.skip_field("foodWarning")?;
			}
			#[cfg(any(
				any(
					feature = "funding-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
					feature = "guideline-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#guideline) {
				serialize_struct.serialize_field("guideline", {
					struct SerializeWith<'a>(&'a Vec<GuidelineProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#guideline)
				})?;
			} else {
				serialize_struct.skip_field("guideline")?;
			}
			#[cfg(any(
				any(
					feature = "has-adult-consideration-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "has-energy-consumption-details-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "has-measurement-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "has-merchant-return-policy-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "has-product-return-policy-property-schema",
					feature = "attic-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(feature = "height-property-schema", feature = "general-schema-section"),
				doc
			))]
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
					feature = "in-product-group-with-id-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "included-in-health-insurance-plan-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#included_in_health_insurance_plan) {
				serialize_struct.serialize_field("includedInHealthInsurancePlan", {
					struct SerializeWith<'a>(&'a Vec<IncludedInHealthInsurancePlanProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#included_in_health_insurance_plan)
				})?;
			} else {
				serialize_struct.skip_field("includedInHealthInsurancePlan")?;
			}
			#[cfg(any(
				any(
					feature = "interacting-drug-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#interacting_drug) {
				serialize_struct.serialize_field("interactingDrug", {
					struct SerializeWith<'a>(&'a Vec<InteractingDrugProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#interacting_drug)
				})?;
			} else {
				serialize_struct.skip_field("interactingDrug")?;
			}
			#[cfg(any(
				any(
					feature = "is-accessory-or-spare-part-for-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "is-available-generically-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#is_available_generically) {
				serialize_struct.serialize_field("isAvailableGenerically", {
					struct SerializeWith<'a>(&'a Vec<IsAvailableGenericallyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_available_generically)
				})?;
			} else {
				serialize_struct.skip_field("isAvailableGenerically")?;
			}
			#[cfg(any(
				any(
					feature = "is-consumable-for-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "is-family-friendly-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "is-proprietary-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#is_proprietary) {
				serialize_struct.serialize_field("isProprietary", {
					struct SerializeWith<'a>(&'a Vec<IsProprietaryProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_proprietary)
				})?;
			} else {
				serialize_struct.skip_field("isProprietary")?;
			}
			#[cfg(any(
				any(
					feature = "is-related-to-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "is-similar-to-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "is-variant-of-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "keywords-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "label-details-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#label_details) {
				serialize_struct.serialize_field("labelDetails", {
					struct SerializeWith<'a>(&'a Vec<LabelDetailsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#label_details)
				})?;
			} else {
				serialize_struct.skip_field("labelDetails")?;
			}
			#[cfg(any(
				any(
					feature = "legal-status-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#legal_status) {
				serialize_struct.serialize_field("legalStatus", {
					struct SerializeWith<'a>(&'a Vec<LegalStatusProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#legal_status)
				})?;
			} else {
				serialize_struct.skip_field("legalStatus")?;
			}
			#[cfg(any(
				any(feature = "logo-property-schema", feature = "general-schema-section"),
				doc
			))]
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
					feature = "manufacturer-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "material-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "maximum-intake-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#maximum_intake) {
				serialize_struct.serialize_field("maximumIntake", {
					struct SerializeWith<'a>(&'a Vec<MaximumIntakeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#maximum_intake)
				})?;
			} else {
				serialize_struct.skip_field("maximumIntake")?;
			}
			#[cfg(any(
				any(
					feature = "mechanism-of-action-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#mechanism_of_action) {
				serialize_struct.serialize_field("mechanismOfAction", {
					struct SerializeWith<'a>(&'a Vec<MechanismOfActionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#mechanism_of_action)
				})?;
			} else {
				serialize_struct.skip_field("mechanismOfAction")?;
			}
			#[cfg(any(
				any(
					feature = "medicine-system-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#medicine_system) {
				serialize_struct.serialize_field("medicineSystem", {
					struct SerializeWith<'a>(&'a Vec<MedicineSystemProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#medicine_system)
				})?;
			} else {
				serialize_struct.skip_field("medicineSystem")?;
			}
			#[cfg(any(
				any(
					feature = "mobile-url-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(feature = "model-property-schema", feature = "general-schema-section"),
				doc
			))]
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
					feature = "negative-notes-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "non-proprietary-name-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#non_proprietary_name) {
				serialize_struct.serialize_field("nonProprietaryName", {
					struct SerializeWith<'a>(&'a Vec<NonProprietaryNameProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#non_proprietary_name)
				})?;
			} else {
				serialize_struct.skip_field("nonProprietaryName")?;
			}
			#[cfg(any(
				any(feature = "nsn-property-schema", feature = "pending-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(feature = "offers-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "overdosage-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#overdosage) {
				serialize_struct.serialize_field("overdosage", {
					struct SerializeWith<'a>(&'a Vec<OverdosageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#overdosage)
				})?;
			} else {
				serialize_struct.skip_field("overdosage")?;
			}
			#[cfg(any(
				any(
					feature = "pattern-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "positive-notes-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
					feature = "pregnancy-category-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#pregnancy_category) {
				serialize_struct.serialize_field("pregnancyCategory", {
					struct SerializeWith<'a>(&'a Vec<PregnancyCategoryProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#pregnancy_category)
				})?;
			} else {
				serialize_struct.skip_field("pregnancyCategory")?;
			}
			#[cfg(any(
				any(
					feature = "pregnancy-warning-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#pregnancy_warning) {
				serialize_struct.serialize_field("pregnancyWarning", {
					struct SerializeWith<'a>(&'a Vec<PregnancyWarningProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#pregnancy_warning)
				})?;
			} else {
				serialize_struct.skip_field("pregnancyWarning")?;
			}
			#[cfg(any(
				any(
					feature = "prescribing-info-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#prescribing_info) {
				serialize_struct.serialize_field("prescribingInfo", {
					struct SerializeWith<'a>(&'a Vec<PrescribingInfoProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#prescribing_info)
				})?;
			} else {
				serialize_struct.skip_field("prescribingInfo")?;
			}
			#[cfg(any(
				any(
					feature = "prescription-status-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#prescription_status) {
				serialize_struct.serialize_field("prescriptionStatus", {
					struct SerializeWith<'a>(&'a Vec<PrescriptionStatusProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#prescription_status)
				})?;
			} else {
				serialize_struct.skip_field("prescriptionStatus")?;
			}
			#[cfg(any(
				any(
					feature = "product-id-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "production-date-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "proprietary-name-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#proprietary_name) {
				serialize_struct.serialize_field("proprietaryName", {
					struct SerializeWith<'a>(&'a Vec<ProprietaryNameProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#proprietary_name)
				})?;
			} else {
				serialize_struct.skip_field("proprietaryName")?;
			}
			#[cfg(any(
				any(
					feature = "purchase-date-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "recognizing-authority-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#recognizing_authority) {
				serialize_struct.serialize_field("recognizingAuthority", {
					struct SerializeWith<'a>(&'a Vec<RecognizingAuthorityProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#recognizing_authority)
				})?;
			} else {
				serialize_struct.skip_field("recognizingAuthority")?;
			}
			#[cfg(any(
				any(
					feature = "related-drug-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#related_drug) {
				serialize_struct.serialize_field("relatedDrug", {
					struct SerializeWith<'a>(&'a Vec<RelatedDrugProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#related_drug)
				})?;
			} else {
				serialize_struct.skip_field("relatedDrug")?;
			}
			#[cfg(any(
				any(
					feature = "release-date-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "relevant-specialty-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#relevant_specialty) {
				serialize_struct.serialize_field("relevantSpecialty", {
					struct SerializeWith<'a>(&'a Vec<RelevantSpecialtyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#relevant_specialty)
				})?;
			} else {
				serialize_struct.skip_field("relevantSpecialty")?;
			}
			#[cfg(any(
				any(feature = "review-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "reviews-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(feature = "rxcui-property-schema", feature = "pending-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#rxcui) {
				serialize_struct.serialize_field("rxcui", {
					struct SerializeWith<'a>(&'a Vec<RxcuiProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#rxcui)
				})?;
			} else {
				serialize_struct.skip_field("rxcui")?;
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
				any(feature = "size-property-schema", feature = "pending-schema-section"),
				doc
			))]
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
				any(feature = "slogan-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "study-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#study) {
				serialize_struct.serialize_field("study", {
					struct SerializeWith<'a>(&'a Vec<StudyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#study)
				})?;
			} else {
				serialize_struct.skip_field("study")?;
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
					feature = "warning-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#warning) {
				serialize_struct.serialize_field("warning", {
					struct SerializeWith<'a>(&'a Vec<WarningProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#warning)
				})?;
			} else {
				serialize_struct.skip_field("warning")?;
			}
			#[cfg(any(
				any(feature = "weight-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(feature = "width-property-schema", feature = "general-schema-section"),
				doc
			))]
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
	impl<'de> Deserialize<'de> for Drug {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				#[cfg(any(
					any(
						feature = "active-ingredient-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				ActiveIngredient,
				#[cfg(any(
					any(
						feature = "additional-property-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AdditionalProperty,
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
						feature = "administration-route-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				AdministrationRoute,
				#[cfg(any(
					any(
						feature = "aggregate-rating-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AggregateRating,
				#[cfg(any(
					any(
						feature = "alcohol-warning-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				AlcoholWarning,
				#[cfg(any(
					any(
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AlternateName,
				#[cfg(any(
					any(feature = "asin-property-schema", feature = "pending-schema-section"),
					doc
				))]
				Asin,
				#[cfg(any(
					any(
						feature = "audience-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Audience,
				#[cfg(any(
					any(
						feature = "available-strength-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				AvailableStrength,
				#[cfg(any(
					any(feature = "award-property-schema", feature = "general-schema-section"),
					doc
				))]
				Award,
				#[cfg(any(
					any(feature = "awards-property-schema", feature = "general-schema-section"),
					doc
				))]
				Awards,
				#[cfg(any(
					any(feature = "brand-property-schema", feature = "general-schema-section"),
					doc
				))]
				Brand,
				#[cfg(any(
					any(
						feature = "breastfeeding-warning-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				BreastfeedingWarning,
				#[cfg(any(
					any(
						feature = "category-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Category,
				#[cfg(any(
					any(
						feature = "clincal-pharmacology-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				ClincalPharmacology,
				#[cfg(any(
					any(
						feature = "clinical-pharmacology-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				ClinicalPharmacology,
				#[cfg(any(
					any(
						feature = "code-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				Code,
				#[cfg(any(
					any(feature = "color-property-schema", feature = "general-schema-section"),
					doc
				))]
				Color,
				#[cfg(any(
					any(
						feature = "country-of-assembly-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CountryOfAssembly,
				#[cfg(any(
					any(
						feature = "country-of-last-processing-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CountryOfLastProcessing,
				#[cfg(any(
					any(
						feature = "country-of-origin-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				CountryOfOrigin,
				#[cfg(any(
					any(feature = "depth-property-schema", feature = "general-schema-section"),
					doc
				))]
				Depth,
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
						feature = "dosage-form-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				DosageForm,
				#[cfg(any(
					any(
						feature = "dose-schedule-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				DoseSchedule,
				#[cfg(any(
					any(
						feature = "drug-class-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				DrugClass,
				#[cfg(any(
					any(
						feature = "drug-unit-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				DrugUnit,
				#[cfg(any(
					any(
						feature = "food-warning-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				FoodWarning,
				#[cfg(any(
					any(
						feature = "funding-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Funding,
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
						feature = "guideline-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				Guideline,
				#[cfg(any(
					any(
						feature = "has-adult-consideration-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HasAdultConsideration,
				#[cfg(any(
					any(
						feature = "has-energy-consumption-details-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HasEnergyConsumptionDetails,
				#[cfg(any(
					any(
						feature = "has-measurement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HasMeasurement,
				#[cfg(any(
					any(
						feature = "has-merchant-return-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HasMerchantReturnPolicy,
				#[cfg(any(
					any(
						feature = "has-product-return-policy-property-schema",
						feature = "attic-schema-section"
					),
					doc
				))]
				HasProductReturnPolicy,
				#[cfg(any(
					any(feature = "height-property-schema", feature = "general-schema-section"),
					doc
				))]
				Height,
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
						feature = "in-product-group-with-id-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				InProductGroupWithId,
				#[cfg(any(
					any(
						feature = "included-in-health-insurance-plan-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				IncludedInHealthInsurancePlan,
				#[cfg(any(
					any(
						feature = "interacting-drug-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				InteractingDrug,
				#[cfg(any(
					any(
						feature = "is-accessory-or-spare-part-for-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsAccessoryOrSparePartFor,
				#[cfg(any(
					any(
						feature = "is-available-generically-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				IsAvailableGenerically,
				#[cfg(any(
					any(
						feature = "is-consumable-for-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsConsumableFor,
				#[cfg(any(
					any(
						feature = "is-family-friendly-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsFamilyFriendly,
				#[cfg(any(
					any(
						feature = "is-proprietary-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				IsProprietary,
				#[cfg(any(
					any(
						feature = "is-related-to-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsRelatedTo,
				#[cfg(any(
					any(
						feature = "is-similar-to-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsSimilarTo,
				#[cfg(any(
					any(
						feature = "is-variant-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsVariantOf,
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
						feature = "keywords-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Keywords,
				#[cfg(any(
					any(
						feature = "label-details-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				LabelDetails,
				#[cfg(any(
					any(
						feature = "legal-status-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				LegalStatus,
				#[cfg(any(
					any(feature = "logo-property-schema", feature = "general-schema-section"),
					doc
				))]
				Logo,
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
						feature = "manufacturer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Manufacturer,
				#[cfg(any(
					any(
						feature = "material-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Material,
				#[cfg(any(
					any(
						feature = "maximum-intake-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				MaximumIntake,
				#[cfg(any(
					any(
						feature = "mechanism-of-action-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				MechanismOfAction,
				#[cfg(any(
					any(
						feature = "medicine-system-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				MedicineSystem,
				#[cfg(any(
					any(
						feature = "mobile-url-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				MobileUrl,
				#[cfg(any(
					any(feature = "model-property-schema", feature = "general-schema-section"),
					doc
				))]
				Model,
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
						feature = "negative-notes-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				NegativeNotes,
				#[cfg(any(
					any(
						feature = "non-proprietary-name-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				NonProprietaryName,
				#[cfg(any(
					any(feature = "nsn-property-schema", feature = "pending-schema-section"),
					doc
				))]
				Nsn,
				#[cfg(any(
					any(feature = "offers-property-schema", feature = "general-schema-section"),
					doc
				))]
				Offers,
				#[cfg(any(
					any(
						feature = "overdosage-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				Overdosage,
				#[cfg(any(
					any(
						feature = "pattern-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Pattern,
				#[cfg(any(
					any(
						feature = "positive-notes-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				PositiveNotes,
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
						feature = "pregnancy-category-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				PregnancyCategory,
				#[cfg(any(
					any(
						feature = "pregnancy-warning-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				PregnancyWarning,
				#[cfg(any(
					any(
						feature = "prescribing-info-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				PrescribingInfo,
				#[cfg(any(
					any(
						feature = "prescription-status-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				PrescriptionStatus,
				#[cfg(any(
					any(
						feature = "product-id-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ProductId,
				#[cfg(any(
					any(
						feature = "production-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ProductionDate,
				#[cfg(any(
					any(
						feature = "proprietary-name-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				ProprietaryName,
				#[cfg(any(
					any(
						feature = "purchase-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PurchaseDate,
				#[cfg(any(
					any(
						feature = "recognizing-authority-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				RecognizingAuthority,
				#[cfg(any(
					any(
						feature = "related-drug-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				RelatedDrug,
				#[cfg(any(
					any(
						feature = "release-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ReleaseDate,
				#[cfg(any(
					any(
						feature = "relevant-specialty-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				RelevantSpecialty,
				#[cfg(any(
					any(feature = "review-property-schema", feature = "general-schema-section"),
					doc
				))]
				Review,
				#[cfg(any(
					any(
						feature = "reviews-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Reviews,
				#[cfg(any(
					any(feature = "rxcui-property-schema", feature = "pending-schema-section"),
					doc
				))]
				Rxcui,
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SameAs,
				#[cfg(any(
					any(feature = "size-property-schema", feature = "pending-schema-section"),
					doc
				))]
				Size,
				#[cfg(any(
					any(feature = "sku-property-schema", feature = "general-schema-section"),
					doc
				))]
				Sku,
				#[cfg(any(
					any(feature = "slogan-property-schema", feature = "general-schema-section"),
					doc
				))]
				Slogan,
				#[cfg(any(
					any(
						feature = "study-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				Study,
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
						feature = "warning-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				Warning,
				#[cfg(any(
					any(feature = "weight-property-schema", feature = "general-schema-section"),
					doc
				))]
				Weight,
				#[cfg(any(
					any(feature = "width-property-schema", feature = "general-schema-section"),
					doc
				))]
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
						#[cfg(any(
							any(
								feature = "active-ingredient-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"activeIngredient" => Ok(Field::ActiveIngredient),
						#[cfg(any(
							any(
								feature = "additional-property-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"additionalProperty" => Ok(Field::AdditionalProperty),
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
								feature = "administration-route-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"administrationRoute" => Ok(Field::AdministrationRoute),
						#[cfg(any(
							any(
								feature = "aggregate-rating-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"aggregateRating" => Ok(Field::AggregateRating),
						#[cfg(any(
							any(
								feature = "alcohol-warning-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"alcoholWarning" => Ok(Field::AlcoholWarning),
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
								feature = "asin-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"asin" => Ok(Field::Asin),
						#[cfg(any(
							any(
								feature = "audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"audience" => Ok(Field::Audience),
						#[cfg(any(
							any(
								feature = "available-strength-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"availableStrength" => Ok(Field::AvailableStrength),
						#[cfg(any(
							any(
								feature = "award-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"award" => Ok(Field::Award),
						#[cfg(any(
							any(
								feature = "awards-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"awards" => Ok(Field::Awards),
						#[cfg(any(
							any(
								feature = "brand-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"brand" => Ok(Field::Brand),
						#[cfg(any(
							any(
								feature = "breastfeeding-warning-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"breastfeedingWarning" => Ok(Field::BreastfeedingWarning),
						#[cfg(any(
							any(
								feature = "category-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"category" => Ok(Field::Category),
						#[cfg(any(
							any(
								feature = "clincal-pharmacology-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"clincalPharmacology" => Ok(Field::ClincalPharmacology),
						#[cfg(any(
							any(
								feature = "clinical-pharmacology-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"clinicalPharmacology" => Ok(Field::ClinicalPharmacology),
						#[cfg(any(
							any(
								feature = "code-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"code" => Ok(Field::Code),
						#[cfg(any(
							any(
								feature = "color-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"color" => Ok(Field::Color),
						#[cfg(any(
							any(
								feature = "country-of-assembly-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"countryOfAssembly" => Ok(Field::CountryOfAssembly),
						#[cfg(any(
							any(
								feature = "country-of-last-processing-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"countryOfLastProcessing" => Ok(Field::CountryOfLastProcessing),
						#[cfg(any(
							any(
								feature = "country-of-origin-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"countryOfOrigin" => Ok(Field::CountryOfOrigin),
						#[cfg(any(
							any(
								feature = "depth-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"depth" => Ok(Field::Depth),
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
								feature = "dosage-form-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"dosageForm" => Ok(Field::DosageForm),
						#[cfg(any(
							any(
								feature = "dose-schedule-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"doseSchedule" => Ok(Field::DoseSchedule),
						#[cfg(any(
							any(
								feature = "drug-class-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"drugClass" => Ok(Field::DrugClass),
						#[cfg(any(
							any(
								feature = "drug-unit-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"drugUnit" => Ok(Field::DrugUnit),
						#[cfg(any(
							any(
								feature = "food-warning-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"foodWarning" => Ok(Field::FoodWarning),
						#[cfg(any(
							any(
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"funding" => Ok(Field::Funding),
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
								feature = "guideline-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"guideline" => Ok(Field::Guideline),
						#[cfg(any(
							any(
								feature = "has-adult-consideration-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"hasAdultConsideration" => Ok(Field::HasAdultConsideration),
						#[cfg(any(
							any(
								feature = "has-energy-consumption-details-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"hasEnergyConsumptionDetails" => Ok(Field::HasEnergyConsumptionDetails),
						#[cfg(any(
							any(
								feature = "has-measurement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"hasMeasurement" => Ok(Field::HasMeasurement),
						#[cfg(any(
							any(
								feature = "has-merchant-return-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"hasMerchantReturnPolicy" => Ok(Field::HasMerchantReturnPolicy),
						#[cfg(any(
							any(
								feature = "has-product-return-policy-property-schema",
								feature = "attic-schema-section"
							),
							doc
						))]
						"hasProductReturnPolicy" => Ok(Field::HasProductReturnPolicy),
						#[cfg(any(
							any(
								feature = "height-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"height" => Ok(Field::Height),
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
								feature = "in-product-group-with-id-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"inProductGroupWithID" => Ok(Field::InProductGroupWithId),
						#[cfg(any(
							any(
								feature = "included-in-health-insurance-plan-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"includedInHealthInsurancePlan" => Ok(Field::IncludedInHealthInsurancePlan),
						#[cfg(any(
							any(
								feature = "interacting-drug-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"interactingDrug" => Ok(Field::InteractingDrug),
						#[cfg(any(
							any(
								feature = "is-accessory-or-spare-part-for-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isAccessoryOrSparePartFor" => Ok(Field::IsAccessoryOrSparePartFor),
						#[cfg(any(
							any(
								feature = "is-available-generically-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"isAvailableGenerically" => Ok(Field::IsAvailableGenerically),
						#[cfg(any(
							any(
								feature = "is-consumable-for-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isConsumableFor" => Ok(Field::IsConsumableFor),
						#[cfg(any(
							any(
								feature = "is-family-friendly-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isFamilyFriendly" => Ok(Field::IsFamilyFriendly),
						#[cfg(any(
							any(
								feature = "is-proprietary-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"isProprietary" => Ok(Field::IsProprietary),
						#[cfg(any(
							any(
								feature = "is-related-to-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isRelatedTo" => Ok(Field::IsRelatedTo),
						#[cfg(any(
							any(
								feature = "is-similar-to-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isSimilarTo" => Ok(Field::IsSimilarTo),
						#[cfg(any(
							any(
								feature = "is-variant-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isVariantOf" => Ok(Field::IsVariantOf),
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
								feature = "keywords-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"keywords" => Ok(Field::Keywords),
						#[cfg(any(
							any(
								feature = "label-details-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"labelDetails" => Ok(Field::LabelDetails),
						#[cfg(any(
							any(
								feature = "legal-status-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"legalStatus" => Ok(Field::LegalStatus),
						#[cfg(any(
							any(
								feature = "logo-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"logo" => Ok(Field::Logo),
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
								feature = "manufacturer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"manufacturer" => Ok(Field::Manufacturer),
						#[cfg(any(
							any(
								feature = "material-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"material" => Ok(Field::Material),
						#[cfg(any(
							any(
								feature = "maximum-intake-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"maximumIntake" => Ok(Field::MaximumIntake),
						#[cfg(any(
							any(
								feature = "mechanism-of-action-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"mechanismOfAction" => Ok(Field::MechanismOfAction),
						#[cfg(any(
							any(
								feature = "medicine-system-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"medicineSystem" => Ok(Field::MedicineSystem),
						#[cfg(any(
							any(
								feature = "mobile-url-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"mobileUrl" => Ok(Field::MobileUrl),
						#[cfg(any(
							any(
								feature = "model-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"model" => Ok(Field::Model),
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
								feature = "negative-notes-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"negativeNotes" => Ok(Field::NegativeNotes),
						#[cfg(any(
							any(
								feature = "non-proprietary-name-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"nonProprietaryName" => Ok(Field::NonProprietaryName),
						#[cfg(any(
							any(
								feature = "nsn-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"nsn" => Ok(Field::Nsn),
						#[cfg(any(
							any(
								feature = "offers-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"offers" => Ok(Field::Offers),
						#[cfg(any(
							any(
								feature = "overdosage-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"overdosage" => Ok(Field::Overdosage),
						#[cfg(any(
							any(
								feature = "pattern-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"pattern" => Ok(Field::Pattern),
						#[cfg(any(
							any(
								feature = "positive-notes-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"positiveNotes" => Ok(Field::PositiveNotes),
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
								feature = "pregnancy-category-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"pregnancyCategory" => Ok(Field::PregnancyCategory),
						#[cfg(any(
							any(
								feature = "pregnancy-warning-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"pregnancyWarning" => Ok(Field::PregnancyWarning),
						#[cfg(any(
							any(
								feature = "prescribing-info-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"prescribingInfo" => Ok(Field::PrescribingInfo),
						#[cfg(any(
							any(
								feature = "prescription-status-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"prescriptionStatus" => Ok(Field::PrescriptionStatus),
						#[cfg(any(
							any(
								feature = "product-id-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"productID" => Ok(Field::ProductId),
						#[cfg(any(
							any(
								feature = "production-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"productionDate" => Ok(Field::ProductionDate),
						#[cfg(any(
							any(
								feature = "proprietary-name-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"proprietaryName" => Ok(Field::ProprietaryName),
						#[cfg(any(
							any(
								feature = "purchase-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"purchaseDate" => Ok(Field::PurchaseDate),
						#[cfg(any(
							any(
								feature = "recognizing-authority-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"recognizingAuthority" => Ok(Field::RecognizingAuthority),
						#[cfg(any(
							any(
								feature = "related-drug-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"relatedDrug" => Ok(Field::RelatedDrug),
						#[cfg(any(
							any(
								feature = "release-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"releaseDate" => Ok(Field::ReleaseDate),
						#[cfg(any(
							any(
								feature = "relevant-specialty-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"relevantSpecialty" => Ok(Field::RelevantSpecialty),
						#[cfg(any(
							any(
								feature = "review-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"review" => Ok(Field::Review),
						#[cfg(any(
							any(
								feature = "reviews-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"reviews" => Ok(Field::Reviews),
						#[cfg(any(
							any(
								feature = "rxcui-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"rxcui" => Ok(Field::Rxcui),
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
								feature = "size-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"size" => Ok(Field::Size),
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
								feature = "slogan-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"slogan" => Ok(Field::Slogan),
						#[cfg(any(
							any(
								feature = "study-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"study" => Ok(Field::Study),
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
								feature = "warning-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"warning" => Ok(Field::Warning),
						#[cfg(any(
							any(
								feature = "weight-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"weight" => Ok(Field::Weight),
						#[cfg(any(
							any(
								feature = "width-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"width" => Ok(Field::Width),
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
								feature = "active-ingredient-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"activeIngredient" => Ok(Field::ActiveIngredient),
						#[cfg(any(
							any(
								feature = "additional-property-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"additionalProperty" => Ok(Field::AdditionalProperty),
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
								feature = "administration-route-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"administrationRoute" => Ok(Field::AdministrationRoute),
						#[cfg(any(
							any(
								feature = "aggregate-rating-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"aggregateRating" => Ok(Field::AggregateRating),
						#[cfg(any(
							any(
								feature = "alcohol-warning-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"alcoholWarning" => Ok(Field::AlcoholWarning),
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
								feature = "asin-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"asin" => Ok(Field::Asin),
						#[cfg(any(
							any(
								feature = "audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"audience" => Ok(Field::Audience),
						#[cfg(any(
							any(
								feature = "available-strength-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"availableStrength" => Ok(Field::AvailableStrength),
						#[cfg(any(
							any(
								feature = "award-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"award" => Ok(Field::Award),
						#[cfg(any(
							any(
								feature = "awards-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"awards" => Ok(Field::Awards),
						#[cfg(any(
							any(
								feature = "brand-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"brand" => Ok(Field::Brand),
						#[cfg(any(
							any(
								feature = "breastfeeding-warning-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"breastfeedingWarning" => Ok(Field::BreastfeedingWarning),
						#[cfg(any(
							any(
								feature = "category-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"category" => Ok(Field::Category),
						#[cfg(any(
							any(
								feature = "clincal-pharmacology-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"clincalPharmacology" => Ok(Field::ClincalPharmacology),
						#[cfg(any(
							any(
								feature = "clinical-pharmacology-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"clinicalPharmacology" => Ok(Field::ClinicalPharmacology),
						#[cfg(any(
							any(
								feature = "code-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"code" => Ok(Field::Code),
						#[cfg(any(
							any(
								feature = "color-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"color" => Ok(Field::Color),
						#[cfg(any(
							any(
								feature = "country-of-assembly-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"countryOfAssembly" => Ok(Field::CountryOfAssembly),
						#[cfg(any(
							any(
								feature = "country-of-last-processing-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"countryOfLastProcessing" => Ok(Field::CountryOfLastProcessing),
						#[cfg(any(
							any(
								feature = "country-of-origin-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"countryOfOrigin" => Ok(Field::CountryOfOrigin),
						#[cfg(any(
							any(
								feature = "depth-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"depth" => Ok(Field::Depth),
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
								feature = "dosage-form-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"dosageForm" => Ok(Field::DosageForm),
						#[cfg(any(
							any(
								feature = "dose-schedule-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"doseSchedule" => Ok(Field::DoseSchedule),
						#[cfg(any(
							any(
								feature = "drug-class-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"drugClass" => Ok(Field::DrugClass),
						#[cfg(any(
							any(
								feature = "drug-unit-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"drugUnit" => Ok(Field::DrugUnit),
						#[cfg(any(
							any(
								feature = "food-warning-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"foodWarning" => Ok(Field::FoodWarning),
						#[cfg(any(
							any(
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"funding" => Ok(Field::Funding),
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
								feature = "guideline-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"guideline" => Ok(Field::Guideline),
						#[cfg(any(
							any(
								feature = "has-adult-consideration-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"hasAdultConsideration" => Ok(Field::HasAdultConsideration),
						#[cfg(any(
							any(
								feature = "has-energy-consumption-details-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"hasEnergyConsumptionDetails" => Ok(Field::HasEnergyConsumptionDetails),
						#[cfg(any(
							any(
								feature = "has-measurement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"hasMeasurement" => Ok(Field::HasMeasurement),
						#[cfg(any(
							any(
								feature = "has-merchant-return-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"hasMerchantReturnPolicy" => Ok(Field::HasMerchantReturnPolicy),
						#[cfg(any(
							any(
								feature = "has-product-return-policy-property-schema",
								feature = "attic-schema-section"
							),
							doc
						))]
						b"hasProductReturnPolicy" => Ok(Field::HasProductReturnPolicy),
						#[cfg(any(
							any(
								feature = "height-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"height" => Ok(Field::Height),
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
								feature = "in-product-group-with-id-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"inProductGroupWithID" => Ok(Field::InProductGroupWithId),
						#[cfg(any(
							any(
								feature = "included-in-health-insurance-plan-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"includedInHealthInsurancePlan" => Ok(Field::IncludedInHealthInsurancePlan),
						#[cfg(any(
							any(
								feature = "interacting-drug-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"interactingDrug" => Ok(Field::InteractingDrug),
						#[cfg(any(
							any(
								feature = "is-accessory-or-spare-part-for-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isAccessoryOrSparePartFor" => Ok(Field::IsAccessoryOrSparePartFor),
						#[cfg(any(
							any(
								feature = "is-available-generically-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"isAvailableGenerically" => Ok(Field::IsAvailableGenerically),
						#[cfg(any(
							any(
								feature = "is-consumable-for-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isConsumableFor" => Ok(Field::IsConsumableFor),
						#[cfg(any(
							any(
								feature = "is-family-friendly-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isFamilyFriendly" => Ok(Field::IsFamilyFriendly),
						#[cfg(any(
							any(
								feature = "is-proprietary-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"isProprietary" => Ok(Field::IsProprietary),
						#[cfg(any(
							any(
								feature = "is-related-to-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isRelatedTo" => Ok(Field::IsRelatedTo),
						#[cfg(any(
							any(
								feature = "is-similar-to-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isSimilarTo" => Ok(Field::IsSimilarTo),
						#[cfg(any(
							any(
								feature = "is-variant-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isVariantOf" => Ok(Field::IsVariantOf),
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
								feature = "keywords-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"keywords" => Ok(Field::Keywords),
						#[cfg(any(
							any(
								feature = "label-details-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"labelDetails" => Ok(Field::LabelDetails),
						#[cfg(any(
							any(
								feature = "legal-status-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"legalStatus" => Ok(Field::LegalStatus),
						#[cfg(any(
							any(
								feature = "logo-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"logo" => Ok(Field::Logo),
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
								feature = "manufacturer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"manufacturer" => Ok(Field::Manufacturer),
						#[cfg(any(
							any(
								feature = "material-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"material" => Ok(Field::Material),
						#[cfg(any(
							any(
								feature = "maximum-intake-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"maximumIntake" => Ok(Field::MaximumIntake),
						#[cfg(any(
							any(
								feature = "mechanism-of-action-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"mechanismOfAction" => Ok(Field::MechanismOfAction),
						#[cfg(any(
							any(
								feature = "medicine-system-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"medicineSystem" => Ok(Field::MedicineSystem),
						#[cfg(any(
							any(
								feature = "mobile-url-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"mobileUrl" => Ok(Field::MobileUrl),
						#[cfg(any(
							any(
								feature = "model-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"model" => Ok(Field::Model),
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
								feature = "negative-notes-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"negativeNotes" => Ok(Field::NegativeNotes),
						#[cfg(any(
							any(
								feature = "non-proprietary-name-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"nonProprietaryName" => Ok(Field::NonProprietaryName),
						#[cfg(any(
							any(
								feature = "nsn-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"nsn" => Ok(Field::Nsn),
						#[cfg(any(
							any(
								feature = "offers-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"offers" => Ok(Field::Offers),
						#[cfg(any(
							any(
								feature = "overdosage-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"overdosage" => Ok(Field::Overdosage),
						#[cfg(any(
							any(
								feature = "pattern-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"pattern" => Ok(Field::Pattern),
						#[cfg(any(
							any(
								feature = "positive-notes-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"positiveNotes" => Ok(Field::PositiveNotes),
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
								feature = "pregnancy-category-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"pregnancyCategory" => Ok(Field::PregnancyCategory),
						#[cfg(any(
							any(
								feature = "pregnancy-warning-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"pregnancyWarning" => Ok(Field::PregnancyWarning),
						#[cfg(any(
							any(
								feature = "prescribing-info-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"prescribingInfo" => Ok(Field::PrescribingInfo),
						#[cfg(any(
							any(
								feature = "prescription-status-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"prescriptionStatus" => Ok(Field::PrescriptionStatus),
						#[cfg(any(
							any(
								feature = "product-id-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"productID" => Ok(Field::ProductId),
						#[cfg(any(
							any(
								feature = "production-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"productionDate" => Ok(Field::ProductionDate),
						#[cfg(any(
							any(
								feature = "proprietary-name-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"proprietaryName" => Ok(Field::ProprietaryName),
						#[cfg(any(
							any(
								feature = "purchase-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"purchaseDate" => Ok(Field::PurchaseDate),
						#[cfg(any(
							any(
								feature = "recognizing-authority-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"recognizingAuthority" => Ok(Field::RecognizingAuthority),
						#[cfg(any(
							any(
								feature = "related-drug-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"relatedDrug" => Ok(Field::RelatedDrug),
						#[cfg(any(
							any(
								feature = "release-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"releaseDate" => Ok(Field::ReleaseDate),
						#[cfg(any(
							any(
								feature = "relevant-specialty-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"relevantSpecialty" => Ok(Field::RelevantSpecialty),
						#[cfg(any(
							any(
								feature = "review-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"review" => Ok(Field::Review),
						#[cfg(any(
							any(
								feature = "reviews-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"reviews" => Ok(Field::Reviews),
						#[cfg(any(
							any(
								feature = "rxcui-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"rxcui" => Ok(Field::Rxcui),
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
								feature = "size-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"size" => Ok(Field::Size),
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
								feature = "slogan-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"slogan" => Ok(Field::Slogan),
						#[cfg(any(
							any(
								feature = "study-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"study" => Ok(Field::Study),
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
								feature = "warning-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"warning" => Ok(Field::Warning),
						#[cfg(any(
							any(
								feature = "weight-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"weight" => Ok(Field::Weight),
						#[cfg(any(
							any(
								feature = "width-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
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
				type Value = Drug;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Drug")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					#[cfg(any(
						any(
							feature = "active-ingredient-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#active_ingredient_property = None;
					#[cfg(any(
						any(
							feature = "additional-property-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#additional_property_property = None;
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
							feature = "administration-route-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#administration_route_property = None;
					#[cfg(any(
						any(
							feature = "aggregate-rating-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#aggregate_rating_property = None;
					#[cfg(any(
						any(
							feature = "alcohol-warning-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#alcohol_warning_property = None;
					#[cfg(any(
						any(
							feature = "alternate-name-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#alternate_name_property = None;
					#[cfg(any(
						any(feature = "asin-property-schema", feature = "pending-schema-section"),
						doc
					))]
					let mut r#asin_property = None;
					#[cfg(any(
						any(
							feature = "audience-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#audience_property = None;
					#[cfg(any(
						any(
							feature = "available-strength-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#available_strength_property = None;
					#[cfg(any(
						any(feature = "award-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#award_property = None;
					#[cfg(any(
						any(
							feature = "awards-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#awards_property = None;
					#[cfg(any(
						any(feature = "brand-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#brand_property = None;
					#[cfg(any(
						any(
							feature = "breastfeeding-warning-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#breastfeeding_warning_property = None;
					#[cfg(any(
						any(
							feature = "category-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#category_property = None;
					#[cfg(any(
						any(
							feature = "clincal-pharmacology-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#clincal_pharmacology_property = None;
					#[cfg(any(
						any(
							feature = "clinical-pharmacology-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#clinical_pharmacology_property = None;
					#[cfg(any(
						any(
							feature = "code-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#code_property = None;
					#[cfg(any(
						any(feature = "color-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#color_property = None;
					#[cfg(any(
						any(
							feature = "country-of-assembly-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#country_of_assembly_property = None;
					#[cfg(any(
						any(
							feature = "country-of-last-processing-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#country_of_last_processing_property = None;
					#[cfg(any(
						any(
							feature = "country-of-origin-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#country_of_origin_property = None;
					#[cfg(any(
						any(feature = "depth-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#depth_property = None;
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
							feature = "dosage-form-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#dosage_form_property = None;
					#[cfg(any(
						any(
							feature = "dose-schedule-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#dose_schedule_property = None;
					#[cfg(any(
						any(
							feature = "drug-class-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#drug_class_property = None;
					#[cfg(any(
						any(
							feature = "drug-unit-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#drug_unit_property = None;
					#[cfg(any(
						any(
							feature = "food-warning-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#food_warning_property = None;
					#[cfg(any(
						any(
							feature = "funding-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#funding_property = None;
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
							feature = "guideline-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#guideline_property = None;
					#[cfg(any(
						any(
							feature = "has-adult-consideration-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#has_adult_consideration_property = None;
					#[cfg(any(
						any(
							feature = "has-energy-consumption-details-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#has_energy_consumption_details_property = None;
					#[cfg(any(
						any(
							feature = "has-measurement-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#has_measurement_property = None;
					#[cfg(any(
						any(
							feature = "has-merchant-return-policy-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#has_merchant_return_policy_property = None;
					#[cfg(any(
						any(
							feature = "has-product-return-policy-property-schema",
							feature = "attic-schema-section"
						),
						doc
					))]
					let mut r#has_product_return_policy_property = None;
					#[cfg(any(
						any(
							feature = "height-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#height_property = None;
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
							feature = "in-product-group-with-id-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#in_product_group_with_id_property = None;
					#[cfg(any(
						any(
							feature = "included-in-health-insurance-plan-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#included_in_health_insurance_plan_property = None;
					#[cfg(any(
						any(
							feature = "interacting-drug-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#interacting_drug_property = None;
					#[cfg(any(
						any(
							feature = "is-accessory-or-spare-part-for-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_accessory_or_spare_part_for_property = None;
					#[cfg(any(
						any(
							feature = "is-available-generically-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#is_available_generically_property = None;
					#[cfg(any(
						any(
							feature = "is-consumable-for-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_consumable_for_property = None;
					#[cfg(any(
						any(
							feature = "is-family-friendly-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_family_friendly_property = None;
					#[cfg(any(
						any(
							feature = "is-proprietary-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#is_proprietary_property = None;
					#[cfg(any(
						any(
							feature = "is-related-to-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_related_to_property = None;
					#[cfg(any(
						any(
							feature = "is-similar-to-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_similar_to_property = None;
					#[cfg(any(
						any(
							feature = "is-variant-of-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_variant_of_property = None;
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
							feature = "keywords-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#keywords_property = None;
					#[cfg(any(
						any(
							feature = "label-details-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#label_details_property = None;
					#[cfg(any(
						any(
							feature = "legal-status-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#legal_status_property = None;
					#[cfg(any(
						any(feature = "logo-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#logo_property = None;
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
							feature = "manufacturer-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#manufacturer_property = None;
					#[cfg(any(
						any(
							feature = "material-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#material_property = None;
					#[cfg(any(
						any(
							feature = "maximum-intake-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#maximum_intake_property = None;
					#[cfg(any(
						any(
							feature = "mechanism-of-action-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#mechanism_of_action_property = None;
					#[cfg(any(
						any(
							feature = "medicine-system-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#medicine_system_property = None;
					#[cfg(any(
						any(
							feature = "mobile-url-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#mobile_url_property = None;
					#[cfg(any(
						any(feature = "model-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#model_property = None;
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
							feature = "negative-notes-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#negative_notes_property = None;
					#[cfg(any(
						any(
							feature = "non-proprietary-name-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#non_proprietary_name_property = None;
					#[cfg(any(
						any(feature = "nsn-property-schema", feature = "pending-schema-section"),
						doc
					))]
					let mut r#nsn_property = None;
					#[cfg(any(
						any(
							feature = "offers-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#offers_property = None;
					#[cfg(any(
						any(
							feature = "overdosage-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#overdosage_property = None;
					#[cfg(any(
						any(
							feature = "pattern-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#pattern_property = None;
					#[cfg(any(
						any(
							feature = "positive-notes-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#positive_notes_property = None;
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
							feature = "pregnancy-category-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#pregnancy_category_property = None;
					#[cfg(any(
						any(
							feature = "pregnancy-warning-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#pregnancy_warning_property = None;
					#[cfg(any(
						any(
							feature = "prescribing-info-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#prescribing_info_property = None;
					#[cfg(any(
						any(
							feature = "prescription-status-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#prescription_status_property = None;
					#[cfg(any(
						any(
							feature = "product-id-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#product_id_property = None;
					#[cfg(any(
						any(
							feature = "production-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#production_date_property = None;
					#[cfg(any(
						any(
							feature = "proprietary-name-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#proprietary_name_property = None;
					#[cfg(any(
						any(
							feature = "purchase-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#purchase_date_property = None;
					#[cfg(any(
						any(
							feature = "recognizing-authority-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#recognizing_authority_property = None;
					#[cfg(any(
						any(
							feature = "related-drug-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#related_drug_property = None;
					#[cfg(any(
						any(
							feature = "release-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#release_date_property = None;
					#[cfg(any(
						any(
							feature = "relevant-specialty-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#relevant_specialty_property = None;
					#[cfg(any(
						any(
							feature = "review-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#review_property = None;
					#[cfg(any(
						any(
							feature = "reviews-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#reviews_property = None;
					#[cfg(any(
						any(feature = "rxcui-property-schema", feature = "pending-schema-section"),
						doc
					))]
					let mut r#rxcui_property = None;
					#[cfg(any(
						any(
							feature = "same-as-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#same_as_property = None;
					#[cfg(any(
						any(feature = "size-property-schema", feature = "pending-schema-section"),
						doc
					))]
					let mut r#size_property = None;
					#[cfg(any(
						any(feature = "sku-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#sku_property = None;
					#[cfg(any(
						any(
							feature = "slogan-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#slogan_property = None;
					#[cfg(any(
						any(
							feature = "study-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#study_property = None;
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
							feature = "warning-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#warning_property = None;
					#[cfg(any(
						any(
							feature = "weight-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#weight_property = None;
					#[cfg(any(
						any(feature = "width-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#width_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
							#[cfg(any(
								any(
									feature = "active-ingredient-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::ActiveIngredient => {
								if r#active_ingredient_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"activeIngredient",
									));
								}
								r#active_ingredient_property = Some({
									struct DeserializeWith(Vec<ActiveIngredientProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "additional-property-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "administration-route-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::AdministrationRoute => {
								if r#administration_route_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"administrationRoute",
									));
								}
								r#administration_route_property = Some({
									struct DeserializeWith(Vec<AdministrationRouteProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "aggregate-rating-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "alcohol-warning-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::AlcoholWarning => {
								if r#alcohol_warning_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"alcoholWarning",
									));
								}
								r#alcohol_warning_property = Some({
									struct DeserializeWith(Vec<AlcoholWarningProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "audience-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "available-strength-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::AvailableStrength => {
								if r#available_strength_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"availableStrength",
									));
								}
								r#available_strength_property = Some({
									struct DeserializeWith(Vec<AvailableStrengthProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "award-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "awards-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "brand-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "breastfeeding-warning-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::BreastfeedingWarning => {
								if r#breastfeeding_warning_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"breastfeedingWarning",
									));
								}
								r#breastfeeding_warning_property = Some({
									struct DeserializeWith(Vec<BreastfeedingWarningProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "category-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "clincal-pharmacology-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::ClincalPharmacology => {
								if r#clincal_pharmacology_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"clincalPharmacology",
									));
								}
								r#clincal_pharmacology_property = Some({
									struct DeserializeWith(Vec<ClincalPharmacologyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "clinical-pharmacology-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::ClinicalPharmacology => {
								if r#clinical_pharmacology_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"clinicalPharmacology",
									));
								}
								r#clinical_pharmacology_property = Some({
									struct DeserializeWith(Vec<ClinicalPharmacologyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "code-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::Code => {
								if r#code_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("code"));
								}
								r#code_property = Some({
									struct DeserializeWith(Vec<CodeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "color-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "country-of-assembly-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "country-of-last-processing-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "country-of-origin-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "depth-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "dosage-form-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::DosageForm => {
								if r#dosage_form_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"dosageForm",
									));
								}
								r#dosage_form_property = Some({
									struct DeserializeWith(Vec<DosageFormProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "dose-schedule-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::DoseSchedule => {
								if r#dose_schedule_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"doseSchedule",
									));
								}
								r#dose_schedule_property = Some({
									struct DeserializeWith(Vec<DoseScheduleProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "drug-class-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::DrugClass => {
								if r#drug_class_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"drugClass",
									));
								}
								r#drug_class_property = Some({
									struct DeserializeWith(Vec<DrugClassProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "drug-unit-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::DrugUnit => {
								if r#drug_unit_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"drugUnit",
									));
								}
								r#drug_unit_property = Some({
									struct DeserializeWith(Vec<DrugUnitProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "food-warning-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::FoodWarning => {
								if r#food_warning_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"foodWarning",
									));
								}
								r#food_warning_property = Some({
									struct DeserializeWith(Vec<FoodWarningProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "funding-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
									feature = "guideline-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::Guideline => {
								if r#guideline_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"guideline",
									));
								}
								r#guideline_property = Some({
									struct DeserializeWith(Vec<GuidelineProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "has-adult-consideration-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "has-energy-consumption-details-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "has-measurement-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "has-merchant-return-policy-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "has-product-return-policy-property-schema",
									feature = "attic-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "height-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "in-product-group-with-id-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "included-in-health-insurance-plan-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::IncludedInHealthInsurancePlan => {
								if r#included_in_health_insurance_plan_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"includedInHealthInsurancePlan",
									));
								}
								r#included_in_health_insurance_plan_property = Some({
									struct DeserializeWith(
										Vec<IncludedInHealthInsurancePlanProperty>,
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
							#[cfg(any(
								any(
									feature = "interacting-drug-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::InteractingDrug => {
								if r#interacting_drug_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"interactingDrug",
									));
								}
								r#interacting_drug_property = Some({
									struct DeserializeWith(Vec<InteractingDrugProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "is-accessory-or-spare-part-for-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "is-available-generically-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::IsAvailableGenerically => {
								if r#is_available_generically_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isAvailableGenerically",
									));
								}
								r#is_available_generically_property = Some({
									struct DeserializeWith(Vec<IsAvailableGenericallyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "is-consumable-for-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "is-family-friendly-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "is-proprietary-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::IsProprietary => {
								if r#is_proprietary_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isProprietary",
									));
								}
								r#is_proprietary_property = Some({
									struct DeserializeWith(Vec<IsProprietaryProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "is-related-to-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "is-similar-to-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "is-variant-of-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "keywords-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "label-details-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::LabelDetails => {
								if r#label_details_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"labelDetails",
									));
								}
								r#label_details_property = Some({
									struct DeserializeWith(Vec<LabelDetailsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "legal-status-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::LegalStatus => {
								if r#legal_status_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"legalStatus",
									));
								}
								r#legal_status_property = Some({
									struct DeserializeWith(Vec<LegalStatusProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "logo-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "manufacturer-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "material-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "maximum-intake-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::MaximumIntake => {
								if r#maximum_intake_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"maximumIntake",
									));
								}
								r#maximum_intake_property = Some({
									struct DeserializeWith(Vec<MaximumIntakeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "mechanism-of-action-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::MechanismOfAction => {
								if r#mechanism_of_action_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"mechanismOfAction",
									));
								}
								r#mechanism_of_action_property = Some({
									struct DeserializeWith(Vec<MechanismOfActionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "medicine-system-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::MedicineSystem => {
								if r#medicine_system_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"medicineSystem",
									));
								}
								r#medicine_system_property = Some({
									struct DeserializeWith(Vec<MedicineSystemProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "mobile-url-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "model-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "negative-notes-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "non-proprietary-name-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::NonProprietaryName => {
								if r#non_proprietary_name_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"nonProprietaryName",
									));
								}
								r#non_proprietary_name_property = Some({
									struct DeserializeWith(Vec<NonProprietaryNameProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "nsn-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "offers-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "overdosage-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::Overdosage => {
								if r#overdosage_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"overdosage",
									));
								}
								r#overdosage_property = Some({
									struct DeserializeWith(Vec<OverdosageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "pattern-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "positive-notes-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
									feature = "pregnancy-category-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::PregnancyCategory => {
								if r#pregnancy_category_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"pregnancyCategory",
									));
								}
								r#pregnancy_category_property = Some({
									struct DeserializeWith(Vec<PregnancyCategoryProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "pregnancy-warning-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::PregnancyWarning => {
								if r#pregnancy_warning_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"pregnancyWarning",
									));
								}
								r#pregnancy_warning_property = Some({
									struct DeserializeWith(Vec<PregnancyWarningProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "prescribing-info-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::PrescribingInfo => {
								if r#prescribing_info_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"prescribingInfo",
									));
								}
								r#prescribing_info_property = Some({
									struct DeserializeWith(Vec<PrescribingInfoProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "prescription-status-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::PrescriptionStatus => {
								if r#prescription_status_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"prescriptionStatus",
									));
								}
								r#prescription_status_property = Some({
									struct DeserializeWith(Vec<PrescriptionStatusProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "product-id-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "production-date-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "proprietary-name-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::ProprietaryName => {
								if r#proprietary_name_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"proprietaryName",
									));
								}
								r#proprietary_name_property = Some({
									struct DeserializeWith(Vec<ProprietaryNameProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "purchase-date-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "recognizing-authority-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::RecognizingAuthority => {
								if r#recognizing_authority_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"recognizingAuthority",
									));
								}
								r#recognizing_authority_property = Some({
									struct DeserializeWith(Vec<RecognizingAuthorityProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "related-drug-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::RelatedDrug => {
								if r#related_drug_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"relatedDrug",
									));
								}
								r#related_drug_property = Some({
									struct DeserializeWith(Vec<RelatedDrugProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "release-date-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "relevant-specialty-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::RelevantSpecialty => {
								if r#relevant_specialty_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"relevantSpecialty",
									));
								}
								r#relevant_specialty_property = Some({
									struct DeserializeWith(Vec<RelevantSpecialtyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "review-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "reviews-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "rxcui-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::Rxcui => {
								if r#rxcui_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("rxcui"));
								}
								r#rxcui_property = Some({
									struct DeserializeWith(Vec<RxcuiProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "size-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
									feature = "slogan-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "study-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::Study => {
								if r#study_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("study"));
								}
								r#study_property = Some({
									struct DeserializeWith(Vec<StudyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "warning-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::Warning => {
								if r#warning_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"warning",
									));
								}
								r#warning_property = Some({
									struct DeserializeWith(Vec<WarningProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "weight-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "width-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
					Ok(Drug {
						#[cfg(any(
							any(
								feature = "active-ingredient-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#active_ingredient: r#active_ingredient_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "additional-property-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#additional_property: r#additional_property_property.unwrap_or_default(),
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
								feature = "administration-route-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#administration_route: r#administration_route_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "aggregate-rating-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#aggregate_rating: r#aggregate_rating_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "alcohol-warning-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#alcohol_warning: r#alcohol_warning_property.unwrap_or_default(),
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
								feature = "asin-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#asin: r#asin_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#audience: r#audience_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "available-strength-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#available_strength: r#available_strength_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "award-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#award: r#award_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "awards-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#awards: r#awards_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "brand-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#brand: r#brand_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "breastfeeding-warning-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#breastfeeding_warning: r#breastfeeding_warning_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "category-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#category: r#category_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "clincal-pharmacology-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#clincal_pharmacology: r#clincal_pharmacology_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "clinical-pharmacology-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#clinical_pharmacology: r#clinical_pharmacology_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "code-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#code: r#code_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "color-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#color: r#color_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "country-of-assembly-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#country_of_assembly: r#country_of_assembly_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "country-of-last-processing-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#country_of_last_processing: r#country_of_last_processing_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "country-of-origin-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#country_of_origin: r#country_of_origin_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "depth-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#depth: r#depth_property.unwrap_or_default(),
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
								feature = "dosage-form-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#dosage_form: r#dosage_form_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "dose-schedule-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#dose_schedule: r#dose_schedule_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "drug-class-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#drug_class: r#drug_class_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "drug-unit-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#drug_unit: r#drug_unit_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "food-warning-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#food_warning: r#food_warning_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#funding: r#funding_property.unwrap_or_default(),
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
								feature = "guideline-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#guideline: r#guideline_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-adult-consideration-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#has_adult_consideration: r#has_adult_consideration_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-energy-consumption-details-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#has_energy_consumption_details: r#has_energy_consumption_details_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-measurement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#has_measurement: r#has_measurement_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-merchant-return-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#has_merchant_return_policy: r#has_merchant_return_policy_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-product-return-policy-property-schema",
								feature = "attic-schema-section"
							),
							doc
						))]
						r#has_product_return_policy: r#has_product_return_policy_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "height-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#height: r#height_property.unwrap_or_default(),
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
								feature = "in-product-group-with-id-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#in_product_group_with_id: r#in_product_group_with_id_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "included-in-health-insurance-plan-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#included_in_health_insurance_plan:
							r#included_in_health_insurance_plan_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "interacting-drug-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#interacting_drug: r#interacting_drug_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-accessory-or-spare-part-for-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_accessory_or_spare_part_for: r#is_accessory_or_spare_part_for_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-available-generically-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#is_available_generically: r#is_available_generically_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-consumable-for-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_consumable_for: r#is_consumable_for_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-family-friendly-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_family_friendly: r#is_family_friendly_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-proprietary-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#is_proprietary: r#is_proprietary_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-related-to-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_related_to: r#is_related_to_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-similar-to-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_similar_to: r#is_similar_to_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-variant-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_variant_of: r#is_variant_of_property.unwrap_or_default(),
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
								feature = "keywords-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#keywords: r#keywords_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "label-details-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#label_details: r#label_details_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "legal-status-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#legal_status: r#legal_status_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "logo-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#logo: r#logo_property.unwrap_or_default(),
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
								feature = "manufacturer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#manufacturer: r#manufacturer_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "material-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#material: r#material_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "maximum-intake-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#maximum_intake: r#maximum_intake_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "mechanism-of-action-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#mechanism_of_action: r#mechanism_of_action_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "medicine-system-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#medicine_system: r#medicine_system_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "mobile-url-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#mobile_url: r#mobile_url_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "model-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#model: r#model_property.unwrap_or_default(),
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
								feature = "negative-notes-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#negative_notes: r#negative_notes_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "non-proprietary-name-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#non_proprietary_name: r#non_proprietary_name_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "nsn-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#nsn: r#nsn_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "offers-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#offers: r#offers_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "overdosage-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#overdosage: r#overdosage_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "pattern-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#pattern: r#pattern_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "positive-notes-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#positive_notes: r#positive_notes_property.unwrap_or_default(),
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
								feature = "pregnancy-category-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#pregnancy_category: r#pregnancy_category_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "pregnancy-warning-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#pregnancy_warning: r#pregnancy_warning_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "prescribing-info-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#prescribing_info: r#prescribing_info_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "prescription-status-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#prescription_status: r#prescription_status_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "product-id-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#product_id: r#product_id_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "production-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#production_date: r#production_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "proprietary-name-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#proprietary_name: r#proprietary_name_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "purchase-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#purchase_date: r#purchase_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "recognizing-authority-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#recognizing_authority: r#recognizing_authority_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "related-drug-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#related_drug: r#related_drug_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "release-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#release_date: r#release_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "relevant-specialty-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#relevant_specialty: r#relevant_specialty_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "review-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#review: r#review_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "reviews-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#reviews: r#reviews_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "rxcui-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#rxcui: r#rxcui_property.unwrap_or_default(),
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
								feature = "size-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#size: r#size_property.unwrap_or_default(),
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
								feature = "slogan-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#slogan: r#slogan_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "study-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#study: r#study_property.unwrap_or_default(),
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
								feature = "warning-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#warning: r#warning_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "weight-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#weight: r#weight_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "width-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#width: r#width_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				#[cfg(any(
					any(
						feature = "active-ingredient-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"activeIngredient",
				#[cfg(any(
					any(
						feature = "additional-property-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"additionalProperty",
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
						feature = "administration-route-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"administrationRoute",
				#[cfg(any(
					any(
						feature = "aggregate-rating-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"aggregateRating",
				#[cfg(any(
					any(
						feature = "alcohol-warning-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"alcoholWarning",
				#[cfg(any(
					any(
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"alternateName",
				#[cfg(any(
					any(feature = "asin-property-schema", feature = "pending-schema-section"),
					doc
				))]
				"asin",
				#[cfg(any(
					any(
						feature = "audience-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"audience",
				#[cfg(any(
					any(
						feature = "available-strength-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"availableStrength",
				#[cfg(any(
					any(feature = "award-property-schema", feature = "general-schema-section"),
					doc
				))]
				"award",
				#[cfg(any(
					any(feature = "awards-property-schema", feature = "general-schema-section"),
					doc
				))]
				"awards",
				#[cfg(any(
					any(feature = "brand-property-schema", feature = "general-schema-section"),
					doc
				))]
				"brand",
				#[cfg(any(
					any(
						feature = "breastfeeding-warning-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"breastfeedingWarning",
				#[cfg(any(
					any(
						feature = "category-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"category",
				#[cfg(any(
					any(
						feature = "clincal-pharmacology-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"clincalPharmacology",
				#[cfg(any(
					any(
						feature = "clinical-pharmacology-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"clinicalPharmacology",
				#[cfg(any(
					any(
						feature = "code-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"code",
				#[cfg(any(
					any(feature = "color-property-schema", feature = "general-schema-section"),
					doc
				))]
				"color",
				#[cfg(any(
					any(
						feature = "country-of-assembly-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"countryOfAssembly",
				#[cfg(any(
					any(
						feature = "country-of-last-processing-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"countryOfLastProcessing",
				#[cfg(any(
					any(
						feature = "country-of-origin-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"countryOfOrigin",
				#[cfg(any(
					any(feature = "depth-property-schema", feature = "general-schema-section"),
					doc
				))]
				"depth",
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
						feature = "dosage-form-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"dosageForm",
				#[cfg(any(
					any(
						feature = "dose-schedule-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"doseSchedule",
				#[cfg(any(
					any(
						feature = "drug-class-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"drugClass",
				#[cfg(any(
					any(
						feature = "drug-unit-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"drugUnit",
				#[cfg(any(
					any(
						feature = "food-warning-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"foodWarning",
				#[cfg(any(
					any(
						feature = "funding-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"funding",
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
						feature = "guideline-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"guideline",
				#[cfg(any(
					any(
						feature = "has-adult-consideration-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"hasAdultConsideration",
				#[cfg(any(
					any(
						feature = "has-energy-consumption-details-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"hasEnergyConsumptionDetails",
				#[cfg(any(
					any(
						feature = "has-measurement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"hasMeasurement",
				#[cfg(any(
					any(
						feature = "has-merchant-return-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"hasMerchantReturnPolicy",
				#[cfg(any(
					any(
						feature = "has-product-return-policy-property-schema",
						feature = "attic-schema-section"
					),
					doc
				))]
				"hasProductReturnPolicy",
				#[cfg(any(
					any(feature = "height-property-schema", feature = "general-schema-section"),
					doc
				))]
				"height",
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
						feature = "in-product-group-with-id-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"inProductGroupWithID",
				#[cfg(any(
					any(
						feature = "included-in-health-insurance-plan-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"includedInHealthInsurancePlan",
				#[cfg(any(
					any(
						feature = "interacting-drug-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"interactingDrug",
				#[cfg(any(
					any(
						feature = "is-accessory-or-spare-part-for-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isAccessoryOrSparePartFor",
				#[cfg(any(
					any(
						feature = "is-available-generically-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"isAvailableGenerically",
				#[cfg(any(
					any(
						feature = "is-consumable-for-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isConsumableFor",
				#[cfg(any(
					any(
						feature = "is-family-friendly-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isFamilyFriendly",
				#[cfg(any(
					any(
						feature = "is-proprietary-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"isProprietary",
				#[cfg(any(
					any(
						feature = "is-related-to-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isRelatedTo",
				#[cfg(any(
					any(
						feature = "is-similar-to-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isSimilarTo",
				#[cfg(any(
					any(
						feature = "is-variant-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isVariantOf",
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
						feature = "keywords-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"keywords",
				#[cfg(any(
					any(
						feature = "label-details-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"labelDetails",
				#[cfg(any(
					any(
						feature = "legal-status-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"legalStatus",
				#[cfg(any(
					any(feature = "logo-property-schema", feature = "general-schema-section"),
					doc
				))]
				"logo",
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
						feature = "manufacturer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"manufacturer",
				#[cfg(any(
					any(
						feature = "material-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"material",
				#[cfg(any(
					any(
						feature = "maximum-intake-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"maximumIntake",
				#[cfg(any(
					any(
						feature = "mechanism-of-action-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"mechanismOfAction",
				#[cfg(any(
					any(
						feature = "medicine-system-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"medicineSystem",
				#[cfg(any(
					any(
						feature = "mobile-url-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"mobileUrl",
				#[cfg(any(
					any(feature = "model-property-schema", feature = "general-schema-section"),
					doc
				))]
				"model",
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
						feature = "negative-notes-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"negativeNotes",
				#[cfg(any(
					any(
						feature = "non-proprietary-name-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"nonProprietaryName",
				#[cfg(any(
					any(feature = "nsn-property-schema", feature = "pending-schema-section"),
					doc
				))]
				"nsn",
				#[cfg(any(
					any(feature = "offers-property-schema", feature = "general-schema-section"),
					doc
				))]
				"offers",
				#[cfg(any(
					any(
						feature = "overdosage-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"overdosage",
				#[cfg(any(
					any(
						feature = "pattern-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"pattern",
				#[cfg(any(
					any(
						feature = "positive-notes-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"positiveNotes",
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
						feature = "pregnancy-category-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"pregnancyCategory",
				#[cfg(any(
					any(
						feature = "pregnancy-warning-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"pregnancyWarning",
				#[cfg(any(
					any(
						feature = "prescribing-info-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"prescribingInfo",
				#[cfg(any(
					any(
						feature = "prescription-status-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"prescriptionStatus",
				#[cfg(any(
					any(
						feature = "product-id-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"productID",
				#[cfg(any(
					any(
						feature = "production-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"productionDate",
				#[cfg(any(
					any(
						feature = "proprietary-name-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"proprietaryName",
				#[cfg(any(
					any(
						feature = "purchase-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"purchaseDate",
				#[cfg(any(
					any(
						feature = "recognizing-authority-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"recognizingAuthority",
				#[cfg(any(
					any(
						feature = "related-drug-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"relatedDrug",
				#[cfg(any(
					any(
						feature = "release-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"releaseDate",
				#[cfg(any(
					any(
						feature = "relevant-specialty-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"relevantSpecialty",
				#[cfg(any(
					any(feature = "review-property-schema", feature = "general-schema-section"),
					doc
				))]
				"review",
				#[cfg(any(
					any(
						feature = "reviews-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"reviews",
				#[cfg(any(
					any(feature = "rxcui-property-schema", feature = "pending-schema-section"),
					doc
				))]
				"rxcui",
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"sameAs",
				#[cfg(any(
					any(feature = "size-property-schema", feature = "pending-schema-section"),
					doc
				))]
				"size",
				#[cfg(any(
					any(feature = "sku-property-schema", feature = "general-schema-section"),
					doc
				))]
				"sku",
				#[cfg(any(
					any(feature = "slogan-property-schema", feature = "general-schema-section"),
					doc
				))]
				"slogan",
				#[cfg(any(
					any(
						feature = "study-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"study",
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
						feature = "warning-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"warning",
				#[cfg(any(
					any(feature = "weight-property-schema", feature = "general-schema-section"),
					doc
				))]
				"weight",
				#[cfg(any(
					any(feature = "width-property-schema", feature = "general-schema-section"),
					doc
				))]
				"width",
			];
			deserializer.deserialize_struct("Drug", FIELDS, ClassVisitor)
		}
	}
}
