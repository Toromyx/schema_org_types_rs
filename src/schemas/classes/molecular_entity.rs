use super::*;
/// <https://schema.org/MolecularEntity>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct MolecularEntity {
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
			feature = "associated-disease-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#associated_disease: Vec<AssociatedDiseaseProperty>,
	#[cfg(any(
		any(
			feature = "bio-chem-interaction-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#bio_chem_interaction: Vec<BioChemInteractionProperty>,
	#[cfg(any(
		any(
			feature = "bio-chem-similarity-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#bio_chem_similarity: Vec<BioChemSimilarityProperty>,
	#[cfg(any(
		any(
			feature = "biological-role-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#biological_role: Vec<BiologicalRoleProperty>,
	#[cfg(any(
		any(
			feature = "chemical-role-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#chemical_role: Vec<ChemicalRoleProperty>,
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
			feature = "funding-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#funding: Vec<FundingProperty>,
	#[cfg(any(
		any(
			feature = "has-bio-chem-entity-part-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#has_bio_chem_entity_part: Vec<HasBioChemEntityPartProperty>,
	#[cfg(any(
		any(
			feature = "has-molecular-function-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#has_molecular_function: Vec<HasMolecularFunctionProperty>,
	#[cfg(any(
		any(
			feature = "has-representation-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#has_representation: Vec<HasRepresentationProperty>,
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
			feature = "in-ch-i-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#in_ch_i: Vec<InChIProperty>,
	#[cfg(any(
		any(
			feature = "in-ch-i-key-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#in_ch_i_key: Vec<InChIKeyProperty>,
	#[cfg(any(
		any(
			feature = "is-encoded-by-bio-chem-entity-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#is_encoded_by_bio_chem_entity: Vec<IsEncodedByBioChemEntityProperty>,
	#[cfg(any(
		any(
			feature = "is-involved-in-biological-process-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#is_involved_in_biological_process: Vec<IsInvolvedInBiologicalProcessProperty>,
	#[cfg(any(
		any(
			feature = "is-located-in-subcellular-location-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#is_located_in_subcellular_location: Vec<IsLocatedInSubcellularLocationProperty>,
	#[cfg(any(
		any(
			feature = "is-part-of-bio-chem-entity-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#is_part_of_bio_chem_entity: Vec<IsPartOfBioChemEntityProperty>,
	#[cfg(any(
		any(
			feature = "iupac-name-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#iupac_name: Vec<IupacNameProperty>,
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
			feature = "molecular-formula-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#molecular_formula: Vec<MolecularFormulaProperty>,
	#[cfg(any(
		any(
			feature = "molecular-weight-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#molecular_weight: Vec<MolecularWeightProperty>,
	#[cfg(any(
		any(
			feature = "monoisotopic-molecular-weight-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#monoisotopic_molecular_weight: Vec<MonoisotopicMolecularWeightProperty>,
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
			feature = "potential-use-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#potential_use: Vec<PotentialUseProperty>,
	#[cfg(any(
		any(
			feature = "same-as-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#same_as: Vec<SameAsProperty>,
	#[cfg(any(
		any(feature = "smiles-property-schema", feature = "pending-schema-section"),
		doc
	))]
	pub r#smiles: Vec<SmilesProperty>,
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
			feature = "taxonomic-range-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#taxonomic_range: Vec<TaxonomicRangeProperty>,
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
	impl Serialize for MolecularEntity {
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
						feature = "associated-disease-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#associated_disease) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "bio-chem-interaction-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#bio_chem_interaction) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "bio-chem-similarity-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#bio_chem_similarity) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "biological-role-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#biological_role) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "chemical-role-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#chemical_role) as usize
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
						feature = "has-bio-chem-entity-part-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_bio_chem_entity_part) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-molecular-function-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_molecular_function) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-representation-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_representation) as usize
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
						feature = "in-ch-i-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#in_ch_i) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "in-ch-i-key-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#in_ch_i_key) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-encoded-by-bio-chem-entity-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_encoded_by_bio_chem_entity) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-involved-in-biological-process-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_involved_in_biological_process) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-located-in-subcellular-location-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_located_in_subcellular_location) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-part-of-bio-chem-entity-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_part_of_bio_chem_entity) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "iupac-name-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#iupac_name) as usize
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
						feature = "molecular-formula-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#molecular_formula) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "molecular-weight-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#molecular_weight) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "monoisotopic-molecular-weight-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#monoisotopic_molecular_weight) as usize
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
						feature = "potential-use-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#potential_use) as usize
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
						feature = "smiles-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#smiles) as usize
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
						feature = "taxonomic-range-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#taxonomic_range) as usize
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
				Serializer::serialize_struct(serializer, "MolecularEntity", len)?;
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
					feature = "associated-disease-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#associated_disease) {
				serialize_struct.serialize_field("associatedDisease", {
					struct SerializeWith<'a>(&'a Vec<AssociatedDiseaseProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#associated_disease)
				})?;
			} else {
				serialize_struct.skip_field("associatedDisease")?;
			}
			#[cfg(any(
				any(
					feature = "bio-chem-interaction-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#bio_chem_interaction) {
				serialize_struct.serialize_field("bioChemInteraction", {
					struct SerializeWith<'a>(&'a Vec<BioChemInteractionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#bio_chem_interaction)
				})?;
			} else {
				serialize_struct.skip_field("bioChemInteraction")?;
			}
			#[cfg(any(
				any(
					feature = "bio-chem-similarity-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#bio_chem_similarity) {
				serialize_struct.serialize_field("bioChemSimilarity", {
					struct SerializeWith<'a>(&'a Vec<BioChemSimilarityProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#bio_chem_similarity)
				})?;
			} else {
				serialize_struct.skip_field("bioChemSimilarity")?;
			}
			#[cfg(any(
				any(
					feature = "biological-role-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#biological_role) {
				serialize_struct.serialize_field("biologicalRole", {
					struct SerializeWith<'a>(&'a Vec<BiologicalRoleProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#biological_role)
				})?;
			} else {
				serialize_struct.skip_field("biologicalRole")?;
			}
			#[cfg(any(
				any(
					feature = "chemical-role-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#chemical_role) {
				serialize_struct.serialize_field("chemicalRole", {
					struct SerializeWith<'a>(&'a Vec<ChemicalRoleProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#chemical_role)
				})?;
			} else {
				serialize_struct.skip_field("chemicalRole")?;
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
					feature = "has-bio-chem-entity-part-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#has_bio_chem_entity_part) {
				serialize_struct.serialize_field("hasBioChemEntityPart", {
					struct SerializeWith<'a>(&'a Vec<HasBioChemEntityPartProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_bio_chem_entity_part)
				})?;
			} else {
				serialize_struct.skip_field("hasBioChemEntityPart")?;
			}
			#[cfg(any(
				any(
					feature = "has-molecular-function-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#has_molecular_function) {
				serialize_struct.serialize_field("hasMolecularFunction", {
					struct SerializeWith<'a>(&'a Vec<HasMolecularFunctionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_molecular_function)
				})?;
			} else {
				serialize_struct.skip_field("hasMolecularFunction")?;
			}
			#[cfg(any(
				any(
					feature = "has-representation-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#has_representation) {
				serialize_struct.serialize_field("hasRepresentation", {
					struct SerializeWith<'a>(&'a Vec<HasRepresentationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_representation)
				})?;
			} else {
				serialize_struct.skip_field("hasRepresentation")?;
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
					feature = "in-ch-i-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#in_ch_i) {
				serialize_struct.serialize_field("inChI", {
					struct SerializeWith<'a>(&'a Vec<InChIProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#in_ch_i)
				})?;
			} else {
				serialize_struct.skip_field("inChI")?;
			}
			#[cfg(any(
				any(
					feature = "in-ch-i-key-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#in_ch_i_key) {
				serialize_struct.serialize_field("inChIKey", {
					struct SerializeWith<'a>(&'a Vec<InChIKeyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#in_ch_i_key)
				})?;
			} else {
				serialize_struct.skip_field("inChIKey")?;
			}
			#[cfg(any(
				any(
					feature = "is-encoded-by-bio-chem-entity-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#is_encoded_by_bio_chem_entity) {
				serialize_struct.serialize_field("isEncodedByBioChemEntity", {
					struct SerializeWith<'a>(&'a Vec<IsEncodedByBioChemEntityProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_encoded_by_bio_chem_entity)
				})?;
			} else {
				serialize_struct.skip_field("isEncodedByBioChemEntity")?;
			}
			#[cfg(any(
				any(
					feature = "is-involved-in-biological-process-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#is_involved_in_biological_process) {
				serialize_struct.serialize_field("isInvolvedInBiologicalProcess", {
					struct SerializeWith<'a>(&'a Vec<IsInvolvedInBiologicalProcessProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_involved_in_biological_process)
				})?;
			} else {
				serialize_struct.skip_field("isInvolvedInBiologicalProcess")?;
			}
			#[cfg(any(
				any(
					feature = "is-located-in-subcellular-location-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#is_located_in_subcellular_location) {
				serialize_struct.serialize_field("isLocatedInSubcellularLocation", {
					struct SerializeWith<'a>(&'a Vec<IsLocatedInSubcellularLocationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_located_in_subcellular_location)
				})?;
			} else {
				serialize_struct.skip_field("isLocatedInSubcellularLocation")?;
			}
			#[cfg(any(
				any(
					feature = "is-part-of-bio-chem-entity-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#is_part_of_bio_chem_entity) {
				serialize_struct.serialize_field("isPartOfBioChemEntity", {
					struct SerializeWith<'a>(&'a Vec<IsPartOfBioChemEntityProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_part_of_bio_chem_entity)
				})?;
			} else {
				serialize_struct.skip_field("isPartOfBioChemEntity")?;
			}
			#[cfg(any(
				any(
					feature = "iupac-name-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#iupac_name) {
				serialize_struct.serialize_field("iupacName", {
					struct SerializeWith<'a>(&'a Vec<IupacNameProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#iupac_name)
				})?;
			} else {
				serialize_struct.skip_field("iupacName")?;
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
					feature = "molecular-formula-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#molecular_formula) {
				serialize_struct.serialize_field("molecularFormula", {
					struct SerializeWith<'a>(&'a Vec<MolecularFormulaProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#molecular_formula)
				})?;
			} else {
				serialize_struct.skip_field("molecularFormula")?;
			}
			#[cfg(any(
				any(
					feature = "molecular-weight-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#molecular_weight) {
				serialize_struct.serialize_field("molecularWeight", {
					struct SerializeWith<'a>(&'a Vec<MolecularWeightProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#molecular_weight)
				})?;
			} else {
				serialize_struct.skip_field("molecularWeight")?;
			}
			#[cfg(any(
				any(
					feature = "monoisotopic-molecular-weight-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#monoisotopic_molecular_weight) {
				serialize_struct.serialize_field("monoisotopicMolecularWeight", {
					struct SerializeWith<'a>(&'a Vec<MonoisotopicMolecularWeightProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#monoisotopic_molecular_weight)
				})?;
			} else {
				serialize_struct.skip_field("monoisotopicMolecularWeight")?;
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
					feature = "potential-use-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#potential_use) {
				serialize_struct.serialize_field("potentialUse", {
					struct SerializeWith<'a>(&'a Vec<PotentialUseProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#potential_use)
				})?;
			} else {
				serialize_struct.skip_field("potentialUse")?;
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
				any(feature = "smiles-property-schema", feature = "pending-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#smiles) {
				serialize_struct.serialize_field("smiles", {
					struct SerializeWith<'a>(&'a Vec<SmilesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#smiles)
				})?;
			} else {
				serialize_struct.skip_field("smiles")?;
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
					feature = "taxonomic-range-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#taxonomic_range) {
				serialize_struct.serialize_field("taxonomicRange", {
					struct SerializeWith<'a>(&'a Vec<TaxonomicRangeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#taxonomic_range)
				})?;
			} else {
				serialize_struct.skip_field("taxonomicRange")?;
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
	impl<'de> Deserialize<'de> for MolecularEntity {
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
						feature = "associated-disease-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				AssociatedDisease,
				#[cfg(any(
					any(
						feature = "bio-chem-interaction-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				BioChemInteraction,
				#[cfg(any(
					any(
						feature = "bio-chem-similarity-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				BioChemSimilarity,
				#[cfg(any(
					any(
						feature = "biological-role-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				BiologicalRole,
				#[cfg(any(
					any(
						feature = "chemical-role-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ChemicalRole,
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
						feature = "funding-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Funding,
				#[cfg(any(
					any(
						feature = "has-bio-chem-entity-part-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HasBioChemEntityPart,
				#[cfg(any(
					any(
						feature = "has-molecular-function-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HasMolecularFunction,
				#[cfg(any(
					any(
						feature = "has-representation-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HasRepresentation,
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
						feature = "in-ch-i-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				InChI,
				#[cfg(any(
					any(
						feature = "in-ch-i-key-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				InChIKey,
				#[cfg(any(
					any(
						feature = "is-encoded-by-bio-chem-entity-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				IsEncodedByBioChemEntity,
				#[cfg(any(
					any(
						feature = "is-involved-in-biological-process-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				IsInvolvedInBiologicalProcess,
				#[cfg(any(
					any(
						feature = "is-located-in-subcellular-location-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				IsLocatedInSubcellularLocation,
				#[cfg(any(
					any(
						feature = "is-part-of-bio-chem-entity-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				IsPartOfBioChemEntity,
				#[cfg(any(
					any(
						feature = "iupac-name-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				IupacName,
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
						feature = "molecular-formula-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				MolecularFormula,
				#[cfg(any(
					any(
						feature = "molecular-weight-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				MolecularWeight,
				#[cfg(any(
					any(
						feature = "monoisotopic-molecular-weight-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				MonoisotopicMolecularWeight,
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
						feature = "potential-use-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				PotentialUse,
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SameAs,
				#[cfg(any(
					any(feature = "smiles-property-schema", feature = "pending-schema-section"),
					doc
				))]
				Smiles,
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
						feature = "taxonomic-range-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				TaxonomicRange,
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
								feature = "associated-disease-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"associatedDisease" => Ok(Field::AssociatedDisease),
						#[cfg(any(
							any(
								feature = "bio-chem-interaction-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"bioChemInteraction" => Ok(Field::BioChemInteraction),
						#[cfg(any(
							any(
								feature = "bio-chem-similarity-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"bioChemSimilarity" => Ok(Field::BioChemSimilarity),
						#[cfg(any(
							any(
								feature = "biological-role-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"biologicalRole" => Ok(Field::BiologicalRole),
						#[cfg(any(
							any(
								feature = "chemical-role-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"chemicalRole" => Ok(Field::ChemicalRole),
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
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"funding" => Ok(Field::Funding),
						#[cfg(any(
							any(
								feature = "has-bio-chem-entity-part-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"hasBioChemEntityPart" => Ok(Field::HasBioChemEntityPart),
						#[cfg(any(
							any(
								feature = "has-molecular-function-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"hasMolecularFunction" => Ok(Field::HasMolecularFunction),
						#[cfg(any(
							any(
								feature = "has-representation-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"hasRepresentation" => Ok(Field::HasRepresentation),
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
								feature = "in-ch-i-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"inChI" => Ok(Field::InChI),
						#[cfg(any(
							any(
								feature = "in-ch-i-key-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"inChIKey" => Ok(Field::InChIKey),
						#[cfg(any(
							any(
								feature = "is-encoded-by-bio-chem-entity-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"isEncodedByBioChemEntity" => Ok(Field::IsEncodedByBioChemEntity),
						#[cfg(any(
							any(
								feature = "is-involved-in-biological-process-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"isInvolvedInBiologicalProcess" => Ok(Field::IsInvolvedInBiologicalProcess),
						#[cfg(any(
							any(
								feature = "is-located-in-subcellular-location-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"isLocatedInSubcellularLocation" => Ok(Field::IsLocatedInSubcellularLocation),
						#[cfg(any(
							any(
								feature = "is-part-of-bio-chem-entity-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"isPartOfBioChemEntity" => Ok(Field::IsPartOfBioChemEntity),
						#[cfg(any(
							any(
								feature = "iupac-name-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"iupacName" => Ok(Field::IupacName),
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
								feature = "molecular-formula-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"molecularFormula" => Ok(Field::MolecularFormula),
						#[cfg(any(
							any(
								feature = "molecular-weight-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"molecularWeight" => Ok(Field::MolecularWeight),
						#[cfg(any(
							any(
								feature = "monoisotopic-molecular-weight-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"monoisotopicMolecularWeight" => Ok(Field::MonoisotopicMolecularWeight),
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
								feature = "potential-use-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"potentialUse" => Ok(Field::PotentialUse),
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
								feature = "smiles-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"smiles" => Ok(Field::Smiles),
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
								feature = "taxonomic-range-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"taxonomicRange" => Ok(Field::TaxonomicRange),
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
								feature = "associated-disease-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"associatedDisease" => Ok(Field::AssociatedDisease),
						#[cfg(any(
							any(
								feature = "bio-chem-interaction-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"bioChemInteraction" => Ok(Field::BioChemInteraction),
						#[cfg(any(
							any(
								feature = "bio-chem-similarity-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"bioChemSimilarity" => Ok(Field::BioChemSimilarity),
						#[cfg(any(
							any(
								feature = "biological-role-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"biologicalRole" => Ok(Field::BiologicalRole),
						#[cfg(any(
							any(
								feature = "chemical-role-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"chemicalRole" => Ok(Field::ChemicalRole),
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
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"funding" => Ok(Field::Funding),
						#[cfg(any(
							any(
								feature = "has-bio-chem-entity-part-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"hasBioChemEntityPart" => Ok(Field::HasBioChemEntityPart),
						#[cfg(any(
							any(
								feature = "has-molecular-function-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"hasMolecularFunction" => Ok(Field::HasMolecularFunction),
						#[cfg(any(
							any(
								feature = "has-representation-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"hasRepresentation" => Ok(Field::HasRepresentation),
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
								feature = "in-ch-i-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"inChI" => Ok(Field::InChI),
						#[cfg(any(
							any(
								feature = "in-ch-i-key-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"inChIKey" => Ok(Field::InChIKey),
						#[cfg(any(
							any(
								feature = "is-encoded-by-bio-chem-entity-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"isEncodedByBioChemEntity" => Ok(Field::IsEncodedByBioChemEntity),
						#[cfg(any(
							any(
								feature = "is-involved-in-biological-process-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"isInvolvedInBiologicalProcess" => Ok(Field::IsInvolvedInBiologicalProcess),
						#[cfg(any(
							any(
								feature = "is-located-in-subcellular-location-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"isLocatedInSubcellularLocation" => Ok(Field::IsLocatedInSubcellularLocation),
						#[cfg(any(
							any(
								feature = "is-part-of-bio-chem-entity-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"isPartOfBioChemEntity" => Ok(Field::IsPartOfBioChemEntity),
						#[cfg(any(
							any(
								feature = "iupac-name-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"iupacName" => Ok(Field::IupacName),
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
								feature = "molecular-formula-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"molecularFormula" => Ok(Field::MolecularFormula),
						#[cfg(any(
							any(
								feature = "molecular-weight-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"molecularWeight" => Ok(Field::MolecularWeight),
						#[cfg(any(
							any(
								feature = "monoisotopic-molecular-weight-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"monoisotopicMolecularWeight" => Ok(Field::MonoisotopicMolecularWeight),
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
								feature = "potential-use-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"potentialUse" => Ok(Field::PotentialUse),
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
								feature = "smiles-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"smiles" => Ok(Field::Smiles),
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
								feature = "taxonomic-range-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"taxonomicRange" => Ok(Field::TaxonomicRange),
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
				type Value = MolecularEntity;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MolecularEntity")
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
							feature = "associated-disease-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#associated_disease_property = None;
					#[cfg(any(
						any(
							feature = "bio-chem-interaction-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#bio_chem_interaction_property = None;
					#[cfg(any(
						any(
							feature = "bio-chem-similarity-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#bio_chem_similarity_property = None;
					#[cfg(any(
						any(
							feature = "biological-role-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#biological_role_property = None;
					#[cfg(any(
						any(
							feature = "chemical-role-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#chemical_role_property = None;
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
							feature = "funding-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#funding_property = None;
					#[cfg(any(
						any(
							feature = "has-bio-chem-entity-part-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#has_bio_chem_entity_part_property = None;
					#[cfg(any(
						any(
							feature = "has-molecular-function-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#has_molecular_function_property = None;
					#[cfg(any(
						any(
							feature = "has-representation-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#has_representation_property = None;
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
							feature = "in-ch-i-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#in_ch_i_property = None;
					#[cfg(any(
						any(
							feature = "in-ch-i-key-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#in_ch_i_key_property = None;
					#[cfg(any(
						any(
							feature = "is-encoded-by-bio-chem-entity-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#is_encoded_by_bio_chem_entity_property = None;
					#[cfg(any(
						any(
							feature = "is-involved-in-biological-process-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#is_involved_in_biological_process_property = None;
					#[cfg(any(
						any(
							feature = "is-located-in-subcellular-location-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#is_located_in_subcellular_location_property = None;
					#[cfg(any(
						any(
							feature = "is-part-of-bio-chem-entity-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#is_part_of_bio_chem_entity_property = None;
					#[cfg(any(
						any(
							feature = "iupac-name-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#iupac_name_property = None;
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
							feature = "molecular-formula-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#molecular_formula_property = None;
					#[cfg(any(
						any(
							feature = "molecular-weight-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#molecular_weight_property = None;
					#[cfg(any(
						any(
							feature = "monoisotopic-molecular-weight-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#monoisotopic_molecular_weight_property = None;
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
							feature = "potential-use-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#potential_use_property = None;
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
							feature = "smiles-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#smiles_property = None;
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
							feature = "taxonomic-range-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#taxonomic_range_property = None;
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
									feature = "associated-disease-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::AssociatedDisease => {
								if r#associated_disease_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"associatedDisease",
									));
								}
								r#associated_disease_property = Some({
									struct DeserializeWith(Vec<AssociatedDiseaseProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "bio-chem-interaction-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::BioChemInteraction => {
								if r#bio_chem_interaction_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"bioChemInteraction",
									));
								}
								r#bio_chem_interaction_property = Some({
									struct DeserializeWith(Vec<BioChemInteractionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "bio-chem-similarity-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::BioChemSimilarity => {
								if r#bio_chem_similarity_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"bioChemSimilarity",
									));
								}
								r#bio_chem_similarity_property = Some({
									struct DeserializeWith(Vec<BioChemSimilarityProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "biological-role-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::BiologicalRole => {
								if r#biological_role_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"biologicalRole",
									));
								}
								r#biological_role_property = Some({
									struct DeserializeWith(Vec<BiologicalRoleProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "chemical-role-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ChemicalRole => {
								if r#chemical_role_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"chemicalRole",
									));
								}
								r#chemical_role_property = Some({
									struct DeserializeWith(Vec<ChemicalRoleProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "has-bio-chem-entity-part-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::HasBioChemEntityPart => {
								if r#has_bio_chem_entity_part_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasBioChemEntityPart",
									));
								}
								r#has_bio_chem_entity_part_property = Some({
									struct DeserializeWith(Vec<HasBioChemEntityPartProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "has-molecular-function-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::HasMolecularFunction => {
								if r#has_molecular_function_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasMolecularFunction",
									));
								}
								r#has_molecular_function_property = Some({
									struct DeserializeWith(Vec<HasMolecularFunctionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "has-representation-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::HasRepresentation => {
								if r#has_representation_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasRepresentation",
									));
								}
								r#has_representation_property = Some({
									struct DeserializeWith(Vec<HasRepresentationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "in-ch-i-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::InChI => {
								if r#in_ch_i_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("inChI"));
								}
								r#in_ch_i_property = Some({
									struct DeserializeWith(Vec<InChIProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "in-ch-i-key-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::InChIKey => {
								if r#in_ch_i_key_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"inChIKey",
									));
								}
								r#in_ch_i_key_property = Some({
									struct DeserializeWith(Vec<InChIKeyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "is-encoded-by-bio-chem-entity-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::IsEncodedByBioChemEntity => {
								if r#is_encoded_by_bio_chem_entity_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isEncodedByBioChemEntity",
									));
								}
								r#is_encoded_by_bio_chem_entity_property = Some({
									struct DeserializeWith(Vec<IsEncodedByBioChemEntityProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "is-involved-in-biological-process-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::IsInvolvedInBiologicalProcess => {
								if r#is_involved_in_biological_process_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isInvolvedInBiologicalProcess",
									));
								}
								r#is_involved_in_biological_process_property = Some({
									struct DeserializeWith(
										Vec<IsInvolvedInBiologicalProcessProperty>,
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
									feature = "is-located-in-subcellular-location-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::IsLocatedInSubcellularLocation => {
								if r#is_located_in_subcellular_location_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isLocatedInSubcellularLocation",
									));
								}
								r#is_located_in_subcellular_location_property = Some({
									struct DeserializeWith(
										Vec<IsLocatedInSubcellularLocationProperty>,
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
									feature = "is-part-of-bio-chem-entity-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::IsPartOfBioChemEntity => {
								if r#is_part_of_bio_chem_entity_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isPartOfBioChemEntity",
									));
								}
								r#is_part_of_bio_chem_entity_property = Some({
									struct DeserializeWith(Vec<IsPartOfBioChemEntityProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "iupac-name-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::IupacName => {
								if r#iupac_name_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"iupacName",
									));
								}
								r#iupac_name_property = Some({
									struct DeserializeWith(Vec<IupacNameProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "molecular-formula-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::MolecularFormula => {
								if r#molecular_formula_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"molecularFormula",
									));
								}
								r#molecular_formula_property = Some({
									struct DeserializeWith(Vec<MolecularFormulaProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "molecular-weight-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::MolecularWeight => {
								if r#molecular_weight_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"molecularWeight",
									));
								}
								r#molecular_weight_property = Some({
									struct DeserializeWith(Vec<MolecularWeightProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "monoisotopic-molecular-weight-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::MonoisotopicMolecularWeight => {
								if r#monoisotopic_molecular_weight_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"monoisotopicMolecularWeight",
									));
								}
								r#monoisotopic_molecular_weight_property = Some({
									struct DeserializeWith(
										Vec<MonoisotopicMolecularWeightProperty>,
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
									feature = "potential-use-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::PotentialUse => {
								if r#potential_use_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"potentialUse",
									));
								}
								r#potential_use_property = Some({
									struct DeserializeWith(Vec<PotentialUseProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "smiles-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::Smiles => {
								if r#smiles_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("smiles"));
								}
								r#smiles_property = Some({
									struct DeserializeWith(Vec<SmilesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "taxonomic-range-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::TaxonomicRange => {
								if r#taxonomic_range_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"taxonomicRange",
									));
								}
								r#taxonomic_range_property = Some({
									struct DeserializeWith(Vec<TaxonomicRangeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
					Ok(MolecularEntity {
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
								feature = "associated-disease-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#associated_disease: r#associated_disease_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "bio-chem-interaction-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#bio_chem_interaction: r#bio_chem_interaction_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "bio-chem-similarity-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#bio_chem_similarity: r#bio_chem_similarity_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "biological-role-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#biological_role: r#biological_role_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "chemical-role-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#chemical_role: r#chemical_role_property.unwrap_or_default(),
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
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#funding: r#funding_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-bio-chem-entity-part-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#has_bio_chem_entity_part: r#has_bio_chem_entity_part_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-molecular-function-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#has_molecular_function: r#has_molecular_function_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-representation-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#has_representation: r#has_representation_property.unwrap_or_default(),
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
								feature = "in-ch-i-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#in_ch_i: r#in_ch_i_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "in-ch-i-key-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#in_ch_i_key: r#in_ch_i_key_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-encoded-by-bio-chem-entity-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#is_encoded_by_bio_chem_entity: r#is_encoded_by_bio_chem_entity_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-involved-in-biological-process-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#is_involved_in_biological_process:
							r#is_involved_in_biological_process_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-located-in-subcellular-location-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#is_located_in_subcellular_location:
							r#is_located_in_subcellular_location_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-part-of-bio-chem-entity-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#is_part_of_bio_chem_entity: r#is_part_of_bio_chem_entity_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "iupac-name-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#iupac_name: r#iupac_name_property.unwrap_or_default(),
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
								feature = "molecular-formula-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#molecular_formula: r#molecular_formula_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "molecular-weight-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#molecular_weight: r#molecular_weight_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "monoisotopic-molecular-weight-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#monoisotopic_molecular_weight: r#monoisotopic_molecular_weight_property
							.unwrap_or_default(),
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
								feature = "potential-use-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#potential_use: r#potential_use_property.unwrap_or_default(),
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
								feature = "smiles-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#smiles: r#smiles_property.unwrap_or_default(),
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
								feature = "taxonomic-range-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#taxonomic_range: r#taxonomic_range_property.unwrap_or_default(),
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
						feature = "associated-disease-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"associatedDisease",
				#[cfg(any(
					any(
						feature = "bio-chem-interaction-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"bioChemInteraction",
				#[cfg(any(
					any(
						feature = "bio-chem-similarity-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"bioChemSimilarity",
				#[cfg(any(
					any(
						feature = "biological-role-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"biologicalRole",
				#[cfg(any(
					any(
						feature = "chemical-role-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"chemicalRole",
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
						feature = "funding-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"funding",
				#[cfg(any(
					any(
						feature = "has-bio-chem-entity-part-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"hasBioChemEntityPart",
				#[cfg(any(
					any(
						feature = "has-molecular-function-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"hasMolecularFunction",
				#[cfg(any(
					any(
						feature = "has-representation-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"hasRepresentation",
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
						feature = "in-ch-i-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"inChI",
				#[cfg(any(
					any(
						feature = "in-ch-i-key-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"inChIKey",
				#[cfg(any(
					any(
						feature = "is-encoded-by-bio-chem-entity-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"isEncodedByBioChemEntity",
				#[cfg(any(
					any(
						feature = "is-involved-in-biological-process-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"isInvolvedInBiologicalProcess",
				#[cfg(any(
					any(
						feature = "is-located-in-subcellular-location-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"isLocatedInSubcellularLocation",
				#[cfg(any(
					any(
						feature = "is-part-of-bio-chem-entity-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"isPartOfBioChemEntity",
				#[cfg(any(
					any(
						feature = "iupac-name-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"iupacName",
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
						feature = "molecular-formula-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"molecularFormula",
				#[cfg(any(
					any(
						feature = "molecular-weight-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"molecularWeight",
				#[cfg(any(
					any(
						feature = "monoisotopic-molecular-weight-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"monoisotopicMolecularWeight",
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
						feature = "potential-use-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"potentialUse",
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"sameAs",
				#[cfg(any(
					any(feature = "smiles-property-schema", feature = "pending-schema-section"),
					doc
				))]
				"smiles",
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
						feature = "taxonomic-range-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"taxonomicRange",
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				"url",
			];
			deserializer.deserialize_struct("MolecularEntity", FIELDS, ClassVisitor)
		}
	}
}
