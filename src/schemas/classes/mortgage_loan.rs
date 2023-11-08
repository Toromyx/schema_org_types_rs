use super::*;
/// <https://schema.org/MortgageLoan>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct MortgageLoan {
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	pub r#aggregate_rating: Vec<AggregateRatingProperty>,
	pub r#alternate_name: Vec<AlternateNameProperty>,
	pub r#amount: Vec<AmountProperty>,
	pub r#annual_percentage_rate: Vec<AnnualPercentageRateProperty>,
	pub r#area_served: Vec<AreaServedProperty>,
	pub r#audience: Vec<AudienceProperty>,
	pub r#available_channel: Vec<AvailableChannelProperty>,
	pub r#award: Vec<AwardProperty>,
	pub r#brand: Vec<BrandProperty>,
	pub r#broker: Vec<BrokerProperty>,
	pub r#category: Vec<CategoryProperty>,
	pub r#currency: Vec<CurrencyProperty>,
	pub r#description: Vec<DescriptionProperty>,
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	pub r#domiciled_mortgage: Vec<DomiciledMortgageProperty>,
	pub r#fees_and_commissions_specification: Vec<FeesAndCommissionsSpecificationProperty>,
	pub r#grace_period: Vec<GracePeriodProperty>,
	pub r#has_offer_catalog: Vec<HasOfferCatalogProperty>,
	pub r#hours_available: Vec<HoursAvailableProperty>,
	pub r#identifier: Vec<IdentifierProperty>,
	pub r#image: Vec<ImageProperty>,
	pub r#interest_rate: Vec<InterestRateProperty>,
	pub r#is_related_to: Vec<IsRelatedToProperty>,
	pub r#is_similar_to: Vec<IsSimilarToProperty>,
	pub r#loan_mortgage_mandate_amount: Vec<LoanMortgageMandateAmountProperty>,
	pub r#loan_repayment_form: Vec<LoanRepaymentFormProperty>,
	pub r#loan_term: Vec<LoanTermProperty>,
	pub r#loan_type: Vec<LoanTypeProperty>,
	pub r#logo: Vec<LogoProperty>,
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	pub r#name: Vec<NameProperty>,
	pub r#offers: Vec<OffersProperty>,
	pub r#potential_action: Vec<PotentialActionProperty>,
	pub r#produces: Vec<ProducesProperty>,
	pub r#provider: Vec<ProviderProperty>,
	pub r#provider_mobility: Vec<ProviderMobilityProperty>,
	pub r#recourse_loan: Vec<RecourseLoanProperty>,
	pub r#renegotiable_loan: Vec<RenegotiableLoanProperty>,
	pub r#required_collateral: Vec<RequiredCollateralProperty>,
	pub r#review: Vec<ReviewProperty>,
	pub r#same_as: Vec<SameAsProperty>,
	pub r#service_area: Vec<ServiceAreaProperty>,
	pub r#service_audience: Vec<ServiceAudienceProperty>,
	pub r#service_output: Vec<ServiceOutputProperty>,
	pub r#service_type: Vec<ServiceTypeProperty>,
	pub r#slogan: Vec<SloganProperty>,
	pub r#subject_of: Vec<SubjectOfProperty>,
	pub r#terms_of_service: Vec<TermsOfServiceProperty>,
	pub r#url: Vec<UrlProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MortgageLoan {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#additional_type) as usize,
				!Vec::is_empty(&self.r#aggregate_rating) as usize,
				!Vec::is_empty(&self.r#alternate_name) as usize,
				!Vec::is_empty(&self.r#amount) as usize,
				!Vec::is_empty(&self.r#annual_percentage_rate) as usize,
				!Vec::is_empty(&self.r#area_served) as usize,
				!Vec::is_empty(&self.r#audience) as usize,
				!Vec::is_empty(&self.r#available_channel) as usize,
				!Vec::is_empty(&self.r#award) as usize,
				!Vec::is_empty(&self.r#brand) as usize,
				!Vec::is_empty(&self.r#broker) as usize,
				!Vec::is_empty(&self.r#category) as usize,
				!Vec::is_empty(&self.r#currency) as usize,
				!Vec::is_empty(&self.r#description) as usize,
				!Vec::is_empty(&self.r#disambiguating_description) as usize,
				!Vec::is_empty(&self.r#domiciled_mortgage) as usize,
				!Vec::is_empty(&self.r#fees_and_commissions_specification) as usize,
				!Vec::is_empty(&self.r#grace_period) as usize,
				!Vec::is_empty(&self.r#has_offer_catalog) as usize,
				!Vec::is_empty(&self.r#hours_available) as usize,
				!Vec::is_empty(&self.r#identifier) as usize,
				!Vec::is_empty(&self.r#image) as usize,
				!Vec::is_empty(&self.r#interest_rate) as usize,
				!Vec::is_empty(&self.r#is_related_to) as usize,
				!Vec::is_empty(&self.r#is_similar_to) as usize,
				!Vec::is_empty(&self.r#loan_mortgage_mandate_amount) as usize,
				!Vec::is_empty(&self.r#loan_repayment_form) as usize,
				!Vec::is_empty(&self.r#loan_term) as usize,
				!Vec::is_empty(&self.r#loan_type) as usize,
				!Vec::is_empty(&self.r#logo) as usize,
				!Vec::is_empty(&self.r#main_entity_of_page) as usize,
				!Vec::is_empty(&self.r#name) as usize,
				!Vec::is_empty(&self.r#offers) as usize,
				!Vec::is_empty(&self.r#potential_action) as usize,
				!Vec::is_empty(&self.r#produces) as usize,
				!Vec::is_empty(&self.r#provider) as usize,
				!Vec::is_empty(&self.r#provider_mobility) as usize,
				!Vec::is_empty(&self.r#recourse_loan) as usize,
				!Vec::is_empty(&self.r#renegotiable_loan) as usize,
				!Vec::is_empty(&self.r#required_collateral) as usize,
				!Vec::is_empty(&self.r#review) as usize,
				!Vec::is_empty(&self.r#same_as) as usize,
				!Vec::is_empty(&self.r#service_area) as usize,
				!Vec::is_empty(&self.r#service_audience) as usize,
				!Vec::is_empty(&self.r#service_output) as usize,
				!Vec::is_empty(&self.r#service_type) as usize,
				!Vec::is_empty(&self.r#slogan) as usize,
				!Vec::is_empty(&self.r#subject_of) as usize,
				!Vec::is_empty(&self.r#terms_of_service) as usize,
				!Vec::is_empty(&self.r#url) as usize,
			]
			.iter()
			.sum();
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "MortgageLoan", len)?;
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
			if !Vec::is_empty(&self.r#amount) {
				serialize_struct.serialize_field("amount", {
					struct SerializeWith<'a>(&'a Vec<AmountProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#amount)
				})?;
			} else {
				serialize_struct.skip_field("amount")?;
			}
			if !Vec::is_empty(&self.r#annual_percentage_rate) {
				serialize_struct.serialize_field("annualPercentageRate", {
					struct SerializeWith<'a>(&'a Vec<AnnualPercentageRateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#annual_percentage_rate)
				})?;
			} else {
				serialize_struct.skip_field("annualPercentageRate")?;
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
			if !Vec::is_empty(&self.r#available_channel) {
				serialize_struct.serialize_field("availableChannel", {
					struct SerializeWith<'a>(&'a Vec<AvailableChannelProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#available_channel)
				})?;
			} else {
				serialize_struct.skip_field("availableChannel")?;
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
			if !Vec::is_empty(&self.r#currency) {
				serialize_struct.serialize_field("currency", {
					struct SerializeWith<'a>(&'a Vec<CurrencyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#currency)
				})?;
			} else {
				serialize_struct.skip_field("currency")?;
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
			if !Vec::is_empty(&self.r#domiciled_mortgage) {
				serialize_struct.serialize_field("domiciledMortgage", {
					struct SerializeWith<'a>(&'a Vec<DomiciledMortgageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#domiciled_mortgage)
				})?;
			} else {
				serialize_struct.skip_field("domiciledMortgage")?;
			}
			if !Vec::is_empty(&self.r#fees_and_commissions_specification) {
				serialize_struct.serialize_field("feesAndCommissionsSpecification", {
					struct SerializeWith<'a>(&'a Vec<FeesAndCommissionsSpecificationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#fees_and_commissions_specification)
				})?;
			} else {
				serialize_struct.skip_field("feesAndCommissionsSpecification")?;
			}
			if !Vec::is_empty(&self.r#grace_period) {
				serialize_struct.serialize_field("gracePeriod", {
					struct SerializeWith<'a>(&'a Vec<GracePeriodProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#grace_period)
				})?;
			} else {
				serialize_struct.skip_field("gracePeriod")?;
			}
			if !Vec::is_empty(&self.r#has_offer_catalog) {
				serialize_struct.serialize_field("hasOfferCatalog", {
					struct SerializeWith<'a>(&'a Vec<HasOfferCatalogProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_offer_catalog)
				})?;
			} else {
				serialize_struct.skip_field("hasOfferCatalog")?;
			}
			if !Vec::is_empty(&self.r#hours_available) {
				serialize_struct.serialize_field("hoursAvailable", {
					struct SerializeWith<'a>(&'a Vec<HoursAvailableProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#hours_available)
				})?;
			} else {
				serialize_struct.skip_field("hoursAvailable")?;
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
			if !Vec::is_empty(&self.r#interest_rate) {
				serialize_struct.serialize_field("interestRate", {
					struct SerializeWith<'a>(&'a Vec<InterestRateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#interest_rate)
				})?;
			} else {
				serialize_struct.skip_field("interestRate")?;
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
			if !Vec::is_empty(&self.r#loan_mortgage_mandate_amount) {
				serialize_struct.serialize_field("loanMortgageMandateAmount", {
					struct SerializeWith<'a>(&'a Vec<LoanMortgageMandateAmountProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#loan_mortgage_mandate_amount)
				})?;
			} else {
				serialize_struct.skip_field("loanMortgageMandateAmount")?;
			}
			if !Vec::is_empty(&self.r#loan_repayment_form) {
				serialize_struct.serialize_field("loanRepaymentForm", {
					struct SerializeWith<'a>(&'a Vec<LoanRepaymentFormProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#loan_repayment_form)
				})?;
			} else {
				serialize_struct.skip_field("loanRepaymentForm")?;
			}
			if !Vec::is_empty(&self.r#loan_term) {
				serialize_struct.serialize_field("loanTerm", {
					struct SerializeWith<'a>(&'a Vec<LoanTermProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#loan_term)
				})?;
			} else {
				serialize_struct.skip_field("loanTerm")?;
			}
			if !Vec::is_empty(&self.r#loan_type) {
				serialize_struct.serialize_field("loanType", {
					struct SerializeWith<'a>(&'a Vec<LoanTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#loan_type)
				})?;
			} else {
				serialize_struct.skip_field("loanType")?;
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
			if !Vec::is_empty(&self.r#produces) {
				serialize_struct.serialize_field("produces", {
					struct SerializeWith<'a>(&'a Vec<ProducesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#produces)
				})?;
			} else {
				serialize_struct.skip_field("produces")?;
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
			if !Vec::is_empty(&self.r#provider_mobility) {
				serialize_struct.serialize_field("providerMobility", {
					struct SerializeWith<'a>(&'a Vec<ProviderMobilityProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#provider_mobility)
				})?;
			} else {
				serialize_struct.skip_field("providerMobility")?;
			}
			if !Vec::is_empty(&self.r#recourse_loan) {
				serialize_struct.serialize_field("recourseLoan", {
					struct SerializeWith<'a>(&'a Vec<RecourseLoanProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#recourse_loan)
				})?;
			} else {
				serialize_struct.skip_field("recourseLoan")?;
			}
			if !Vec::is_empty(&self.r#renegotiable_loan) {
				serialize_struct.serialize_field("renegotiableLoan", {
					struct SerializeWith<'a>(&'a Vec<RenegotiableLoanProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#renegotiable_loan)
				})?;
			} else {
				serialize_struct.skip_field("renegotiableLoan")?;
			}
			if !Vec::is_empty(&self.r#required_collateral) {
				serialize_struct.serialize_field("requiredCollateral", {
					struct SerializeWith<'a>(&'a Vec<RequiredCollateralProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#required_collateral)
				})?;
			} else {
				serialize_struct.skip_field("requiredCollateral")?;
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
			if !Vec::is_empty(&self.r#service_area) {
				serialize_struct.serialize_field("serviceArea", {
					struct SerializeWith<'a>(&'a Vec<ServiceAreaProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#service_area)
				})?;
			} else {
				serialize_struct.skip_field("serviceArea")?;
			}
			if !Vec::is_empty(&self.r#service_audience) {
				serialize_struct.serialize_field("serviceAudience", {
					struct SerializeWith<'a>(&'a Vec<ServiceAudienceProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#service_audience)
				})?;
			} else {
				serialize_struct.skip_field("serviceAudience")?;
			}
			if !Vec::is_empty(&self.r#service_output) {
				serialize_struct.serialize_field("serviceOutput", {
					struct SerializeWith<'a>(&'a Vec<ServiceOutputProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#service_output)
				})?;
			} else {
				serialize_struct.skip_field("serviceOutput")?;
			}
			if !Vec::is_empty(&self.r#service_type) {
				serialize_struct.serialize_field("serviceType", {
					struct SerializeWith<'a>(&'a Vec<ServiceTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#service_type)
				})?;
			} else {
				serialize_struct.skip_field("serviceType")?;
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
			if !Vec::is_empty(&self.r#terms_of_service) {
				serialize_struct.serialize_field("termsOfService", {
					struct SerializeWith<'a>(&'a Vec<TermsOfServiceProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#terms_of_service)
				})?;
			} else {
				serialize_struct.skip_field("termsOfService")?;
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
	impl<'de> Deserialize<'de> for MortgageLoan {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AdditionalType,
				AggregateRating,
				AlternateName,
				Amount,
				AnnualPercentageRate,
				AreaServed,
				Audience,
				AvailableChannel,
				Award,
				Brand,
				Broker,
				Category,
				Currency,
				Description,
				DisambiguatingDescription,
				DomiciledMortgage,
				FeesAndCommissionsSpecification,
				GracePeriod,
				HasOfferCatalog,
				HoursAvailable,
				Identifier,
				Image,
				InterestRate,
				IsRelatedTo,
				IsSimilarTo,
				LoanMortgageMandateAmount,
				LoanRepaymentForm,
				LoanTerm,
				LoanType,
				Logo,
				MainEntityOfPage,
				Name,
				Offers,
				PotentialAction,
				Produces,
				Provider,
				ProviderMobility,
				RecourseLoan,
				RenegotiableLoan,
				RequiredCollateral,
				Review,
				SameAs,
				ServiceArea,
				ServiceAudience,
				ServiceOutput,
				ServiceType,
				Slogan,
				SubjectOf,
				TermsOfService,
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
						"additionalType" => Ok(Field::AdditionalType),
						"aggregateRating" => Ok(Field::AggregateRating),
						"alternateName" => Ok(Field::AlternateName),
						"amount" => Ok(Field::Amount),
						"annualPercentageRate" => Ok(Field::AnnualPercentageRate),
						"areaServed" => Ok(Field::AreaServed),
						"audience" => Ok(Field::Audience),
						"availableChannel" => Ok(Field::AvailableChannel),
						"award" => Ok(Field::Award),
						"brand" => Ok(Field::Brand),
						"broker" => Ok(Field::Broker),
						"category" => Ok(Field::Category),
						"currency" => Ok(Field::Currency),
						"description" => Ok(Field::Description),
						"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						"domiciledMortgage" => Ok(Field::DomiciledMortgage),
						"feesAndCommissionsSpecification" => {
							Ok(Field::FeesAndCommissionsSpecification)
						}
						"gracePeriod" => Ok(Field::GracePeriod),
						"hasOfferCatalog" => Ok(Field::HasOfferCatalog),
						"hoursAvailable" => Ok(Field::HoursAvailable),
						"identifier" => Ok(Field::Identifier),
						"image" => Ok(Field::Image),
						"interestRate" => Ok(Field::InterestRate),
						"isRelatedTo" => Ok(Field::IsRelatedTo),
						"isSimilarTo" => Ok(Field::IsSimilarTo),
						"loanMortgageMandateAmount" => Ok(Field::LoanMortgageMandateAmount),
						"loanRepaymentForm" => Ok(Field::LoanRepaymentForm),
						"loanTerm" => Ok(Field::LoanTerm),
						"loanType" => Ok(Field::LoanType),
						"logo" => Ok(Field::Logo),
						"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						"name" => Ok(Field::Name),
						"offers" => Ok(Field::Offers),
						"potentialAction" => Ok(Field::PotentialAction),
						"produces" => Ok(Field::Produces),
						"provider" => Ok(Field::Provider),
						"providerMobility" => Ok(Field::ProviderMobility),
						"recourseLoan" => Ok(Field::RecourseLoan),
						"renegotiableLoan" => Ok(Field::RenegotiableLoan),
						"requiredCollateral" => Ok(Field::RequiredCollateral),
						"review" => Ok(Field::Review),
						"sameAs" => Ok(Field::SameAs),
						"serviceArea" => Ok(Field::ServiceArea),
						"serviceAudience" => Ok(Field::ServiceAudience),
						"serviceOutput" => Ok(Field::ServiceOutput),
						"serviceType" => Ok(Field::ServiceType),
						"slogan" => Ok(Field::Slogan),
						"subjectOf" => Ok(Field::SubjectOf),
						"termsOfService" => Ok(Field::TermsOfService),
						"url" => Ok(Field::Url),
						_ => Ok(Field::Ignore),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"additionalType" => Ok(Field::AdditionalType),
						b"aggregateRating" => Ok(Field::AggregateRating),
						b"alternateName" => Ok(Field::AlternateName),
						b"amount" => Ok(Field::Amount),
						b"annualPercentageRate" => Ok(Field::AnnualPercentageRate),
						b"areaServed" => Ok(Field::AreaServed),
						b"audience" => Ok(Field::Audience),
						b"availableChannel" => Ok(Field::AvailableChannel),
						b"award" => Ok(Field::Award),
						b"brand" => Ok(Field::Brand),
						b"broker" => Ok(Field::Broker),
						b"category" => Ok(Field::Category),
						b"currency" => Ok(Field::Currency),
						b"description" => Ok(Field::Description),
						b"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						b"domiciledMortgage" => Ok(Field::DomiciledMortgage),
						b"feesAndCommissionsSpecification" => {
							Ok(Field::FeesAndCommissionsSpecification)
						}
						b"gracePeriod" => Ok(Field::GracePeriod),
						b"hasOfferCatalog" => Ok(Field::HasOfferCatalog),
						b"hoursAvailable" => Ok(Field::HoursAvailable),
						b"identifier" => Ok(Field::Identifier),
						b"image" => Ok(Field::Image),
						b"interestRate" => Ok(Field::InterestRate),
						b"isRelatedTo" => Ok(Field::IsRelatedTo),
						b"isSimilarTo" => Ok(Field::IsSimilarTo),
						b"loanMortgageMandateAmount" => Ok(Field::LoanMortgageMandateAmount),
						b"loanRepaymentForm" => Ok(Field::LoanRepaymentForm),
						b"loanTerm" => Ok(Field::LoanTerm),
						b"loanType" => Ok(Field::LoanType),
						b"logo" => Ok(Field::Logo),
						b"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						b"name" => Ok(Field::Name),
						b"offers" => Ok(Field::Offers),
						b"potentialAction" => Ok(Field::PotentialAction),
						b"produces" => Ok(Field::Produces),
						b"provider" => Ok(Field::Provider),
						b"providerMobility" => Ok(Field::ProviderMobility),
						b"recourseLoan" => Ok(Field::RecourseLoan),
						b"renegotiableLoan" => Ok(Field::RenegotiableLoan),
						b"requiredCollateral" => Ok(Field::RequiredCollateral),
						b"review" => Ok(Field::Review),
						b"sameAs" => Ok(Field::SameAs),
						b"serviceArea" => Ok(Field::ServiceArea),
						b"serviceAudience" => Ok(Field::ServiceAudience),
						b"serviceOutput" => Ok(Field::ServiceOutput),
						b"serviceType" => Ok(Field::ServiceType),
						b"slogan" => Ok(Field::Slogan),
						b"subjectOf" => Ok(Field::SubjectOf),
						b"termsOfService" => Ok(Field::TermsOfService),
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
				type Value = MortgageLoan;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MortgageLoan")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#additional_type_property = None;
					let mut r#aggregate_rating_property = None;
					let mut r#alternate_name_property = None;
					let mut r#amount_property = None;
					let mut r#annual_percentage_rate_property = None;
					let mut r#area_served_property = None;
					let mut r#audience_property = None;
					let mut r#available_channel_property = None;
					let mut r#award_property = None;
					let mut r#brand_property = None;
					let mut r#broker_property = None;
					let mut r#category_property = None;
					let mut r#currency_property = None;
					let mut r#description_property = None;
					let mut r#disambiguating_description_property = None;
					let mut r#domiciled_mortgage_property = None;
					let mut r#fees_and_commissions_specification_property = None;
					let mut r#grace_period_property = None;
					let mut r#has_offer_catalog_property = None;
					let mut r#hours_available_property = None;
					let mut r#identifier_property = None;
					let mut r#image_property = None;
					let mut r#interest_rate_property = None;
					let mut r#is_related_to_property = None;
					let mut r#is_similar_to_property = None;
					let mut r#loan_mortgage_mandate_amount_property = None;
					let mut r#loan_repayment_form_property = None;
					let mut r#loan_term_property = None;
					let mut r#loan_type_property = None;
					let mut r#logo_property = None;
					let mut r#main_entity_of_page_property = None;
					let mut r#name_property = None;
					let mut r#offers_property = None;
					let mut r#potential_action_property = None;
					let mut r#produces_property = None;
					let mut r#provider_property = None;
					let mut r#provider_mobility_property = None;
					let mut r#recourse_loan_property = None;
					let mut r#renegotiable_loan_property = None;
					let mut r#required_collateral_property = None;
					let mut r#review_property = None;
					let mut r#same_as_property = None;
					let mut r#service_area_property = None;
					let mut r#service_audience_property = None;
					let mut r#service_output_property = None;
					let mut r#service_type_property = None;
					let mut r#slogan_property = None;
					let mut r#subject_of_property = None;
					let mut r#terms_of_service_property = None;
					let mut r#url_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
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
							Field::Amount => {
								if r#amount_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("amount"));
								}
								r#amount_property = Some({
									struct DeserializeWith(Vec<AmountProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::AnnualPercentageRate => {
								if r#annual_percentage_rate_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"annualPercentageRate",
									));
								}
								r#annual_percentage_rate_property = Some({
									struct DeserializeWith(Vec<AnnualPercentageRateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::AvailableChannel => {
								if r#available_channel_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"availableChannel",
									));
								}
								r#available_channel_property = Some({
									struct DeserializeWith(Vec<AvailableChannelProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::Currency => {
								if r#currency_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"currency",
									));
								}
								r#currency_property = Some({
									struct DeserializeWith(Vec<CurrencyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::DomiciledMortgage => {
								if r#domiciled_mortgage_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"domiciledMortgage",
									));
								}
								r#domiciled_mortgage_property = Some({
									struct DeserializeWith(Vec<DomiciledMortgageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::FeesAndCommissionsSpecification => {
								if r#fees_and_commissions_specification_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"feesAndCommissionsSpecification",
									));
								}
								r#fees_and_commissions_specification_property = Some({
									struct DeserializeWith(
										Vec<FeesAndCommissionsSpecificationProperty>,
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
							Field::GracePeriod => {
								if r#grace_period_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"gracePeriod",
									));
								}
								r#grace_period_property = Some({
									struct DeserializeWith(Vec<GracePeriodProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::HasOfferCatalog => {
								if r#has_offer_catalog_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasOfferCatalog",
									));
								}
								r#has_offer_catalog_property = Some({
									struct DeserializeWith(Vec<HasOfferCatalogProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::HoursAvailable => {
								if r#hours_available_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hoursAvailable",
									));
								}
								r#hours_available_property = Some({
									struct DeserializeWith(Vec<HoursAvailableProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::InterestRate => {
								if r#interest_rate_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"interestRate",
									));
								}
								r#interest_rate_property = Some({
									struct DeserializeWith(Vec<InterestRateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::LoanMortgageMandateAmount => {
								if r#loan_mortgage_mandate_amount_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"loanMortgageMandateAmount",
									));
								}
								r#loan_mortgage_mandate_amount_property = Some({
									struct DeserializeWith(Vec<LoanMortgageMandateAmountProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::LoanRepaymentForm => {
								if r#loan_repayment_form_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"loanRepaymentForm",
									));
								}
								r#loan_repayment_form_property = Some({
									struct DeserializeWith(Vec<LoanRepaymentFormProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::LoanTerm => {
								if r#loan_term_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"loanTerm",
									));
								}
								r#loan_term_property = Some({
									struct DeserializeWith(Vec<LoanTermProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::LoanType => {
								if r#loan_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"loanType",
									));
								}
								r#loan_type_property = Some({
									struct DeserializeWith(Vec<LoanTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::Produces => {
								if r#produces_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"produces",
									));
								}
								r#produces_property = Some({
									struct DeserializeWith(Vec<ProducesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::ProviderMobility => {
								if r#provider_mobility_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"providerMobility",
									));
								}
								r#provider_mobility_property = Some({
									struct DeserializeWith(Vec<ProviderMobilityProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::RecourseLoan => {
								if r#recourse_loan_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"recourseLoan",
									));
								}
								r#recourse_loan_property = Some({
									struct DeserializeWith(Vec<RecourseLoanProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::RenegotiableLoan => {
								if r#renegotiable_loan_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"renegotiableLoan",
									));
								}
								r#renegotiable_loan_property = Some({
									struct DeserializeWith(Vec<RenegotiableLoanProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::RequiredCollateral => {
								if r#required_collateral_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"requiredCollateral",
									));
								}
								r#required_collateral_property = Some({
									struct DeserializeWith(Vec<RequiredCollateralProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::ServiceArea => {
								if r#service_area_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"serviceArea",
									));
								}
								r#service_area_property = Some({
									struct DeserializeWith(Vec<ServiceAreaProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ServiceAudience => {
								if r#service_audience_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"serviceAudience",
									));
								}
								r#service_audience_property = Some({
									struct DeserializeWith(Vec<ServiceAudienceProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ServiceOutput => {
								if r#service_output_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"serviceOutput",
									));
								}
								r#service_output_property = Some({
									struct DeserializeWith(Vec<ServiceOutputProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ServiceType => {
								if r#service_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"serviceType",
									));
								}
								r#service_type_property = Some({
									struct DeserializeWith(Vec<ServiceTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::TermsOfService => {
								if r#terms_of_service_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"termsOfService",
									));
								}
								r#terms_of_service_property = Some({
									struct DeserializeWith(Vec<TermsOfServiceProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
					Ok(MortgageLoan {
						r#additional_type: r#additional_type_property.unwrap_or_default(),
						r#aggregate_rating: r#aggregate_rating_property.unwrap_or_default(),
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
						r#amount: r#amount_property.unwrap_or_default(),
						r#annual_percentage_rate: r#annual_percentage_rate_property
							.unwrap_or_default(),
						r#area_served: r#area_served_property.unwrap_or_default(),
						r#audience: r#audience_property.unwrap_or_default(),
						r#available_channel: r#available_channel_property.unwrap_or_default(),
						r#award: r#award_property.unwrap_or_default(),
						r#brand: r#brand_property.unwrap_or_default(),
						r#broker: r#broker_property.unwrap_or_default(),
						r#category: r#category_property.unwrap_or_default(),
						r#currency: r#currency_property.unwrap_or_default(),
						r#description: r#description_property.unwrap_or_default(),
						r#disambiguating_description: r#disambiguating_description_property
							.unwrap_or_default(),
						r#domiciled_mortgage: r#domiciled_mortgage_property.unwrap_or_default(),
						r#fees_and_commissions_specification:
							r#fees_and_commissions_specification_property.unwrap_or_default(),
						r#grace_period: r#grace_period_property.unwrap_or_default(),
						r#has_offer_catalog: r#has_offer_catalog_property.unwrap_or_default(),
						r#hours_available: r#hours_available_property.unwrap_or_default(),
						r#identifier: r#identifier_property.unwrap_or_default(),
						r#image: r#image_property.unwrap_or_default(),
						r#interest_rate: r#interest_rate_property.unwrap_or_default(),
						r#is_related_to: r#is_related_to_property.unwrap_or_default(),
						r#is_similar_to: r#is_similar_to_property.unwrap_or_default(),
						r#loan_mortgage_mandate_amount: r#loan_mortgage_mandate_amount_property
							.unwrap_or_default(),
						r#loan_repayment_form: r#loan_repayment_form_property.unwrap_or_default(),
						r#loan_term: r#loan_term_property.unwrap_or_default(),
						r#loan_type: r#loan_type_property.unwrap_or_default(),
						r#logo: r#logo_property.unwrap_or_default(),
						r#main_entity_of_page: r#main_entity_of_page_property.unwrap_or_default(),
						r#name: r#name_property.unwrap_or_default(),
						r#offers: r#offers_property.unwrap_or_default(),
						r#potential_action: r#potential_action_property.unwrap_or_default(),
						r#produces: r#produces_property.unwrap_or_default(),
						r#provider: r#provider_property.unwrap_or_default(),
						r#provider_mobility: r#provider_mobility_property.unwrap_or_default(),
						r#recourse_loan: r#recourse_loan_property.unwrap_or_default(),
						r#renegotiable_loan: r#renegotiable_loan_property.unwrap_or_default(),
						r#required_collateral: r#required_collateral_property.unwrap_or_default(),
						r#review: r#review_property.unwrap_or_default(),
						r#same_as: r#same_as_property.unwrap_or_default(),
						r#service_area: r#service_area_property.unwrap_or_default(),
						r#service_audience: r#service_audience_property.unwrap_or_default(),
						r#service_output: r#service_output_property.unwrap_or_default(),
						r#service_type: r#service_type_property.unwrap_or_default(),
						r#slogan: r#slogan_property.unwrap_or_default(),
						r#subject_of: r#subject_of_property.unwrap_or_default(),
						r#terms_of_service: r#terms_of_service_property.unwrap_or_default(),
						r#url: r#url_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				"additionalType",
				"aggregateRating",
				"alternateName",
				"amount",
				"annualPercentageRate",
				"areaServed",
				"audience",
				"availableChannel",
				"award",
				"brand",
				"broker",
				"category",
				"currency",
				"description",
				"disambiguatingDescription",
				"domiciledMortgage",
				"feesAndCommissionsSpecification",
				"gracePeriod",
				"hasOfferCatalog",
				"hoursAvailable",
				"identifier",
				"image",
				"interestRate",
				"isRelatedTo",
				"isSimilarTo",
				"loanMortgageMandateAmount",
				"loanRepaymentForm",
				"loanTerm",
				"loanType",
				"logo",
				"mainEntityOfPage",
				"name",
				"offers",
				"potentialAction",
				"produces",
				"provider",
				"providerMobility",
				"recourseLoan",
				"renegotiableLoan",
				"requiredCollateral",
				"review",
				"sameAs",
				"serviceArea",
				"serviceAudience",
				"serviceOutput",
				"serviceType",
				"slogan",
				"subjectOf",
				"termsOfService",
				"url",
			];
			deserializer.deserialize_struct("MortgageLoan", FIELDS, ClassVisitor)
		}
	}
}
