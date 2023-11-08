use super::*;
/// <https://schema.org/JobPosting>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct JobPosting {
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	pub r#alternate_name: Vec<AlternateNameProperty>,
	pub r#applicant_location_requirements: Vec<ApplicantLocationRequirementsProperty>,
	pub r#application_contact: Vec<ApplicationContactProperty>,
	pub r#base_salary: Vec<BaseSalaryProperty>,
	pub r#benefits: Vec<BenefitsProperty>,
	pub r#date_posted: Vec<DatePostedProperty>,
	pub r#description: Vec<DescriptionProperty>,
	pub r#direct_apply: Vec<DirectApplyProperty>,
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	pub r#education_requirements: Vec<EducationRequirementsProperty>,
	pub r#eligibility_to_work_requirement: Vec<EligibilityToWorkRequirementProperty>,
	pub r#employer_overview: Vec<EmployerOverviewProperty>,
	pub r#employment_type: Vec<EmploymentTypeProperty>,
	pub r#employment_unit: Vec<EmploymentUnitProperty>,
	pub r#estimated_salary: Vec<EstimatedSalaryProperty>,
	pub r#experience_in_place_of_education: Vec<ExperienceInPlaceOfEducationProperty>,
	pub r#experience_requirements: Vec<ExperienceRequirementsProperty>,
	pub r#hiring_organization: Vec<HiringOrganizationProperty>,
	pub r#identifier: Vec<IdentifierProperty>,
	pub r#image: Vec<ImageProperty>,
	pub r#incentive_compensation: Vec<IncentiveCompensationProperty>,
	pub r#incentives: Vec<IncentivesProperty>,
	pub r#industry: Vec<IndustryProperty>,
	pub r#job_benefits: Vec<JobBenefitsProperty>,
	pub r#job_immediate_start: Vec<JobImmediateStartProperty>,
	pub r#job_location: Vec<JobLocationProperty>,
	pub r#job_location_type: Vec<JobLocationTypeProperty>,
	pub r#job_start_date: Vec<JobStartDateProperty>,
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	pub r#name: Vec<NameProperty>,
	pub r#occupational_category: Vec<OccupationalCategoryProperty>,
	pub r#physical_requirement: Vec<PhysicalRequirementProperty>,
	pub r#potential_action: Vec<PotentialActionProperty>,
	pub r#qualifications: Vec<QualificationsProperty>,
	pub r#relevant_occupation: Vec<RelevantOccupationProperty>,
	pub r#responsibilities: Vec<ResponsibilitiesProperty>,
	pub r#salary_currency: Vec<SalaryCurrencyProperty>,
	pub r#same_as: Vec<SameAsProperty>,
	pub r#security_clearance_requirement: Vec<SecurityClearanceRequirementProperty>,
	pub r#sensory_requirement: Vec<SensoryRequirementProperty>,
	pub r#skills: Vec<SkillsProperty>,
	pub r#special_commitments: Vec<SpecialCommitmentsProperty>,
	pub r#subject_of: Vec<SubjectOfProperty>,
	pub r#title: Vec<TitleProperty>,
	pub r#total_job_openings: Vec<TotalJobOpeningsProperty>,
	pub r#url: Vec<UrlProperty>,
	pub r#valid_through: Vec<ValidThroughProperty>,
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
				!Vec::is_empty(&self.r#additional_type) as usize,
				!Vec::is_empty(&self.r#alternate_name) as usize,
				!Vec::is_empty(&self.r#applicant_location_requirements) as usize,
				!Vec::is_empty(&self.r#application_contact) as usize,
				!Vec::is_empty(&self.r#base_salary) as usize,
				!Vec::is_empty(&self.r#benefits) as usize,
				!Vec::is_empty(&self.r#date_posted) as usize,
				!Vec::is_empty(&self.r#description) as usize,
				!Vec::is_empty(&self.r#direct_apply) as usize,
				!Vec::is_empty(&self.r#disambiguating_description) as usize,
				!Vec::is_empty(&self.r#education_requirements) as usize,
				!Vec::is_empty(&self.r#eligibility_to_work_requirement) as usize,
				!Vec::is_empty(&self.r#employer_overview) as usize,
				!Vec::is_empty(&self.r#employment_type) as usize,
				!Vec::is_empty(&self.r#employment_unit) as usize,
				!Vec::is_empty(&self.r#estimated_salary) as usize,
				!Vec::is_empty(&self.r#experience_in_place_of_education) as usize,
				!Vec::is_empty(&self.r#experience_requirements) as usize,
				!Vec::is_empty(&self.r#hiring_organization) as usize,
				!Vec::is_empty(&self.r#identifier) as usize,
				!Vec::is_empty(&self.r#image) as usize,
				!Vec::is_empty(&self.r#incentive_compensation) as usize,
				!Vec::is_empty(&self.r#incentives) as usize,
				!Vec::is_empty(&self.r#industry) as usize,
				!Vec::is_empty(&self.r#job_benefits) as usize,
				!Vec::is_empty(&self.r#job_immediate_start) as usize,
				!Vec::is_empty(&self.r#job_location) as usize,
				!Vec::is_empty(&self.r#job_location_type) as usize,
				!Vec::is_empty(&self.r#job_start_date) as usize,
				!Vec::is_empty(&self.r#main_entity_of_page) as usize,
				!Vec::is_empty(&self.r#name) as usize,
				!Vec::is_empty(&self.r#occupational_category) as usize,
				!Vec::is_empty(&self.r#physical_requirement) as usize,
				!Vec::is_empty(&self.r#potential_action) as usize,
				!Vec::is_empty(&self.r#qualifications) as usize,
				!Vec::is_empty(&self.r#relevant_occupation) as usize,
				!Vec::is_empty(&self.r#responsibilities) as usize,
				!Vec::is_empty(&self.r#salary_currency) as usize,
				!Vec::is_empty(&self.r#same_as) as usize,
				!Vec::is_empty(&self.r#security_clearance_requirement) as usize,
				!Vec::is_empty(&self.r#sensory_requirement) as usize,
				!Vec::is_empty(&self.r#skills) as usize,
				!Vec::is_empty(&self.r#special_commitments) as usize,
				!Vec::is_empty(&self.r#subject_of) as usize,
				!Vec::is_empty(&self.r#title) as usize,
				!Vec::is_empty(&self.r#total_job_openings) as usize,
				!Vec::is_empty(&self.r#url) as usize,
				!Vec::is_empty(&self.r#valid_through) as usize,
				!Vec::is_empty(&self.r#work_hours) as usize,
			]
			.iter()
			.sum();
			let mut serialize_struct = Serializer::serialize_struct(serializer, "JobPosting", len)?;
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
				AdditionalType,
				AlternateName,
				ApplicantLocationRequirements,
				ApplicationContact,
				BaseSalary,
				Benefits,
				DatePosted,
				Description,
				DirectApply,
				DisambiguatingDescription,
				EducationRequirements,
				EligibilityToWorkRequirement,
				EmployerOverview,
				EmploymentType,
				EmploymentUnit,
				EstimatedSalary,
				ExperienceInPlaceOfEducation,
				ExperienceRequirements,
				HiringOrganization,
				Identifier,
				Image,
				IncentiveCompensation,
				Incentives,
				Industry,
				JobBenefits,
				JobImmediateStart,
				JobLocation,
				JobLocationType,
				JobStartDate,
				MainEntityOfPage,
				Name,
				OccupationalCategory,
				PhysicalRequirement,
				PotentialAction,
				Qualifications,
				RelevantOccupation,
				Responsibilities,
				SalaryCurrency,
				SameAs,
				SecurityClearanceRequirement,
				SensoryRequirement,
				Skills,
				SpecialCommitments,
				SubjectOf,
				Title,
				TotalJobOpenings,
				Url,
				ValidThrough,
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
						"additionalType" => Ok(Field::AdditionalType),
						"alternateName" => Ok(Field::AlternateName),
						"applicantLocationRequirements" => Ok(Field::ApplicantLocationRequirements),
						"applicationContact" => Ok(Field::ApplicationContact),
						"baseSalary" => Ok(Field::BaseSalary),
						"benefits" => Ok(Field::Benefits),
						"datePosted" => Ok(Field::DatePosted),
						"description" => Ok(Field::Description),
						"directApply" => Ok(Field::DirectApply),
						"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						"educationRequirements" => Ok(Field::EducationRequirements),
						"eligibilityToWorkRequirement" => Ok(Field::EligibilityToWorkRequirement),
						"employerOverview" => Ok(Field::EmployerOverview),
						"employmentType" => Ok(Field::EmploymentType),
						"employmentUnit" => Ok(Field::EmploymentUnit),
						"estimatedSalary" => Ok(Field::EstimatedSalary),
						"experienceInPlaceOfEducation" => Ok(Field::ExperienceInPlaceOfEducation),
						"experienceRequirements" => Ok(Field::ExperienceRequirements),
						"hiringOrganization" => Ok(Field::HiringOrganization),
						"identifier" => Ok(Field::Identifier),
						"image" => Ok(Field::Image),
						"incentiveCompensation" => Ok(Field::IncentiveCompensation),
						"incentives" => Ok(Field::Incentives),
						"industry" => Ok(Field::Industry),
						"jobBenefits" => Ok(Field::JobBenefits),
						"jobImmediateStart" => Ok(Field::JobImmediateStart),
						"jobLocation" => Ok(Field::JobLocation),
						"jobLocationType" => Ok(Field::JobLocationType),
						"jobStartDate" => Ok(Field::JobStartDate),
						"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						"name" => Ok(Field::Name),
						"occupationalCategory" => Ok(Field::OccupationalCategory),
						"physicalRequirement" => Ok(Field::PhysicalRequirement),
						"potentialAction" => Ok(Field::PotentialAction),
						"qualifications" => Ok(Field::Qualifications),
						"relevantOccupation" => Ok(Field::RelevantOccupation),
						"responsibilities" => Ok(Field::Responsibilities),
						"salaryCurrency" => Ok(Field::SalaryCurrency),
						"sameAs" => Ok(Field::SameAs),
						"securityClearanceRequirement" => Ok(Field::SecurityClearanceRequirement),
						"sensoryRequirement" => Ok(Field::SensoryRequirement),
						"skills" => Ok(Field::Skills),
						"specialCommitments" => Ok(Field::SpecialCommitments),
						"subjectOf" => Ok(Field::SubjectOf),
						"title" => Ok(Field::Title),
						"totalJobOpenings" => Ok(Field::TotalJobOpenings),
						"url" => Ok(Field::Url),
						"validThrough" => Ok(Field::ValidThrough),
						"workHours" => Ok(Field::WorkHours),
						_ => Ok(Field::Ignore),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"additionalType" => Ok(Field::AdditionalType),
						b"alternateName" => Ok(Field::AlternateName),
						b"applicantLocationRequirements" => {
							Ok(Field::ApplicantLocationRequirements)
						}
						b"applicationContact" => Ok(Field::ApplicationContact),
						b"baseSalary" => Ok(Field::BaseSalary),
						b"benefits" => Ok(Field::Benefits),
						b"datePosted" => Ok(Field::DatePosted),
						b"description" => Ok(Field::Description),
						b"directApply" => Ok(Field::DirectApply),
						b"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						b"educationRequirements" => Ok(Field::EducationRequirements),
						b"eligibilityToWorkRequirement" => Ok(Field::EligibilityToWorkRequirement),
						b"employerOverview" => Ok(Field::EmployerOverview),
						b"employmentType" => Ok(Field::EmploymentType),
						b"employmentUnit" => Ok(Field::EmploymentUnit),
						b"estimatedSalary" => Ok(Field::EstimatedSalary),
						b"experienceInPlaceOfEducation" => Ok(Field::ExperienceInPlaceOfEducation),
						b"experienceRequirements" => Ok(Field::ExperienceRequirements),
						b"hiringOrganization" => Ok(Field::HiringOrganization),
						b"identifier" => Ok(Field::Identifier),
						b"image" => Ok(Field::Image),
						b"incentiveCompensation" => Ok(Field::IncentiveCompensation),
						b"incentives" => Ok(Field::Incentives),
						b"industry" => Ok(Field::Industry),
						b"jobBenefits" => Ok(Field::JobBenefits),
						b"jobImmediateStart" => Ok(Field::JobImmediateStart),
						b"jobLocation" => Ok(Field::JobLocation),
						b"jobLocationType" => Ok(Field::JobLocationType),
						b"jobStartDate" => Ok(Field::JobStartDate),
						b"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						b"name" => Ok(Field::Name),
						b"occupationalCategory" => Ok(Field::OccupationalCategory),
						b"physicalRequirement" => Ok(Field::PhysicalRequirement),
						b"potentialAction" => Ok(Field::PotentialAction),
						b"qualifications" => Ok(Field::Qualifications),
						b"relevantOccupation" => Ok(Field::RelevantOccupation),
						b"responsibilities" => Ok(Field::Responsibilities),
						b"salaryCurrency" => Ok(Field::SalaryCurrency),
						b"sameAs" => Ok(Field::SameAs),
						b"securityClearanceRequirement" => Ok(Field::SecurityClearanceRequirement),
						b"sensoryRequirement" => Ok(Field::SensoryRequirement),
						b"skills" => Ok(Field::Skills),
						b"specialCommitments" => Ok(Field::SpecialCommitments),
						b"subjectOf" => Ok(Field::SubjectOf),
						b"title" => Ok(Field::Title),
						b"totalJobOpenings" => Ok(Field::TotalJobOpenings),
						b"url" => Ok(Field::Url),
						b"validThrough" => Ok(Field::ValidThrough),
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
					let mut r#additional_type_property = None;
					let mut r#alternate_name_property = None;
					let mut r#applicant_location_requirements_property = None;
					let mut r#application_contact_property = None;
					let mut r#base_salary_property = None;
					let mut r#benefits_property = None;
					let mut r#date_posted_property = None;
					let mut r#description_property = None;
					let mut r#direct_apply_property = None;
					let mut r#disambiguating_description_property = None;
					let mut r#education_requirements_property = None;
					let mut r#eligibility_to_work_requirement_property = None;
					let mut r#employer_overview_property = None;
					let mut r#employment_type_property = None;
					let mut r#employment_unit_property = None;
					let mut r#estimated_salary_property = None;
					let mut r#experience_in_place_of_education_property = None;
					let mut r#experience_requirements_property = None;
					let mut r#hiring_organization_property = None;
					let mut r#identifier_property = None;
					let mut r#image_property = None;
					let mut r#incentive_compensation_property = None;
					let mut r#incentives_property = None;
					let mut r#industry_property = None;
					let mut r#job_benefits_property = None;
					let mut r#job_immediate_start_property = None;
					let mut r#job_location_property = None;
					let mut r#job_location_type_property = None;
					let mut r#job_start_date_property = None;
					let mut r#main_entity_of_page_property = None;
					let mut r#name_property = None;
					let mut r#occupational_category_property = None;
					let mut r#physical_requirement_property = None;
					let mut r#potential_action_property = None;
					let mut r#qualifications_property = None;
					let mut r#relevant_occupation_property = None;
					let mut r#responsibilities_property = None;
					let mut r#salary_currency_property = None;
					let mut r#same_as_property = None;
					let mut r#security_clearance_requirement_property = None;
					let mut r#sensory_requirement_property = None;
					let mut r#skills_property = None;
					let mut r#special_commitments_property = None;
					let mut r#subject_of_property = None;
					let mut r#title_property = None;
					let mut r#total_job_openings_property = None;
					let mut r#url_property = None;
					let mut r#valid_through_property = None;
					let mut r#work_hours_property = None;
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
						r#additional_type: r#additional_type_property.unwrap_or_default(),
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
						r#applicant_location_requirements:
							r#applicant_location_requirements_property.unwrap_or_default(),
						r#application_contact: r#application_contact_property.unwrap_or_default(),
						r#base_salary: r#base_salary_property.unwrap_or_default(),
						r#benefits: r#benefits_property.unwrap_or_default(),
						r#date_posted: r#date_posted_property.unwrap_or_default(),
						r#description: r#description_property.unwrap_or_default(),
						r#direct_apply: r#direct_apply_property.unwrap_or_default(),
						r#disambiguating_description: r#disambiguating_description_property
							.unwrap_or_default(),
						r#education_requirements: r#education_requirements_property
							.unwrap_or_default(),
						r#eligibility_to_work_requirement:
							r#eligibility_to_work_requirement_property.unwrap_or_default(),
						r#employer_overview: r#employer_overview_property.unwrap_or_default(),
						r#employment_type: r#employment_type_property.unwrap_or_default(),
						r#employment_unit: r#employment_unit_property.unwrap_or_default(),
						r#estimated_salary: r#estimated_salary_property.unwrap_or_default(),
						r#experience_in_place_of_education:
							r#experience_in_place_of_education_property.unwrap_or_default(),
						r#experience_requirements: r#experience_requirements_property
							.unwrap_or_default(),
						r#hiring_organization: r#hiring_organization_property.unwrap_or_default(),
						r#identifier: r#identifier_property.unwrap_or_default(),
						r#image: r#image_property.unwrap_or_default(),
						r#incentive_compensation: r#incentive_compensation_property
							.unwrap_or_default(),
						r#incentives: r#incentives_property.unwrap_or_default(),
						r#industry: r#industry_property.unwrap_or_default(),
						r#job_benefits: r#job_benefits_property.unwrap_or_default(),
						r#job_immediate_start: r#job_immediate_start_property.unwrap_or_default(),
						r#job_location: r#job_location_property.unwrap_or_default(),
						r#job_location_type: r#job_location_type_property.unwrap_or_default(),
						r#job_start_date: r#job_start_date_property.unwrap_or_default(),
						r#main_entity_of_page: r#main_entity_of_page_property.unwrap_or_default(),
						r#name: r#name_property.unwrap_or_default(),
						r#occupational_category: r#occupational_category_property
							.unwrap_or_default(),
						r#physical_requirement: r#physical_requirement_property.unwrap_or_default(),
						r#potential_action: r#potential_action_property.unwrap_or_default(),
						r#qualifications: r#qualifications_property.unwrap_or_default(),
						r#relevant_occupation: r#relevant_occupation_property.unwrap_or_default(),
						r#responsibilities: r#responsibilities_property.unwrap_or_default(),
						r#salary_currency: r#salary_currency_property.unwrap_or_default(),
						r#same_as: r#same_as_property.unwrap_or_default(),
						r#security_clearance_requirement: r#security_clearance_requirement_property
							.unwrap_or_default(),
						r#sensory_requirement: r#sensory_requirement_property.unwrap_or_default(),
						r#skills: r#skills_property.unwrap_or_default(),
						r#special_commitments: r#special_commitments_property.unwrap_or_default(),
						r#subject_of: r#subject_of_property.unwrap_or_default(),
						r#title: r#title_property.unwrap_or_default(),
						r#total_job_openings: r#total_job_openings_property.unwrap_or_default(),
						r#url: r#url_property.unwrap_or_default(),
						r#valid_through: r#valid_through_property.unwrap_or_default(),
						r#work_hours: r#work_hours_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				"additionalType",
				"alternateName",
				"applicantLocationRequirements",
				"applicationContact",
				"baseSalary",
				"benefits",
				"datePosted",
				"description",
				"directApply",
				"disambiguatingDescription",
				"educationRequirements",
				"eligibilityToWorkRequirement",
				"employerOverview",
				"employmentType",
				"employmentUnit",
				"estimatedSalary",
				"experienceInPlaceOfEducation",
				"experienceRequirements",
				"hiringOrganization",
				"identifier",
				"image",
				"incentiveCompensation",
				"incentives",
				"industry",
				"jobBenefits",
				"jobImmediateStart",
				"jobLocation",
				"jobLocationType",
				"jobStartDate",
				"mainEntityOfPage",
				"name",
				"occupationalCategory",
				"physicalRequirement",
				"potentialAction",
				"qualifications",
				"relevantOccupation",
				"responsibilities",
				"salaryCurrency",
				"sameAs",
				"securityClearanceRequirement",
				"sensoryRequirement",
				"skills",
				"specialCommitments",
				"subjectOf",
				"title",
				"totalJobOpenings",
				"url",
				"validThrough",
				"workHours",
			];
			deserializer.deserialize_struct("JobPosting", FIELDS, ClassVisitor)
		}
	}
}
