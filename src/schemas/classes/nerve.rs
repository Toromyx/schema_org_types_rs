use super::*;
/// <https://schema.org/Nerve>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Nerve {
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
			feature = "associated-pathophysiology-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#associated_pathophysiology: Vec<AssociatedPathophysiologyProperty>,
	#[cfg(any(
		any(
			feature = "body-location-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#body_location: Vec<BodyLocationProperty>,
	#[cfg(any(
		any(
			feature = "branch-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#branch: Vec<BranchProperty>,
	#[cfg(any(
		any(
			feature = "code-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#code: Vec<CodeProperty>,
	#[cfg(any(
		any(
			feature = "connected-to-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#connected_to: Vec<ConnectedToProperty>,
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
			feature = "diagram-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#diagram: Vec<DiagramProperty>,
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
			feature = "funding-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#funding: Vec<FundingProperty>,
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
			feature = "legal-status-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#legal_status: Vec<LegalStatusProperty>,
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
			feature = "medicine-system-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#medicine_system: Vec<MedicineSystemProperty>,
	#[cfg(any(
		any(feature = "name-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#name: Vec<NameProperty>,
	#[cfg(any(
		any(
			feature = "nerve-motor-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#nerve_motor: Vec<NerveMotorProperty>,
	#[cfg(any(
		any(
			feature = "part-of-system-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#part_of_system: Vec<PartOfSystemProperty>,
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
			feature = "recognizing-authority-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#recognizing_authority: Vec<RecognizingAuthorityProperty>,
	#[cfg(any(
		any(
			feature = "related-condition-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#related_condition: Vec<RelatedConditionProperty>,
	#[cfg(any(
		any(
			feature = "related-therapy-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#related_therapy: Vec<RelatedTherapyProperty>,
	#[cfg(any(
		any(
			feature = "relevant-specialty-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#relevant_specialty: Vec<RelevantSpecialtyProperty>,
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
			feature = "sensory-unit-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#sensory_unit: Vec<SensoryUnitProperty>,
	#[cfg(any(
		any(
			feature = "sourced-from-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#sourced_from: Vec<SourcedFromProperty>,
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
			feature = "sub-structure-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#sub_structure: Vec<SubStructureProperty>,
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
						feature = "associated-pathophysiology-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#associated_pathophysiology) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "body-location-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#body_location) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "branch-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#branch) as usize
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
						feature = "connected-to-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#connected_to) as usize
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
						feature = "diagram-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#diagram) as usize
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
						feature = "nerve-motor-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#nerve_motor) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "part-of-system-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#part_of_system) as usize
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
						feature = "related-condition-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#related_condition) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "related-therapy-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#related_therapy) as usize
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
						feature = "sensory-unit-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#sensory_unit) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "sourced-from-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#sourced_from) as usize
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
						feature = "sub-structure-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#sub_structure) as usize
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
			]
			.iter()
			.sum();
			let mut serialize_struct = Serializer::serialize_struct(serializer, "Nerve", len)?;
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
					feature = "associated-pathophysiology-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "body-location-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "branch-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
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
				any(
					feature = "connected-to-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
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
					feature = "diagram-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
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
					feature = "nerve-motor-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "part-of-system-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
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
					feature = "related-condition-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "related-therapy-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
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
					feature = "sensory-unit-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "sourced-from-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
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
					feature = "sub-structure-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
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
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for Nerve {
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
						feature = "associated-pathophysiology-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				AssociatedPathophysiology,
				#[cfg(any(
					any(
						feature = "body-location-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				BodyLocation,
				#[cfg(any(
					any(
						feature = "branch-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				Branch,
				#[cfg(any(
					any(
						feature = "code-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				Code,
				#[cfg(any(
					any(
						feature = "connected-to-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				ConnectedTo,
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
						feature = "diagram-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				Diagram,
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
						feature = "funding-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Funding,
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
						feature = "legal-status-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				LegalStatus,
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
						feature = "medicine-system-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				MedicineSystem,
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				Name,
				#[cfg(any(
					any(
						feature = "nerve-motor-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				NerveMotor,
				#[cfg(any(
					any(
						feature = "part-of-system-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				PartOfSystem,
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
						feature = "recognizing-authority-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				RecognizingAuthority,
				#[cfg(any(
					any(
						feature = "related-condition-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				RelatedCondition,
				#[cfg(any(
					any(
						feature = "related-therapy-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				RelatedTherapy,
				#[cfg(any(
					any(
						feature = "relevant-specialty-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				RelevantSpecialty,
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
						feature = "sensory-unit-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				SensoryUnit,
				#[cfg(any(
					any(
						feature = "sourced-from-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				SourcedFrom,
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
						feature = "sub-structure-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				SubStructure,
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
								feature = "associated-pathophysiology-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"associatedPathophysiology" => Ok(Field::AssociatedPathophysiology),
						#[cfg(any(
							any(
								feature = "body-location-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"bodyLocation" => Ok(Field::BodyLocation),
						#[cfg(any(
							any(
								feature = "branch-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"branch" => Ok(Field::Branch),
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
								feature = "connected-to-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"connectedTo" => Ok(Field::ConnectedTo),
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
								feature = "diagram-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"diagram" => Ok(Field::Diagram),
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
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"funding" => Ok(Field::Funding),
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
								feature = "legal-status-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"legalStatus" => Ok(Field::LegalStatus),
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
								feature = "medicine-system-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"medicineSystem" => Ok(Field::MedicineSystem),
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
								feature = "nerve-motor-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"nerveMotor" => Ok(Field::NerveMotor),
						#[cfg(any(
							any(
								feature = "part-of-system-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"partOfSystem" => Ok(Field::PartOfSystem),
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
								feature = "recognizing-authority-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"recognizingAuthority" => Ok(Field::RecognizingAuthority),
						#[cfg(any(
							any(
								feature = "related-condition-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"relatedCondition" => Ok(Field::RelatedCondition),
						#[cfg(any(
							any(
								feature = "related-therapy-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"relatedTherapy" => Ok(Field::RelatedTherapy),
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
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"sameAs" => Ok(Field::SameAs),
						#[cfg(any(
							any(
								feature = "sensory-unit-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"sensoryUnit" => Ok(Field::SensoryUnit),
						#[cfg(any(
							any(
								feature = "sourced-from-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"sourcedFrom" => Ok(Field::SourcedFrom),
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
								feature = "sub-structure-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"subStructure" => Ok(Field::SubStructure),
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
								feature = "associated-pathophysiology-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"associatedPathophysiology" => Ok(Field::AssociatedPathophysiology),
						#[cfg(any(
							any(
								feature = "body-location-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"bodyLocation" => Ok(Field::BodyLocation),
						#[cfg(any(
							any(
								feature = "branch-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"branch" => Ok(Field::Branch),
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
								feature = "connected-to-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"connectedTo" => Ok(Field::ConnectedTo),
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
								feature = "diagram-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"diagram" => Ok(Field::Diagram),
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
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"funding" => Ok(Field::Funding),
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
								feature = "legal-status-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"legalStatus" => Ok(Field::LegalStatus),
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
								feature = "medicine-system-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"medicineSystem" => Ok(Field::MedicineSystem),
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
								feature = "nerve-motor-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"nerveMotor" => Ok(Field::NerveMotor),
						#[cfg(any(
							any(
								feature = "part-of-system-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"partOfSystem" => Ok(Field::PartOfSystem),
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
								feature = "recognizing-authority-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"recognizingAuthority" => Ok(Field::RecognizingAuthority),
						#[cfg(any(
							any(
								feature = "related-condition-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"relatedCondition" => Ok(Field::RelatedCondition),
						#[cfg(any(
							any(
								feature = "related-therapy-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"relatedTherapy" => Ok(Field::RelatedTherapy),
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
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"sameAs" => Ok(Field::SameAs),
						#[cfg(any(
							any(
								feature = "sensory-unit-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"sensoryUnit" => Ok(Field::SensoryUnit),
						#[cfg(any(
							any(
								feature = "sourced-from-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"sourcedFrom" => Ok(Field::SourcedFrom),
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
								feature = "sub-structure-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"subStructure" => Ok(Field::SubStructure),
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
							feature = "associated-pathophysiology-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#associated_pathophysiology_property = None;
					#[cfg(any(
						any(
							feature = "body-location-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#body_location_property = None;
					#[cfg(any(
						any(
							feature = "branch-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#branch_property = None;
					#[cfg(any(
						any(
							feature = "code-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#code_property = None;
					#[cfg(any(
						any(
							feature = "connected-to-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#connected_to_property = None;
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
							feature = "diagram-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#diagram_property = None;
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
							feature = "funding-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#funding_property = None;
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
							feature = "legal-status-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#legal_status_property = None;
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
							feature = "medicine-system-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#medicine_system_property = None;
					#[cfg(any(
						any(feature = "name-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#name_property = None;
					#[cfg(any(
						any(
							feature = "nerve-motor-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#nerve_motor_property = None;
					#[cfg(any(
						any(
							feature = "part-of-system-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#part_of_system_property = None;
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
							feature = "recognizing-authority-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#recognizing_authority_property = None;
					#[cfg(any(
						any(
							feature = "related-condition-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#related_condition_property = None;
					#[cfg(any(
						any(
							feature = "related-therapy-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#related_therapy_property = None;
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
							feature = "same-as-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#same_as_property = None;
					#[cfg(any(
						any(
							feature = "sensory-unit-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#sensory_unit_property = None;
					#[cfg(any(
						any(
							feature = "sourced-from-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#sourced_from_property = None;
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
							feature = "sub-structure-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#sub_structure_property = None;
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
									feature = "associated-pathophysiology-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "body-location-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "branch-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
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
									feature = "connected-to-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
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
									feature = "diagram-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
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
									feature = "nerve-motor-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "part-of-system-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
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
									feature = "related-condition-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "related-therapy-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
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
									feature = "sensory-unit-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "sourced-from-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
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
									feature = "sub-structure-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
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
							_ => {
								let _ = map.next_value::<de::IgnoredAny>()?;
							}
						}
					}
					Ok(Nerve {
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
								feature = "associated-pathophysiology-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#associated_pathophysiology: r#associated_pathophysiology_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "body-location-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#body_location: r#body_location_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "branch-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#branch: r#branch_property.unwrap_or_default(),
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
								feature = "connected-to-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#connected_to: r#connected_to_property.unwrap_or_default(),
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
								feature = "diagram-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#diagram: r#diagram_property.unwrap_or_default(),
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
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#funding: r#funding_property.unwrap_or_default(),
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
								feature = "legal-status-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#legal_status: r#legal_status_property.unwrap_or_default(),
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
								feature = "medicine-system-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#medicine_system: r#medicine_system_property.unwrap_or_default(),
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
								feature = "nerve-motor-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#nerve_motor: r#nerve_motor_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "part-of-system-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#part_of_system: r#part_of_system_property.unwrap_or_default(),
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
								feature = "recognizing-authority-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#recognizing_authority: r#recognizing_authority_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "related-condition-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#related_condition: r#related_condition_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "related-therapy-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#related_therapy: r#related_therapy_property.unwrap_or_default(),
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
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#same_as: r#same_as_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "sensory-unit-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#sensory_unit: r#sensory_unit_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "sourced-from-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#sourced_from: r#sourced_from_property.unwrap_or_default(),
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
								feature = "sub-structure-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#sub_structure: r#sub_structure_property.unwrap_or_default(),
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
						feature = "associated-pathophysiology-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"associatedPathophysiology",
				#[cfg(any(
					any(
						feature = "body-location-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"bodyLocation",
				#[cfg(any(
					any(
						feature = "branch-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"branch",
				#[cfg(any(
					any(
						feature = "code-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"code",
				#[cfg(any(
					any(
						feature = "connected-to-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"connectedTo",
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
						feature = "diagram-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"diagram",
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
						feature = "funding-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"funding",
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
						feature = "legal-status-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"legalStatus",
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
						feature = "medicine-system-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"medicineSystem",
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				"name",
				#[cfg(any(
					any(
						feature = "nerve-motor-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"nerveMotor",
				#[cfg(any(
					any(
						feature = "part-of-system-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"partOfSystem",
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
						feature = "recognizing-authority-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"recognizingAuthority",
				#[cfg(any(
					any(
						feature = "related-condition-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"relatedCondition",
				#[cfg(any(
					any(
						feature = "related-therapy-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"relatedTherapy",
				#[cfg(any(
					any(
						feature = "relevant-specialty-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"relevantSpecialty",
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
						feature = "sensory-unit-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"sensoryUnit",
				#[cfg(any(
					any(
						feature = "sourced-from-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"sourcedFrom",
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
						feature = "sub-structure-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"subStructure",
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
			];
			deserializer.deserialize_struct("Nerve", FIELDS, ClassVisitor)
		}
	}
}
