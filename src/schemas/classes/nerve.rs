use super::*;
/// <https://schema.org/Nerve>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Nerve {
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	pub r#alternate_name: Vec<AlternateNameProperty>,
	pub r#associated_pathophysiology: Vec<AssociatedPathophysiologyProperty>,
	pub r#body_location: Vec<BodyLocationProperty>,
	pub r#branch: Vec<BranchProperty>,
	pub r#code: Vec<CodeProperty>,
	pub r#connected_to: Vec<ConnectedToProperty>,
	pub r#description: Vec<DescriptionProperty>,
	pub r#diagram: Vec<DiagramProperty>,
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	pub r#funding: Vec<FundingProperty>,
	pub r#guideline: Vec<GuidelineProperty>,
	pub r#identifier: Vec<IdentifierProperty>,
	pub r#image: Vec<ImageProperty>,
	pub r#legal_status: Vec<LegalStatusProperty>,
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	pub r#medicine_system: Vec<MedicineSystemProperty>,
	pub r#name: Vec<NameProperty>,
	pub r#nerve_motor: Vec<NerveMotorProperty>,
	pub r#part_of_system: Vec<PartOfSystemProperty>,
	pub r#potential_action: Vec<PotentialActionProperty>,
	pub r#recognizing_authority: Vec<RecognizingAuthorityProperty>,
	pub r#related_condition: Vec<RelatedConditionProperty>,
	pub r#related_therapy: Vec<RelatedTherapyProperty>,
	pub r#relevant_specialty: Vec<RelevantSpecialtyProperty>,
	pub r#same_as: Vec<SameAsProperty>,
	pub r#sensory_unit: Vec<SensoryUnitProperty>,
	pub r#sourced_from: Vec<SourcedFromProperty>,
	pub r#study: Vec<StudyProperty>,
	pub r#sub_structure: Vec<SubStructureProperty>,
	pub r#subject_of: Vec<SubjectOfProperty>,
	pub r#url: Vec<UrlProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for Nerve {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#additional_type) as usize,
				!Vec::is_empty(&self.r#alternate_name) as usize,
				!Vec::is_empty(&self.r#associated_pathophysiology) as usize,
				!Vec::is_empty(&self.r#body_location) as usize,
				!Vec::is_empty(&self.r#branch) as usize,
				!Vec::is_empty(&self.r#code) as usize,
				!Vec::is_empty(&self.r#connected_to) as usize,
				!Vec::is_empty(&self.r#description) as usize,
				!Vec::is_empty(&self.r#diagram) as usize,
				!Vec::is_empty(&self.r#disambiguating_description) as usize,
				!Vec::is_empty(&self.r#funding) as usize,
				!Vec::is_empty(&self.r#guideline) as usize,
				!Vec::is_empty(&self.r#identifier) as usize,
				!Vec::is_empty(&self.r#image) as usize,
				!Vec::is_empty(&self.r#legal_status) as usize,
				!Vec::is_empty(&self.r#main_entity_of_page) as usize,
				!Vec::is_empty(&self.r#medicine_system) as usize,
				!Vec::is_empty(&self.r#name) as usize,
				!Vec::is_empty(&self.r#nerve_motor) as usize,
				!Vec::is_empty(&self.r#part_of_system) as usize,
				!Vec::is_empty(&self.r#potential_action) as usize,
				!Vec::is_empty(&self.r#recognizing_authority) as usize,
				!Vec::is_empty(&self.r#related_condition) as usize,
				!Vec::is_empty(&self.r#related_therapy) as usize,
				!Vec::is_empty(&self.r#relevant_specialty) as usize,
				!Vec::is_empty(&self.r#same_as) as usize,
				!Vec::is_empty(&self.r#sensory_unit) as usize,
				!Vec::is_empty(&self.r#sourced_from) as usize,
				!Vec::is_empty(&self.r#study) as usize,
				!Vec::is_empty(&self.r#sub_structure) as usize,
				!Vec::is_empty(&self.r#subject_of) as usize,
				!Vec::is_empty(&self.r#url) as usize,
			]
			.iter()
			.sum();
			let mut serialize_struct = Serializer::serialize_struct(serializer, "Nerve", len)?;
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
			if !Vec::is_empty(&self.r#associated_pathophysiology) {
				serialize_struct.serialize_field("associatedPathophysiology", {
					struct SerializeWith<'a>(&'a Vec<AssociatedPathophysiologyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#associated_pathophysiology)
				})?;
			} else {
				serialize_struct.skip_field("associatedPathophysiology")?;
			}
			if !Vec::is_empty(&self.r#body_location) {
				serialize_struct.serialize_field("bodyLocation", {
					struct SerializeWith<'a>(&'a Vec<BodyLocationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#body_location)
				})?;
			} else {
				serialize_struct.skip_field("bodyLocation")?;
			}
			if !Vec::is_empty(&self.r#branch) {
				serialize_struct.serialize_field("branch", {
					struct SerializeWith<'a>(&'a Vec<BranchProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#branch)
				})?;
			} else {
				serialize_struct.skip_field("branch")?;
			}
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
			if !Vec::is_empty(&self.r#connected_to) {
				serialize_struct.serialize_field("connectedTo", {
					struct SerializeWith<'a>(&'a Vec<ConnectedToProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#connected_to)
				})?;
			} else {
				serialize_struct.skip_field("connectedTo")?;
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
			if !Vec::is_empty(&self.r#diagram) {
				serialize_struct.serialize_field("diagram", {
					struct SerializeWith<'a>(&'a Vec<DiagramProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#diagram)
				})?;
			} else {
				serialize_struct.skip_field("diagram")?;
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
			if !Vec::is_empty(&self.r#nerve_motor) {
				serialize_struct.serialize_field("nerveMotor", {
					struct SerializeWith<'a>(&'a Vec<NerveMotorProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#nerve_motor)
				})?;
			} else {
				serialize_struct.skip_field("nerveMotor")?;
			}
			if !Vec::is_empty(&self.r#part_of_system) {
				serialize_struct.serialize_field("partOfSystem", {
					struct SerializeWith<'a>(&'a Vec<PartOfSystemProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#part_of_system)
				})?;
			} else {
				serialize_struct.skip_field("partOfSystem")?;
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
			if !Vec::is_empty(&self.r#related_condition) {
				serialize_struct.serialize_field("relatedCondition", {
					struct SerializeWith<'a>(&'a Vec<RelatedConditionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#related_condition)
				})?;
			} else {
				serialize_struct.skip_field("relatedCondition")?;
			}
			if !Vec::is_empty(&self.r#related_therapy) {
				serialize_struct.serialize_field("relatedTherapy", {
					struct SerializeWith<'a>(&'a Vec<RelatedTherapyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#related_therapy)
				})?;
			} else {
				serialize_struct.skip_field("relatedTherapy")?;
			}
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
			if !Vec::is_empty(&self.r#sensory_unit) {
				serialize_struct.serialize_field("sensoryUnit", {
					struct SerializeWith<'a>(&'a Vec<SensoryUnitProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#sensory_unit)
				})?;
			} else {
				serialize_struct.skip_field("sensoryUnit")?;
			}
			if !Vec::is_empty(&self.r#sourced_from) {
				serialize_struct.serialize_field("sourcedFrom", {
					struct SerializeWith<'a>(&'a Vec<SourcedFromProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#sourced_from)
				})?;
			} else {
				serialize_struct.skip_field("sourcedFrom")?;
			}
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
			if !Vec::is_empty(&self.r#sub_structure) {
				serialize_struct.serialize_field("subStructure", {
					struct SerializeWith<'a>(&'a Vec<SubStructureProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#sub_structure)
				})?;
			} else {
				serialize_struct.skip_field("subStructure")?;
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
	impl<'de> Deserialize<'de> for Nerve {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AdditionalType,
				AlternateName,
				AssociatedPathophysiology,
				BodyLocation,
				Branch,
				Code,
				ConnectedTo,
				Description,
				Diagram,
				DisambiguatingDescription,
				Funding,
				Guideline,
				Identifier,
				Image,
				LegalStatus,
				MainEntityOfPage,
				MedicineSystem,
				Name,
				NerveMotor,
				PartOfSystem,
				PotentialAction,
				RecognizingAuthority,
				RelatedCondition,
				RelatedTherapy,
				RelevantSpecialty,
				SameAs,
				SensoryUnit,
				SourcedFrom,
				Study,
				SubStructure,
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
						"additionalType" => Ok(Field::AdditionalType),
						"alternateName" => Ok(Field::AlternateName),
						"associatedPathophysiology" => Ok(Field::AssociatedPathophysiology),
						"bodyLocation" => Ok(Field::BodyLocation),
						"branch" => Ok(Field::Branch),
						"code" => Ok(Field::Code),
						"connectedTo" => Ok(Field::ConnectedTo),
						"description" => Ok(Field::Description),
						"diagram" => Ok(Field::Diagram),
						"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						"funding" => Ok(Field::Funding),
						"guideline" => Ok(Field::Guideline),
						"identifier" => Ok(Field::Identifier),
						"image" => Ok(Field::Image),
						"legalStatus" => Ok(Field::LegalStatus),
						"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						"medicineSystem" => Ok(Field::MedicineSystem),
						"name" => Ok(Field::Name),
						"nerveMotor" => Ok(Field::NerveMotor),
						"partOfSystem" => Ok(Field::PartOfSystem),
						"potentialAction" => Ok(Field::PotentialAction),
						"recognizingAuthority" => Ok(Field::RecognizingAuthority),
						"relatedCondition" => Ok(Field::RelatedCondition),
						"relatedTherapy" => Ok(Field::RelatedTherapy),
						"relevantSpecialty" => Ok(Field::RelevantSpecialty),
						"sameAs" => Ok(Field::SameAs),
						"sensoryUnit" => Ok(Field::SensoryUnit),
						"sourcedFrom" => Ok(Field::SourcedFrom),
						"study" => Ok(Field::Study),
						"subStructure" => Ok(Field::SubStructure),
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
						b"additionalType" => Ok(Field::AdditionalType),
						b"alternateName" => Ok(Field::AlternateName),
						b"associatedPathophysiology" => Ok(Field::AssociatedPathophysiology),
						b"bodyLocation" => Ok(Field::BodyLocation),
						b"branch" => Ok(Field::Branch),
						b"code" => Ok(Field::Code),
						b"connectedTo" => Ok(Field::ConnectedTo),
						b"description" => Ok(Field::Description),
						b"diagram" => Ok(Field::Diagram),
						b"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						b"funding" => Ok(Field::Funding),
						b"guideline" => Ok(Field::Guideline),
						b"identifier" => Ok(Field::Identifier),
						b"image" => Ok(Field::Image),
						b"legalStatus" => Ok(Field::LegalStatus),
						b"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						b"medicineSystem" => Ok(Field::MedicineSystem),
						b"name" => Ok(Field::Name),
						b"nerveMotor" => Ok(Field::NerveMotor),
						b"partOfSystem" => Ok(Field::PartOfSystem),
						b"potentialAction" => Ok(Field::PotentialAction),
						b"recognizingAuthority" => Ok(Field::RecognizingAuthority),
						b"relatedCondition" => Ok(Field::RelatedCondition),
						b"relatedTherapy" => Ok(Field::RelatedTherapy),
						b"relevantSpecialty" => Ok(Field::RelevantSpecialty),
						b"sameAs" => Ok(Field::SameAs),
						b"sensoryUnit" => Ok(Field::SensoryUnit),
						b"sourcedFrom" => Ok(Field::SourcedFrom),
						b"study" => Ok(Field::Study),
						b"subStructure" => Ok(Field::SubStructure),
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
				type Value = Nerve;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Nerve")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#additional_type_property = None;
					let mut r#alternate_name_property = None;
					let mut r#associated_pathophysiology_property = None;
					let mut r#body_location_property = None;
					let mut r#branch_property = None;
					let mut r#code_property = None;
					let mut r#connected_to_property = None;
					let mut r#description_property = None;
					let mut r#diagram_property = None;
					let mut r#disambiguating_description_property = None;
					let mut r#funding_property = None;
					let mut r#guideline_property = None;
					let mut r#identifier_property = None;
					let mut r#image_property = None;
					let mut r#legal_status_property = None;
					let mut r#main_entity_of_page_property = None;
					let mut r#medicine_system_property = None;
					let mut r#name_property = None;
					let mut r#nerve_motor_property = None;
					let mut r#part_of_system_property = None;
					let mut r#potential_action_property = None;
					let mut r#recognizing_authority_property = None;
					let mut r#related_condition_property = None;
					let mut r#related_therapy_property = None;
					let mut r#relevant_specialty_property = None;
					let mut r#same_as_property = None;
					let mut r#sensory_unit_property = None;
					let mut r#sourced_from_property = None;
					let mut r#study_property = None;
					let mut r#sub_structure_property = None;
					let mut r#subject_of_property = None;
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
							Field::AssociatedPathophysiology => {
								if r#associated_pathophysiology_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"associatedPathophysiology",
									));
								}
								r#associated_pathophysiology_property = Some({
									struct DeserializeWith(Vec<AssociatedPathophysiologyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::BodyLocation => {
								if r#body_location_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"bodyLocation",
									));
								}
								r#body_location_property = Some({
									struct DeserializeWith(Vec<BodyLocationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Branch => {
								if r#branch_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("branch"));
								}
								r#branch_property = Some({
									struct DeserializeWith(Vec<BranchProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
							Field::ConnectedTo => {
								if r#connected_to_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"connectedTo",
									));
								}
								r#connected_to_property = Some({
									struct DeserializeWith(Vec<ConnectedToProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::Diagram => {
								if r#diagram_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"diagram",
									));
								}
								r#diagram_property = Some({
									struct DeserializeWith(Vec<DiagramProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::NerveMotor => {
								if r#nerve_motor_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"nerveMotor",
									));
								}
								r#nerve_motor_property = Some({
									struct DeserializeWith(Vec<NerveMotorProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::PartOfSystem => {
								if r#part_of_system_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"partOfSystem",
									));
								}
								r#part_of_system_property = Some({
									struct DeserializeWith(Vec<PartOfSystemProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::RelatedCondition => {
								if r#related_condition_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"relatedCondition",
									));
								}
								r#related_condition_property = Some({
									struct DeserializeWith(Vec<RelatedConditionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::RelatedTherapy => {
								if r#related_therapy_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"relatedTherapy",
									));
								}
								r#related_therapy_property = Some({
									struct DeserializeWith(Vec<RelatedTherapyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
							Field::SensoryUnit => {
								if r#sensory_unit_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"sensoryUnit",
									));
								}
								r#sensory_unit_property = Some({
									struct DeserializeWith(Vec<SensoryUnitProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::SourcedFrom => {
								if r#sourced_from_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"sourcedFrom",
									));
								}
								r#sourced_from_property = Some({
									struct DeserializeWith(Vec<SourcedFromProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
							Field::SubStructure => {
								if r#sub_structure_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"subStructure",
									));
								}
								r#sub_structure_property = Some({
									struct DeserializeWith(Vec<SubStructureProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
					Ok(Nerve {
						r#additional_type: r#additional_type_property.unwrap_or_default(),
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
						r#associated_pathophysiology: r#associated_pathophysiology_property
							.unwrap_or_default(),
						r#body_location: r#body_location_property.unwrap_or_default(),
						r#branch: r#branch_property.unwrap_or_default(),
						r#code: r#code_property.unwrap_or_default(),
						r#connected_to: r#connected_to_property.unwrap_or_default(),
						r#description: r#description_property.unwrap_or_default(),
						r#diagram: r#diagram_property.unwrap_or_default(),
						r#disambiguating_description: r#disambiguating_description_property
							.unwrap_or_default(),
						r#funding: r#funding_property.unwrap_or_default(),
						r#guideline: r#guideline_property.unwrap_or_default(),
						r#identifier: r#identifier_property.unwrap_or_default(),
						r#image: r#image_property.unwrap_or_default(),
						r#legal_status: r#legal_status_property.unwrap_or_default(),
						r#main_entity_of_page: r#main_entity_of_page_property.unwrap_or_default(),
						r#medicine_system: r#medicine_system_property.unwrap_or_default(),
						r#name: r#name_property.unwrap_or_default(),
						r#nerve_motor: r#nerve_motor_property.unwrap_or_default(),
						r#part_of_system: r#part_of_system_property.unwrap_or_default(),
						r#potential_action: r#potential_action_property.unwrap_or_default(),
						r#recognizing_authority: r#recognizing_authority_property
							.unwrap_or_default(),
						r#related_condition: r#related_condition_property.unwrap_or_default(),
						r#related_therapy: r#related_therapy_property.unwrap_or_default(),
						r#relevant_specialty: r#relevant_specialty_property.unwrap_or_default(),
						r#same_as: r#same_as_property.unwrap_or_default(),
						r#sensory_unit: r#sensory_unit_property.unwrap_or_default(),
						r#sourced_from: r#sourced_from_property.unwrap_or_default(),
						r#study: r#study_property.unwrap_or_default(),
						r#sub_structure: r#sub_structure_property.unwrap_or_default(),
						r#subject_of: r#subject_of_property.unwrap_or_default(),
						r#url: r#url_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				"additionalType",
				"alternateName",
				"associatedPathophysiology",
				"bodyLocation",
				"branch",
				"code",
				"connectedTo",
				"description",
				"diagram",
				"disambiguatingDescription",
				"funding",
				"guideline",
				"identifier",
				"image",
				"legalStatus",
				"mainEntityOfPage",
				"medicineSystem",
				"name",
				"nerveMotor",
				"partOfSystem",
				"potentialAction",
				"recognizingAuthority",
				"relatedCondition",
				"relatedTherapy",
				"relevantSpecialty",
				"sameAs",
				"sensoryUnit",
				"sourcedFrom",
				"study",
				"subStructure",
				"subjectOf",
				"url",
			];
			deserializer.deserialize_struct("Nerve", FIELDS, ClassVisitor)
		}
	}
}
