use super::*;
/// <https://schema.org/BroadcastService>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct BroadcastService {
	pub r#area: Vec<AreaProperty>,
	pub r#broadcast_affiliate_of: Vec<BroadcastAffiliateOfProperty>,
	pub r#broadcast_display_name: Vec<BroadcastDisplayNameProperty>,
	pub r#broadcast_frequency: Vec<BroadcastFrequencyProperty>,
	pub r#broadcast_timezone: Vec<BroadcastTimezoneProperty>,
	pub r#broadcaster: Vec<BroadcasterProperty>,
	pub r#call_sign: Vec<CallSignProperty>,
	pub r#has_broadcast_channel: Vec<HasBroadcastChannelProperty>,
	pub r#in_language: Vec<InLanguageProperty>,
	pub r#parent_service: Vec<ParentServiceProperty>,
	pub r#video_format: Vec<VideoFormatProperty>,
	pub r#aggregate_rating: Vec<AggregateRatingProperty>,
	pub r#area_served: Vec<AreaServedProperty>,
	pub r#audience: Vec<AudienceProperty>,
	pub r#available_channel: Vec<AvailableChannelProperty>,
	pub r#award: Vec<AwardProperty>,
	pub r#brand: Vec<BrandProperty>,
	pub r#broker: Vec<BrokerProperty>,
	pub r#category: Vec<CategoryProperty>,
	pub r#has_offer_catalog: Vec<HasOfferCatalogProperty>,
	pub r#hours_available: Vec<HoursAvailableProperty>,
	pub r#is_related_to: Vec<IsRelatedToProperty>,
	pub r#is_similar_to: Vec<IsSimilarToProperty>,
	pub r#logo: Vec<LogoProperty>,
	pub r#offers: Vec<OffersProperty>,
	pub r#produces: Vec<ProducesProperty>,
	pub r#provider: Vec<ProviderProperty>,
	pub r#provider_mobility: Vec<ProviderMobilityProperty>,
	pub r#review: Vec<ReviewProperty>,
	pub r#service_area: Vec<ServiceAreaProperty>,
	pub r#service_audience: Vec<ServiceAudienceProperty>,
	pub r#service_output: Vec<ServiceOutputProperty>,
	pub r#service_type: Vec<ServiceTypeProperty>,
	pub r#slogan: Vec<SloganProperty>,
	pub r#terms_of_service: Vec<TermsOfServiceProperty>,
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	pub r#alternate_name: Vec<AlternateNameProperty>,
	pub r#description: Vec<DescriptionProperty>,
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	pub r#identifier: Vec<IdentifierProperty>,
	pub r#image: Vec<ImageProperty>,
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	pub r#name: Vec<NameProperty>,
	pub r#potential_action: Vec<PotentialActionProperty>,
	pub r#same_as: Vec<SameAsProperty>,
	pub r#subject_of: Vec<SubjectOfProperty>,
	pub r#url: Vec<UrlProperty>,
}
pub trait BroadcastServiceTrait {
	fn get_area(&self) -> &[AreaProperty];
	fn take_area(&mut self) -> Vec<AreaProperty>;
	fn get_broadcast_affiliate_of(&self) -> &[BroadcastAffiliateOfProperty];
	fn take_broadcast_affiliate_of(&mut self) -> Vec<BroadcastAffiliateOfProperty>;
	fn get_broadcast_display_name(&self) -> &[BroadcastDisplayNameProperty];
	fn take_broadcast_display_name(&mut self) -> Vec<BroadcastDisplayNameProperty>;
	fn get_broadcast_frequency(&self) -> &[BroadcastFrequencyProperty];
	fn take_broadcast_frequency(&mut self) -> Vec<BroadcastFrequencyProperty>;
	fn get_broadcast_timezone(&self) -> &[BroadcastTimezoneProperty];
	fn take_broadcast_timezone(&mut self) -> Vec<BroadcastTimezoneProperty>;
	fn get_broadcaster(&self) -> &[BroadcasterProperty];
	fn take_broadcaster(&mut self) -> Vec<BroadcasterProperty>;
	fn get_call_sign(&self) -> &[CallSignProperty];
	fn take_call_sign(&mut self) -> Vec<CallSignProperty>;
	fn get_has_broadcast_channel(&self) -> &[HasBroadcastChannelProperty];
	fn take_has_broadcast_channel(&mut self) -> Vec<HasBroadcastChannelProperty>;
	fn get_in_language(&self) -> &[InLanguageProperty];
	fn take_in_language(&mut self) -> Vec<InLanguageProperty>;
	fn get_parent_service(&self) -> &[ParentServiceProperty];
	fn take_parent_service(&mut self) -> Vec<ParentServiceProperty>;
	fn get_video_format(&self) -> &[VideoFormatProperty];
	fn take_video_format(&mut self) -> Vec<VideoFormatProperty>;
}
impl BroadcastServiceTrait for BroadcastService {
	fn get_area(&self) -> &[AreaProperty] {
		self.r#area.as_slice()
	}
	fn take_area(&mut self) -> Vec<AreaProperty> {
		std::mem::take(&mut self.r#area)
	}
	fn get_broadcast_affiliate_of(&self) -> &[BroadcastAffiliateOfProperty] {
		self.r#broadcast_affiliate_of.as_slice()
	}
	fn take_broadcast_affiliate_of(&mut self) -> Vec<BroadcastAffiliateOfProperty> {
		std::mem::take(&mut self.r#broadcast_affiliate_of)
	}
	fn get_broadcast_display_name(&self) -> &[BroadcastDisplayNameProperty] {
		self.r#broadcast_display_name.as_slice()
	}
	fn take_broadcast_display_name(&mut self) -> Vec<BroadcastDisplayNameProperty> {
		std::mem::take(&mut self.r#broadcast_display_name)
	}
	fn get_broadcast_frequency(&self) -> &[BroadcastFrequencyProperty] {
		self.r#broadcast_frequency.as_slice()
	}
	fn take_broadcast_frequency(&mut self) -> Vec<BroadcastFrequencyProperty> {
		std::mem::take(&mut self.r#broadcast_frequency)
	}
	fn get_broadcast_timezone(&self) -> &[BroadcastTimezoneProperty] {
		self.r#broadcast_timezone.as_slice()
	}
	fn take_broadcast_timezone(&mut self) -> Vec<BroadcastTimezoneProperty> {
		std::mem::take(&mut self.r#broadcast_timezone)
	}
	fn get_broadcaster(&self) -> &[BroadcasterProperty] {
		self.r#broadcaster.as_slice()
	}
	fn take_broadcaster(&mut self) -> Vec<BroadcasterProperty> {
		std::mem::take(&mut self.r#broadcaster)
	}
	fn get_call_sign(&self) -> &[CallSignProperty] {
		self.r#call_sign.as_slice()
	}
	fn take_call_sign(&mut self) -> Vec<CallSignProperty> {
		std::mem::take(&mut self.r#call_sign)
	}
	fn get_has_broadcast_channel(&self) -> &[HasBroadcastChannelProperty] {
		self.r#has_broadcast_channel.as_slice()
	}
	fn take_has_broadcast_channel(&mut self) -> Vec<HasBroadcastChannelProperty> {
		std::mem::take(&mut self.r#has_broadcast_channel)
	}
	fn get_in_language(&self) -> &[InLanguageProperty] {
		self.r#in_language.as_slice()
	}
	fn take_in_language(&mut self) -> Vec<InLanguageProperty> {
		std::mem::take(&mut self.r#in_language)
	}
	fn get_parent_service(&self) -> &[ParentServiceProperty] {
		self.r#parent_service.as_slice()
	}
	fn take_parent_service(&mut self) -> Vec<ParentServiceProperty> {
		std::mem::take(&mut self.r#parent_service)
	}
	fn get_video_format(&self) -> &[VideoFormatProperty] {
		self.r#video_format.as_slice()
	}
	fn take_video_format(&mut self) -> Vec<VideoFormatProperty> {
		std::mem::take(&mut self.r#video_format)
	}
}
impl ServiceTrait for BroadcastService {
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
	fn get_audience(&self) -> &[AudienceProperty] {
		self.r#audience.as_slice()
	}
	fn take_audience(&mut self) -> Vec<AudienceProperty> {
		std::mem::take(&mut self.r#audience)
	}
	fn get_available_channel(&self) -> &[AvailableChannelProperty] {
		self.r#available_channel.as_slice()
	}
	fn take_available_channel(&mut self) -> Vec<AvailableChannelProperty> {
		std::mem::take(&mut self.r#available_channel)
	}
	fn get_award(&self) -> &[AwardProperty] {
		self.r#award.as_slice()
	}
	fn take_award(&mut self) -> Vec<AwardProperty> {
		std::mem::take(&mut self.r#award)
	}
	fn get_brand(&self) -> &[BrandProperty] {
		self.r#brand.as_slice()
	}
	fn take_brand(&mut self) -> Vec<BrandProperty> {
		std::mem::take(&mut self.r#brand)
	}
	fn get_broker(&self) -> &[BrokerProperty] {
		self.r#broker.as_slice()
	}
	fn take_broker(&mut self) -> Vec<BrokerProperty> {
		std::mem::take(&mut self.r#broker)
	}
	fn get_category(&self) -> &[CategoryProperty] {
		self.r#category.as_slice()
	}
	fn take_category(&mut self) -> Vec<CategoryProperty> {
		std::mem::take(&mut self.r#category)
	}
	fn get_has_offer_catalog(&self) -> &[HasOfferCatalogProperty] {
		self.r#has_offer_catalog.as_slice()
	}
	fn take_has_offer_catalog(&mut self) -> Vec<HasOfferCatalogProperty> {
		std::mem::take(&mut self.r#has_offer_catalog)
	}
	fn get_hours_available(&self) -> &[HoursAvailableProperty] {
		self.r#hours_available.as_slice()
	}
	fn take_hours_available(&mut self) -> Vec<HoursAvailableProperty> {
		std::mem::take(&mut self.r#hours_available)
	}
	fn get_is_related_to(&self) -> &[IsRelatedToProperty] {
		self.r#is_related_to.as_slice()
	}
	fn take_is_related_to(&mut self) -> Vec<IsRelatedToProperty> {
		std::mem::take(&mut self.r#is_related_to)
	}
	fn get_is_similar_to(&self) -> &[IsSimilarToProperty] {
		self.r#is_similar_to.as_slice()
	}
	fn take_is_similar_to(&mut self) -> Vec<IsSimilarToProperty> {
		std::mem::take(&mut self.r#is_similar_to)
	}
	fn get_logo(&self) -> &[LogoProperty] {
		self.r#logo.as_slice()
	}
	fn take_logo(&mut self) -> Vec<LogoProperty> {
		std::mem::take(&mut self.r#logo)
	}
	fn get_offers(&self) -> &[OffersProperty] {
		self.r#offers.as_slice()
	}
	fn take_offers(&mut self) -> Vec<OffersProperty> {
		std::mem::take(&mut self.r#offers)
	}
	fn get_produces(&self) -> &[ProducesProperty] {
		self.r#produces.as_slice()
	}
	fn take_produces(&mut self) -> Vec<ProducesProperty> {
		std::mem::take(&mut self.r#produces)
	}
	fn get_provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn take_provider(&mut self) -> Vec<ProviderProperty> {
		std::mem::take(&mut self.r#provider)
	}
	fn get_provider_mobility(&self) -> &[ProviderMobilityProperty] {
		self.r#provider_mobility.as_slice()
	}
	fn take_provider_mobility(&mut self) -> Vec<ProviderMobilityProperty> {
		std::mem::take(&mut self.r#provider_mobility)
	}
	fn get_review(&self) -> &[ReviewProperty] {
		self.r#review.as_slice()
	}
	fn take_review(&mut self) -> Vec<ReviewProperty> {
		std::mem::take(&mut self.r#review)
	}
	fn get_service_area(&self) -> &[ServiceAreaProperty] {
		self.r#service_area.as_slice()
	}
	fn take_service_area(&mut self) -> Vec<ServiceAreaProperty> {
		std::mem::take(&mut self.r#service_area)
	}
	fn get_service_audience(&self) -> &[ServiceAudienceProperty] {
		self.r#service_audience.as_slice()
	}
	fn take_service_audience(&mut self) -> Vec<ServiceAudienceProperty> {
		std::mem::take(&mut self.r#service_audience)
	}
	fn get_service_output(&self) -> &[ServiceOutputProperty] {
		self.r#service_output.as_slice()
	}
	fn take_service_output(&mut self) -> Vec<ServiceOutputProperty> {
		std::mem::take(&mut self.r#service_output)
	}
	fn get_service_type(&self) -> &[ServiceTypeProperty] {
		self.r#service_type.as_slice()
	}
	fn take_service_type(&mut self) -> Vec<ServiceTypeProperty> {
		std::mem::take(&mut self.r#service_type)
	}
	fn get_slogan(&self) -> &[SloganProperty] {
		self.r#slogan.as_slice()
	}
	fn take_slogan(&mut self) -> Vec<SloganProperty> {
		std::mem::take(&mut self.r#slogan)
	}
	fn get_terms_of_service(&self) -> &[TermsOfServiceProperty] {
		self.r#terms_of_service.as_slice()
	}
	fn take_terms_of_service(&mut self) -> Vec<TermsOfServiceProperty> {
		std::mem::take(&mut self.r#terms_of_service)
	}
}
impl ThingTrait for BroadcastService {
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
	impl Serialize for BroadcastService {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#area) as usize,
				!Vec::is_empty(&self.r#broadcast_affiliate_of) as usize,
				!Vec::is_empty(&self.r#broadcast_display_name) as usize,
				!Vec::is_empty(&self.r#broadcast_frequency) as usize,
				!Vec::is_empty(&self.r#broadcast_timezone) as usize,
				!Vec::is_empty(&self.r#broadcaster) as usize,
				!Vec::is_empty(&self.r#call_sign) as usize,
				!Vec::is_empty(&self.r#has_broadcast_channel) as usize,
				!Vec::is_empty(&self.r#in_language) as usize,
				!Vec::is_empty(&self.r#parent_service) as usize,
				!Vec::is_empty(&self.r#video_format) as usize,
				!Vec::is_empty(&self.r#aggregate_rating) as usize,
				!Vec::is_empty(&self.r#area_served) as usize,
				!Vec::is_empty(&self.r#audience) as usize,
				!Vec::is_empty(&self.r#available_channel) as usize,
				!Vec::is_empty(&self.r#award) as usize,
				!Vec::is_empty(&self.r#brand) as usize,
				!Vec::is_empty(&self.r#broker) as usize,
				!Vec::is_empty(&self.r#category) as usize,
				!Vec::is_empty(&self.r#has_offer_catalog) as usize,
				!Vec::is_empty(&self.r#hours_available) as usize,
				!Vec::is_empty(&self.r#is_related_to) as usize,
				!Vec::is_empty(&self.r#is_similar_to) as usize,
				!Vec::is_empty(&self.r#logo) as usize,
				!Vec::is_empty(&self.r#offers) as usize,
				!Vec::is_empty(&self.r#produces) as usize,
				!Vec::is_empty(&self.r#provider) as usize,
				!Vec::is_empty(&self.r#provider_mobility) as usize,
				!Vec::is_empty(&self.r#review) as usize,
				!Vec::is_empty(&self.r#service_area) as usize,
				!Vec::is_empty(&self.r#service_audience) as usize,
				!Vec::is_empty(&self.r#service_output) as usize,
				!Vec::is_empty(&self.r#service_type) as usize,
				!Vec::is_empty(&self.r#slogan) as usize,
				!Vec::is_empty(&self.r#terms_of_service) as usize,
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
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "BroadcastService", len)?;
			if !Vec::is_empty(&self.r#area) {
				serialize_struct.serialize_field("area", {
					struct SerializeWith<'a>(&'a Vec<AreaProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#area)
				})?;
			} else {
				serialize_struct.skip_field("area")?;
			}
			if !Vec::is_empty(&self.r#broadcast_affiliate_of) {
				serialize_struct.serialize_field("broadcastAffiliateOf", {
					struct SerializeWith<'a>(&'a Vec<BroadcastAffiliateOfProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#broadcast_affiliate_of)
				})?;
			} else {
				serialize_struct.skip_field("broadcastAffiliateOf")?;
			}
			if !Vec::is_empty(&self.r#broadcast_display_name) {
				serialize_struct.serialize_field("broadcastDisplayName", {
					struct SerializeWith<'a>(&'a Vec<BroadcastDisplayNameProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#broadcast_display_name)
				})?;
			} else {
				serialize_struct.skip_field("broadcastDisplayName")?;
			}
			if !Vec::is_empty(&self.r#broadcast_frequency) {
				serialize_struct.serialize_field("broadcastFrequency", {
					struct SerializeWith<'a>(&'a Vec<BroadcastFrequencyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#broadcast_frequency)
				})?;
			} else {
				serialize_struct.skip_field("broadcastFrequency")?;
			}
			if !Vec::is_empty(&self.r#broadcast_timezone) {
				serialize_struct.serialize_field("broadcastTimezone", {
					struct SerializeWith<'a>(&'a Vec<BroadcastTimezoneProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#broadcast_timezone)
				})?;
			} else {
				serialize_struct.skip_field("broadcastTimezone")?;
			}
			if !Vec::is_empty(&self.r#broadcaster) {
				serialize_struct.serialize_field("broadcaster", {
					struct SerializeWith<'a>(&'a Vec<BroadcasterProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#broadcaster)
				})?;
			} else {
				serialize_struct.skip_field("broadcaster")?;
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
			if !Vec::is_empty(&self.r#has_broadcast_channel) {
				serialize_struct.serialize_field("hasBroadcastChannel", {
					struct SerializeWith<'a>(&'a Vec<HasBroadcastChannelProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_broadcast_channel)
				})?;
			} else {
				serialize_struct.skip_field("hasBroadcastChannel")?;
			}
			if !Vec::is_empty(&self.r#in_language) {
				serialize_struct.serialize_field("inLanguage", {
					struct SerializeWith<'a>(&'a Vec<InLanguageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#in_language)
				})?;
			} else {
				serialize_struct.skip_field("inLanguage")?;
			}
			if !Vec::is_empty(&self.r#parent_service) {
				serialize_struct.serialize_field("parentService", {
					struct SerializeWith<'a>(&'a Vec<ParentServiceProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#parent_service)
				})?;
			} else {
				serialize_struct.skip_field("parentService")?;
			}
			if !Vec::is_empty(&self.r#video_format) {
				serialize_struct.serialize_field("videoFormat", {
					struct SerializeWith<'a>(&'a Vec<VideoFormatProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#video_format)
				})?;
			} else {
				serialize_struct.skip_field("videoFormat")?;
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
	impl<'de> Deserialize<'de> for BroadcastService {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				Area,
				BroadcastAffiliateOf,
				BroadcastDisplayName,
				BroadcastFrequency,
				BroadcastTimezone,
				Broadcaster,
				CallSign,
				HasBroadcastChannel,
				InLanguage,
				ParentService,
				VideoFormat,
				AggregateRating,
				AreaServed,
				Audience,
				AvailableChannel,
				Award,
				Brand,
				Broker,
				Category,
				HasOfferCatalog,
				HoursAvailable,
				IsRelatedTo,
				IsSimilarTo,
				Logo,
				Offers,
				Produces,
				Provider,
				ProviderMobility,
				Review,
				ServiceArea,
				ServiceAudience,
				ServiceOutput,
				ServiceType,
				Slogan,
				TermsOfService,
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
						"area" => Ok(Field::Area),
						"broadcastAffiliateOf" => Ok(Field::BroadcastAffiliateOf),
						"broadcastDisplayName" => Ok(Field::BroadcastDisplayName),
						"broadcastFrequency" => Ok(Field::BroadcastFrequency),
						"broadcastTimezone" => Ok(Field::BroadcastTimezone),
						"broadcaster" => Ok(Field::Broadcaster),
						"callSign" => Ok(Field::CallSign),
						"hasBroadcastChannel" => Ok(Field::HasBroadcastChannel),
						"inLanguage" => Ok(Field::InLanguage),
						"parentService" => Ok(Field::ParentService),
						"videoFormat" => Ok(Field::VideoFormat),
						"aggregateRating" => Ok(Field::AggregateRating),
						"areaServed" => Ok(Field::AreaServed),
						"audience" => Ok(Field::Audience),
						"availableChannel" => Ok(Field::AvailableChannel),
						"award" => Ok(Field::Award),
						"brand" => Ok(Field::Brand),
						"broker" => Ok(Field::Broker),
						"category" => Ok(Field::Category),
						"hasOfferCatalog" => Ok(Field::HasOfferCatalog),
						"hoursAvailable" => Ok(Field::HoursAvailable),
						"isRelatedTo" => Ok(Field::IsRelatedTo),
						"isSimilarTo" => Ok(Field::IsSimilarTo),
						"logo" => Ok(Field::Logo),
						"offers" => Ok(Field::Offers),
						"produces" => Ok(Field::Produces),
						"provider" => Ok(Field::Provider),
						"providerMobility" => Ok(Field::ProviderMobility),
						"review" => Ok(Field::Review),
						"serviceArea" => Ok(Field::ServiceArea),
						"serviceAudience" => Ok(Field::ServiceAudience),
						"serviceOutput" => Ok(Field::ServiceOutput),
						"serviceType" => Ok(Field::ServiceType),
						"slogan" => Ok(Field::Slogan),
						"termsOfService" => Ok(Field::TermsOfService),
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
						_ => Ok(Field::Ignore),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"area" => Ok(Field::Area),
						b"broadcastAffiliateOf" => Ok(Field::BroadcastAffiliateOf),
						b"broadcastDisplayName" => Ok(Field::BroadcastDisplayName),
						b"broadcastFrequency" => Ok(Field::BroadcastFrequency),
						b"broadcastTimezone" => Ok(Field::BroadcastTimezone),
						b"broadcaster" => Ok(Field::Broadcaster),
						b"callSign" => Ok(Field::CallSign),
						b"hasBroadcastChannel" => Ok(Field::HasBroadcastChannel),
						b"inLanguage" => Ok(Field::InLanguage),
						b"parentService" => Ok(Field::ParentService),
						b"videoFormat" => Ok(Field::VideoFormat),
						b"aggregateRating" => Ok(Field::AggregateRating),
						b"areaServed" => Ok(Field::AreaServed),
						b"audience" => Ok(Field::Audience),
						b"availableChannel" => Ok(Field::AvailableChannel),
						b"award" => Ok(Field::Award),
						b"brand" => Ok(Field::Brand),
						b"broker" => Ok(Field::Broker),
						b"category" => Ok(Field::Category),
						b"hasOfferCatalog" => Ok(Field::HasOfferCatalog),
						b"hoursAvailable" => Ok(Field::HoursAvailable),
						b"isRelatedTo" => Ok(Field::IsRelatedTo),
						b"isSimilarTo" => Ok(Field::IsSimilarTo),
						b"logo" => Ok(Field::Logo),
						b"offers" => Ok(Field::Offers),
						b"produces" => Ok(Field::Produces),
						b"provider" => Ok(Field::Provider),
						b"providerMobility" => Ok(Field::ProviderMobility),
						b"review" => Ok(Field::Review),
						b"serviceArea" => Ok(Field::ServiceArea),
						b"serviceAudience" => Ok(Field::ServiceAudience),
						b"serviceOutput" => Ok(Field::ServiceOutput),
						b"serviceType" => Ok(Field::ServiceType),
						b"slogan" => Ok(Field::Slogan),
						b"termsOfService" => Ok(Field::TermsOfService),
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
				type Value = BroadcastService;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema BroadcastService")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#area_property = None;
					let mut r#broadcast_affiliate_of_property = None;
					let mut r#broadcast_display_name_property = None;
					let mut r#broadcast_frequency_property = None;
					let mut r#broadcast_timezone_property = None;
					let mut r#broadcaster_property = None;
					let mut r#call_sign_property = None;
					let mut r#has_broadcast_channel_property = None;
					let mut r#in_language_property = None;
					let mut r#parent_service_property = None;
					let mut r#video_format_property = None;
					let mut r#aggregate_rating_property = None;
					let mut r#area_served_property = None;
					let mut r#audience_property = None;
					let mut r#available_channel_property = None;
					let mut r#award_property = None;
					let mut r#brand_property = None;
					let mut r#broker_property = None;
					let mut r#category_property = None;
					let mut r#has_offer_catalog_property = None;
					let mut r#hours_available_property = None;
					let mut r#is_related_to_property = None;
					let mut r#is_similar_to_property = None;
					let mut r#logo_property = None;
					let mut r#offers_property = None;
					let mut r#produces_property = None;
					let mut r#provider_property = None;
					let mut r#provider_mobility_property = None;
					let mut r#review_property = None;
					let mut r#service_area_property = None;
					let mut r#service_audience_property = None;
					let mut r#service_output_property = None;
					let mut r#service_type_property = None;
					let mut r#slogan_property = None;
					let mut r#terms_of_service_property = None;
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
							Field::Area => {
								if r#area_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("area"));
								}
								r#area_property = Some({
									struct DeserializeWith(Vec<AreaProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::BroadcastAffiliateOf => {
								if r#broadcast_affiliate_of_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"broadcastAffiliateOf",
									));
								}
								r#broadcast_affiliate_of_property = Some({
									struct DeserializeWith(Vec<BroadcastAffiliateOfProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::BroadcastDisplayName => {
								if r#broadcast_display_name_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"broadcastDisplayName",
									));
								}
								r#broadcast_display_name_property = Some({
									struct DeserializeWith(Vec<BroadcastDisplayNameProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::BroadcastFrequency => {
								if r#broadcast_frequency_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"broadcastFrequency",
									));
								}
								r#broadcast_frequency_property = Some({
									struct DeserializeWith(Vec<BroadcastFrequencyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::BroadcastTimezone => {
								if r#broadcast_timezone_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"broadcastTimezone",
									));
								}
								r#broadcast_timezone_property = Some({
									struct DeserializeWith(Vec<BroadcastTimezoneProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Broadcaster => {
								if r#broadcaster_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"broadcaster",
									));
								}
								r#broadcaster_property = Some({
									struct DeserializeWith(Vec<BroadcasterProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::HasBroadcastChannel => {
								if r#has_broadcast_channel_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasBroadcastChannel",
									));
								}
								r#has_broadcast_channel_property = Some({
									struct DeserializeWith(Vec<HasBroadcastChannelProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::InLanguage => {
								if r#in_language_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"inLanguage",
									));
								}
								r#in_language_property = Some({
									struct DeserializeWith(Vec<InLanguageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ParentService => {
								if r#parent_service_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"parentService",
									));
								}
								r#parent_service_property = Some({
									struct DeserializeWith(Vec<ParentServiceProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::VideoFormat => {
								if r#video_format_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"videoFormat",
									));
								}
								r#video_format_property = Some({
									struct DeserializeWith(Vec<VideoFormatProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							_ => {
								let _ = map.next_value::<de::IgnoredAny>()?;
							}
						}
					}
					Ok(BroadcastService {
						r#area: r#area_property.unwrap_or_default(),
						r#broadcast_affiliate_of: r#broadcast_affiliate_of_property
							.unwrap_or_default(),
						r#broadcast_display_name: r#broadcast_display_name_property
							.unwrap_or_default(),
						r#broadcast_frequency: r#broadcast_frequency_property.unwrap_or_default(),
						r#broadcast_timezone: r#broadcast_timezone_property.unwrap_or_default(),
						r#broadcaster: r#broadcaster_property.unwrap_or_default(),
						r#call_sign: r#call_sign_property.unwrap_or_default(),
						r#has_broadcast_channel: r#has_broadcast_channel_property
							.unwrap_or_default(),
						r#in_language: r#in_language_property.unwrap_or_default(),
						r#parent_service: r#parent_service_property.unwrap_or_default(),
						r#video_format: r#video_format_property.unwrap_or_default(),
						r#aggregate_rating: r#aggregate_rating_property.unwrap_or_default(),
						r#area_served: r#area_served_property.unwrap_or_default(),
						r#audience: r#audience_property.unwrap_or_default(),
						r#available_channel: r#available_channel_property.unwrap_or_default(),
						r#award: r#award_property.unwrap_or_default(),
						r#brand: r#brand_property.unwrap_or_default(),
						r#broker: r#broker_property.unwrap_or_default(),
						r#category: r#category_property.unwrap_or_default(),
						r#has_offer_catalog: r#has_offer_catalog_property.unwrap_or_default(),
						r#hours_available: r#hours_available_property.unwrap_or_default(),
						r#is_related_to: r#is_related_to_property.unwrap_or_default(),
						r#is_similar_to: r#is_similar_to_property.unwrap_or_default(),
						r#logo: r#logo_property.unwrap_or_default(),
						r#offers: r#offers_property.unwrap_or_default(),
						r#produces: r#produces_property.unwrap_or_default(),
						r#provider: r#provider_property.unwrap_or_default(),
						r#provider_mobility: r#provider_mobility_property.unwrap_or_default(),
						r#review: r#review_property.unwrap_or_default(),
						r#service_area: r#service_area_property.unwrap_or_default(),
						r#service_audience: r#service_audience_property.unwrap_or_default(),
						r#service_output: r#service_output_property.unwrap_or_default(),
						r#service_type: r#service_type_property.unwrap_or_default(),
						r#slogan: r#slogan_property.unwrap_or_default(),
						r#terms_of_service: r#terms_of_service_property.unwrap_or_default(),
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
				"area",
				"broadcastAffiliateOf",
				"broadcastDisplayName",
				"broadcastFrequency",
				"broadcastTimezone",
				"broadcaster",
				"callSign",
				"hasBroadcastChannel",
				"inLanguage",
				"parentService",
				"videoFormat",
				"aggregateRating",
				"areaServed",
				"audience",
				"availableChannel",
				"award",
				"brand",
				"broker",
				"category",
				"hasOfferCatalog",
				"hoursAvailable",
				"isRelatedTo",
				"isSimilarTo",
				"logo",
				"offers",
				"produces",
				"provider",
				"providerMobility",
				"review",
				"serviceArea",
				"serviceAudience",
				"serviceOutput",
				"serviceType",
				"slogan",
				"termsOfService",
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
			deserializer.deserialize_struct("BroadcastService", FIELDS, ClassVisitor)
		}
	}
}
