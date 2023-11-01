use super::*;
/// <https://schema.org/JobPosting>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct JobPosting {
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
			feature = "applicant-location-requirements-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#applicant_location_requirements: Vec<ApplicantLocationRequirementsProperty>,
	#[cfg(any(
		any(
			feature = "application-contact-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#application_contact: Vec<ApplicationContactProperty>,
	#[cfg(any(
		any(
			feature = "base-salary-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#base_salary: Vec<BaseSalaryProperty>,
	#[cfg(any(
		any(
			feature = "benefits-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#benefits: Vec<BenefitsProperty>,
	#[cfg(any(
		any(
			feature = "date-posted-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#date_posted: Vec<DatePostedProperty>,
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
			feature = "direct-apply-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#direct_apply: Vec<DirectApplyProperty>,
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
			feature = "education-requirements-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#education_requirements: Vec<EducationRequirementsProperty>,
	#[cfg(any(
		any(
			feature = "eligibility-to-work-requirement-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#eligibility_to_work_requirement: Vec<EligibilityToWorkRequirementProperty>,
	#[cfg(any(
		any(
			feature = "employer-overview-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#employer_overview: Vec<EmployerOverviewProperty>,
	#[cfg(any(
		any(
			feature = "employment-type-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#employment_type: Vec<EmploymentTypeProperty>,
	#[cfg(any(
		any(
			feature = "employment-unit-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#employment_unit: Vec<EmploymentUnitProperty>,
	#[cfg(any(
		any(
			feature = "estimated-salary-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#estimated_salary: Vec<EstimatedSalaryProperty>,
	#[cfg(any(
		any(
			feature = "experience-in-place-of-education-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#experience_in_place_of_education: Vec<ExperienceInPlaceOfEducationProperty>,
	#[cfg(any(
		any(
			feature = "experience-requirements-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#experience_requirements: Vec<ExperienceRequirementsProperty>,
	#[cfg(any(
		any(
			feature = "hiring-organization-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#hiring_organization: Vec<HiringOrganizationProperty>,
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
			feature = "incentive-compensation-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#incentive_compensation: Vec<IncentiveCompensationProperty>,
	#[cfg(any(
		any(
			feature = "incentives-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#incentives: Vec<IncentivesProperty>,
	#[cfg(any(
		any(
			feature = "industry-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#industry: Vec<IndustryProperty>,
	#[cfg(any(
		any(
			feature = "job-benefits-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#job_benefits: Vec<JobBenefitsProperty>,
	#[cfg(any(
		any(
			feature = "job-immediate-start-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#job_immediate_start: Vec<JobImmediateStartProperty>,
	#[cfg(any(
		any(
			feature = "job-location-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#job_location: Vec<JobLocationProperty>,
	#[cfg(any(
		any(
			feature = "job-location-type-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#job_location_type: Vec<JobLocationTypeProperty>,
	#[cfg(any(
		any(
			feature = "job-start-date-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#job_start_date: Vec<JobStartDateProperty>,
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
			feature = "occupational-category-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#occupational_category: Vec<OccupationalCategoryProperty>,
	#[cfg(any(
		any(
			feature = "physical-requirement-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#physical_requirement: Vec<PhysicalRequirementProperty>,
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
			feature = "qualifications-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#qualifications: Vec<QualificationsProperty>,
	#[cfg(any(
		any(
			feature = "relevant-occupation-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#relevant_occupation: Vec<RelevantOccupationProperty>,
	#[cfg(any(
		any(
			feature = "responsibilities-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#responsibilities: Vec<ResponsibilitiesProperty>,
	#[cfg(any(
		any(
			feature = "salary-currency-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#salary_currency: Vec<SalaryCurrencyProperty>,
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
			feature = "security-clearance-requirement-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#security_clearance_requirement: Vec<SecurityClearanceRequirementProperty>,
	#[cfg(any(
		any(
			feature = "sensory-requirement-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#sensory_requirement: Vec<SensoryRequirementProperty>,
	#[cfg(any(
		any(feature = "skills-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#skills: Vec<SkillsProperty>,
	#[cfg(any(
		any(
			feature = "special-commitments-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#special_commitments: Vec<SpecialCommitmentsProperty>,
	#[cfg(any(
		any(
			feature = "subject-of-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#subject_of: Vec<SubjectOfProperty>,
	#[cfg(any(
		any(feature = "title-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#title: Vec<TitleProperty>,
	#[cfg(any(
		any(
			feature = "total-job-openings-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#total_job_openings: Vec<TotalJobOpeningsProperty>,
	#[cfg(any(
		any(feature = "url-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#url: Vec<UrlProperty>,
	#[cfg(any(
		any(
			feature = "valid-through-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#valid_through: Vec<ValidThroughProperty>,
	#[cfg(any(
		any(
			feature = "work-hours-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#work_hours: Vec<WorkHoursProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for JobPosting {
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
						feature = "applicant-location-requirements-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#applicant_location_requirements) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "application-contact-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#application_contact) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "base-salary-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#base_salary) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "benefits-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#benefits) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "date-posted-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#date_posted) as usize
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
						feature = "direct-apply-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#direct_apply) as usize
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
						feature = "education-requirements-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#education_requirements) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "eligibility-to-work-requirement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#eligibility_to_work_requirement) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "employer-overview-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#employer_overview) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "employment-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#employment_type) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "employment-unit-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#employment_unit) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "estimated-salary-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#estimated_salary) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "experience-in-place-of-education-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#experience_in_place_of_education) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "experience-requirements-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#experience_requirements) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "hiring-organization-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#hiring_organization) as usize
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
						feature = "incentive-compensation-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#incentive_compensation) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "incentives-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#incentives) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "industry-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#industry) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "job-benefits-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#job_benefits) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "job-immediate-start-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#job_immediate_start) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "job-location-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#job_location) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "job-location-type-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#job_location_type) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "job-start-date-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#job_start_date) as usize
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
						feature = "occupational-category-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#occupational_category) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "physical-requirement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#physical_requirement) as usize
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
						feature = "qualifications-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#qualifications) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "relevant-occupation-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#relevant_occupation) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "responsibilities-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#responsibilities) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "salary-currency-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#salary_currency) as usize
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
						feature = "security-clearance-requirement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#security_clearance_requirement) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "sensory-requirement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#sensory_requirement) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "skills-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#skills) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "special-commitments-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#special_commitments) as usize
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
						feature = "title-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#title) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "total-job-openings-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#total_job_openings) as usize
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
						feature = "valid-through-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#valid_through) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "work-hours-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#work_hours) as usize
				} else {
					0
				},
			]
			.iter()
			.sum();
			let mut serialize_struct = Serializer::serialize_struct(serializer, "JobPosting", len)?;
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
					feature = "applicant-location-requirements-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#applicant_location_requirements) {
				serialize_struct.serialize_field("applicantLocationRequirements", {
					struct SerializeWith<'a>(&'a Vec<ApplicantLocationRequirementsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#applicant_location_requirements)
				})?;
			} else {
				serialize_struct.skip_field("applicantLocationRequirements")?;
			}
			#[cfg(any(
				any(
					feature = "application-contact-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#application_contact) {
				serialize_struct.serialize_field("applicationContact", {
					struct SerializeWith<'a>(&'a Vec<ApplicationContactProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#application_contact)
				})?;
			} else {
				serialize_struct.skip_field("applicationContact")?;
			}
			#[cfg(any(
				any(
					feature = "base-salary-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#base_salary) {
				serialize_struct.serialize_field("baseSalary", {
					struct SerializeWith<'a>(&'a Vec<BaseSalaryProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#base_salary)
				})?;
			} else {
				serialize_struct.skip_field("baseSalary")?;
			}
			#[cfg(any(
				any(
					feature = "benefits-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#benefits) {
				serialize_struct.serialize_field("benefits", {
					struct SerializeWith<'a>(&'a Vec<BenefitsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#benefits)
				})?;
			} else {
				serialize_struct.skip_field("benefits")?;
			}
			#[cfg(any(
				any(
					feature = "date-posted-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#date_posted) {
				serialize_struct.serialize_field("datePosted", {
					struct SerializeWith<'a>(&'a Vec<DatePostedProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#date_posted)
				})?;
			} else {
				serialize_struct.skip_field("datePosted")?;
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
					feature = "direct-apply-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#direct_apply) {
				serialize_struct.serialize_field("directApply", {
					struct SerializeWith<'a>(&'a Vec<DirectApplyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#direct_apply)
				})?;
			} else {
				serialize_struct.skip_field("directApply")?;
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
					feature = "education-requirements-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#education_requirements) {
				serialize_struct.serialize_field("educationRequirements", {
					struct SerializeWith<'a>(&'a Vec<EducationRequirementsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#education_requirements)
				})?;
			} else {
				serialize_struct.skip_field("educationRequirements")?;
			}
			#[cfg(any(
				any(
					feature = "eligibility-to-work-requirement-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#eligibility_to_work_requirement) {
				serialize_struct.serialize_field("eligibilityToWorkRequirement", {
					struct SerializeWith<'a>(&'a Vec<EligibilityToWorkRequirementProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#eligibility_to_work_requirement)
				})?;
			} else {
				serialize_struct.skip_field("eligibilityToWorkRequirement")?;
			}
			#[cfg(any(
				any(
					feature = "employer-overview-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#employer_overview) {
				serialize_struct.serialize_field("employerOverview", {
					struct SerializeWith<'a>(&'a Vec<EmployerOverviewProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#employer_overview)
				})?;
			} else {
				serialize_struct.skip_field("employerOverview")?;
			}
			#[cfg(any(
				any(
					feature = "employment-type-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#employment_type) {
				serialize_struct.serialize_field("employmentType", {
					struct SerializeWith<'a>(&'a Vec<EmploymentTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#employment_type)
				})?;
			} else {
				serialize_struct.skip_field("employmentType")?;
			}
			#[cfg(any(
				any(
					feature = "employment-unit-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#employment_unit) {
				serialize_struct.serialize_field("employmentUnit", {
					struct SerializeWith<'a>(&'a Vec<EmploymentUnitProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#employment_unit)
				})?;
			} else {
				serialize_struct.skip_field("employmentUnit")?;
			}
			#[cfg(any(
				any(
					feature = "estimated-salary-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#estimated_salary) {
				serialize_struct.serialize_field("estimatedSalary", {
					struct SerializeWith<'a>(&'a Vec<EstimatedSalaryProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#estimated_salary)
				})?;
			} else {
				serialize_struct.skip_field("estimatedSalary")?;
			}
			#[cfg(any(
				any(
					feature = "experience-in-place-of-education-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#experience_in_place_of_education) {
				serialize_struct.serialize_field("experienceInPlaceOfEducation", {
					struct SerializeWith<'a>(&'a Vec<ExperienceInPlaceOfEducationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#experience_in_place_of_education)
				})?;
			} else {
				serialize_struct.skip_field("experienceInPlaceOfEducation")?;
			}
			#[cfg(any(
				any(
					feature = "experience-requirements-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#experience_requirements) {
				serialize_struct.serialize_field("experienceRequirements", {
					struct SerializeWith<'a>(&'a Vec<ExperienceRequirementsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#experience_requirements)
				})?;
			} else {
				serialize_struct.skip_field("experienceRequirements")?;
			}
			#[cfg(any(
				any(
					feature = "hiring-organization-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#hiring_organization) {
				serialize_struct.serialize_field("hiringOrganization", {
					struct SerializeWith<'a>(&'a Vec<HiringOrganizationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#hiring_organization)
				})?;
			} else {
				serialize_struct.skip_field("hiringOrganization")?;
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
					feature = "incentive-compensation-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#incentive_compensation) {
				serialize_struct.serialize_field("incentiveCompensation", {
					struct SerializeWith<'a>(&'a Vec<IncentiveCompensationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#incentive_compensation)
				})?;
			} else {
				serialize_struct.skip_field("incentiveCompensation")?;
			}
			#[cfg(any(
				any(
					feature = "incentives-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#incentives) {
				serialize_struct.serialize_field("incentives", {
					struct SerializeWith<'a>(&'a Vec<IncentivesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#incentives)
				})?;
			} else {
				serialize_struct.skip_field("incentives")?;
			}
			#[cfg(any(
				any(
					feature = "industry-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#industry) {
				serialize_struct.serialize_field("industry", {
					struct SerializeWith<'a>(&'a Vec<IndustryProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#industry)
				})?;
			} else {
				serialize_struct.skip_field("industry")?;
			}
			#[cfg(any(
				any(
					feature = "job-benefits-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#job_benefits) {
				serialize_struct.serialize_field("jobBenefits", {
					struct SerializeWith<'a>(&'a Vec<JobBenefitsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#job_benefits)
				})?;
			} else {
				serialize_struct.skip_field("jobBenefits")?;
			}
			#[cfg(any(
				any(
					feature = "job-immediate-start-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#job_immediate_start) {
				serialize_struct.serialize_field("jobImmediateStart", {
					struct SerializeWith<'a>(&'a Vec<JobImmediateStartProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#job_immediate_start)
				})?;
			} else {
				serialize_struct.skip_field("jobImmediateStart")?;
			}
			#[cfg(any(
				any(
					feature = "job-location-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#job_location) {
				serialize_struct.serialize_field("jobLocation", {
					struct SerializeWith<'a>(&'a Vec<JobLocationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#job_location)
				})?;
			} else {
				serialize_struct.skip_field("jobLocation")?;
			}
			#[cfg(any(
				any(
					feature = "job-location-type-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#job_location_type) {
				serialize_struct.serialize_field("jobLocationType", {
					struct SerializeWith<'a>(&'a Vec<JobLocationTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#job_location_type)
				})?;
			} else {
				serialize_struct.skip_field("jobLocationType")?;
			}
			#[cfg(any(
				any(
					feature = "job-start-date-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#job_start_date) {
				serialize_struct.serialize_field("jobStartDate", {
					struct SerializeWith<'a>(&'a Vec<JobStartDateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#job_start_date)
				})?;
			} else {
				serialize_struct.skip_field("jobStartDate")?;
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
					feature = "occupational-category-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#occupational_category) {
				serialize_struct.serialize_field("occupationalCategory", {
					struct SerializeWith<'a>(&'a Vec<OccupationalCategoryProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#occupational_category)
				})?;
			} else {
				serialize_struct.skip_field("occupationalCategory")?;
			}
			#[cfg(any(
				any(
					feature = "physical-requirement-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#physical_requirement) {
				serialize_struct.serialize_field("physicalRequirement", {
					struct SerializeWith<'a>(&'a Vec<PhysicalRequirementProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#physical_requirement)
				})?;
			} else {
				serialize_struct.skip_field("physicalRequirement")?;
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
					feature = "qualifications-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#qualifications) {
				serialize_struct.serialize_field("qualifications", {
					struct SerializeWith<'a>(&'a Vec<QualificationsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#qualifications)
				})?;
			} else {
				serialize_struct.skip_field("qualifications")?;
			}
			#[cfg(any(
				any(
					feature = "relevant-occupation-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#relevant_occupation) {
				serialize_struct.serialize_field("relevantOccupation", {
					struct SerializeWith<'a>(&'a Vec<RelevantOccupationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#relevant_occupation)
				})?;
			} else {
				serialize_struct.skip_field("relevantOccupation")?;
			}
			#[cfg(any(
				any(
					feature = "responsibilities-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#responsibilities) {
				serialize_struct.serialize_field("responsibilities", {
					struct SerializeWith<'a>(&'a Vec<ResponsibilitiesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#responsibilities)
				})?;
			} else {
				serialize_struct.skip_field("responsibilities")?;
			}
			#[cfg(any(
				any(
					feature = "salary-currency-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#salary_currency) {
				serialize_struct.serialize_field("salaryCurrency", {
					struct SerializeWith<'a>(&'a Vec<SalaryCurrencyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#salary_currency)
				})?;
			} else {
				serialize_struct.skip_field("salaryCurrency")?;
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
					feature = "security-clearance-requirement-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#security_clearance_requirement) {
				serialize_struct.serialize_field("securityClearanceRequirement", {
					struct SerializeWith<'a>(&'a Vec<SecurityClearanceRequirementProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#security_clearance_requirement)
				})?;
			} else {
				serialize_struct.skip_field("securityClearanceRequirement")?;
			}
			#[cfg(any(
				any(
					feature = "sensory-requirement-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#sensory_requirement) {
				serialize_struct.serialize_field("sensoryRequirement", {
					struct SerializeWith<'a>(&'a Vec<SensoryRequirementProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#sensory_requirement)
				})?;
			} else {
				serialize_struct.skip_field("sensoryRequirement")?;
			}
			#[cfg(any(
				any(feature = "skills-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#skills) {
				serialize_struct.serialize_field("skills", {
					struct SerializeWith<'a>(&'a Vec<SkillsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#skills)
				})?;
			} else {
				serialize_struct.skip_field("skills")?;
			}
			#[cfg(any(
				any(
					feature = "special-commitments-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#special_commitments) {
				serialize_struct.serialize_field("specialCommitments", {
					struct SerializeWith<'a>(&'a Vec<SpecialCommitmentsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#special_commitments)
				})?;
			} else {
				serialize_struct.skip_field("specialCommitments")?;
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
				any(feature = "title-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#title) {
				serialize_struct.serialize_field("title", {
					struct SerializeWith<'a>(&'a Vec<TitleProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#title)
				})?;
			} else {
				serialize_struct.skip_field("title")?;
			}
			#[cfg(any(
				any(
					feature = "total-job-openings-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#total_job_openings) {
				serialize_struct.serialize_field("totalJobOpenings", {
					struct SerializeWith<'a>(&'a Vec<TotalJobOpeningsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#total_job_openings)
				})?;
			} else {
				serialize_struct.skip_field("totalJobOpenings")?;
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
					feature = "valid-through-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "work-hours-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#work_hours) {
				serialize_struct.serialize_field("workHours", {
					struct SerializeWith<'a>(&'a Vec<WorkHoursProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#work_hours)
				})?;
			} else {
				serialize_struct.skip_field("workHours")?;
			}
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for JobPosting {
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
						feature = "applicant-location-requirements-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ApplicantLocationRequirements,
				#[cfg(any(
					any(
						feature = "application-contact-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ApplicationContact,
				#[cfg(any(
					any(
						feature = "base-salary-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				BaseSalary,
				#[cfg(any(
					any(
						feature = "benefits-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Benefits,
				#[cfg(any(
					any(
						feature = "date-posted-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DatePosted,
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
						feature = "direct-apply-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				DirectApply,
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
						feature = "education-requirements-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				EducationRequirements,
				#[cfg(any(
					any(
						feature = "eligibility-to-work-requirement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				EligibilityToWorkRequirement,
				#[cfg(any(
					any(
						feature = "employer-overview-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				EmployerOverview,
				#[cfg(any(
					any(
						feature = "employment-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EmploymentType,
				#[cfg(any(
					any(
						feature = "employment-unit-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				EmploymentUnit,
				#[cfg(any(
					any(
						feature = "estimated-salary-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EstimatedSalary,
				#[cfg(any(
					any(
						feature = "experience-in-place-of-education-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ExperienceInPlaceOfEducation,
				#[cfg(any(
					any(
						feature = "experience-requirements-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ExperienceRequirements,
				#[cfg(any(
					any(
						feature = "hiring-organization-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				HiringOrganization,
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
						feature = "incentive-compensation-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IncentiveCompensation,
				#[cfg(any(
					any(
						feature = "incentives-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Incentives,
				#[cfg(any(
					any(
						feature = "industry-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Industry,
				#[cfg(any(
					any(
						feature = "job-benefits-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				JobBenefits,
				#[cfg(any(
					any(
						feature = "job-immediate-start-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				JobImmediateStart,
				#[cfg(any(
					any(
						feature = "job-location-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				JobLocation,
				#[cfg(any(
					any(
						feature = "job-location-type-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				JobLocationType,
				#[cfg(any(
					any(
						feature = "job-start-date-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				JobStartDate,
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
						feature = "occupational-category-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				OccupationalCategory,
				#[cfg(any(
					any(
						feature = "physical-requirement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				PhysicalRequirement,
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
						feature = "qualifications-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Qualifications,
				#[cfg(any(
					any(
						feature = "relevant-occupation-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				RelevantOccupation,
				#[cfg(any(
					any(
						feature = "responsibilities-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Responsibilities,
				#[cfg(any(
					any(
						feature = "salary-currency-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SalaryCurrency,
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
						feature = "security-clearance-requirement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				SecurityClearanceRequirement,
				#[cfg(any(
					any(
						feature = "sensory-requirement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				SensoryRequirement,
				#[cfg(any(
					any(feature = "skills-property-schema", feature = "general-schema-section"),
					doc
				))]
				Skills,
				#[cfg(any(
					any(
						feature = "special-commitments-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SpecialCommitments,
				#[cfg(any(
					any(
						feature = "subject-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SubjectOf,
				#[cfg(any(
					any(feature = "title-property-schema", feature = "general-schema-section"),
					doc
				))]
				Title,
				#[cfg(any(
					any(
						feature = "total-job-openings-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				TotalJobOpenings,
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				Url,
				#[cfg(any(
					any(
						feature = "valid-through-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ValidThrough,
				#[cfg(any(
					any(
						feature = "work-hours-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				WorkHours,
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
								feature = "applicant-location-requirements-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"applicantLocationRequirements" => Ok(Field::ApplicantLocationRequirements),
						#[cfg(any(
							any(
								feature = "application-contact-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"applicationContact" => Ok(Field::ApplicationContact),
						#[cfg(any(
							any(
								feature = "base-salary-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"baseSalary" => Ok(Field::BaseSalary),
						#[cfg(any(
							any(
								feature = "benefits-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"benefits" => Ok(Field::Benefits),
						#[cfg(any(
							any(
								feature = "date-posted-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"datePosted" => Ok(Field::DatePosted),
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
								feature = "direct-apply-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"directApply" => Ok(Field::DirectApply),
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
								feature = "education-requirements-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"educationRequirements" => Ok(Field::EducationRequirements),
						#[cfg(any(
							any(
								feature = "eligibility-to-work-requirement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"eligibilityToWorkRequirement" => Ok(Field::EligibilityToWorkRequirement),
						#[cfg(any(
							any(
								feature = "employer-overview-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"employerOverview" => Ok(Field::EmployerOverview),
						#[cfg(any(
							any(
								feature = "employment-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"employmentType" => Ok(Field::EmploymentType),
						#[cfg(any(
							any(
								feature = "employment-unit-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"employmentUnit" => Ok(Field::EmploymentUnit),
						#[cfg(any(
							any(
								feature = "estimated-salary-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"estimatedSalary" => Ok(Field::EstimatedSalary),
						#[cfg(any(
							any(
								feature = "experience-in-place-of-education-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"experienceInPlaceOfEducation" => Ok(Field::ExperienceInPlaceOfEducation),
						#[cfg(any(
							any(
								feature = "experience-requirements-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"experienceRequirements" => Ok(Field::ExperienceRequirements),
						#[cfg(any(
							any(
								feature = "hiring-organization-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"hiringOrganization" => Ok(Field::HiringOrganization),
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
								feature = "incentive-compensation-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"incentiveCompensation" => Ok(Field::IncentiveCompensation),
						#[cfg(any(
							any(
								feature = "incentives-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"incentives" => Ok(Field::Incentives),
						#[cfg(any(
							any(
								feature = "industry-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"industry" => Ok(Field::Industry),
						#[cfg(any(
							any(
								feature = "job-benefits-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"jobBenefits" => Ok(Field::JobBenefits),
						#[cfg(any(
							any(
								feature = "job-immediate-start-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"jobImmediateStart" => Ok(Field::JobImmediateStart),
						#[cfg(any(
							any(
								feature = "job-location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"jobLocation" => Ok(Field::JobLocation),
						#[cfg(any(
							any(
								feature = "job-location-type-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"jobLocationType" => Ok(Field::JobLocationType),
						#[cfg(any(
							any(
								feature = "job-start-date-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"jobStartDate" => Ok(Field::JobStartDate),
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
								feature = "occupational-category-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"occupationalCategory" => Ok(Field::OccupationalCategory),
						#[cfg(any(
							any(
								feature = "physical-requirement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"physicalRequirement" => Ok(Field::PhysicalRequirement),
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
								feature = "qualifications-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"qualifications" => Ok(Field::Qualifications),
						#[cfg(any(
							any(
								feature = "relevant-occupation-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"relevantOccupation" => Ok(Field::RelevantOccupation),
						#[cfg(any(
							any(
								feature = "responsibilities-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"responsibilities" => Ok(Field::Responsibilities),
						#[cfg(any(
							any(
								feature = "salary-currency-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"salaryCurrency" => Ok(Field::SalaryCurrency),
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
								feature = "security-clearance-requirement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"securityClearanceRequirement" => Ok(Field::SecurityClearanceRequirement),
						#[cfg(any(
							any(
								feature = "sensory-requirement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"sensoryRequirement" => Ok(Field::SensoryRequirement),
						#[cfg(any(
							any(
								feature = "skills-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"skills" => Ok(Field::Skills),
						#[cfg(any(
							any(
								feature = "special-commitments-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"specialCommitments" => Ok(Field::SpecialCommitments),
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
								feature = "title-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"title" => Ok(Field::Title),
						#[cfg(any(
							any(
								feature = "total-job-openings-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"totalJobOpenings" => Ok(Field::TotalJobOpenings),
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
								feature = "valid-through-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"validThrough" => Ok(Field::ValidThrough),
						#[cfg(any(
							any(
								feature = "work-hours-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"workHours" => Ok(Field::WorkHours),
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
								feature = "applicant-location-requirements-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"applicantLocationRequirements" => Ok(Field::ApplicantLocationRequirements),
						#[cfg(any(
							any(
								feature = "application-contact-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"applicationContact" => Ok(Field::ApplicationContact),
						#[cfg(any(
							any(
								feature = "base-salary-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"baseSalary" => Ok(Field::BaseSalary),
						#[cfg(any(
							any(
								feature = "benefits-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"benefits" => Ok(Field::Benefits),
						#[cfg(any(
							any(
								feature = "date-posted-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"datePosted" => Ok(Field::DatePosted),
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
								feature = "direct-apply-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"directApply" => Ok(Field::DirectApply),
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
								feature = "education-requirements-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"educationRequirements" => Ok(Field::EducationRequirements),
						#[cfg(any(
							any(
								feature = "eligibility-to-work-requirement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"eligibilityToWorkRequirement" => Ok(Field::EligibilityToWorkRequirement),
						#[cfg(any(
							any(
								feature = "employer-overview-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"employerOverview" => Ok(Field::EmployerOverview),
						#[cfg(any(
							any(
								feature = "employment-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"employmentType" => Ok(Field::EmploymentType),
						#[cfg(any(
							any(
								feature = "employment-unit-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"employmentUnit" => Ok(Field::EmploymentUnit),
						#[cfg(any(
							any(
								feature = "estimated-salary-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"estimatedSalary" => Ok(Field::EstimatedSalary),
						#[cfg(any(
							any(
								feature = "experience-in-place-of-education-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"experienceInPlaceOfEducation" => Ok(Field::ExperienceInPlaceOfEducation),
						#[cfg(any(
							any(
								feature = "experience-requirements-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"experienceRequirements" => Ok(Field::ExperienceRequirements),
						#[cfg(any(
							any(
								feature = "hiring-organization-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"hiringOrganization" => Ok(Field::HiringOrganization),
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
								feature = "incentive-compensation-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"incentiveCompensation" => Ok(Field::IncentiveCompensation),
						#[cfg(any(
							any(
								feature = "incentives-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"incentives" => Ok(Field::Incentives),
						#[cfg(any(
							any(
								feature = "industry-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"industry" => Ok(Field::Industry),
						#[cfg(any(
							any(
								feature = "job-benefits-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"jobBenefits" => Ok(Field::JobBenefits),
						#[cfg(any(
							any(
								feature = "job-immediate-start-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"jobImmediateStart" => Ok(Field::JobImmediateStart),
						#[cfg(any(
							any(
								feature = "job-location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"jobLocation" => Ok(Field::JobLocation),
						#[cfg(any(
							any(
								feature = "job-location-type-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"jobLocationType" => Ok(Field::JobLocationType),
						#[cfg(any(
							any(
								feature = "job-start-date-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"jobStartDate" => Ok(Field::JobStartDate),
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
								feature = "occupational-category-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"occupationalCategory" => Ok(Field::OccupationalCategory),
						#[cfg(any(
							any(
								feature = "physical-requirement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"physicalRequirement" => Ok(Field::PhysicalRequirement),
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
								feature = "qualifications-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"qualifications" => Ok(Field::Qualifications),
						#[cfg(any(
							any(
								feature = "relevant-occupation-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"relevantOccupation" => Ok(Field::RelevantOccupation),
						#[cfg(any(
							any(
								feature = "responsibilities-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"responsibilities" => Ok(Field::Responsibilities),
						#[cfg(any(
							any(
								feature = "salary-currency-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"salaryCurrency" => Ok(Field::SalaryCurrency),
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
								feature = "security-clearance-requirement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"securityClearanceRequirement" => Ok(Field::SecurityClearanceRequirement),
						#[cfg(any(
							any(
								feature = "sensory-requirement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"sensoryRequirement" => Ok(Field::SensoryRequirement),
						#[cfg(any(
							any(
								feature = "skills-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"skills" => Ok(Field::Skills),
						#[cfg(any(
							any(
								feature = "special-commitments-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"specialCommitments" => Ok(Field::SpecialCommitments),
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
								feature = "title-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"title" => Ok(Field::Title),
						#[cfg(any(
							any(
								feature = "total-job-openings-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"totalJobOpenings" => Ok(Field::TotalJobOpenings),
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
								feature = "valid-through-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"validThrough" => Ok(Field::ValidThrough),
						#[cfg(any(
							any(
								feature = "work-hours-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"workHours" => Ok(Field::WorkHours),
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
				type Value = JobPosting;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema JobPosting")
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
							feature = "applicant-location-requirements-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#applicant_location_requirements_property = None;
					#[cfg(any(
						any(
							feature = "application-contact-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#application_contact_property = None;
					#[cfg(any(
						any(
							feature = "base-salary-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#base_salary_property = None;
					#[cfg(any(
						any(
							feature = "benefits-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#benefits_property = None;
					#[cfg(any(
						any(
							feature = "date-posted-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#date_posted_property = None;
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
							feature = "direct-apply-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#direct_apply_property = None;
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
							feature = "education-requirements-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#education_requirements_property = None;
					#[cfg(any(
						any(
							feature = "eligibility-to-work-requirement-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#eligibility_to_work_requirement_property = None;
					#[cfg(any(
						any(
							feature = "employer-overview-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#employer_overview_property = None;
					#[cfg(any(
						any(
							feature = "employment-type-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#employment_type_property = None;
					#[cfg(any(
						any(
							feature = "employment-unit-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#employment_unit_property = None;
					#[cfg(any(
						any(
							feature = "estimated-salary-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#estimated_salary_property = None;
					#[cfg(any(
						any(
							feature = "experience-in-place-of-education-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#experience_in_place_of_education_property = None;
					#[cfg(any(
						any(
							feature = "experience-requirements-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#experience_requirements_property = None;
					#[cfg(any(
						any(
							feature = "hiring-organization-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#hiring_organization_property = None;
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
							feature = "incentive-compensation-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#incentive_compensation_property = None;
					#[cfg(any(
						any(
							feature = "incentives-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#incentives_property = None;
					#[cfg(any(
						any(
							feature = "industry-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#industry_property = None;
					#[cfg(any(
						any(
							feature = "job-benefits-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#job_benefits_property = None;
					#[cfg(any(
						any(
							feature = "job-immediate-start-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#job_immediate_start_property = None;
					#[cfg(any(
						any(
							feature = "job-location-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#job_location_property = None;
					#[cfg(any(
						any(
							feature = "job-location-type-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#job_location_type_property = None;
					#[cfg(any(
						any(
							feature = "job-start-date-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#job_start_date_property = None;
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
							feature = "occupational-category-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#occupational_category_property = None;
					#[cfg(any(
						any(
							feature = "physical-requirement-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#physical_requirement_property = None;
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
							feature = "qualifications-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#qualifications_property = None;
					#[cfg(any(
						any(
							feature = "relevant-occupation-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#relevant_occupation_property = None;
					#[cfg(any(
						any(
							feature = "responsibilities-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#responsibilities_property = None;
					#[cfg(any(
						any(
							feature = "salary-currency-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#salary_currency_property = None;
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
							feature = "security-clearance-requirement-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#security_clearance_requirement_property = None;
					#[cfg(any(
						any(
							feature = "sensory-requirement-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#sensory_requirement_property = None;
					#[cfg(any(
						any(
							feature = "skills-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#skills_property = None;
					#[cfg(any(
						any(
							feature = "special-commitments-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#special_commitments_property = None;
					#[cfg(any(
						any(
							feature = "subject-of-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#subject_of_property = None;
					#[cfg(any(
						any(feature = "title-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#title_property = None;
					#[cfg(any(
						any(
							feature = "total-job-openings-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#total_job_openings_property = None;
					#[cfg(any(
						any(feature = "url-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#url_property = None;
					#[cfg(any(
						any(
							feature = "valid-through-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#valid_through_property = None;
					#[cfg(any(
						any(
							feature = "work-hours-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#work_hours_property = None;
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
									feature = "applicant-location-requirements-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ApplicantLocationRequirements => {
								if r#applicant_location_requirements_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"applicantLocationRequirements",
									));
								}
								r#applicant_location_requirements_property = Some({
									struct DeserializeWith(
										Vec<ApplicantLocationRequirementsProperty>,
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
									feature = "application-contact-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ApplicationContact => {
								if r#application_contact_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"applicationContact",
									));
								}
								r#application_contact_property = Some({
									struct DeserializeWith(Vec<ApplicationContactProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "base-salary-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::BaseSalary => {
								if r#base_salary_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"baseSalary",
									));
								}
								r#base_salary_property = Some({
									struct DeserializeWith(Vec<BaseSalaryProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "benefits-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Benefits => {
								if r#benefits_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"benefits",
									));
								}
								r#benefits_property = Some({
									struct DeserializeWith(Vec<BenefitsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "date-posted-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::DatePosted => {
								if r#date_posted_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"datePosted",
									));
								}
								r#date_posted_property = Some({
									struct DeserializeWith(Vec<DatePostedProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "direct-apply-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::DirectApply => {
								if r#direct_apply_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"directApply",
									));
								}
								r#direct_apply_property = Some({
									struct DeserializeWith(Vec<DirectApplyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "education-requirements-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::EducationRequirements => {
								if r#education_requirements_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"educationRequirements",
									));
								}
								r#education_requirements_property = Some({
									struct DeserializeWith(Vec<EducationRequirementsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "eligibility-to-work-requirement-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::EligibilityToWorkRequirement => {
								if r#eligibility_to_work_requirement_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"eligibilityToWorkRequirement",
									));
								}
								r#eligibility_to_work_requirement_property = Some({
									struct DeserializeWith(
										Vec<EligibilityToWorkRequirementProperty>,
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
									feature = "employer-overview-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::EmployerOverview => {
								if r#employer_overview_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"employerOverview",
									));
								}
								r#employer_overview_property = Some({
									struct DeserializeWith(Vec<EmployerOverviewProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "employment-type-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::EmploymentType => {
								if r#employment_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"employmentType",
									));
								}
								r#employment_type_property = Some({
									struct DeserializeWith(Vec<EmploymentTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "employment-unit-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::EmploymentUnit => {
								if r#employment_unit_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"employmentUnit",
									));
								}
								r#employment_unit_property = Some({
									struct DeserializeWith(Vec<EmploymentUnitProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "estimated-salary-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::EstimatedSalary => {
								if r#estimated_salary_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"estimatedSalary",
									));
								}
								r#estimated_salary_property = Some({
									struct DeserializeWith(Vec<EstimatedSalaryProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "experience-in-place-of-education-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ExperienceInPlaceOfEducation => {
								if r#experience_in_place_of_education_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"experienceInPlaceOfEducation",
									));
								}
								r#experience_in_place_of_education_property = Some({
									struct DeserializeWith(
										Vec<ExperienceInPlaceOfEducationProperty>,
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
									feature = "experience-requirements-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ExperienceRequirements => {
								if r#experience_requirements_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"experienceRequirements",
									));
								}
								r#experience_requirements_property = Some({
									struct DeserializeWith(Vec<ExperienceRequirementsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "hiring-organization-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::HiringOrganization => {
								if r#hiring_organization_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hiringOrganization",
									));
								}
								r#hiring_organization_property = Some({
									struct DeserializeWith(Vec<HiringOrganizationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "incentive-compensation-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::IncentiveCompensation => {
								if r#incentive_compensation_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"incentiveCompensation",
									));
								}
								r#incentive_compensation_property = Some({
									struct DeserializeWith(Vec<IncentiveCompensationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "incentives-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Incentives => {
								if r#incentives_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"incentives",
									));
								}
								r#incentives_property = Some({
									struct DeserializeWith(Vec<IncentivesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "industry-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Industry => {
								if r#industry_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"industry",
									));
								}
								r#industry_property = Some({
									struct DeserializeWith(Vec<IndustryProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "job-benefits-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::JobBenefits => {
								if r#job_benefits_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"jobBenefits",
									));
								}
								r#job_benefits_property = Some({
									struct DeserializeWith(Vec<JobBenefitsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "job-immediate-start-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::JobImmediateStart => {
								if r#job_immediate_start_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"jobImmediateStart",
									));
								}
								r#job_immediate_start_property = Some({
									struct DeserializeWith(Vec<JobImmediateStartProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "job-location-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::JobLocation => {
								if r#job_location_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"jobLocation",
									));
								}
								r#job_location_property = Some({
									struct DeserializeWith(Vec<JobLocationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "job-location-type-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::JobLocationType => {
								if r#job_location_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"jobLocationType",
									));
								}
								r#job_location_type_property = Some({
									struct DeserializeWith(Vec<JobLocationTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "job-start-date-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::JobStartDate => {
								if r#job_start_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"jobStartDate",
									));
								}
								r#job_start_date_property = Some({
									struct DeserializeWith(Vec<JobStartDateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "occupational-category-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::OccupationalCategory => {
								if r#occupational_category_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"occupationalCategory",
									));
								}
								r#occupational_category_property = Some({
									struct DeserializeWith(Vec<OccupationalCategoryProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "physical-requirement-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::PhysicalRequirement => {
								if r#physical_requirement_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"physicalRequirement",
									));
								}
								r#physical_requirement_property = Some({
									struct DeserializeWith(Vec<PhysicalRequirementProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "qualifications-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::Qualifications => {
								if r#qualifications_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"qualifications",
									));
								}
								r#qualifications_property = Some({
									struct DeserializeWith(Vec<QualificationsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "relevant-occupation-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::RelevantOccupation => {
								if r#relevant_occupation_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"relevantOccupation",
									));
								}
								r#relevant_occupation_property = Some({
									struct DeserializeWith(Vec<RelevantOccupationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "responsibilities-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Responsibilities => {
								if r#responsibilities_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"responsibilities",
									));
								}
								r#responsibilities_property = Some({
									struct DeserializeWith(Vec<ResponsibilitiesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "salary-currency-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::SalaryCurrency => {
								if r#salary_currency_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"salaryCurrency",
									));
								}
								r#salary_currency_property = Some({
									struct DeserializeWith(Vec<SalaryCurrencyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "security-clearance-requirement-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::SecurityClearanceRequirement => {
								if r#security_clearance_requirement_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"securityClearanceRequirement",
									));
								}
								r#security_clearance_requirement_property = Some({
									struct DeserializeWith(
										Vec<SecurityClearanceRequirementProperty>,
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
									feature = "sensory-requirement-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::SensoryRequirement => {
								if r#sensory_requirement_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"sensoryRequirement",
									));
								}
								r#sensory_requirement_property = Some({
									struct DeserializeWith(Vec<SensoryRequirementProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "skills-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Skills => {
								if r#skills_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("skills"));
								}
								r#skills_property = Some({
									struct DeserializeWith(Vec<SkillsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "special-commitments-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::SpecialCommitments => {
								if r#special_commitments_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"specialCommitments",
									));
								}
								r#special_commitments_property = Some({
									struct DeserializeWith(Vec<SpecialCommitmentsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "title-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Title => {
								if r#title_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("title"));
								}
								r#title_property = Some({
									struct DeserializeWith(Vec<TitleProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "total-job-openings-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::TotalJobOpenings => {
								if r#total_job_openings_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"totalJobOpenings",
									));
								}
								r#total_job_openings_property = Some({
									struct DeserializeWith(Vec<TotalJobOpeningsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "valid-through-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "work-hours-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::WorkHours => {
								if r#work_hours_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"workHours",
									));
								}
								r#work_hours_property = Some({
									struct DeserializeWith(Vec<WorkHoursProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
					Ok(JobPosting {
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
								feature = "applicant-location-requirements-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#applicant_location_requirements:
							r#applicant_location_requirements_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "application-contact-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#application_contact: r#application_contact_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "base-salary-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#base_salary: r#base_salary_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "benefits-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#benefits: r#benefits_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "date-posted-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#date_posted: r#date_posted_property.unwrap_or_default(),
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
								feature = "direct-apply-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#direct_apply: r#direct_apply_property.unwrap_or_default(),
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
								feature = "education-requirements-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#education_requirements: r#education_requirements_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "eligibility-to-work-requirement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#eligibility_to_work_requirement:
							r#eligibility_to_work_requirement_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "employer-overview-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#employer_overview: r#employer_overview_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "employment-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#employment_type: r#employment_type_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "employment-unit-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#employment_unit: r#employment_unit_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "estimated-salary-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#estimated_salary: r#estimated_salary_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "experience-in-place-of-education-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#experience_in_place_of_education:
							r#experience_in_place_of_education_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "experience-requirements-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#experience_requirements: r#experience_requirements_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "hiring-organization-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#hiring_organization: r#hiring_organization_property.unwrap_or_default(),
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
								feature = "incentive-compensation-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#incentive_compensation: r#incentive_compensation_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "incentives-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#incentives: r#incentives_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "industry-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#industry: r#industry_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "job-benefits-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#job_benefits: r#job_benefits_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "job-immediate-start-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#job_immediate_start: r#job_immediate_start_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "job-location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#job_location: r#job_location_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "job-location-type-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#job_location_type: r#job_location_type_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "job-start-date-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#job_start_date: r#job_start_date_property.unwrap_or_default(),
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
								feature = "occupational-category-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#occupational_category: r#occupational_category_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "physical-requirement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#physical_requirement: r#physical_requirement_property.unwrap_or_default(),
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
								feature = "qualifications-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#qualifications: r#qualifications_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "relevant-occupation-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#relevant_occupation: r#relevant_occupation_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "responsibilities-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#responsibilities: r#responsibilities_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "salary-currency-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#salary_currency: r#salary_currency_property.unwrap_or_default(),
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
								feature = "security-clearance-requirement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#security_clearance_requirement: r#security_clearance_requirement_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "sensory-requirement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#sensory_requirement: r#sensory_requirement_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "skills-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#skills: r#skills_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "special-commitments-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#special_commitments: r#special_commitments_property.unwrap_or_default(),
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
								feature = "title-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#title: r#title_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "total-job-openings-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#total_job_openings: r#total_job_openings_property.unwrap_or_default(),
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
								feature = "valid-through-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#valid_through: r#valid_through_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "work-hours-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#work_hours: r#work_hours_property.unwrap_or_default(),
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
						feature = "applicant-location-requirements-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"applicantLocationRequirements",
				#[cfg(any(
					any(
						feature = "application-contact-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"applicationContact",
				#[cfg(any(
					any(
						feature = "base-salary-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"baseSalary",
				#[cfg(any(
					any(
						feature = "benefits-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"benefits",
				#[cfg(any(
					any(
						feature = "date-posted-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"datePosted",
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
						feature = "direct-apply-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"directApply",
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
						feature = "education-requirements-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"educationRequirements",
				#[cfg(any(
					any(
						feature = "eligibility-to-work-requirement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"eligibilityToWorkRequirement",
				#[cfg(any(
					any(
						feature = "employer-overview-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"employerOverview",
				#[cfg(any(
					any(
						feature = "employment-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"employmentType",
				#[cfg(any(
					any(
						feature = "employment-unit-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"employmentUnit",
				#[cfg(any(
					any(
						feature = "estimated-salary-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"estimatedSalary",
				#[cfg(any(
					any(
						feature = "experience-in-place-of-education-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"experienceInPlaceOfEducation",
				#[cfg(any(
					any(
						feature = "experience-requirements-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"experienceRequirements",
				#[cfg(any(
					any(
						feature = "hiring-organization-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"hiringOrganization",
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
						feature = "incentive-compensation-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"incentiveCompensation",
				#[cfg(any(
					any(
						feature = "incentives-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"incentives",
				#[cfg(any(
					any(
						feature = "industry-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"industry",
				#[cfg(any(
					any(
						feature = "job-benefits-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"jobBenefits",
				#[cfg(any(
					any(
						feature = "job-immediate-start-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"jobImmediateStart",
				#[cfg(any(
					any(
						feature = "job-location-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"jobLocation",
				#[cfg(any(
					any(
						feature = "job-location-type-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"jobLocationType",
				#[cfg(any(
					any(
						feature = "job-start-date-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"jobStartDate",
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
						feature = "occupational-category-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"occupationalCategory",
				#[cfg(any(
					any(
						feature = "physical-requirement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"physicalRequirement",
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
						feature = "qualifications-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"qualifications",
				#[cfg(any(
					any(
						feature = "relevant-occupation-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"relevantOccupation",
				#[cfg(any(
					any(
						feature = "responsibilities-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"responsibilities",
				#[cfg(any(
					any(
						feature = "salary-currency-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"salaryCurrency",
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
						feature = "security-clearance-requirement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"securityClearanceRequirement",
				#[cfg(any(
					any(
						feature = "sensory-requirement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"sensoryRequirement",
				#[cfg(any(
					any(feature = "skills-property-schema", feature = "general-schema-section"),
					doc
				))]
				"skills",
				#[cfg(any(
					any(
						feature = "special-commitments-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"specialCommitments",
				#[cfg(any(
					any(
						feature = "subject-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"subjectOf",
				#[cfg(any(
					any(feature = "title-property-schema", feature = "general-schema-section"),
					doc
				))]
				"title",
				#[cfg(any(
					any(
						feature = "total-job-openings-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"totalJobOpenings",
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				"url",
				#[cfg(any(
					any(
						feature = "valid-through-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"validThrough",
				#[cfg(any(
					any(
						feature = "work-hours-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"workHours",
			];
			deserializer.deserialize_struct("JobPosting", FIELDS, ClassVisitor)
		}
	}
}
