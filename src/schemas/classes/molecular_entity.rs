use super::*;
/// <https://schema.org/MolecularEntity>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct MolecularEntity {
	/// <https://schema.org/chemicalRole>
	pub r#chemical_role: Vec<ChemicalRoleProperty>,
	/// <https://schema.org/inChI>
	pub r#in_ch_i: Vec<InChIProperty>,
	/// <https://schema.org/inChIKey>
	pub r#in_ch_i_key: Vec<InChIKeyProperty>,
	/// <https://schema.org/iupacName>
	pub r#iupac_name: Vec<IupacNameProperty>,
	/// <https://schema.org/molecularFormula>
	pub r#molecular_formula: Vec<MolecularFormulaProperty>,
	/// <https://schema.org/molecularWeight>
	pub r#molecular_weight: Vec<MolecularWeightProperty>,
	/// <https://schema.org/monoisotopicMolecularWeight>
	pub r#monoisotopic_molecular_weight: Vec<MonoisotopicMolecularWeightProperty>,
	/// <https://schema.org/potentialUse>
	pub r#potential_use: Vec<PotentialUseProperty>,
	/// <https://schema.org/smiles>
	pub r#smiles: Vec<SmilesProperty>,
	/// <https://schema.org/associatedDisease>
	pub r#associated_disease: Vec<AssociatedDiseaseProperty>,
	/// <https://schema.org/bioChemInteraction>
	pub r#bio_chem_interaction: Vec<BioChemInteractionProperty>,
	/// <https://schema.org/bioChemSimilarity>
	pub r#bio_chem_similarity: Vec<BioChemSimilarityProperty>,
	/// <https://schema.org/biologicalRole>
	pub r#biological_role: Vec<BiologicalRoleProperty>,
	/// <https://schema.org/funding>
	pub r#funding: Vec<FundingProperty>,
	/// <https://schema.org/hasBioChemEntityPart>
	pub r#has_bio_chem_entity_part: Vec<HasBioChemEntityPartProperty>,
	/// <https://schema.org/hasMolecularFunction>
	pub r#has_molecular_function: Vec<HasMolecularFunctionProperty>,
	/// <https://schema.org/hasRepresentation>
	pub r#has_representation: Vec<HasRepresentationProperty>,
	/// <https://schema.org/isEncodedByBioChemEntity>
	pub r#is_encoded_by_bio_chem_entity: Vec<IsEncodedByBioChemEntityProperty>,
	/// <https://schema.org/isInvolvedInBiologicalProcess>
	pub r#is_involved_in_biological_process: Vec<IsInvolvedInBiologicalProcessProperty>,
	/// <https://schema.org/isLocatedInSubcellularLocation>
	pub r#is_located_in_subcellular_location: Vec<IsLocatedInSubcellularLocationProperty>,
	/// <https://schema.org/isPartOfBioChemEntity>
	pub r#is_part_of_bio_chem_entity: Vec<IsPartOfBioChemEntityProperty>,
	/// <https://schema.org/taxonomicRange>
	pub r#taxonomic_range: Vec<TaxonomicRangeProperty>,
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
/// This trait is for properties from <https://schema.org/MolecularEntity>.
pub trait MolecularEntityTrait {
	/// Get <https://schema.org/chemicalRole> from [`Self`] as borrowed slice.
	fn get_chemical_role(&self) -> &[ChemicalRoleProperty];
	/// Take <https://schema.org/chemicalRole> from [`Self`] as owned vector.
	fn take_chemical_role(&mut self) -> Vec<ChemicalRoleProperty>;
	/// Get <https://schema.org/inChI> from [`Self`] as borrowed slice.
	fn get_in_ch_i(&self) -> &[InChIProperty];
	/// Take <https://schema.org/inChI> from [`Self`] as owned vector.
	fn take_in_ch_i(&mut self) -> Vec<InChIProperty>;
	/// Get <https://schema.org/inChIKey> from [`Self`] as borrowed slice.
	fn get_in_ch_i_key(&self) -> &[InChIKeyProperty];
	/// Take <https://schema.org/inChIKey> from [`Self`] as owned vector.
	fn take_in_ch_i_key(&mut self) -> Vec<InChIKeyProperty>;
	/// Get <https://schema.org/iupacName> from [`Self`] as borrowed slice.
	fn get_iupac_name(&self) -> &[IupacNameProperty];
	/// Take <https://schema.org/iupacName> from [`Self`] as owned vector.
	fn take_iupac_name(&mut self) -> Vec<IupacNameProperty>;
	/// Get <https://schema.org/molecularFormula> from [`Self`] as borrowed slice.
	fn get_molecular_formula(&self) -> &[MolecularFormulaProperty];
	/// Take <https://schema.org/molecularFormula> from [`Self`] as owned vector.
	fn take_molecular_formula(&mut self) -> Vec<MolecularFormulaProperty>;
	/// Get <https://schema.org/molecularWeight> from [`Self`] as borrowed slice.
	fn get_molecular_weight(&self) -> &[MolecularWeightProperty];
	/// Take <https://schema.org/molecularWeight> from [`Self`] as owned vector.
	fn take_molecular_weight(&mut self) -> Vec<MolecularWeightProperty>;
	/// Get <https://schema.org/monoisotopicMolecularWeight> from [`Self`] as borrowed slice.
	fn get_monoisotopic_molecular_weight(&self) -> &[MonoisotopicMolecularWeightProperty];
	/// Take <https://schema.org/monoisotopicMolecularWeight> from [`Self`] as owned vector.
	fn take_monoisotopic_molecular_weight(&mut self) -> Vec<MonoisotopicMolecularWeightProperty>;
	/// Get <https://schema.org/potentialUse> from [`Self`] as borrowed slice.
	fn get_potential_use(&self) -> &[PotentialUseProperty];
	/// Take <https://schema.org/potentialUse> from [`Self`] as owned vector.
	fn take_potential_use(&mut self) -> Vec<PotentialUseProperty>;
	/// Get <https://schema.org/smiles> from [`Self`] as borrowed slice.
	fn get_smiles(&self) -> &[SmilesProperty];
	/// Take <https://schema.org/smiles> from [`Self`] as owned vector.
	fn take_smiles(&mut self) -> Vec<SmilesProperty>;
}
impl MolecularEntityTrait for MolecularEntity {
	fn get_chemical_role(&self) -> &[ChemicalRoleProperty] {
		self.r#chemical_role.as_slice()
	}
	fn take_chemical_role(&mut self) -> Vec<ChemicalRoleProperty> {
		std::mem::take(&mut self.r#chemical_role)
	}
	fn get_in_ch_i(&self) -> &[InChIProperty] {
		self.r#in_ch_i.as_slice()
	}
	fn take_in_ch_i(&mut self) -> Vec<InChIProperty> {
		std::mem::take(&mut self.r#in_ch_i)
	}
	fn get_in_ch_i_key(&self) -> &[InChIKeyProperty] {
		self.r#in_ch_i_key.as_slice()
	}
	fn take_in_ch_i_key(&mut self) -> Vec<InChIKeyProperty> {
		std::mem::take(&mut self.r#in_ch_i_key)
	}
	fn get_iupac_name(&self) -> &[IupacNameProperty] {
		self.r#iupac_name.as_slice()
	}
	fn take_iupac_name(&mut self) -> Vec<IupacNameProperty> {
		std::mem::take(&mut self.r#iupac_name)
	}
	fn get_molecular_formula(&self) -> &[MolecularFormulaProperty] {
		self.r#molecular_formula.as_slice()
	}
	fn take_molecular_formula(&mut self) -> Vec<MolecularFormulaProperty> {
		std::mem::take(&mut self.r#molecular_formula)
	}
	fn get_molecular_weight(&self) -> &[MolecularWeightProperty] {
		self.r#molecular_weight.as_slice()
	}
	fn take_molecular_weight(&mut self) -> Vec<MolecularWeightProperty> {
		std::mem::take(&mut self.r#molecular_weight)
	}
	fn get_monoisotopic_molecular_weight(&self) -> &[MonoisotopicMolecularWeightProperty] {
		self.r#monoisotopic_molecular_weight.as_slice()
	}
	fn take_monoisotopic_molecular_weight(&mut self) -> Vec<MonoisotopicMolecularWeightProperty> {
		std::mem::take(&mut self.r#monoisotopic_molecular_weight)
	}
	fn get_potential_use(&self) -> &[PotentialUseProperty] {
		self.r#potential_use.as_slice()
	}
	fn take_potential_use(&mut self) -> Vec<PotentialUseProperty> {
		std::mem::take(&mut self.r#potential_use)
	}
	fn get_smiles(&self) -> &[SmilesProperty] {
		self.r#smiles.as_slice()
	}
	fn take_smiles(&mut self) -> Vec<SmilesProperty> {
		std::mem::take(&mut self.r#smiles)
	}
}
impl BioChemEntityTrait for MolecularEntity {
	fn get_associated_disease(&self) -> &[AssociatedDiseaseProperty] {
		self.r#associated_disease.as_slice()
	}
	fn take_associated_disease(&mut self) -> Vec<AssociatedDiseaseProperty> {
		std::mem::take(&mut self.r#associated_disease)
	}
	fn get_bio_chem_interaction(&self) -> &[BioChemInteractionProperty] {
		self.r#bio_chem_interaction.as_slice()
	}
	fn take_bio_chem_interaction(&mut self) -> Vec<BioChemInteractionProperty> {
		std::mem::take(&mut self.r#bio_chem_interaction)
	}
	fn get_bio_chem_similarity(&self) -> &[BioChemSimilarityProperty] {
		self.r#bio_chem_similarity.as_slice()
	}
	fn take_bio_chem_similarity(&mut self) -> Vec<BioChemSimilarityProperty> {
		std::mem::take(&mut self.r#bio_chem_similarity)
	}
	fn get_biological_role(&self) -> &[BiologicalRoleProperty] {
		self.r#biological_role.as_slice()
	}
	fn take_biological_role(&mut self) -> Vec<BiologicalRoleProperty> {
		std::mem::take(&mut self.r#biological_role)
	}
	fn get_funding(&self) -> &[FundingProperty] {
		self.r#funding.as_slice()
	}
	fn take_funding(&mut self) -> Vec<FundingProperty> {
		std::mem::take(&mut self.r#funding)
	}
	fn get_has_bio_chem_entity_part(&self) -> &[HasBioChemEntityPartProperty] {
		self.r#has_bio_chem_entity_part.as_slice()
	}
	fn take_has_bio_chem_entity_part(&mut self) -> Vec<HasBioChemEntityPartProperty> {
		std::mem::take(&mut self.r#has_bio_chem_entity_part)
	}
	fn get_has_molecular_function(&self) -> &[HasMolecularFunctionProperty] {
		self.r#has_molecular_function.as_slice()
	}
	fn take_has_molecular_function(&mut self) -> Vec<HasMolecularFunctionProperty> {
		std::mem::take(&mut self.r#has_molecular_function)
	}
	fn get_has_representation(&self) -> &[HasRepresentationProperty] {
		self.r#has_representation.as_slice()
	}
	fn take_has_representation(&mut self) -> Vec<HasRepresentationProperty> {
		std::mem::take(&mut self.r#has_representation)
	}
	fn get_is_encoded_by_bio_chem_entity(&self) -> &[IsEncodedByBioChemEntityProperty] {
		self.r#is_encoded_by_bio_chem_entity.as_slice()
	}
	fn take_is_encoded_by_bio_chem_entity(&mut self) -> Vec<IsEncodedByBioChemEntityProperty> {
		std::mem::take(&mut self.r#is_encoded_by_bio_chem_entity)
	}
	fn get_is_involved_in_biological_process(&self) -> &[IsInvolvedInBiologicalProcessProperty] {
		self.r#is_involved_in_biological_process.as_slice()
	}
	fn take_is_involved_in_biological_process(
		&mut self,
	) -> Vec<IsInvolvedInBiologicalProcessProperty> {
		std::mem::take(&mut self.r#is_involved_in_biological_process)
	}
	fn get_is_located_in_subcellular_location(&self) -> &[IsLocatedInSubcellularLocationProperty] {
		self.r#is_located_in_subcellular_location.as_slice()
	}
	fn take_is_located_in_subcellular_location(
		&mut self,
	) -> Vec<IsLocatedInSubcellularLocationProperty> {
		std::mem::take(&mut self.r#is_located_in_subcellular_location)
	}
	fn get_is_part_of_bio_chem_entity(&self) -> &[IsPartOfBioChemEntityProperty] {
		self.r#is_part_of_bio_chem_entity.as_slice()
	}
	fn take_is_part_of_bio_chem_entity(&mut self) -> Vec<IsPartOfBioChemEntityProperty> {
		std::mem::take(&mut self.r#is_part_of_bio_chem_entity)
	}
	fn get_taxonomic_range(&self) -> &[TaxonomicRangeProperty] {
		self.r#taxonomic_range.as_slice()
	}
	fn take_taxonomic_range(&mut self) -> Vec<TaxonomicRangeProperty> {
		std::mem::take(&mut self.r#taxonomic_range)
	}
}
impl ThingTrait for MolecularEntity {
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
	impl Serialize for MolecularEntity {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#chemical_role) as usize,
				!Vec::is_empty(&self.r#in_ch_i) as usize,
				!Vec::is_empty(&self.r#in_ch_i_key) as usize,
				!Vec::is_empty(&self.r#iupac_name) as usize,
				!Vec::is_empty(&self.r#molecular_formula) as usize,
				!Vec::is_empty(&self.r#molecular_weight) as usize,
				!Vec::is_empty(&self.r#monoisotopic_molecular_weight) as usize,
				!Vec::is_empty(&self.r#potential_use) as usize,
				!Vec::is_empty(&self.r#smiles) as usize,
				!Vec::is_empty(&self.r#associated_disease) as usize,
				!Vec::is_empty(&self.r#bio_chem_interaction) as usize,
				!Vec::is_empty(&self.r#bio_chem_similarity) as usize,
				!Vec::is_empty(&self.r#biological_role) as usize,
				!Vec::is_empty(&self.r#funding) as usize,
				!Vec::is_empty(&self.r#has_bio_chem_entity_part) as usize,
				!Vec::is_empty(&self.r#has_molecular_function) as usize,
				!Vec::is_empty(&self.r#has_representation) as usize,
				!Vec::is_empty(&self.r#is_encoded_by_bio_chem_entity) as usize,
				!Vec::is_empty(&self.r#is_involved_in_biological_process) as usize,
				!Vec::is_empty(&self.r#is_located_in_subcellular_location) as usize,
				!Vec::is_empty(&self.r#is_part_of_bio_chem_entity) as usize,
				!Vec::is_empty(&self.r#taxonomic_range) as usize,
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
				Serializer::serialize_struct(serializer, "MolecularEntity", len)?;
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
	impl<'de> Deserialize<'de> for MolecularEntity {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				ChemicalRole,
				InChI,
				InChIKey,
				IupacName,
				MolecularFormula,
				MolecularWeight,
				MonoisotopicMolecularWeight,
				PotentialUse,
				Smiles,
				AssociatedDisease,
				BioChemInteraction,
				BioChemSimilarity,
				BiologicalRole,
				Funding,
				HasBioChemEntityPart,
				HasMolecularFunction,
				HasRepresentation,
				IsEncodedByBioChemEntity,
				IsInvolvedInBiologicalProcess,
				IsLocatedInSubcellularLocation,
				IsPartOfBioChemEntity,
				TaxonomicRange,
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
						"chemicalRole" => Ok(Field::ChemicalRole),
						"inChI" => Ok(Field::InChI),
						"inChIKey" => Ok(Field::InChIKey),
						"iupacName" => Ok(Field::IupacName),
						"molecularFormula" => Ok(Field::MolecularFormula),
						"molecularWeight" => Ok(Field::MolecularWeight),
						"monoisotopicMolecularWeight" => Ok(Field::MonoisotopicMolecularWeight),
						"potentialUse" => Ok(Field::PotentialUse),
						"smiles" => Ok(Field::Smiles),
						"associatedDisease" => Ok(Field::AssociatedDisease),
						"bioChemInteraction" => Ok(Field::BioChemInteraction),
						"bioChemSimilarity" => Ok(Field::BioChemSimilarity),
						"biologicalRole" => Ok(Field::BiologicalRole),
						"funding" => Ok(Field::Funding),
						"hasBioChemEntityPart" => Ok(Field::HasBioChemEntityPart),
						"hasMolecularFunction" => Ok(Field::HasMolecularFunction),
						"hasRepresentation" => Ok(Field::HasRepresentation),
						"isEncodedByBioChemEntity" => Ok(Field::IsEncodedByBioChemEntity),
						"isInvolvedInBiologicalProcess" => Ok(Field::IsInvolvedInBiologicalProcess),
						"isLocatedInSubcellularLocation" => {
							Ok(Field::IsLocatedInSubcellularLocation)
						}
						"isPartOfBioChemEntity" => Ok(Field::IsPartOfBioChemEntity),
						"taxonomicRange" => Ok(Field::TaxonomicRange),
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
						"id" | "type" => Ok(Field::Ignore),
						_ => Err(de::Error::unknown_field(value, FIELDS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"chemicalRole" => Ok(Field::ChemicalRole),
						b"inChI" => Ok(Field::InChI),
						b"inChIKey" => Ok(Field::InChIKey),
						b"iupacName" => Ok(Field::IupacName),
						b"molecularFormula" => Ok(Field::MolecularFormula),
						b"molecularWeight" => Ok(Field::MolecularWeight),
						b"monoisotopicMolecularWeight" => Ok(Field::MonoisotopicMolecularWeight),
						b"potentialUse" => Ok(Field::PotentialUse),
						b"smiles" => Ok(Field::Smiles),
						b"associatedDisease" => Ok(Field::AssociatedDisease),
						b"bioChemInteraction" => Ok(Field::BioChemInteraction),
						b"bioChemSimilarity" => Ok(Field::BioChemSimilarity),
						b"biologicalRole" => Ok(Field::BiologicalRole),
						b"funding" => Ok(Field::Funding),
						b"hasBioChemEntityPart" => Ok(Field::HasBioChemEntityPart),
						b"hasMolecularFunction" => Ok(Field::HasMolecularFunction),
						b"hasRepresentation" => Ok(Field::HasRepresentation),
						b"isEncodedByBioChemEntity" => Ok(Field::IsEncodedByBioChemEntity),
						b"isInvolvedInBiologicalProcess" => {
							Ok(Field::IsInvolvedInBiologicalProcess)
						}
						b"isLocatedInSubcellularLocation" => {
							Ok(Field::IsLocatedInSubcellularLocation)
						}
						b"isPartOfBioChemEntity" => Ok(Field::IsPartOfBioChemEntity),
						b"taxonomicRange" => Ok(Field::TaxonomicRange),
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
						b"id" | b"type" => Ok(Field::Ignore),
						_ => {
							let value = &String::from_utf8_lossy(value);
							Err(de::Error::unknown_field(value, FIELDS))
						}
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
					let mut r#chemical_role_property = None;
					let mut r#in_ch_i_property = None;
					let mut r#in_ch_i_key_property = None;
					let mut r#iupac_name_property = None;
					let mut r#molecular_formula_property = None;
					let mut r#molecular_weight_property = None;
					let mut r#monoisotopic_molecular_weight_property = None;
					let mut r#potential_use_property = None;
					let mut r#smiles_property = None;
					let mut r#associated_disease_property = None;
					let mut r#bio_chem_interaction_property = None;
					let mut r#bio_chem_similarity_property = None;
					let mut r#biological_role_property = None;
					let mut r#funding_property = None;
					let mut r#has_bio_chem_entity_part_property = None;
					let mut r#has_molecular_function_property = None;
					let mut r#has_representation_property = None;
					let mut r#is_encoded_by_bio_chem_entity_property = None;
					let mut r#is_involved_in_biological_process_property = None;
					let mut r#is_located_in_subcellular_location_property = None;
					let mut r#is_part_of_bio_chem_entity_property = None;
					let mut r#taxonomic_range_property = None;
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
							Field::Ignore => {
								let _ = map.next_value::<de::IgnoredAny>()?;
							}
						}
					}
					Ok(MolecularEntity {
						r#chemical_role: r#chemical_role_property.unwrap_or_default(),
						r#in_ch_i: r#in_ch_i_property.unwrap_or_default(),
						r#in_ch_i_key: r#in_ch_i_key_property.unwrap_or_default(),
						r#iupac_name: r#iupac_name_property.unwrap_or_default(),
						r#molecular_formula: r#molecular_formula_property.unwrap_or_default(),
						r#molecular_weight: r#molecular_weight_property.unwrap_or_default(),
						r#monoisotopic_molecular_weight: r#monoisotopic_molecular_weight_property
							.unwrap_or_default(),
						r#potential_use: r#potential_use_property.unwrap_or_default(),
						r#smiles: r#smiles_property.unwrap_or_default(),
						r#associated_disease: r#associated_disease_property.unwrap_or_default(),
						r#bio_chem_interaction: r#bio_chem_interaction_property.unwrap_or_default(),
						r#bio_chem_similarity: r#bio_chem_similarity_property.unwrap_or_default(),
						r#biological_role: r#biological_role_property.unwrap_or_default(),
						r#funding: r#funding_property.unwrap_or_default(),
						r#has_bio_chem_entity_part: r#has_bio_chem_entity_part_property
							.unwrap_or_default(),
						r#has_molecular_function: r#has_molecular_function_property
							.unwrap_or_default(),
						r#has_representation: r#has_representation_property.unwrap_or_default(),
						r#is_encoded_by_bio_chem_entity: r#is_encoded_by_bio_chem_entity_property
							.unwrap_or_default(),
						r#is_involved_in_biological_process:
							r#is_involved_in_biological_process_property.unwrap_or_default(),
						r#is_located_in_subcellular_location:
							r#is_located_in_subcellular_location_property.unwrap_or_default(),
						r#is_part_of_bio_chem_entity: r#is_part_of_bio_chem_entity_property
							.unwrap_or_default(),
						r#taxonomic_range: r#taxonomic_range_property.unwrap_or_default(),
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
				"chemicalRole",
				"inChI",
				"inChIKey",
				"iupacName",
				"molecularFormula",
				"molecularWeight",
				"monoisotopicMolecularWeight",
				"potentialUse",
				"smiles",
				"associatedDisease",
				"bioChemInteraction",
				"bioChemSimilarity",
				"biologicalRole",
				"funding",
				"hasBioChemEntityPart",
				"hasMolecularFunction",
				"hasRepresentation",
				"isEncodedByBioChemEntity",
				"isInvolvedInBiologicalProcess",
				"isLocatedInSubcellularLocation",
				"isPartOfBioChemEntity",
				"taxonomicRange",
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
			deserializer.deserialize_struct("MolecularEntity", FIELDS, ClassVisitor)
		}
	}
}
