use super::*;
/// <https://schema.org/HealthInsurancePlan>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct HealthInsurancePlan {
	/// <https://schema.org/benefitsSummaryUrl>
	pub r#benefits_summary_url: Vec<BenefitsSummaryUrlProperty>,
	/// <https://schema.org/contactPoint>
	pub r#contact_point: Vec<ContactPointProperty>,
	/// <https://schema.org/healthPlanDrugOption>
	pub r#health_plan_drug_option: Vec<HealthPlanDrugOptionProperty>,
	/// <https://schema.org/healthPlanDrugTier>
	pub r#health_plan_drug_tier: Vec<HealthPlanDrugTierProperty>,
	/// <https://schema.org/healthPlanId>
	pub r#health_plan_id: Vec<HealthPlanIdProperty>,
	/// <https://schema.org/healthPlanMarketingUrl>
	pub r#health_plan_marketing_url: Vec<HealthPlanMarketingUrlProperty>,
	/// <https://schema.org/includesHealthPlanFormulary>
	pub r#includes_health_plan_formulary: Vec<IncludesHealthPlanFormularyProperty>,
	/// <https://schema.org/includesHealthPlanNetwork>
	pub r#includes_health_plan_network: Vec<IncludesHealthPlanNetworkProperty>,
	/// <https://schema.org/usesHealthPlanIdStandard>
	pub r#uses_health_plan_id_standard: Vec<UsesHealthPlanIdStandardProperty>,
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
/// This trait is for properties from <https://schema.org/HealthInsurancePlan>.
pub trait HealthInsurancePlanTrait {
	/// Get <https://schema.org/benefitsSummaryUrl> from [`Self`] as borrowed slice.
	fn get_benefits_summary_url(&self) -> &[BenefitsSummaryUrlProperty];
	/// Take <https://schema.org/benefitsSummaryUrl> from [`Self`] as owned vector.
	fn take_benefits_summary_url(&mut self) -> Vec<BenefitsSummaryUrlProperty>;
	/// Get <https://schema.org/contactPoint> from [`Self`] as borrowed slice.
	fn get_contact_point(&self) -> &[ContactPointProperty];
	/// Take <https://schema.org/contactPoint> from [`Self`] as owned vector.
	fn take_contact_point(&mut self) -> Vec<ContactPointProperty>;
	/// Get <https://schema.org/healthPlanDrugOption> from [`Self`] as borrowed slice.
	fn get_health_plan_drug_option(&self) -> &[HealthPlanDrugOptionProperty];
	/// Take <https://schema.org/healthPlanDrugOption> from [`Self`] as owned vector.
	fn take_health_plan_drug_option(&mut self) -> Vec<HealthPlanDrugOptionProperty>;
	/// Get <https://schema.org/healthPlanDrugTier> from [`Self`] as borrowed slice.
	fn get_health_plan_drug_tier(&self) -> &[HealthPlanDrugTierProperty];
	/// Take <https://schema.org/healthPlanDrugTier> from [`Self`] as owned vector.
	fn take_health_plan_drug_tier(&mut self) -> Vec<HealthPlanDrugTierProperty>;
	/// Get <https://schema.org/healthPlanId> from [`Self`] as borrowed slice.
	fn get_health_plan_id(&self) -> &[HealthPlanIdProperty];
	/// Take <https://schema.org/healthPlanId> from [`Self`] as owned vector.
	fn take_health_plan_id(&mut self) -> Vec<HealthPlanIdProperty>;
	/// Get <https://schema.org/healthPlanMarketingUrl> from [`Self`] as borrowed slice.
	fn get_health_plan_marketing_url(&self) -> &[HealthPlanMarketingUrlProperty];
	/// Take <https://schema.org/healthPlanMarketingUrl> from [`Self`] as owned vector.
	fn take_health_plan_marketing_url(&mut self) -> Vec<HealthPlanMarketingUrlProperty>;
	/// Get <https://schema.org/includesHealthPlanFormulary> from [`Self`] as borrowed slice.
	fn get_includes_health_plan_formulary(&self) -> &[IncludesHealthPlanFormularyProperty];
	/// Take <https://schema.org/includesHealthPlanFormulary> from [`Self`] as owned vector.
	fn take_includes_health_plan_formulary(&mut self) -> Vec<IncludesHealthPlanFormularyProperty>;
	/// Get <https://schema.org/includesHealthPlanNetwork> from [`Self`] as borrowed slice.
	fn get_includes_health_plan_network(&self) -> &[IncludesHealthPlanNetworkProperty];
	/// Take <https://schema.org/includesHealthPlanNetwork> from [`Self`] as owned vector.
	fn take_includes_health_plan_network(&mut self) -> Vec<IncludesHealthPlanNetworkProperty>;
	/// Get <https://schema.org/usesHealthPlanIdStandard> from [`Self`] as borrowed slice.
	fn get_uses_health_plan_id_standard(&self) -> &[UsesHealthPlanIdStandardProperty];
	/// Take <https://schema.org/usesHealthPlanIdStandard> from [`Self`] as owned vector.
	fn take_uses_health_plan_id_standard(&mut self) -> Vec<UsesHealthPlanIdStandardProperty>;
}
impl HealthInsurancePlanTrait for HealthInsurancePlan {
	fn get_benefits_summary_url(&self) -> &[BenefitsSummaryUrlProperty] {
		self.r#benefits_summary_url.as_slice()
	}
	fn take_benefits_summary_url(&mut self) -> Vec<BenefitsSummaryUrlProperty> {
		std::mem::take(&mut self.r#benefits_summary_url)
	}
	fn get_contact_point(&self) -> &[ContactPointProperty] {
		self.r#contact_point.as_slice()
	}
	fn take_contact_point(&mut self) -> Vec<ContactPointProperty> {
		std::mem::take(&mut self.r#contact_point)
	}
	fn get_health_plan_drug_option(&self) -> &[HealthPlanDrugOptionProperty] {
		self.r#health_plan_drug_option.as_slice()
	}
	fn take_health_plan_drug_option(&mut self) -> Vec<HealthPlanDrugOptionProperty> {
		std::mem::take(&mut self.r#health_plan_drug_option)
	}
	fn get_health_plan_drug_tier(&self) -> &[HealthPlanDrugTierProperty] {
		self.r#health_plan_drug_tier.as_slice()
	}
	fn take_health_plan_drug_tier(&mut self) -> Vec<HealthPlanDrugTierProperty> {
		std::mem::take(&mut self.r#health_plan_drug_tier)
	}
	fn get_health_plan_id(&self) -> &[HealthPlanIdProperty] {
		self.r#health_plan_id.as_slice()
	}
	fn take_health_plan_id(&mut self) -> Vec<HealthPlanIdProperty> {
		std::mem::take(&mut self.r#health_plan_id)
	}
	fn get_health_plan_marketing_url(&self) -> &[HealthPlanMarketingUrlProperty] {
		self.r#health_plan_marketing_url.as_slice()
	}
	fn take_health_plan_marketing_url(&mut self) -> Vec<HealthPlanMarketingUrlProperty> {
		std::mem::take(&mut self.r#health_plan_marketing_url)
	}
	fn get_includes_health_plan_formulary(&self) -> &[IncludesHealthPlanFormularyProperty] {
		self.r#includes_health_plan_formulary.as_slice()
	}
	fn take_includes_health_plan_formulary(&mut self) -> Vec<IncludesHealthPlanFormularyProperty> {
		std::mem::take(&mut self.r#includes_health_plan_formulary)
	}
	fn get_includes_health_plan_network(&self) -> &[IncludesHealthPlanNetworkProperty] {
		self.r#includes_health_plan_network.as_slice()
	}
	fn take_includes_health_plan_network(&mut self) -> Vec<IncludesHealthPlanNetworkProperty> {
		std::mem::take(&mut self.r#includes_health_plan_network)
	}
	fn get_uses_health_plan_id_standard(&self) -> &[UsesHealthPlanIdStandardProperty] {
		self.r#uses_health_plan_id_standard.as_slice()
	}
	fn take_uses_health_plan_id_standard(&mut self) -> Vec<UsesHealthPlanIdStandardProperty> {
		std::mem::take(&mut self.r#uses_health_plan_id_standard)
	}
}
impl ThingTrait for HealthInsurancePlan {
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
	impl Serialize for HealthInsurancePlan {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#benefits_summary_url) as usize,
				!Vec::is_empty(&self.r#contact_point) as usize,
				!Vec::is_empty(&self.r#health_plan_drug_option) as usize,
				!Vec::is_empty(&self.r#health_plan_drug_tier) as usize,
				!Vec::is_empty(&self.r#health_plan_id) as usize,
				!Vec::is_empty(&self.r#health_plan_marketing_url) as usize,
				!Vec::is_empty(&self.r#includes_health_plan_formulary) as usize,
				!Vec::is_empty(&self.r#includes_health_plan_network) as usize,
				!Vec::is_empty(&self.r#uses_health_plan_id_standard) as usize,
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
				Serializer::serialize_struct(serializer, "HealthInsurancePlan", len)?;
			if !Vec::is_empty(&self.r#benefits_summary_url) {
				serialize_struct.serialize_field("benefitsSummaryUrl", {
					struct SerializeWith<'a>(&'a Vec<BenefitsSummaryUrlProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#benefits_summary_url)
				})?;
			} else {
				serialize_struct.skip_field("benefitsSummaryUrl")?;
			}
			if !Vec::is_empty(&self.r#contact_point) {
				serialize_struct.serialize_field("contactPoint", {
					struct SerializeWith<'a>(&'a Vec<ContactPointProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#contact_point)
				})?;
			} else {
				serialize_struct.skip_field("contactPoint")?;
			}
			if !Vec::is_empty(&self.r#health_plan_drug_option) {
				serialize_struct.serialize_field("healthPlanDrugOption", {
					struct SerializeWith<'a>(&'a Vec<HealthPlanDrugOptionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#health_plan_drug_option)
				})?;
			} else {
				serialize_struct.skip_field("healthPlanDrugOption")?;
			}
			if !Vec::is_empty(&self.r#health_plan_drug_tier) {
				serialize_struct.serialize_field("healthPlanDrugTier", {
					struct SerializeWith<'a>(&'a Vec<HealthPlanDrugTierProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#health_plan_drug_tier)
				})?;
			} else {
				serialize_struct.skip_field("healthPlanDrugTier")?;
			}
			if !Vec::is_empty(&self.r#health_plan_id) {
				serialize_struct.serialize_field("healthPlanId", {
					struct SerializeWith<'a>(&'a Vec<HealthPlanIdProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#health_plan_id)
				})?;
			} else {
				serialize_struct.skip_field("healthPlanId")?;
			}
			if !Vec::is_empty(&self.r#health_plan_marketing_url) {
				serialize_struct.serialize_field("healthPlanMarketingUrl", {
					struct SerializeWith<'a>(&'a Vec<HealthPlanMarketingUrlProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#health_plan_marketing_url)
				})?;
			} else {
				serialize_struct.skip_field("healthPlanMarketingUrl")?;
			}
			if !Vec::is_empty(&self.r#includes_health_plan_formulary) {
				serialize_struct.serialize_field("includesHealthPlanFormulary", {
					struct SerializeWith<'a>(&'a Vec<IncludesHealthPlanFormularyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#includes_health_plan_formulary)
				})?;
			} else {
				serialize_struct.skip_field("includesHealthPlanFormulary")?;
			}
			if !Vec::is_empty(&self.r#includes_health_plan_network) {
				serialize_struct.serialize_field("includesHealthPlanNetwork", {
					struct SerializeWith<'a>(&'a Vec<IncludesHealthPlanNetworkProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#includes_health_plan_network)
				})?;
			} else {
				serialize_struct.skip_field("includesHealthPlanNetwork")?;
			}
			if !Vec::is_empty(&self.r#uses_health_plan_id_standard) {
				serialize_struct.serialize_field("usesHealthPlanIdStandard", {
					struct SerializeWith<'a>(&'a Vec<UsesHealthPlanIdStandardProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#uses_health_plan_id_standard)
				})?;
			} else {
				serialize_struct.skip_field("usesHealthPlanIdStandard")?;
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
	impl<'de> Deserialize<'de> for HealthInsurancePlan {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				BenefitsSummaryUrl,
				ContactPoint,
				HealthPlanDrugOption,
				HealthPlanDrugTier,
				HealthPlanId,
				HealthPlanMarketingUrl,
				IncludesHealthPlanFormulary,
				IncludesHealthPlanNetwork,
				UsesHealthPlanIdStandard,
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
						"benefitsSummaryUrl" => Ok(Field::BenefitsSummaryUrl),
						"contactPoint" => Ok(Field::ContactPoint),
						"healthPlanDrugOption" => Ok(Field::HealthPlanDrugOption),
						"healthPlanDrugTier" => Ok(Field::HealthPlanDrugTier),
						"healthPlanId" => Ok(Field::HealthPlanId),
						"healthPlanMarketingUrl" => Ok(Field::HealthPlanMarketingUrl),
						"includesHealthPlanFormulary" => Ok(Field::IncludesHealthPlanFormulary),
						"includesHealthPlanNetwork" => Ok(Field::IncludesHealthPlanNetwork),
						"usesHealthPlanIdStandard" => Ok(Field::UsesHealthPlanIdStandard),
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
						b"benefitsSummaryUrl" => Ok(Field::BenefitsSummaryUrl),
						b"contactPoint" => Ok(Field::ContactPoint),
						b"healthPlanDrugOption" => Ok(Field::HealthPlanDrugOption),
						b"healthPlanDrugTier" => Ok(Field::HealthPlanDrugTier),
						b"healthPlanId" => Ok(Field::HealthPlanId),
						b"healthPlanMarketingUrl" => Ok(Field::HealthPlanMarketingUrl),
						b"includesHealthPlanFormulary" => Ok(Field::IncludesHealthPlanFormulary),
						b"includesHealthPlanNetwork" => Ok(Field::IncludesHealthPlanNetwork),
						b"usesHealthPlanIdStandard" => Ok(Field::UsesHealthPlanIdStandard),
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
				type Value = HealthInsurancePlan;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema HealthInsurancePlan")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#benefits_summary_url_property = None;
					let mut r#contact_point_property = None;
					let mut r#health_plan_drug_option_property = None;
					let mut r#health_plan_drug_tier_property = None;
					let mut r#health_plan_id_property = None;
					let mut r#health_plan_marketing_url_property = None;
					let mut r#includes_health_plan_formulary_property = None;
					let mut r#includes_health_plan_network_property = None;
					let mut r#uses_health_plan_id_standard_property = None;
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
							Field::BenefitsSummaryUrl => {
								if r#benefits_summary_url_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"benefitsSummaryUrl",
									));
								}
								r#benefits_summary_url_property = Some({
									struct DeserializeWith(Vec<BenefitsSummaryUrlProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ContactPoint => {
								if r#contact_point_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"contactPoint",
									));
								}
								r#contact_point_property = Some({
									struct DeserializeWith(Vec<ContactPointProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::HealthPlanDrugOption => {
								if r#health_plan_drug_option_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"healthPlanDrugOption",
									));
								}
								r#health_plan_drug_option_property = Some({
									struct DeserializeWith(Vec<HealthPlanDrugOptionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::HealthPlanDrugTier => {
								if r#health_plan_drug_tier_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"healthPlanDrugTier",
									));
								}
								r#health_plan_drug_tier_property = Some({
									struct DeserializeWith(Vec<HealthPlanDrugTierProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::HealthPlanId => {
								if r#health_plan_id_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"healthPlanId",
									));
								}
								r#health_plan_id_property = Some({
									struct DeserializeWith(Vec<HealthPlanIdProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::HealthPlanMarketingUrl => {
								if r#health_plan_marketing_url_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"healthPlanMarketingUrl",
									));
								}
								r#health_plan_marketing_url_property = Some({
									struct DeserializeWith(Vec<HealthPlanMarketingUrlProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::IncludesHealthPlanFormulary => {
								if r#includes_health_plan_formulary_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"includesHealthPlanFormulary",
									));
								}
								r#includes_health_plan_formulary_property = Some({
									struct DeserializeWith(
										Vec<IncludesHealthPlanFormularyProperty>,
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
							Field::IncludesHealthPlanNetwork => {
								if r#includes_health_plan_network_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"includesHealthPlanNetwork",
									));
								}
								r#includes_health_plan_network_property = Some({
									struct DeserializeWith(Vec<IncludesHealthPlanNetworkProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::UsesHealthPlanIdStandard => {
								if r#uses_health_plan_id_standard_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"usesHealthPlanIdStandard",
									));
								}
								r#uses_health_plan_id_standard_property = Some({
									struct DeserializeWith(Vec<UsesHealthPlanIdStandardProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
					Ok(HealthInsurancePlan {
						r#benefits_summary_url: r#benefits_summary_url_property.unwrap_or_default(),
						r#contact_point: r#contact_point_property.unwrap_or_default(),
						r#health_plan_drug_option: r#health_plan_drug_option_property
							.unwrap_or_default(),
						r#health_plan_drug_tier: r#health_plan_drug_tier_property
							.unwrap_or_default(),
						r#health_plan_id: r#health_plan_id_property.unwrap_or_default(),
						r#health_plan_marketing_url: r#health_plan_marketing_url_property
							.unwrap_or_default(),
						r#includes_health_plan_formulary: r#includes_health_plan_formulary_property
							.unwrap_or_default(),
						r#includes_health_plan_network: r#includes_health_plan_network_property
							.unwrap_or_default(),
						r#uses_health_plan_id_standard: r#uses_health_plan_id_standard_property
							.unwrap_or_default(),
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
				"benefitsSummaryUrl",
				"contactPoint",
				"healthPlanDrugOption",
				"healthPlanDrugTier",
				"healthPlanId",
				"healthPlanMarketingUrl",
				"includesHealthPlanFormulary",
				"includesHealthPlanNetwork",
				"usesHealthPlanIdStandard",
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
			deserializer.deserialize_struct("HealthInsurancePlan", FIELDS, ClassVisitor)
		}
	}
}
