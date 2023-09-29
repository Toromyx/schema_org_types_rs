use super::*;
/// <https://schema.org/MolecularEntity>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MolecularEntity {
    #[cfg(any(
        any(
            feature = "additional-type-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "additionalType"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/additionalType"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/additionalType"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#additional_type: Vec<AdditionalTypeProperty>,
    #[cfg(any(
        any(
            feature = "alternate-name-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "alternateName"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/alternateName"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/alternateName"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#alternate_name: Vec<AlternateNameProperty>,
    #[cfg(any(
        any(
            feature = "associated-disease-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "associatedDisease"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/associatedDisease")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/associatedDisease")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#associated_disease: Vec<AssociatedDiseaseProperty>,
    #[cfg(any(
        any(
            feature = "bio-chem-interaction-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "bioChemInteraction"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/bioChemInteraction")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/bioChemInteraction")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#bio_chem_interaction: Vec<BioChemInteractionProperty>,
    #[cfg(any(
        any(
            feature = "bio-chem-similarity-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "bioChemSimilarity"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/bioChemSimilarity")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/bioChemSimilarity")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#bio_chem_similarity: Vec<BioChemSimilarityProperty>,
    #[cfg(any(
        any(
            feature = "biological-role-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "biologicalRole"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/biologicalRole"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/biologicalRole"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#biological_role: Vec<BiologicalRoleProperty>,
    #[cfg(any(
        any(
            feature = "chemical-role-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "chemicalRole"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/chemicalRole"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/chemicalRole"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#chemical_role: Vec<ChemicalRoleProperty>,
    #[cfg(any(
        any(
            feature = "description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "description"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/description"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/description"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#description: Vec<DescriptionProperty>,
    #[cfg(any(
        any(
            feature = "disambiguating-description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "disambiguatingDescription"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/disambiguatingDescription")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/disambiguatingDescription")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
    #[cfg(any(
        any(
            feature = "funding-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "funding"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/funding"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/funding"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#funding: Vec<FundingProperty>,
    #[cfg(any(
        any(
            feature = "has-bio-chem-entity-part-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasBioChemEntityPart"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/hasBioChemEntityPart")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/hasBioChemEntityPart")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#has_bio_chem_entity_part: Vec<HasBioChemEntityPartProperty>,
    #[cfg(any(
        any(
            feature = "has-molecular-function-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasMolecularFunction"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/hasMolecularFunction")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/hasMolecularFunction")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#has_molecular_function: Vec<HasMolecularFunctionProperty>,
    #[cfg(any(
        any(
            feature = "has-representation-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasRepresentation"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/hasRepresentation")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/hasRepresentation")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#has_representation: Vec<HasRepresentationProperty>,
    #[cfg(any(
        any(
            feature = "identifier-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "identifier"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/identifier"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/identifier"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#identifier: Vec<IdentifierProperty>,
    #[cfg(any(
        any(feature = "image-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "image"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/image"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/image"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#image: Vec<ImageProperty>,
    #[cfg(any(
        any(
            feature = "in-ch-i-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "inChI"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/inChI"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/inChI"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#in_ch_i: Vec<InChIProperty>,
    #[cfg(any(
        any(
            feature = "in-ch-i-key-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "inChIKey"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/inChIKey"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/inChIKey"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#in_ch_i_key: Vec<InChIKeyProperty>,
    #[cfg(any(
        any(
            feature = "is-encoded-by-bio-chem-entity-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isEncodedByBioChemEntity"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/isEncodedByBioChemEntity")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/isEncodedByBioChemEntity")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#is_encoded_by_bio_chem_entity: Vec<IsEncodedByBioChemEntityProperty>,
    #[cfg(any(
        any(
            feature = "is-involved-in-biological-process-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isInvolvedInBiologicalProcess"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/isInvolvedInBiologicalProcess")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/isInvolvedInBiologicalProcess")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#is_involved_in_biological_process: Vec<IsInvolvedInBiologicalProcessProperty>,
    #[cfg(any(
        any(
            feature = "is-located-in-subcellular-location-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isLocatedInSubcellularLocation"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/isLocatedInSubcellularLocation")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/isLocatedInSubcellularLocation")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#is_located_in_subcellular_location: Vec<IsLocatedInSubcellularLocationProperty>,
    #[cfg(any(
        any(
            feature = "is-part-of-bio-chem-entity-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isPartOfBioChemEntity"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/isPartOfBioChemEntity")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/isPartOfBioChemEntity")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#is_part_of_bio_chem_entity: Vec<IsPartOfBioChemEntityProperty>,
    #[cfg(any(
        any(
            feature = "iupac-name-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "iupacName"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/iupacName"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/iupacName"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#iupac_name: Vec<IupacNameProperty>,
    #[cfg(any(
        any(
            feature = "main-entity-of-page-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mainEntityOfPage"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/mainEntityOfPage")
    )]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/mainEntityOfPage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
    #[cfg(any(
        any(
            feature = "molecular-formula-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "molecularFormula"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/molecularFormula")
    )]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/molecularFormula"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#molecular_formula: Vec<MolecularFormulaProperty>,
    #[cfg(any(
        any(
            feature = "molecular-weight-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "molecularWeight"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/molecularWeight"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/molecularWeight"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#molecular_weight: Vec<MolecularWeightProperty>,
    #[cfg(any(
        any(
            feature = "monoisotopic-molecular-weight-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "monoisotopicMolecularWeight"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/monoisotopicMolecularWeight")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/monoisotopicMolecularWeight")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#monoisotopic_molecular_weight: Vec<MonoisotopicMolecularWeightProperty>,
    #[cfg(any(
        any(feature = "name-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/name"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/name"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#name: Vec<NameProperty>,
    #[cfg(any(
        any(
            feature = "potential-action-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "potentialAction"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/potentialAction"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/potentialAction"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#potential_action: Vec<PotentialActionProperty>,
    #[cfg(any(
        any(
            feature = "potential-use-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "potentialUse"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/potentialUse"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/potentialUse"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#potential_use: Vec<PotentialUseProperty>,
    #[cfg(any(
        any(
            feature = "same-as-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sameAs"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/sameAs"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/sameAs"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#same_as: Vec<SameAsProperty>,
    #[cfg(any(
        any(feature = "smiles-property-schema", feature = "pending-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "smiles"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/smiles"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/smiles"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#smiles: Vec<SmilesProperty>,
    #[cfg(any(
        any(
            feature = "subject-of-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "subjectOf"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/subjectOf"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/subjectOf"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#subject_of: Vec<SubjectOfProperty>,
    #[cfg(any(
        any(
            feature = "taxonomic-range-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "taxonomicRange"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/taxonomicRange"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/taxonomicRange"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#taxonomic_range: Vec<TaxonomicRangeProperty>,
    #[cfg(any(
        any(feature = "url-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "url"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/url"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/url"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#url: Vec<UrlProperty>,
}
