use super::*;
/// <https://schema.org/InvestmentOrDeposit>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct InvestmentOrDeposit {
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
			feature = "aggregate-rating-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#aggregate_rating: Vec<AggregateRatingProperty>,
	#[cfg(any(
		any(
			feature = "alternate-name-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#alternate_name: Vec<AlternateNameProperty>,
	#[cfg(any(
		any(feature = "amount-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#amount: Vec<AmountProperty>,
	#[cfg(any(
		any(
			feature = "annual-percentage-rate-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#annual_percentage_rate: Vec<AnnualPercentageRateProperty>,
	#[cfg(any(
		any(
			feature = "area-served-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#area_served: Vec<AreaServedProperty>,
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
			feature = "available-channel-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#available_channel: Vec<AvailableChannelProperty>,
	#[cfg(any(
		any(feature = "award-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#award: Vec<AwardProperty>,
	#[cfg(any(
		any(feature = "brand-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#brand: Vec<BrandProperty>,
	#[cfg(any(
		any(feature = "broker-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#broker: Vec<BrokerProperty>,
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
			feature = "fees-and-commissions-specification-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#fees_and_commissions_specification: Vec<FeesAndCommissionsSpecificationProperty>,
	#[cfg(any(
		any(
			feature = "has-offer-catalog-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#has_offer_catalog: Vec<HasOfferCatalogProperty>,
	#[cfg(any(
		any(
			feature = "hours-available-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#hours_available: Vec<HoursAvailableProperty>,
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
			feature = "interest-rate-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#interest_rate: Vec<InterestRateProperty>,
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
		any(feature = "name-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#name: Vec<NameProperty>,
	#[cfg(any(
		any(feature = "offers-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#offers: Vec<OffersProperty>,
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
			feature = "produces-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#produces: Vec<ProducesProperty>,
	#[cfg(any(
		any(
			feature = "provider-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#provider: Vec<ProviderProperty>,
	#[cfg(any(
		any(
			feature = "provider-mobility-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#provider_mobility: Vec<ProviderMobilityProperty>,
	#[cfg(any(
		any(feature = "review-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#review: Vec<ReviewProperty>,
	#[cfg(any(
		any(
			feature = "same-as-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#same_as: Vec<SameAsProperty>,
	#[cfg(any(
		any(
			feature = "service-area-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#service_area: Vec<ServiceAreaProperty>,
	#[cfg(any(
		any(
			feature = "service-audience-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#service_audience: Vec<ServiceAudienceProperty>,
	#[cfg(any(
		any(
			feature = "service-output-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#service_output: Vec<ServiceOutputProperty>,
	#[cfg(any(
		any(
			feature = "service-type-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#service_type: Vec<ServiceTypeProperty>,
	#[cfg(any(
		any(feature = "slogan-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#slogan: Vec<SloganProperty>,
	#[cfg(any(
		any(
			feature = "subject-of-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#subject_of: Vec<SubjectOfProperty>,
	#[cfg(any(
		any(
			feature = "terms-of-service-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#terms_of_service: Vec<TermsOfServiceProperty>,
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
	impl Serialize for InvestmentOrDeposit {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
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
						feature = "amount-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#amount) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "annual-percentage-rate-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#annual_percentage_rate) as usize
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
						feature = "available-channel-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#available_channel) as usize
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
						feature = "fees-and-commissions-specification-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#fees_and_commissions_specification) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-offer-catalog-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_offer_catalog) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "hours-available-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#hours_available) as usize
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
						feature = "interest-rate-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#interest_rate) as usize
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
						feature = "produces-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#produces) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "provider-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#provider) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "provider-mobility-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#provider_mobility) as usize
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
						feature = "service-area-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#service_area) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "service-audience-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#service_audience) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "service-output-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#service_output) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "service-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#service_type) as usize
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
						feature = "terms-of-service-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#terms_of_service) as usize
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
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "InvestmentOrDeposit", len)?;
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
				any(feature = "amount-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "annual-percentage-rate-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "available-channel-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "fees-and-commissions-specification-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "has-offer-catalog-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "hours-available-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "interest-rate-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "produces-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "provider-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "provider-mobility-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
				any(
					feature = "service-area-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "service-audience-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "service-output-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "service-type-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
				any(
					feature = "terms-of-service-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
	impl<'de> Deserialize<'de> for InvestmentOrDeposit {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
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
						feature = "aggregate-rating-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AggregateRating,
				#[cfg(any(
					any(
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AlternateName,
				#[cfg(any(
					any(feature = "amount-property-schema", feature = "general-schema-section"),
					doc
				))]
				Amount,
				#[cfg(any(
					any(
						feature = "annual-percentage-rate-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AnnualPercentageRate,
				#[cfg(any(
					any(
						feature = "area-served-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AreaServed,
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
						feature = "available-channel-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AvailableChannel,
				#[cfg(any(
					any(feature = "award-property-schema", feature = "general-schema-section"),
					doc
				))]
				Award,
				#[cfg(any(
					any(feature = "brand-property-schema", feature = "general-schema-section"),
					doc
				))]
				Brand,
				#[cfg(any(
					any(feature = "broker-property-schema", feature = "general-schema-section"),
					doc
				))]
				Broker,
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
						feature = "fees-and-commissions-specification-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				FeesAndCommissionsSpecification,
				#[cfg(any(
					any(
						feature = "has-offer-catalog-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				HasOfferCatalog,
				#[cfg(any(
					any(
						feature = "hours-available-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				HoursAvailable,
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
						feature = "interest-rate-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				InterestRate,
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
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				Name,
				#[cfg(any(
					any(feature = "offers-property-schema", feature = "general-schema-section"),
					doc
				))]
				Offers,
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
						feature = "produces-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Produces,
				#[cfg(any(
					any(
						feature = "provider-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Provider,
				#[cfg(any(
					any(
						feature = "provider-mobility-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ProviderMobility,
				#[cfg(any(
					any(feature = "review-property-schema", feature = "general-schema-section"),
					doc
				))]
				Review,
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SameAs,
				#[cfg(any(
					any(
						feature = "service-area-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ServiceArea,
				#[cfg(any(
					any(
						feature = "service-audience-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ServiceAudience,
				#[cfg(any(
					any(
						feature = "service-output-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ServiceOutput,
				#[cfg(any(
					any(
						feature = "service-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ServiceType,
				#[cfg(any(
					any(feature = "slogan-property-schema", feature = "general-schema-section"),
					doc
				))]
				Slogan,
				#[cfg(any(
					any(
						feature = "subject-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SubjectOf,
				#[cfg(any(
					any(
						feature = "terms-of-service-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				TermsOfService,
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
								feature = "additional-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"additionalType" => Ok(Field::AdditionalType),
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
								feature = "alternate-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"alternateName" => Ok(Field::AlternateName),
						#[cfg(any(
							any(
								feature = "amount-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"amount" => Ok(Field::Amount),
						#[cfg(any(
							any(
								feature = "annual-percentage-rate-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"annualPercentageRate" => Ok(Field::AnnualPercentageRate),
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
								feature = "audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"audience" => Ok(Field::Audience),
						#[cfg(any(
							any(
								feature = "available-channel-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"availableChannel" => Ok(Field::AvailableChannel),
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
								feature = "brand-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"brand" => Ok(Field::Brand),
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
								feature = "category-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"category" => Ok(Field::Category),
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
								feature = "fees-and-commissions-specification-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"feesAndCommissionsSpecification" => Ok(Field::FeesAndCommissionsSpecification),
						#[cfg(any(
							any(
								feature = "has-offer-catalog-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"hasOfferCatalog" => Ok(Field::HasOfferCatalog),
						#[cfg(any(
							any(
								feature = "hours-available-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"hoursAvailable" => Ok(Field::HoursAvailable),
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
								feature = "interest-rate-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"interestRate" => Ok(Field::InterestRate),
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
								feature = "name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"name" => Ok(Field::Name),
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
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"potentialAction" => Ok(Field::PotentialAction),
						#[cfg(any(
							any(
								feature = "produces-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"produces" => Ok(Field::Produces),
						#[cfg(any(
							any(
								feature = "provider-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"provider" => Ok(Field::Provider),
						#[cfg(any(
							any(
								feature = "provider-mobility-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"providerMobility" => Ok(Field::ProviderMobility),
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
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"sameAs" => Ok(Field::SameAs),
						#[cfg(any(
							any(
								feature = "service-area-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"serviceArea" => Ok(Field::ServiceArea),
						#[cfg(any(
							any(
								feature = "service-audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"serviceAudience" => Ok(Field::ServiceAudience),
						#[cfg(any(
							any(
								feature = "service-output-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"serviceOutput" => Ok(Field::ServiceOutput),
						#[cfg(any(
							any(
								feature = "service-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"serviceType" => Ok(Field::ServiceType),
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
								feature = "subject-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"subjectOf" => Ok(Field::SubjectOf),
						#[cfg(any(
							any(
								feature = "terms-of-service-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"termsOfService" => Ok(Field::TermsOfService),
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
								feature = "additional-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"additionalType" => Ok(Field::AdditionalType),
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
								feature = "alternate-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"alternateName" => Ok(Field::AlternateName),
						#[cfg(any(
							any(
								feature = "amount-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"amount" => Ok(Field::Amount),
						#[cfg(any(
							any(
								feature = "annual-percentage-rate-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"annualPercentageRate" => Ok(Field::AnnualPercentageRate),
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
								feature = "audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"audience" => Ok(Field::Audience),
						#[cfg(any(
							any(
								feature = "available-channel-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"availableChannel" => Ok(Field::AvailableChannel),
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
								feature = "brand-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"brand" => Ok(Field::Brand),
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
								feature = "category-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"category" => Ok(Field::Category),
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
								feature = "fees-and-commissions-specification-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"feesAndCommissionsSpecification" => Ok(Field::FeesAndCommissionsSpecification),
						#[cfg(any(
							any(
								feature = "has-offer-catalog-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"hasOfferCatalog" => Ok(Field::HasOfferCatalog),
						#[cfg(any(
							any(
								feature = "hours-available-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"hoursAvailable" => Ok(Field::HoursAvailable),
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
								feature = "interest-rate-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"interestRate" => Ok(Field::InterestRate),
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
								feature = "name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"name" => Ok(Field::Name),
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
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"potentialAction" => Ok(Field::PotentialAction),
						#[cfg(any(
							any(
								feature = "produces-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"produces" => Ok(Field::Produces),
						#[cfg(any(
							any(
								feature = "provider-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"provider" => Ok(Field::Provider),
						#[cfg(any(
							any(
								feature = "provider-mobility-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"providerMobility" => Ok(Field::ProviderMobility),
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
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"sameAs" => Ok(Field::SameAs),
						#[cfg(any(
							any(
								feature = "service-area-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"serviceArea" => Ok(Field::ServiceArea),
						#[cfg(any(
							any(
								feature = "service-audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"serviceAudience" => Ok(Field::ServiceAudience),
						#[cfg(any(
							any(
								feature = "service-output-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"serviceOutput" => Ok(Field::ServiceOutput),
						#[cfg(any(
							any(
								feature = "service-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"serviceType" => Ok(Field::ServiceType),
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
								feature = "subject-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"subjectOf" => Ok(Field::SubjectOf),
						#[cfg(any(
							any(
								feature = "terms-of-service-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"termsOfService" => Ok(Field::TermsOfService),
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
				type Value = InvestmentOrDeposit;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema InvestmentOrDeposit")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
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
							feature = "aggregate-rating-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#aggregate_rating_property = None;
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
							feature = "amount-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#amount_property = None;
					#[cfg(any(
						any(
							feature = "annual-percentage-rate-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#annual_percentage_rate_property = None;
					#[cfg(any(
						any(
							feature = "area-served-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#area_served_property = None;
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
							feature = "available-channel-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#available_channel_property = None;
					#[cfg(any(
						any(feature = "award-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#award_property = None;
					#[cfg(any(
						any(feature = "brand-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#brand_property = None;
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
							feature = "category-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#category_property = None;
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
							feature = "fees-and-commissions-specification-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#fees_and_commissions_specification_property = None;
					#[cfg(any(
						any(
							feature = "has-offer-catalog-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#has_offer_catalog_property = None;
					#[cfg(any(
						any(
							feature = "hours-available-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#hours_available_property = None;
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
							feature = "interest-rate-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#interest_rate_property = None;
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
						any(feature = "name-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#name_property = None;
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
							feature = "potential-action-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#potential_action_property = None;
					#[cfg(any(
						any(
							feature = "produces-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#produces_property = None;
					#[cfg(any(
						any(
							feature = "provider-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#provider_property = None;
					#[cfg(any(
						any(
							feature = "provider-mobility-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#provider_mobility_property = None;
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
							feature = "same-as-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#same_as_property = None;
					#[cfg(any(
						any(
							feature = "service-area-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#service_area_property = None;
					#[cfg(any(
						any(
							feature = "service-audience-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#service_audience_property = None;
					#[cfg(any(
						any(
							feature = "service-output-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#service_output_property = None;
					#[cfg(any(
						any(
							feature = "service-type-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#service_type_property = None;
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
							feature = "subject-of-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#subject_of_property = None;
					#[cfg(any(
						any(
							feature = "terms-of-service-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#terms_of_service_property = None;
					#[cfg(any(
						any(feature = "url-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#url_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
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
									feature = "amount-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "annual-percentage-rate-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "available-channel-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "fees-and-commissions-specification-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "has-offer-catalog-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "hours-available-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "interest-rate-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "produces-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "provider-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "provider-mobility-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "service-area-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "service-audience-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "service-output-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "service-type-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "terms-of-service-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
					Ok(InvestmentOrDeposit {
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
								feature = "aggregate-rating-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#aggregate_rating: r#aggregate_rating_property.unwrap_or_default(),
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
								feature = "amount-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#amount: r#amount_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "annual-percentage-rate-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#annual_percentage_rate: r#annual_percentage_rate_property
							.unwrap_or_default(),
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
								feature = "audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#audience: r#audience_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "available-channel-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#available_channel: r#available_channel_property.unwrap_or_default(),
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
								feature = "brand-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#brand: r#brand_property.unwrap_or_default(),
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
								feature = "category-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#category: r#category_property.unwrap_or_default(),
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
								feature = "fees-and-commissions-specification-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#fees_and_commissions_specification:
							r#fees_and_commissions_specification_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-offer-catalog-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#has_offer_catalog: r#has_offer_catalog_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "hours-available-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#hours_available: r#hours_available_property.unwrap_or_default(),
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
								feature = "interest-rate-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#interest_rate: r#interest_rate_property.unwrap_or_default(),
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
								feature = "name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#name: r#name_property.unwrap_or_default(),
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
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#potential_action: r#potential_action_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "produces-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#produces: r#produces_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "provider-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#provider: r#provider_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "provider-mobility-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#provider_mobility: r#provider_mobility_property.unwrap_or_default(),
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
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#same_as: r#same_as_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "service-area-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#service_area: r#service_area_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "service-audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#service_audience: r#service_audience_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "service-output-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#service_output: r#service_output_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "service-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#service_type: r#service_type_property.unwrap_or_default(),
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
								feature = "subject-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#subject_of: r#subject_of_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "terms-of-service-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#terms_of_service: r#terms_of_service_property.unwrap_or_default(),
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
						feature = "additional-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"additionalType",
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
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"alternateName",
				#[cfg(any(
					any(feature = "amount-property-schema", feature = "general-schema-section"),
					doc
				))]
				"amount",
				#[cfg(any(
					any(
						feature = "annual-percentage-rate-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"annualPercentageRate",
				#[cfg(any(
					any(
						feature = "area-served-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"areaServed",
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
						feature = "available-channel-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"availableChannel",
				#[cfg(any(
					any(feature = "award-property-schema", feature = "general-schema-section"),
					doc
				))]
				"award",
				#[cfg(any(
					any(feature = "brand-property-schema", feature = "general-schema-section"),
					doc
				))]
				"brand",
				#[cfg(any(
					any(feature = "broker-property-schema", feature = "general-schema-section"),
					doc
				))]
				"broker",
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
						feature = "fees-and-commissions-specification-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"feesAndCommissionsSpecification",
				#[cfg(any(
					any(
						feature = "has-offer-catalog-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"hasOfferCatalog",
				#[cfg(any(
					any(
						feature = "hours-available-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"hoursAvailable",
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
						feature = "interest-rate-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"interestRate",
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
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				"name",
				#[cfg(any(
					any(feature = "offers-property-schema", feature = "general-schema-section"),
					doc
				))]
				"offers",
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
						feature = "produces-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"produces",
				#[cfg(any(
					any(
						feature = "provider-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"provider",
				#[cfg(any(
					any(
						feature = "provider-mobility-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"providerMobility",
				#[cfg(any(
					any(feature = "review-property-schema", feature = "general-schema-section"),
					doc
				))]
				"review",
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"sameAs",
				#[cfg(any(
					any(
						feature = "service-area-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"serviceArea",
				#[cfg(any(
					any(
						feature = "service-audience-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"serviceAudience",
				#[cfg(any(
					any(
						feature = "service-output-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"serviceOutput",
				#[cfg(any(
					any(
						feature = "service-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"serviceType",
				#[cfg(any(
					any(feature = "slogan-property-schema", feature = "general-schema-section"),
					doc
				))]
				"slogan",
				#[cfg(any(
					any(
						feature = "subject-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"subjectOf",
				#[cfg(any(
					any(
						feature = "terms-of-service-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"termsOfService",
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				"url",
			];
			deserializer.deserialize_struct("InvestmentOrDeposit", FIELDS, ClassVisitor)
		}
	}
}
