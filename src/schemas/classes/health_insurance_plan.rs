use super::*;
/// <https://schema.org/HealthInsurancePlan>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct HealthInsurancePlan {
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
			feature = "alternate-name-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#alternate_name: Vec<AlternateNameProperty>,
	#[cfg(any(
		any(
			feature = "benefits-summary-url-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#benefits_summary_url: Vec<BenefitsSummaryUrlProperty>,
	#[cfg(any(
		any(
			feature = "contact-point-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#contact_point: Vec<ContactPointProperty>,
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
			feature = "health-plan-drug-option-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#health_plan_drug_option: Vec<HealthPlanDrugOptionProperty>,
	#[cfg(any(
		any(
			feature = "health-plan-drug-tier-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#health_plan_drug_tier: Vec<HealthPlanDrugTierProperty>,
	#[cfg(any(
		any(
			feature = "health-plan-id-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#health_plan_id: Vec<HealthPlanIdProperty>,
	#[cfg(any(
		any(
			feature = "health-plan-marketing-url-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#health_plan_marketing_url: Vec<HealthPlanMarketingUrlProperty>,
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
			feature = "includes-health-plan-formulary-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#includes_health_plan_formulary: Vec<IncludesHealthPlanFormularyProperty>,
	#[cfg(any(
		any(
			feature = "includes-health-plan-network-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#includes_health_plan_network: Vec<IncludesHealthPlanNetworkProperty>,
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
		any(
			feature = "potential-action-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#potential_action: Vec<PotentialActionProperty>,
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
			feature = "uses-health-plan-id-standard-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#uses_health_plan_id_standard: Vec<UsesHealthPlanIdStandardProperty>,
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
						feature = "benefits-summary-url-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#benefits_summary_url) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "contact-point-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#contact_point) as usize
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
						feature = "health-plan-drug-option-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#health_plan_drug_option) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "health-plan-drug-tier-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#health_plan_drug_tier) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "health-plan-id-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#health_plan_id) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "health-plan-marketing-url-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#health_plan_marketing_url) as usize
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
						feature = "includes-health-plan-formulary-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#includes_health_plan_formulary) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "includes-health-plan-network-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#includes_health_plan_network) as usize
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
						feature = "uses-health-plan-id-standard-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#uses_health_plan_id_standard) as usize
				} else {
					0
				},
			]
			.iter()
			.sum();
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "HealthInsurancePlan", len)?;
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
				any(
					feature = "benefits-summary-url-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "contact-point-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "health-plan-drug-option-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "health-plan-drug-tier-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "health-plan-id-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "health-plan-marketing-url-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
					feature = "includes-health-plan-formulary-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "includes-health-plan-network-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
					feature = "uses-health-plan-id-standard-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for HealthInsurancePlan {
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
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AlternateName,
				#[cfg(any(
					any(
						feature = "benefits-summary-url-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				BenefitsSummaryUrl,
				#[cfg(any(
					any(
						feature = "contact-point-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ContactPoint,
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
						feature = "health-plan-drug-option-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HealthPlanDrugOption,
				#[cfg(any(
					any(
						feature = "health-plan-drug-tier-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HealthPlanDrugTier,
				#[cfg(any(
					any(
						feature = "health-plan-id-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HealthPlanId,
				#[cfg(any(
					any(
						feature = "health-plan-marketing-url-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HealthPlanMarketingUrl,
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
						feature = "includes-health-plan-formulary-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				IncludesHealthPlanFormulary,
				#[cfg(any(
					any(
						feature = "includes-health-plan-network-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				IncludesHealthPlanNetwork,
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
					any(
						feature = "potential-action-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PotentialAction,
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
						feature = "uses-health-plan-id-standard-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				UsesHealthPlanIdStandard,
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
								feature = "alternate-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"alternateName" => Ok(Field::AlternateName),
						#[cfg(any(
							any(
								feature = "benefits-summary-url-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"benefitsSummaryUrl" => Ok(Field::BenefitsSummaryUrl),
						#[cfg(any(
							any(
								feature = "contact-point-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"contactPoint" => Ok(Field::ContactPoint),
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
								feature = "health-plan-drug-option-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"healthPlanDrugOption" => Ok(Field::HealthPlanDrugOption),
						#[cfg(any(
							any(
								feature = "health-plan-drug-tier-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"healthPlanDrugTier" => Ok(Field::HealthPlanDrugTier),
						#[cfg(any(
							any(
								feature = "health-plan-id-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"healthPlanId" => Ok(Field::HealthPlanId),
						#[cfg(any(
							any(
								feature = "health-plan-marketing-url-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"healthPlanMarketingUrl" => Ok(Field::HealthPlanMarketingUrl),
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
								feature = "includes-health-plan-formulary-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"includesHealthPlanFormulary" => Ok(Field::IncludesHealthPlanFormulary),
						#[cfg(any(
							any(
								feature = "includes-health-plan-network-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"includesHealthPlanNetwork" => Ok(Field::IncludesHealthPlanNetwork),
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
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"potentialAction" => Ok(Field::PotentialAction),
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
								feature = "uses-health-plan-id-standard-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"usesHealthPlanIdStandard" => Ok(Field::UsesHealthPlanIdStandard),
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
								feature = "alternate-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"alternateName" => Ok(Field::AlternateName),
						#[cfg(any(
							any(
								feature = "benefits-summary-url-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"benefitsSummaryUrl" => Ok(Field::BenefitsSummaryUrl),
						#[cfg(any(
							any(
								feature = "contact-point-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"contactPoint" => Ok(Field::ContactPoint),
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
								feature = "health-plan-drug-option-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"healthPlanDrugOption" => Ok(Field::HealthPlanDrugOption),
						#[cfg(any(
							any(
								feature = "health-plan-drug-tier-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"healthPlanDrugTier" => Ok(Field::HealthPlanDrugTier),
						#[cfg(any(
							any(
								feature = "health-plan-id-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"healthPlanId" => Ok(Field::HealthPlanId),
						#[cfg(any(
							any(
								feature = "health-plan-marketing-url-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"healthPlanMarketingUrl" => Ok(Field::HealthPlanMarketingUrl),
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
								feature = "includes-health-plan-formulary-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"includesHealthPlanFormulary" => Ok(Field::IncludesHealthPlanFormulary),
						#[cfg(any(
							any(
								feature = "includes-health-plan-network-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"includesHealthPlanNetwork" => Ok(Field::IncludesHealthPlanNetwork),
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
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"potentialAction" => Ok(Field::PotentialAction),
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
								feature = "uses-health-plan-id-standard-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"usesHealthPlanIdStandard" => Ok(Field::UsesHealthPlanIdStandard),
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
							feature = "alternate-name-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#alternate_name_property = None;
					#[cfg(any(
						any(
							feature = "benefits-summary-url-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#benefits_summary_url_property = None;
					#[cfg(any(
						any(
							feature = "contact-point-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#contact_point_property = None;
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
							feature = "health-plan-drug-option-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#health_plan_drug_option_property = None;
					#[cfg(any(
						any(
							feature = "health-plan-drug-tier-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#health_plan_drug_tier_property = None;
					#[cfg(any(
						any(
							feature = "health-plan-id-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#health_plan_id_property = None;
					#[cfg(any(
						any(
							feature = "health-plan-marketing-url-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#health_plan_marketing_url_property = None;
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
							feature = "includes-health-plan-formulary-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#includes_health_plan_formulary_property = None;
					#[cfg(any(
						any(
							feature = "includes-health-plan-network-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#includes_health_plan_network_property = None;
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
							feature = "potential-action-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#potential_action_property = None;
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
							feature = "uses-health-plan-id-standard-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#uses_health_plan_id_standard_property = None;
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
									feature = "benefits-summary-url-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "contact-point-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "health-plan-drug-option-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "health-plan-drug-tier-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "health-plan-id-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "health-plan-marketing-url-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
									feature = "includes-health-plan-formulary-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "includes-health-plan-network-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
									feature = "uses-health-plan-id-standard-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							_ => {
								let _ = map.next_value::<de::IgnoredAny>()?;
							}
						}
					}
					Ok(HealthInsurancePlan {
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
								feature = "alternate-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "benefits-summary-url-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#benefits_summary_url: r#benefits_summary_url_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "contact-point-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#contact_point: r#contact_point_property.unwrap_or_default(),
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
								feature = "health-plan-drug-option-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#health_plan_drug_option: r#health_plan_drug_option_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "health-plan-drug-tier-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#health_plan_drug_tier: r#health_plan_drug_tier_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "health-plan-id-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#health_plan_id: r#health_plan_id_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "health-plan-marketing-url-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#health_plan_marketing_url: r#health_plan_marketing_url_property
							.unwrap_or_default(),
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
								feature = "includes-health-plan-formulary-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#includes_health_plan_formulary: r#includes_health_plan_formulary_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "includes-health-plan-network-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#includes_health_plan_network: r#includes_health_plan_network_property
							.unwrap_or_default(),
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
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#potential_action: r#potential_action_property.unwrap_or_default(),
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
								feature = "uses-health-plan-id-standard-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#uses_health_plan_id_standard: r#uses_health_plan_id_standard_property
							.unwrap_or_default(),
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
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"alternateName",
				#[cfg(any(
					any(
						feature = "benefits-summary-url-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"benefitsSummaryUrl",
				#[cfg(any(
					any(
						feature = "contact-point-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"contactPoint",
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
						feature = "health-plan-drug-option-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"healthPlanDrugOption",
				#[cfg(any(
					any(
						feature = "health-plan-drug-tier-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"healthPlanDrugTier",
				#[cfg(any(
					any(
						feature = "health-plan-id-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"healthPlanId",
				#[cfg(any(
					any(
						feature = "health-plan-marketing-url-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"healthPlanMarketingUrl",
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
						feature = "includes-health-plan-formulary-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"includesHealthPlanFormulary",
				#[cfg(any(
					any(
						feature = "includes-health-plan-network-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"includesHealthPlanNetwork",
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
					any(
						feature = "potential-action-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"potentialAction",
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
						feature = "uses-health-plan-id-standard-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"usesHealthPlanIdStandard",
			];
			deserializer.deserialize_struct("HealthInsurancePlan", FIELDS, ClassVisitor)
		}
	}
}
