use super::*;
#[cfg(any(
    any(
        feature = "action-status-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#action_status_type;
#[cfg(any(
    any(
        feature = "action-status-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#action_status_type::*;
#[cfg(any(
    any(
        feature = "adult-oriented-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#adult_oriented_enumeration;
#[cfg(any(
    any(
        feature = "adult-oriented-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#adult_oriented_enumeration::*;
#[cfg(any(
    any(feature = "bed-type-schema", feature = "general-schema-section"),
    doc
))]
mod r#bed_type;
#[cfg(any(
    any(feature = "bed-type-schema", feature = "general-schema-section"),
    doc
))]
pub use r#bed_type::*;
#[cfg(any(
    any(
        feature = "boarding-policy-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#boarding_policy_type;
#[cfg(any(
    any(
        feature = "boarding-policy-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#boarding_policy_type::*;
#[cfg(any(
    any(
        feature = "body-measurement-type-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#body_measurement_type_enumeration;
#[cfg(any(
    any(
        feature = "body-measurement-type-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#body_measurement_type_enumeration::*;
#[cfg(any(
    any(
        feature = "book-format-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#book_format_type;
#[cfg(any(
    any(
        feature = "book-format-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#book_format_type::*;
#[cfg(any(
    any(
        feature = "business-entity-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#business_entity_type;
#[cfg(any(
    any(
        feature = "business-entity-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#business_entity_type::*;
#[cfg(any(
    any(
        feature = "business-function-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#business_function;
#[cfg(any(
    any(
        feature = "business-function-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#business_function::*;
#[cfg(any(
    any(feature = "car-usage-type-schema", feature = "auto-schema-section"),
    doc
))]
mod r#car_usage_type;
#[cfg(any(
    any(feature = "car-usage-type-schema", feature = "auto-schema-section"),
    doc
))]
pub use r#car_usage_type::*;
#[cfg(any(
    any(
        feature = "contact-point-option-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#contact_point_option;
#[cfg(any(
    any(
        feature = "contact-point-option-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#contact_point_option::*;
#[cfg(any(
    any(feature = "credit-card-schema", feature = "general-schema-section"),
    doc
))]
mod r#credit_card;
#[cfg(any(
    any(feature = "credit-card-schema", feature = "general-schema-section"),
    doc
))]
pub use r#credit_card::*;
#[cfg(any(
    any(feature = "day-of-week-schema", feature = "general-schema-section"),
    doc
))]
mod r#day_of_week;
#[cfg(any(
    any(feature = "day-of-week-schema", feature = "general-schema-section"),
    doc
))]
pub use r#day_of_week::*;
#[cfg(any(
    any(feature = "delivery-method-schema", feature = "general-schema-section"),
    doc
))]
mod r#delivery_method;
#[cfg(any(
    any(feature = "delivery-method-schema", feature = "general-schema-section"),
    doc
))]
pub use r#delivery_method::*;
#[cfg(any(
    any(
        feature = "digital-document-permission-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#digital_document_permission_type;
#[cfg(any(
    any(
        feature = "digital-document-permission-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#digital_document_permission_type::*;
#[cfg(any(
    any(
        feature = "digital-platform-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#digital_platform_enumeration;
#[cfg(any(
    any(
        feature = "digital-platform-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#digital_platform_enumeration::*;
#[cfg(any(
    any(
        feature = "drive-wheel-configuration-value-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#drive_wheel_configuration_value;
#[cfg(any(
    any(
        feature = "drive-wheel-configuration-value-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#drive_wheel_configuration_value::*;
#[cfg(any(
    any(
        feature = "drug-cost-category-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#drug_cost_category;
#[cfg(any(
    any(
        feature = "drug-cost-category-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#drug_cost_category::*;
#[cfg(any(
    any(
        feature = "drug-pregnancy-category-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#drug_pregnancy_category;
#[cfg(any(
    any(
        feature = "drug-pregnancy-category-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#drug_pregnancy_category::*;
#[cfg(any(
    any(
        feature = "drug-prescription-status-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#drug_prescription_status;
#[cfg(any(
    any(
        feature = "drug-prescription-status-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#drug_prescription_status::*;
#[cfg(any(
    any(
        feature = "eu-energy-efficiency-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#eu_energy_efficiency_enumeration;
#[cfg(any(
    any(
        feature = "eu-energy-efficiency-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#eu_energy_efficiency_enumeration::*;
#[cfg(any(
    any(
        feature = "energy-efficiency-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#energy_efficiency_enumeration;
#[cfg(any(
    any(
        feature = "energy-efficiency-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#energy_efficiency_enumeration::*;
#[cfg(any(
    any(
        feature = "energy-star-energy-efficiency-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#energy_star_energy_efficiency_enumeration;
#[cfg(any(
    any(
        feature = "energy-star-energy-efficiency-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#energy_star_energy_efficiency_enumeration::*;
#[cfg(any(
    any(
        feature = "event-attendance-mode-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#event_attendance_mode_enumeration;
#[cfg(any(
    any(
        feature = "event-attendance-mode-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#event_attendance_mode_enumeration::*;
#[cfg(any(
    any(
        feature = "event-status-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#event_status_type;
#[cfg(any(
    any(
        feature = "event-status-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#event_status_type::*;
#[cfg(any(
    any(
        feature = "game-availability-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#game_availability_enumeration;
#[cfg(any(
    any(
        feature = "game-availability-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#game_availability_enumeration::*;
#[cfg(any(
    any(feature = "game-play-mode-schema", feature = "general-schema-section"),
    doc
))]
mod r#game_play_mode;
#[cfg(any(
    any(feature = "game-play-mode-schema", feature = "general-schema-section"),
    doc
))]
pub use r#game_play_mode::*;
#[cfg(any(
    any(
        feature = "game-server-status-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#game_server_status;
#[cfg(any(
    any(
        feature = "game-server-status-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#game_server_status::*;
#[cfg(any(
    any(feature = "gender-type-schema", feature = "general-schema-section"),
    doc
))]
mod r#gender_type;
#[cfg(any(
    any(feature = "gender-type-schema", feature = "general-schema-section"),
    doc
))]
pub use r#gender_type::*;
#[cfg(any(
    any(
        feature = "government-benefits-type-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#government_benefits_type;
#[cfg(any(
    any(
        feature = "government-benefits-type-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#government_benefits_type::*;
#[cfg(any(
    any(
        feature = "health-aspect-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#health_aspect_enumeration;
#[cfg(any(
    any(
        feature = "health-aspect-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#health_aspect_enumeration::*;
#[cfg(any(
    any(
        feature = "infectious-agent-class-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#infectious_agent_class;
#[cfg(any(
    any(
        feature = "infectious-agent-class-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#infectious_agent_class::*;
#[cfg(any(
    any(
        feature = "item-availability-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#item_availability;
#[cfg(any(
    any(
        feature = "item-availability-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#item_availability::*;
#[cfg(any(
    any(
        feature = "item-list-order-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#item_list_order_type;
#[cfg(any(
    any(
        feature = "item-list-order-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#item_list_order_type::*;
#[cfg(any(
    any(
        feature = "legal-force-status-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#legal_force_status;
#[cfg(any(
    any(
        feature = "legal-force-status-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#legal_force_status::*;
#[cfg(any(
    any(
        feature = "legal-value-level-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#legal_value_level;
#[cfg(any(
    any(
        feature = "legal-value-level-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#legal_value_level::*;
#[cfg(any(
    any(
        feature = "map-category-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#map_category_type;
#[cfg(any(
    any(
        feature = "map-category-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#map_category_type::*;
#[cfg(any(
    any(
        feature = "measurement-method-enum-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#measurement_method_enum;
#[cfg(any(
    any(
        feature = "measurement-method-enum-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#measurement_method_enum::*;
#[cfg(any(
    any(
        feature = "measurement-type-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#measurement_type_enumeration;
#[cfg(any(
    any(
        feature = "measurement-type-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#measurement_type_enumeration::*;
#[cfg(any(
    any(
        feature = "media-manipulation-rating-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#media_manipulation_rating_enumeration;
#[cfg(any(
    any(
        feature = "media-manipulation-rating-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#media_manipulation_rating_enumeration::*;
#[cfg(any(
    any(
        feature = "medical-audience-type-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#medical_audience_type;
#[cfg(any(
    any(
        feature = "medical-audience-type-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#medical_audience_type::*;
#[cfg(any(
    any(
        feature = "medical-device-purpose-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#medical_device_purpose;
#[cfg(any(
    any(
        feature = "medical-device-purpose-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#medical_device_purpose::*;
#[cfg(any(
    any(
        feature = "medical-enumeration-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#medical_enumeration;
#[cfg(any(
    any(
        feature = "medical-enumeration-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#medical_enumeration::*;
#[cfg(any(
    any(
        feature = "medical-evidence-level-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#medical_evidence_level;
#[cfg(any(
    any(
        feature = "medical-evidence-level-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#medical_evidence_level::*;
#[cfg(any(
    any(
        feature = "medical-imaging-technique-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#medical_imaging_technique;
#[cfg(any(
    any(
        feature = "medical-imaging-technique-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#medical_imaging_technique::*;
#[cfg(any(
    any(
        feature = "medical-observational-study-design-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#medical_observational_study_design;
#[cfg(any(
    any(
        feature = "medical-observational-study-design-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#medical_observational_study_design::*;
#[cfg(any(
    any(
        feature = "medical-procedure-type-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#medical_procedure_type;
#[cfg(any(
    any(
        feature = "medical-procedure-type-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#medical_procedure_type::*;
#[cfg(any(
    any(
        feature = "medical-specialty-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#medical_specialty;
#[cfg(any(
    any(
        feature = "medical-specialty-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#medical_specialty::*;
#[cfg(any(
    any(
        feature = "medical-study-status-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#medical_study_status;
#[cfg(any(
    any(
        feature = "medical-study-status-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#medical_study_status::*;
#[cfg(any(
    any(
        feature = "medical-trial-design-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#medical_trial_design;
#[cfg(any(
    any(
        feature = "medical-trial-design-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#medical_trial_design::*;
#[cfg(any(
    any(
        feature = "medicine-system-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#medicine_system;
#[cfg(any(
    any(
        feature = "medicine-system-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#medicine_system::*;
#[cfg(any(
    any(
        feature = "merchant-return-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#merchant_return_enumeration;
#[cfg(any(
    any(
        feature = "merchant-return-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#merchant_return_enumeration::*;
#[cfg(any(
    any(
        feature = "music-album-production-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#music_album_production_type;
#[cfg(any(
    any(
        feature = "music-album-production-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#music_album_production_type::*;
#[cfg(any(
    any(
        feature = "music-album-release-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#music_album_release_type;
#[cfg(any(
    any(
        feature = "music-album-release-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#music_album_release_type::*;
#[cfg(any(
    any(
        feature = "music-release-format-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#music_release_format_type;
#[cfg(any(
    any(
        feature = "music-release-format-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#music_release_format_type::*;
#[cfg(any(
    any(
        feature = "nl-nonprofit-type-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#nl_nonprofit_type;
#[cfg(any(
    any(
        feature = "nl-nonprofit-type-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#nl_nonprofit_type::*;
#[cfg(any(
    any(feature = "nonprofit-type-schema", feature = "pending-schema-section"),
    doc
))]
mod r#nonprofit_type;
#[cfg(any(
    any(feature = "nonprofit-type-schema", feature = "pending-schema-section"),
    doc
))]
pub use r#nonprofit_type::*;
#[cfg(any(
    any(
        feature = "offer-item-condition-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#offer_item_condition;
#[cfg(any(
    any(
        feature = "offer-item-condition-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#offer_item_condition::*;
#[cfg(any(
    any(feature = "order-status-schema", feature = "general-schema-section"),
    doc
))]
mod r#order_status;
#[cfg(any(
    any(feature = "order-status-schema", feature = "general-schema-section"),
    doc
))]
pub use r#order_status::*;
#[cfg(any(
    any(feature = "payment-card-schema", feature = "general-schema-section"),
    doc
))]
mod r#payment_card;
#[cfg(any(
    any(feature = "payment-card-schema", feature = "general-schema-section"),
    doc
))]
pub use r#payment_card::*;
#[cfg(any(
    any(feature = "payment-method-schema", feature = "general-schema-section"),
    doc
))]
mod r#payment_method;
#[cfg(any(
    any(feature = "payment-method-schema", feature = "general-schema-section"),
    doc
))]
pub use r#payment_method::*;
#[cfg(any(
    any(
        feature = "payment-status-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#payment_status_type;
#[cfg(any(
    any(
        feature = "payment-status-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#payment_status_type::*;
#[cfg(any(
    any(
        feature = "physical-activity-category-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#physical_activity_category;
#[cfg(any(
    any(
        feature = "physical-activity-category-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#physical_activity_category::*;
#[cfg(any(
    any(
        feature = "physical-exam-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#physical_exam;
#[cfg(any(
    any(
        feature = "physical-exam-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#physical_exam::*;
#[cfg(any(
    any(
        feature = "price-component-type-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#price_component_type_enumeration;
#[cfg(any(
    any(
        feature = "price-component-type-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#price_component_type_enumeration::*;
#[cfg(any(
    any(
        feature = "price-type-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#price_type_enumeration;
#[cfg(any(
    any(
        feature = "price-type-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#price_type_enumeration::*;
#[cfg(any(
    any(
        feature = "product-return-enumeration-schema",
        feature = "attic-schema-section"
    ),
    doc
))]
mod r#product_return_enumeration;
#[cfg(any(
    any(
        feature = "product-return-enumeration-schema",
        feature = "attic-schema-section"
    ),
    doc
))]
pub use r#product_return_enumeration::*;
#[cfg(any(
    any(
        feature = "qualitative-value-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#qualitative_value;
#[cfg(any(
    any(
        feature = "qualitative-value-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#qualitative_value::*;
#[cfg(any(
    any(
        feature = "refund-type-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#refund_type_enumeration;
#[cfg(any(
    any(
        feature = "refund-type-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#refund_type_enumeration::*;
#[cfg(any(
    any(
        feature = "reservation-status-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#reservation_status_type;
#[cfg(any(
    any(
        feature = "reservation-status-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#reservation_status_type::*;
#[cfg(any(
    any(feature = "restricted-diet-schema", feature = "general-schema-section"),
    doc
))]
mod r#restricted_diet;
#[cfg(any(
    any(feature = "restricted-diet-schema", feature = "general-schema-section"),
    doc
))]
pub use r#restricted_diet::*;
#[cfg(any(
    any(
        feature = "return-fees-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#return_fees_enumeration;
#[cfg(any(
    any(
        feature = "return-fees-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#return_fees_enumeration::*;
#[cfg(any(
    any(
        feature = "return-label-source-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#return_label_source_enumeration;
#[cfg(any(
    any(
        feature = "return-label-source-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#return_label_source_enumeration::*;
#[cfg(any(
    any(
        feature = "return-method-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#return_method_enumeration;
#[cfg(any(
    any(
        feature = "return-method-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#return_method_enumeration::*;
#[cfg(any(
    any(
        feature = "rsvp-response-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#rsvp_response_type;
#[cfg(any(
    any(
        feature = "rsvp-response-type-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#rsvp_response_type::*;
#[cfg(any(
    any(
        feature = "size-group-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#size_group_enumeration;
#[cfg(any(
    any(
        feature = "size-group-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#size_group_enumeration::*;
#[cfg(any(
    any(
        feature = "size-specification-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#size_specification;
#[cfg(any(
    any(
        feature = "size-specification-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#size_specification::*;
#[cfg(any(
    any(
        feature = "size-system-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#size_system_enumeration;
#[cfg(any(
    any(
        feature = "size-system-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#size_system_enumeration::*;
#[cfg(any(
    any(feature = "specialty-schema", feature = "general-schema-section"),
    doc
))]
mod r#specialty;
#[cfg(any(
    any(feature = "specialty-schema", feature = "general-schema-section"),
    doc
))]
pub use r#specialty::*;
#[cfg(any(
    any(
        feature = "status-enumeration-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#status_enumeration;
#[cfg(any(
    any(
        feature = "status-enumeration-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#status_enumeration::*;
#[cfg(any(
    any(
        feature = "steering-position-value-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#steering_position_value;
#[cfg(any(
    any(
        feature = "steering-position-value-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#steering_position_value::*;
#[cfg(any(
    any(
        feature = "uk-nonprofit-type-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#uk_nonprofit_type;
#[cfg(any(
    any(
        feature = "uk-nonprofit-type-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#uk_nonprofit_type::*;
#[cfg(any(
    any(
        feature = "us-nonprofit-type-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#us_nonprofit_type;
#[cfg(any(
    any(
        feature = "us-nonprofit-type-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#us_nonprofit_type::*;
#[cfg(any(
    any(feature = "warranty-scope-schema", feature = "general-schema-section"),
    doc
))]
mod r#warranty_scope;
#[cfg(any(
    any(feature = "warranty-scope-schema", feature = "general-schema-section"),
    doc
))]
pub use r#warranty_scope::*;
#[cfg(any(
    any(
        feature = "wearable-measurement-type-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#wearable_measurement_type_enumeration;
#[cfg(any(
    any(
        feature = "wearable-measurement-type-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#wearable_measurement_type_enumeration::*;
#[cfg(any(
    any(
        feature = "wearable-size-group-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#wearable_size_group_enumeration;
#[cfg(any(
    any(
        feature = "wearable-size-group-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#wearable_size_group_enumeration::*;
#[cfg(any(
    any(
        feature = "wearable-size-system-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#wearable_size_system_enumeration;
#[cfg(any(
    any(
        feature = "wearable-size-system-enumeration-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#wearable_size_system_enumeration::*;
