use super::*;
/// <https://schema.org/Observation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Observation {
    #[cfg(any(
        any(
            feature = "additional-property-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "additionalProperty"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#additional_property: Vec<AdditionalPropertyProperty>,
    #[cfg(any(
        any(
            feature = "additional-type-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "additionalType"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#additional_type: Vec<AdditionalTypeProperty>,
    #[cfg(any(
        any(
            feature = "alternate-name-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "alternateName"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#alternate_name: Vec<AlternateNameProperty>,
    #[cfg(any(
        any(
            feature = "description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "description"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#description: Vec<DescriptionProperty>,
    #[cfg(any(
        any(
            feature = "disambiguating-description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "disambiguatingDescription"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
    #[cfg(any(
        any(
            feature = "identifier-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "identifier"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#identifier: Vec<IdentifierProperty>,
    #[cfg(any(
        any(feature = "image-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "image"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#image: Vec<ImageProperty>,
    #[cfg(any(
        any(
            feature = "main-entity-of-page-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mainEntityOfPage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
    #[cfg(any(
        any(
            feature = "margin-of-error-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "marginOfError"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#margin_of_error: Vec<MarginOfErrorProperty>,
    #[cfg(any(
        any(
            feature = "max-value-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "maxValue"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#max_value: Vec<MaxValueProperty>,
    #[cfg(any(
        any(
            feature = "measured-property-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "measuredProperty"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#measured_property: Vec<MeasuredPropertyProperty>,
    #[cfg(any(
        any(
            feature = "measurement-denominator-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "measurementDenominator"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#measurement_denominator: Vec<MeasurementDenominatorProperty>,
    #[cfg(any(
        any(
            feature = "measurement-method-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "measurementMethod"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#measurement_method: Vec<MeasurementMethodProperty>,
    #[cfg(any(
        any(
            feature = "measurement-qualifier-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "measurementQualifier"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#measurement_qualifier: Vec<MeasurementQualifierProperty>,
    #[cfg(any(
        any(
            feature = "measurement-technique-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "measurementTechnique"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#measurement_technique: Vec<MeasurementTechniqueProperty>,
    #[cfg(any(
        any(
            feature = "min-value-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "minValue"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#min_value: Vec<MinValueProperty>,
    #[cfg(any(
        any(feature = "name-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#name: Vec<NameProperty>,
    #[cfg(any(
        any(
            feature = "observation-about-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "observationAbout"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#observation_about: Vec<ObservationAboutProperty>,
    #[cfg(any(
        any(
            feature = "observation-date-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "observationDate"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#observation_date: Vec<ObservationDateProperty>,
    #[cfg(any(
        any(
            feature = "observation-period-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "observationPeriod"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#observation_period: Vec<ObservationPeriodProperty>,
    #[cfg(any(
        any(
            feature = "potential-action-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "potentialAction"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#potential_action: Vec<PotentialActionProperty>,
    #[cfg(any(
        any(
            feature = "same-as-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sameAs"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#same_as: Vec<SameAsProperty>,
    #[cfg(any(
        any(
            feature = "subject-of-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "subjectOf"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#subject_of: Vec<SubjectOfProperty>,
    #[cfg(any(
        any(
            feature = "unit-code-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "unitCode"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#unit_code: Vec<UnitCodeProperty>,
    #[cfg(any(
        any(
            feature = "unit-text-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "unitText"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#unit_text: Vec<UnitTextProperty>,
    #[cfg(any(
        any(feature = "url-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "url"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#url: Vec<UrlProperty>,
    #[cfg(any(
        any(feature = "value-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "value"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#value: Vec<ValueProperty>,
    #[cfg(any(
        any(
            feature = "value-reference-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "valueReference"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#value_reference: Vec<ValueReferenceProperty>,
    #[cfg(any(
        any(
            feature = "variable-measured-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "variableMeasured"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#variable_measured: Vec<VariableMeasuredProperty>,
}
