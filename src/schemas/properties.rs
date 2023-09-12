use super::*;
#[cfg(any(feature = "about-property-schema", feature = "general-schema-section"))]
mod r#about;
#[cfg(any(feature = "about-property-schema", feature = "general-schema-section"))]
pub use r#about::*;
#[cfg(any(feature = "abridged-property-schema", feature = "bib-schema-section"))]
mod r#abridged;
#[cfg(any(feature = "abridged-property-schema", feature = "bib-schema-section"))]
pub use r#abridged::*;
#[cfg(any(
    feature = "abstract-property-schema",
    feature = "pending-schema-section"
))]
mod r#abstract;
#[cfg(any(
    feature = "abstract-property-schema",
    feature = "pending-schema-section"
))]
pub use r#abstract::*;
#[cfg(any(
    feature = "acceleration-time-property-schema",
    feature = "auto-schema-section"
))]
mod r#acceleration_time;
#[cfg(any(
    feature = "acceleration-time-property-schema",
    feature = "auto-schema-section"
))]
pub use r#acceleration_time::*;
#[cfg(any(
    feature = "accepted-answer-property-schema",
    feature = "general-schema-section"
))]
mod r#accepted_answer;
#[cfg(any(
    feature = "accepted-answer-property-schema",
    feature = "general-schema-section"
))]
pub use r#accepted_answer::*;
#[cfg(any(
    feature = "accepted-offer-property-schema",
    feature = "general-schema-section"
))]
mod r#accepted_offer;
#[cfg(any(
    feature = "accepted-offer-property-schema",
    feature = "general-schema-section"
))]
pub use r#accepted_offer::*;
#[cfg(any(
    feature = "accepted-payment-method-property-schema",
    feature = "general-schema-section"
))]
mod r#accepted_payment_method;
#[cfg(any(
    feature = "accepted-payment-method-property-schema",
    feature = "general-schema-section"
))]
pub use r#accepted_payment_method::*;
#[cfg(any(
    feature = "accepts-reservations-property-schema",
    feature = "general-schema-section"
))]
mod r#accepts_reservations;
#[cfg(any(
    feature = "accepts-reservations-property-schema",
    feature = "general-schema-section"
))]
pub use r#accepts_reservations::*;
#[cfg(any(
    feature = "access-code-property-schema",
    feature = "general-schema-section"
))]
mod r#access_code;
#[cfg(any(
    feature = "access-code-property-schema",
    feature = "general-schema-section"
))]
pub use r#access_code::*;
#[cfg(any(
    feature = "access-mode-property-schema",
    feature = "general-schema-section"
))]
mod r#access_mode;
#[cfg(any(
    feature = "access-mode-property-schema",
    feature = "general-schema-section"
))]
pub use r#access_mode::*;
#[cfg(any(
    feature = "access-mode-sufficient-property-schema",
    feature = "general-schema-section"
))]
mod r#access_mode_sufficient;
#[cfg(any(
    feature = "access-mode-sufficient-property-schema",
    feature = "general-schema-section"
))]
pub use r#access_mode_sufficient::*;
#[cfg(any(
    feature = "accessibility-api-property-schema",
    feature = "general-schema-section"
))]
mod r#accessibility_api;
#[cfg(any(
    feature = "accessibility-api-property-schema",
    feature = "general-schema-section"
))]
pub use r#accessibility_api::*;
#[cfg(any(
    feature = "accessibility-control-property-schema",
    feature = "general-schema-section"
))]
mod r#accessibility_control;
#[cfg(any(
    feature = "accessibility-control-property-schema",
    feature = "general-schema-section"
))]
pub use r#accessibility_control::*;
#[cfg(any(
    feature = "accessibility-feature-property-schema",
    feature = "general-schema-section"
))]
mod r#accessibility_feature;
#[cfg(any(
    feature = "accessibility-feature-property-schema",
    feature = "general-schema-section"
))]
pub use r#accessibility_feature::*;
#[cfg(any(
    feature = "accessibility-hazard-property-schema",
    feature = "general-schema-section"
))]
mod r#accessibility_hazard;
#[cfg(any(
    feature = "accessibility-hazard-property-schema",
    feature = "general-schema-section"
))]
pub use r#accessibility_hazard::*;
#[cfg(any(
    feature = "accessibility-summary-property-schema",
    feature = "general-schema-section"
))]
mod r#accessibility_summary;
#[cfg(any(
    feature = "accessibility-summary-property-schema",
    feature = "general-schema-section"
))]
pub use r#accessibility_summary::*;
#[cfg(any(
    feature = "accommodation-category-property-schema",
    feature = "pending-schema-section"
))]
mod r#accommodation_category;
#[cfg(any(
    feature = "accommodation-category-property-schema",
    feature = "pending-schema-section"
))]
pub use r#accommodation_category::*;
#[cfg(any(
    feature = "accommodation-floor-plan-property-schema",
    feature = "pending-schema-section"
))]
mod r#accommodation_floor_plan;
#[cfg(any(
    feature = "accommodation-floor-plan-property-schema",
    feature = "pending-schema-section"
))]
pub use r#accommodation_floor_plan::*;
#[cfg(any(
    feature = "account-id-property-schema",
    feature = "general-schema-section"
))]
mod r#account_id;
#[cfg(any(
    feature = "account-id-property-schema",
    feature = "general-schema-section"
))]
pub use r#account_id::*;
#[cfg(any(
    feature = "account-minimum-inflow-property-schema",
    feature = "pending-schema-section"
))]
mod r#account_minimum_inflow;
#[cfg(any(
    feature = "account-minimum-inflow-property-schema",
    feature = "pending-schema-section"
))]
pub use r#account_minimum_inflow::*;
#[cfg(any(
    feature = "account-overdraft-limit-property-schema",
    feature = "pending-schema-section"
))]
mod r#account_overdraft_limit;
#[cfg(any(
    feature = "account-overdraft-limit-property-schema",
    feature = "pending-schema-section"
))]
pub use r#account_overdraft_limit::*;
#[cfg(any(
    feature = "accountable-person-property-schema",
    feature = "general-schema-section"
))]
mod r#accountable_person;
#[cfg(any(
    feature = "accountable-person-property-schema",
    feature = "general-schema-section"
))]
pub use r#accountable_person::*;
#[cfg(any(
    feature = "acquire-license-page-property-schema",
    feature = "pending-schema-section"
))]
mod r#acquire_license_page;
#[cfg(any(
    feature = "acquire-license-page-property-schema",
    feature = "pending-schema-section"
))]
pub use r#acquire_license_page::*;
#[cfg(any(
    feature = "acquired-from-property-schema",
    feature = "general-schema-section"
))]
mod r#acquired_from;
#[cfg(any(
    feature = "acquired-from-property-schema",
    feature = "general-schema-section"
))]
pub use r#acquired_from::*;
#[cfg(any(
    feature = "acriss-code-property-schema",
    feature = "auto-schema-section"
))]
mod r#acriss_code;
#[cfg(any(
    feature = "acriss-code-property-schema",
    feature = "auto-schema-section"
))]
pub use r#acriss_code::*;
#[cfg(any(
    feature = "action-accessibility-requirement-property-schema",
    feature = "general-schema-section"
))]
mod r#action_accessibility_requirement;
#[cfg(any(
    feature = "action-accessibility-requirement-property-schema",
    feature = "general-schema-section"
))]
pub use r#action_accessibility_requirement::*;
#[cfg(any(
    feature = "action-application-property-schema",
    feature = "general-schema-section"
))]
mod r#action_application;
#[cfg(any(
    feature = "action-application-property-schema",
    feature = "general-schema-section"
))]
pub use r#action_application::*;
#[cfg(any(
    feature = "action-option-property-schema",
    feature = "general-schema-section"
))]
mod r#action_option;
#[cfg(any(
    feature = "action-option-property-schema",
    feature = "general-schema-section"
))]
pub use r#action_option::*;
#[cfg(any(
    feature = "action-platform-property-schema",
    feature = "general-schema-section"
))]
mod r#action_platform;
#[cfg(any(
    feature = "action-platform-property-schema",
    feature = "general-schema-section"
))]
pub use r#action_platform::*;
#[cfg(any(
    feature = "action-status-property-schema",
    feature = "general-schema-section"
))]
mod r#action_status;
#[cfg(any(
    feature = "action-status-property-schema",
    feature = "general-schema-section"
))]
pub use r#action_status::*;
#[cfg(any(
    feature = "actionable-feedback-policy-property-schema",
    feature = "pending-schema-section"
))]
mod r#actionable_feedback_policy;
#[cfg(any(
    feature = "actionable-feedback-policy-property-schema",
    feature = "pending-schema-section"
))]
pub use r#actionable_feedback_policy::*;
#[cfg(any(
    feature = "active-ingredient-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#active_ingredient;
#[cfg(any(
    feature = "active-ingredient-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#active_ingredient::*;
#[cfg(any(
    feature = "activity-duration-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#activity_duration;
#[cfg(any(
    feature = "activity-duration-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#activity_duration::*;
#[cfg(any(
    feature = "activity-frequency-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#activity_frequency;
#[cfg(any(
    feature = "activity-frequency-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#activity_frequency::*;
#[cfg(any(feature = "actor-property-schema", feature = "general-schema-section"))]
mod r#actor;
#[cfg(any(feature = "actor-property-schema", feature = "general-schema-section"))]
pub use r#actor::*;
#[cfg(any(feature = "actors-property-schema", feature = "general-schema-section"))]
mod r#actors;
#[cfg(any(feature = "actors-property-schema", feature = "general-schema-section"))]
pub use r#actors::*;
#[cfg(any(feature = "add-on-property-schema", feature = "general-schema-section"))]
mod r#add_on;
#[cfg(any(feature = "add-on-property-schema", feature = "general-schema-section"))]
pub use r#add_on::*;
#[cfg(any(
    feature = "additional-name-property-schema",
    feature = "general-schema-section"
))]
mod r#additional_name;
#[cfg(any(
    feature = "additional-name-property-schema",
    feature = "general-schema-section"
))]
pub use r#additional_name::*;
#[cfg(any(
    feature = "additional-number-of-guests-property-schema",
    feature = "general-schema-section"
))]
mod r#additional_number_of_guests;
#[cfg(any(
    feature = "additional-number-of-guests-property-schema",
    feature = "general-schema-section"
))]
pub use r#additional_number_of_guests::*;
#[cfg(any(
    feature = "additional-property-property-schema",
    feature = "general-schema-section"
))]
mod r#additional_property;
#[cfg(any(
    feature = "additional-property-property-schema",
    feature = "general-schema-section"
))]
pub use r#additional_property::*;
#[cfg(any(
    feature = "additional-type-property-schema",
    feature = "general-schema-section"
))]
mod r#additional_type;
#[cfg(any(
    feature = "additional-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#additional_type::*;
#[cfg(any(
    feature = "additional-variable-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#additional_variable;
#[cfg(any(
    feature = "additional-variable-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#additional_variable::*;
#[cfg(any(
    feature = "address-property-schema",
    feature = "general-schema-section"
))]
mod r#address;
#[cfg(any(
    feature = "address-property-schema",
    feature = "general-schema-section"
))]
pub use r#address::*;
#[cfg(any(
    feature = "address-country-property-schema",
    feature = "general-schema-section"
))]
mod r#address_country;
#[cfg(any(
    feature = "address-country-property-schema",
    feature = "general-schema-section"
))]
pub use r#address_country::*;
#[cfg(any(
    feature = "address-locality-property-schema",
    feature = "general-schema-section"
))]
mod r#address_locality;
#[cfg(any(
    feature = "address-locality-property-schema",
    feature = "general-schema-section"
))]
pub use r#address_locality::*;
#[cfg(any(
    feature = "address-region-property-schema",
    feature = "general-schema-section"
))]
mod r#address_region;
#[cfg(any(
    feature = "address-region-property-schema",
    feature = "general-schema-section"
))]
pub use r#address_region::*;
#[cfg(any(
    feature = "administration-route-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#administration_route;
#[cfg(any(
    feature = "administration-route-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#administration_route::*;
#[cfg(any(
    feature = "advance-booking-requirement-property-schema",
    feature = "general-schema-section"
))]
mod r#advance_booking_requirement;
#[cfg(any(
    feature = "advance-booking-requirement-property-schema",
    feature = "general-schema-section"
))]
pub use r#advance_booking_requirement::*;
#[cfg(any(
    feature = "adverse-outcome-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#adverse_outcome;
#[cfg(any(
    feature = "adverse-outcome-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#adverse_outcome::*;
#[cfg(any(
    feature = "affected-by-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#affected_by;
#[cfg(any(
    feature = "affected-by-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#affected_by::*;
#[cfg(any(
    feature = "affiliation-property-schema",
    feature = "general-schema-section"
))]
mod r#affiliation;
#[cfg(any(
    feature = "affiliation-property-schema",
    feature = "general-schema-section"
))]
pub use r#affiliation::*;
#[cfg(any(
    feature = "after-media-property-schema",
    feature = "general-schema-section"
))]
mod r#after_media;
#[cfg(any(
    feature = "after-media-property-schema",
    feature = "general-schema-section"
))]
pub use r#after_media::*;
#[cfg(any(feature = "agent-property-schema", feature = "general-schema-section"))]
mod r#agent;
#[cfg(any(feature = "agent-property-schema", feature = "general-schema-section"))]
pub use r#agent::*;
#[cfg(any(
    feature = "aggregate-rating-property-schema",
    feature = "general-schema-section"
))]
mod r#aggregate_rating;
#[cfg(any(
    feature = "aggregate-rating-property-schema",
    feature = "general-schema-section"
))]
pub use r#aggregate_rating::*;
#[cfg(any(
    feature = "aircraft-property-schema",
    feature = "general-schema-section"
))]
mod r#aircraft;
#[cfg(any(
    feature = "aircraft-property-schema",
    feature = "general-schema-section"
))]
pub use r#aircraft::*;
#[cfg(any(feature = "album-property-schema", feature = "general-schema-section"))]
mod r#album;
#[cfg(any(feature = "album-property-schema", feature = "general-schema-section"))]
pub use r#album::*;
#[cfg(any(
    feature = "album-production-type-property-schema",
    feature = "general-schema-section"
))]
mod r#album_production_type;
#[cfg(any(
    feature = "album-production-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#album_production_type::*;
#[cfg(any(
    feature = "album-release-property-schema",
    feature = "general-schema-section"
))]
mod r#album_release;
#[cfg(any(
    feature = "album-release-property-schema",
    feature = "general-schema-section"
))]
pub use r#album_release::*;
#[cfg(any(
    feature = "album-release-type-property-schema",
    feature = "general-schema-section"
))]
mod r#album_release_type;
#[cfg(any(
    feature = "album-release-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#album_release_type::*;
#[cfg(any(feature = "albums-property-schema", feature = "general-schema-section"))]
mod r#albums;
#[cfg(any(feature = "albums-property-schema", feature = "general-schema-section"))]
pub use r#albums::*;
#[cfg(any(
    feature = "alcohol-warning-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#alcohol_warning;
#[cfg(any(
    feature = "alcohol-warning-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#alcohol_warning::*;
#[cfg(any(
    feature = "algorithm-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#algorithm;
#[cfg(any(
    feature = "algorithm-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#algorithm::*;
#[cfg(any(
    feature = "alignment-type-property-schema",
    feature = "general-schema-section"
))]
mod r#alignment_type;
#[cfg(any(
    feature = "alignment-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#alignment_type::*;
#[cfg(any(
    feature = "alternate-name-property-schema",
    feature = "general-schema-section"
))]
mod r#alternate_name;
#[cfg(any(
    feature = "alternate-name-property-schema",
    feature = "general-schema-section"
))]
pub use r#alternate_name::*;
#[cfg(any(
    feature = "alternative-headline-property-schema",
    feature = "general-schema-section"
))]
mod r#alternative_headline;
#[cfg(any(
    feature = "alternative-headline-property-schema",
    feature = "general-schema-section"
))]
pub use r#alternative_headline::*;
#[cfg(any(
    feature = "alternative-of-property-schema",
    feature = "pending-schema-section"
))]
mod r#alternative_of;
#[cfg(any(
    feature = "alternative-of-property-schema",
    feature = "pending-schema-section"
))]
pub use r#alternative_of::*;
#[cfg(any(feature = "alumni-property-schema", feature = "general-schema-section"))]
mod r#alumni;
#[cfg(any(feature = "alumni-property-schema", feature = "general-schema-section"))]
pub use r#alumni::*;
#[cfg(any(
    feature = "alumni-of-property-schema",
    feature = "general-schema-section"
))]
mod r#alumni_of;
#[cfg(any(
    feature = "alumni-of-property-schema",
    feature = "general-schema-section"
))]
pub use r#alumni_of::*;
#[cfg(any(
    feature = "amenity-feature-property-schema",
    feature = "general-schema-section"
))]
mod r#amenity_feature;
#[cfg(any(
    feature = "amenity-feature-property-schema",
    feature = "general-schema-section"
))]
pub use r#amenity_feature::*;
#[cfg(any(feature = "amount-property-schema", feature = "general-schema-section"))]
mod r#amount;
#[cfg(any(feature = "amount-property-schema", feature = "general-schema-section"))]
pub use r#amount::*;
#[cfg(any(
    feature = "amount-of-this-good-property-schema",
    feature = "general-schema-section"
))]
mod r#amount_of_this_good;
#[cfg(any(
    feature = "amount-of-this-good-property-schema",
    feature = "general-schema-section"
))]
pub use r#amount_of_this_good::*;
#[cfg(any(
    feature = "announcement-location-property-schema",
    feature = "pending-schema-section"
))]
mod r#announcement_location;
#[cfg(any(
    feature = "announcement-location-property-schema",
    feature = "pending-schema-section"
))]
pub use r#announcement_location::*;
#[cfg(any(
    feature = "annual-percentage-rate-property-schema",
    feature = "general-schema-section"
))]
mod r#annual_percentage_rate;
#[cfg(any(
    feature = "annual-percentage-rate-property-schema",
    feature = "general-schema-section"
))]
pub use r#annual_percentage_rate::*;
#[cfg(any(
    feature = "answer-count-property-schema",
    feature = "general-schema-section"
))]
mod r#answer_count;
#[cfg(any(
    feature = "answer-count-property-schema",
    feature = "general-schema-section"
))]
pub use r#answer_count::*;
#[cfg(any(
    feature = "answer-explanation-property-schema",
    feature = "pending-schema-section"
))]
mod r#answer_explanation;
#[cfg(any(
    feature = "answer-explanation-property-schema",
    feature = "pending-schema-section"
))]
pub use r#answer_explanation::*;
#[cfg(any(
    feature = "antagonist-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#antagonist;
#[cfg(any(
    feature = "antagonist-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#antagonist::*;
#[cfg(any(
    feature = "appearance-property-schema",
    feature = "pending-schema-section"
))]
mod r#appearance;
#[cfg(any(
    feature = "appearance-property-schema",
    feature = "pending-schema-section"
))]
pub use r#appearance::*;
#[cfg(any(
    feature = "applicable-country-property-schema",
    feature = "pending-schema-section"
))]
mod r#applicable_country;
#[cfg(any(
    feature = "applicable-country-property-schema",
    feature = "pending-schema-section"
))]
pub use r#applicable_country::*;
#[cfg(any(
    feature = "applicable-location-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#applicable_location;
#[cfg(any(
    feature = "applicable-location-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#applicable_location::*;
#[cfg(any(
    feature = "applicant-location-requirements-property-schema",
    feature = "pending-schema-section"
))]
mod r#applicant_location_requirements;
#[cfg(any(
    feature = "applicant-location-requirements-property-schema",
    feature = "pending-schema-section"
))]
pub use r#applicant_location_requirements::*;
#[cfg(any(
    feature = "application-property-schema",
    feature = "general-schema-section"
))]
mod r#application;
#[cfg(any(
    feature = "application-property-schema",
    feature = "general-schema-section"
))]
pub use r#application::*;
#[cfg(any(
    feature = "application-category-property-schema",
    feature = "general-schema-section"
))]
mod r#application_category;
#[cfg(any(
    feature = "application-category-property-schema",
    feature = "general-schema-section"
))]
pub use r#application_category::*;
#[cfg(any(
    feature = "application-contact-property-schema",
    feature = "pending-schema-section"
))]
mod r#application_contact;
#[cfg(any(
    feature = "application-contact-property-schema",
    feature = "pending-schema-section"
))]
pub use r#application_contact::*;
#[cfg(any(
    feature = "application-deadline-property-schema",
    feature = "pending-schema-section"
))]
mod r#application_deadline;
#[cfg(any(
    feature = "application-deadline-property-schema",
    feature = "pending-schema-section"
))]
pub use r#application_deadline::*;
#[cfg(any(
    feature = "application-start-date-property-schema",
    feature = "pending-schema-section"
))]
mod r#application_start_date;
#[cfg(any(
    feature = "application-start-date-property-schema",
    feature = "pending-schema-section"
))]
pub use r#application_start_date::*;
#[cfg(any(
    feature = "application-sub-category-property-schema",
    feature = "general-schema-section"
))]
mod r#application_sub_category;
#[cfg(any(
    feature = "application-sub-category-property-schema",
    feature = "general-schema-section"
))]
pub use r#application_sub_category::*;
#[cfg(any(
    feature = "application-suite-property-schema",
    feature = "general-schema-section"
))]
mod r#application_suite;
#[cfg(any(
    feature = "application-suite-property-schema",
    feature = "general-schema-section"
))]
pub use r#application_suite::*;
#[cfg(any(
    feature = "applies-to-delivery-method-property-schema",
    feature = "general-schema-section"
))]
mod r#applies_to_delivery_method;
#[cfg(any(
    feature = "applies-to-delivery-method-property-schema",
    feature = "general-schema-section"
))]
pub use r#applies_to_delivery_method::*;
#[cfg(any(
    feature = "applies-to-payment-method-property-schema",
    feature = "general-schema-section"
))]
mod r#applies_to_payment_method;
#[cfg(any(
    feature = "applies-to-payment-method-property-schema",
    feature = "general-schema-section"
))]
pub use r#applies_to_payment_method::*;
#[cfg(any(
    feature = "archive-held-property-schema",
    feature = "pending-schema-section"
))]
mod r#archive_held;
#[cfg(any(
    feature = "archive-held-property-schema",
    feature = "pending-schema-section"
))]
pub use r#archive_held::*;
#[cfg(any(
    feature = "archived-at-property-schema",
    feature = "pending-schema-section"
))]
mod r#archived_at;
#[cfg(any(
    feature = "archived-at-property-schema",
    feature = "pending-schema-section"
))]
pub use r#archived_at::*;
#[cfg(any(feature = "area-property-schema", feature = "general-schema-section"))]
mod r#area;
#[cfg(any(feature = "area-property-schema", feature = "general-schema-section"))]
pub use r#area::*;
#[cfg(any(
    feature = "area-served-property-schema",
    feature = "general-schema-section"
))]
mod r#area_served;
#[cfg(any(
    feature = "area-served-property-schema",
    feature = "general-schema-section"
))]
pub use r#area_served::*;
#[cfg(any(
    feature = "arrival-airport-property-schema",
    feature = "general-schema-section"
))]
mod r#arrival_airport;
#[cfg(any(
    feature = "arrival-airport-property-schema",
    feature = "general-schema-section"
))]
pub use r#arrival_airport::*;
#[cfg(any(
    feature = "arrival-boat-terminal-property-schema",
    feature = "pending-schema-section"
))]
mod r#arrival_boat_terminal;
#[cfg(any(
    feature = "arrival-boat-terminal-property-schema",
    feature = "pending-schema-section"
))]
pub use r#arrival_boat_terminal::*;
#[cfg(any(
    feature = "arrival-bus-stop-property-schema",
    feature = "general-schema-section"
))]
mod r#arrival_bus_stop;
#[cfg(any(
    feature = "arrival-bus-stop-property-schema",
    feature = "general-schema-section"
))]
pub use r#arrival_bus_stop::*;
#[cfg(any(
    feature = "arrival-gate-property-schema",
    feature = "general-schema-section"
))]
mod r#arrival_gate;
#[cfg(any(
    feature = "arrival-gate-property-schema",
    feature = "general-schema-section"
))]
pub use r#arrival_gate::*;
#[cfg(any(
    feature = "arrival-platform-property-schema",
    feature = "general-schema-section"
))]
mod r#arrival_platform;
#[cfg(any(
    feature = "arrival-platform-property-schema",
    feature = "general-schema-section"
))]
pub use r#arrival_platform::*;
#[cfg(any(
    feature = "arrival-station-property-schema",
    feature = "general-schema-section"
))]
mod r#arrival_station;
#[cfg(any(
    feature = "arrival-station-property-schema",
    feature = "general-schema-section"
))]
pub use r#arrival_station::*;
#[cfg(any(
    feature = "arrival-terminal-property-schema",
    feature = "general-schema-section"
))]
mod r#arrival_terminal;
#[cfg(any(
    feature = "arrival-terminal-property-schema",
    feature = "general-schema-section"
))]
pub use r#arrival_terminal::*;
#[cfg(any(
    feature = "arrival-time-property-schema",
    feature = "general-schema-section"
))]
mod r#arrival_time;
#[cfg(any(
    feature = "arrival-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#arrival_time::*;
#[cfg(any(
    feature = "art-edition-property-schema",
    feature = "general-schema-section"
))]
mod r#art_edition;
#[cfg(any(
    feature = "art-edition-property-schema",
    feature = "general-schema-section"
))]
pub use r#art_edition::*;
#[cfg(any(
    feature = "art-medium-property-schema",
    feature = "general-schema-section"
))]
mod r#art_medium;
#[cfg(any(
    feature = "art-medium-property-schema",
    feature = "general-schema-section"
))]
pub use r#art_medium::*;
#[cfg(any(
    feature = "arterial-branch-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#arterial_branch;
#[cfg(any(
    feature = "arterial-branch-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#arterial_branch::*;
#[cfg(any(
    feature = "artform-property-schema",
    feature = "general-schema-section"
))]
mod r#artform;
#[cfg(any(
    feature = "artform-property-schema",
    feature = "general-schema-section"
))]
pub use r#artform::*;
#[cfg(any(
    feature = "article-body-property-schema",
    feature = "general-schema-section"
))]
mod r#article_body;
#[cfg(any(
    feature = "article-body-property-schema",
    feature = "general-schema-section"
))]
pub use r#article_body::*;
#[cfg(any(
    feature = "article-section-property-schema",
    feature = "general-schema-section"
))]
mod r#article_section;
#[cfg(any(
    feature = "article-section-property-schema",
    feature = "general-schema-section"
))]
pub use r#article_section::*;
#[cfg(any(feature = "artist-property-schema", feature = "bib-schema-section"))]
mod r#artist;
#[cfg(any(feature = "artist-property-schema", feature = "bib-schema-section"))]
pub use r#artist::*;
#[cfg(any(
    feature = "artwork-surface-property-schema",
    feature = "general-schema-section"
))]
mod r#artwork_surface;
#[cfg(any(
    feature = "artwork-surface-property-schema",
    feature = "general-schema-section"
))]
pub use r#artwork_surface::*;
#[cfg(any(feature = "asin-property-schema", feature = "pending-schema-section"))]
mod r#asin;
#[cfg(any(feature = "asin-property-schema", feature = "pending-schema-section"))]
pub use r#asin::*;
#[cfg(any(
    feature = "aspect-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#aspect;
#[cfg(any(
    feature = "aspect-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#aspect::*;
#[cfg(any(
    feature = "assembly-property-schema",
    feature = "general-schema-section"
))]
mod r#assembly;
#[cfg(any(
    feature = "assembly-property-schema",
    feature = "general-schema-section"
))]
pub use r#assembly::*;
#[cfg(any(
    feature = "assembly-version-property-schema",
    feature = "general-schema-section"
))]
mod r#assembly_version;
#[cfg(any(
    feature = "assembly-version-property-schema",
    feature = "general-schema-section"
))]
pub use r#assembly_version::*;
#[cfg(any(
    feature = "assesses-property-schema",
    feature = "pending-schema-section"
))]
mod r#assesses;
#[cfg(any(
    feature = "assesses-property-schema",
    feature = "pending-schema-section"
))]
pub use r#assesses::*;
#[cfg(any(
    feature = "associated-anatomy-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#associated_anatomy;
#[cfg(any(
    feature = "associated-anatomy-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#associated_anatomy::*;
#[cfg(any(
    feature = "associated-article-property-schema",
    feature = "general-schema-section"
))]
mod r#associated_article;
#[cfg(any(
    feature = "associated-article-property-schema",
    feature = "general-schema-section"
))]
pub use r#associated_article::*;
#[cfg(any(
    feature = "associated-claim-review-property-schema",
    feature = "pending-schema-section"
))]
mod r#associated_claim_review;
#[cfg(any(
    feature = "associated-claim-review-property-schema",
    feature = "pending-schema-section"
))]
pub use r#associated_claim_review::*;
#[cfg(any(
    feature = "associated-disease-property-schema",
    feature = "pending-schema-section"
))]
mod r#associated_disease;
#[cfg(any(
    feature = "associated-disease-property-schema",
    feature = "pending-schema-section"
))]
pub use r#associated_disease::*;
#[cfg(any(
    feature = "associated-media-property-schema",
    feature = "general-schema-section"
))]
mod r#associated_media;
#[cfg(any(
    feature = "associated-media-property-schema",
    feature = "general-schema-section"
))]
pub use r#associated_media::*;
#[cfg(any(
    feature = "associated-media-review-property-schema",
    feature = "pending-schema-section"
))]
mod r#associated_media_review;
#[cfg(any(
    feature = "associated-media-review-property-schema",
    feature = "pending-schema-section"
))]
pub use r#associated_media_review::*;
#[cfg(any(
    feature = "associated-pathophysiology-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#associated_pathophysiology;
#[cfg(any(
    feature = "associated-pathophysiology-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#associated_pathophysiology::*;
#[cfg(any(
    feature = "associated-review-property-schema",
    feature = "pending-schema-section"
))]
mod r#associated_review;
#[cfg(any(
    feature = "associated-review-property-schema",
    feature = "pending-schema-section"
))]
pub use r#associated_review::*;
#[cfg(any(
    feature = "athlete-property-schema",
    feature = "general-schema-section"
))]
mod r#athlete;
#[cfg(any(
    feature = "athlete-property-schema",
    feature = "general-schema-section"
))]
pub use r#athlete::*;
#[cfg(any(
    feature = "attendee-property-schema",
    feature = "general-schema-section"
))]
mod r#attendee;
#[cfg(any(
    feature = "attendee-property-schema",
    feature = "general-schema-section"
))]
pub use r#attendee::*;
#[cfg(any(
    feature = "attendees-property-schema",
    feature = "general-schema-section"
))]
mod r#attendees;
#[cfg(any(
    feature = "attendees-property-schema",
    feature = "general-schema-section"
))]
pub use r#attendees::*;
#[cfg(any(
    feature = "audience-property-schema",
    feature = "general-schema-section"
))]
mod r#audience;
#[cfg(any(
    feature = "audience-property-schema",
    feature = "general-schema-section"
))]
pub use r#audience::*;
#[cfg(any(
    feature = "audience-type-property-schema",
    feature = "general-schema-section"
))]
mod r#audience_type;
#[cfg(any(
    feature = "audience-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#audience_type::*;
#[cfg(any(feature = "audio-property-schema", feature = "general-schema-section"))]
mod r#audio;
#[cfg(any(feature = "audio-property-schema", feature = "general-schema-section"))]
pub use r#audio::*;
#[cfg(any(
    feature = "authenticator-property-schema",
    feature = "general-schema-section"
))]
mod r#authenticator;
#[cfg(any(
    feature = "authenticator-property-schema",
    feature = "general-schema-section"
))]
pub use r#authenticator::*;
#[cfg(any(feature = "author-property-schema", feature = "general-schema-section"))]
mod r#author;
#[cfg(any(feature = "author-property-schema", feature = "general-schema-section"))]
pub use r#author::*;
#[cfg(any(
    feature = "availability-property-schema",
    feature = "general-schema-section"
))]
mod r#availability;
#[cfg(any(
    feature = "availability-property-schema",
    feature = "general-schema-section"
))]
pub use r#availability::*;
#[cfg(any(
    feature = "availability-ends-property-schema",
    feature = "general-schema-section"
))]
mod r#availability_ends;
#[cfg(any(
    feature = "availability-ends-property-schema",
    feature = "general-schema-section"
))]
pub use r#availability_ends::*;
#[cfg(any(
    feature = "availability-starts-property-schema",
    feature = "general-schema-section"
))]
mod r#availability_starts;
#[cfg(any(
    feature = "availability-starts-property-schema",
    feature = "general-schema-section"
))]
pub use r#availability_starts::*;
#[cfg(any(
    feature = "available-at-or-from-property-schema",
    feature = "general-schema-section"
))]
mod r#available_at_or_from;
#[cfg(any(
    feature = "available-at-or-from-property-schema",
    feature = "general-schema-section"
))]
pub use r#available_at_or_from::*;
#[cfg(any(
    feature = "available-channel-property-schema",
    feature = "general-schema-section"
))]
mod r#available_channel;
#[cfg(any(
    feature = "available-channel-property-schema",
    feature = "general-schema-section"
))]
pub use r#available_channel::*;
#[cfg(any(
    feature = "available-delivery-method-property-schema",
    feature = "general-schema-section"
))]
mod r#available_delivery_method;
#[cfg(any(
    feature = "available-delivery-method-property-schema",
    feature = "general-schema-section"
))]
pub use r#available_delivery_method::*;
#[cfg(any(
    feature = "available-from-property-schema",
    feature = "general-schema-section"
))]
mod r#available_from;
#[cfg(any(
    feature = "available-from-property-schema",
    feature = "general-schema-section"
))]
pub use r#available_from::*;
#[cfg(any(
    feature = "available-in-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#available_in;
#[cfg(any(
    feature = "available-in-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#available_in::*;
#[cfg(any(
    feature = "available-language-property-schema",
    feature = "general-schema-section"
))]
mod r#available_language;
#[cfg(any(
    feature = "available-language-property-schema",
    feature = "general-schema-section"
))]
pub use r#available_language::*;
#[cfg(any(
    feature = "available-on-device-property-schema",
    feature = "general-schema-section"
))]
mod r#available_on_device;
#[cfg(any(
    feature = "available-on-device-property-schema",
    feature = "general-schema-section"
))]
pub use r#available_on_device::*;
#[cfg(any(
    feature = "available-service-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#available_service;
#[cfg(any(
    feature = "available-service-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#available_service::*;
#[cfg(any(
    feature = "available-strength-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#available_strength;
#[cfg(any(
    feature = "available-strength-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#available_strength::*;
#[cfg(any(
    feature = "available-test-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#available_test;
#[cfg(any(
    feature = "available-test-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#available_test::*;
#[cfg(any(
    feature = "available-through-property-schema",
    feature = "general-schema-section"
))]
mod r#available_through;
#[cfg(any(
    feature = "available-through-property-schema",
    feature = "general-schema-section"
))]
pub use r#available_through::*;
#[cfg(any(feature = "award-property-schema", feature = "general-schema-section"))]
mod r#award;
#[cfg(any(feature = "award-property-schema", feature = "general-schema-section"))]
pub use r#award::*;
#[cfg(any(feature = "awards-property-schema", feature = "general-schema-section"))]
mod r#awards;
#[cfg(any(feature = "awards-property-schema", feature = "general-schema-section"))]
pub use r#awards::*;
#[cfg(any(
    feature = "away-team-property-schema",
    feature = "general-schema-section"
))]
mod r#away_team;
#[cfg(any(
    feature = "away-team-property-schema",
    feature = "general-schema-section"
))]
pub use r#away_team::*;
#[cfg(any(
    feature = "backstory-property-schema",
    feature = "pending-schema-section"
))]
mod r#backstory;
#[cfg(any(
    feature = "backstory-property-schema",
    feature = "pending-schema-section"
))]
pub use r#backstory::*;
#[cfg(any(
    feature = "bank-account-type-property-schema",
    feature = "pending-schema-section"
))]
mod r#bank_account_type;
#[cfg(any(
    feature = "bank-account-type-property-schema",
    feature = "pending-schema-section"
))]
pub use r#bank_account_type::*;
#[cfg(any(
    feature = "base-salary-property-schema",
    feature = "general-schema-section"
))]
mod r#base_salary;
#[cfg(any(
    feature = "base-salary-property-schema",
    feature = "general-schema-section"
))]
pub use r#base_salary::*;
#[cfg(any(
    feature = "bcc-recipient-property-schema",
    feature = "general-schema-section"
))]
mod r#bcc_recipient;
#[cfg(any(
    feature = "bcc-recipient-property-schema",
    feature = "general-schema-section"
))]
pub use r#bcc_recipient::*;
#[cfg(any(feature = "bed-property-schema", feature = "general-schema-section"))]
mod r#bed;
#[cfg(any(feature = "bed-property-schema", feature = "general-schema-section"))]
pub use r#bed::*;
#[cfg(any(
    feature = "before-media-property-schema",
    feature = "general-schema-section"
))]
mod r#before_media;
#[cfg(any(
    feature = "before-media-property-schema",
    feature = "general-schema-section"
))]
pub use r#before_media::*;
#[cfg(any(
    feature = "beneficiary-bank-property-schema",
    feature = "pending-schema-section"
))]
mod r#beneficiary_bank;
#[cfg(any(
    feature = "beneficiary-bank-property-schema",
    feature = "pending-schema-section"
))]
pub use r#beneficiary_bank::*;
#[cfg(any(
    feature = "benefits-property-schema",
    feature = "general-schema-section"
))]
mod r#benefits;
#[cfg(any(
    feature = "benefits-property-schema",
    feature = "general-schema-section"
))]
pub use r#benefits::*;
#[cfg(any(
    feature = "benefits-summary-url-property-schema",
    feature = "pending-schema-section"
))]
mod r#benefits_summary_url;
#[cfg(any(
    feature = "benefits-summary-url-property-schema",
    feature = "pending-schema-section"
))]
pub use r#benefits_summary_url::*;
#[cfg(any(
    feature = "best-rating-property-schema",
    feature = "general-schema-section"
))]
mod r#best_rating;
#[cfg(any(
    feature = "best-rating-property-schema",
    feature = "general-schema-section"
))]
pub use r#best_rating::*;
#[cfg(any(
    feature = "billing-address-property-schema",
    feature = "general-schema-section"
))]
mod r#billing_address;
#[cfg(any(
    feature = "billing-address-property-schema",
    feature = "general-schema-section"
))]
pub use r#billing_address::*;
#[cfg(any(
    feature = "billing-duration-property-schema",
    feature = "pending-schema-section"
))]
mod r#billing_duration;
#[cfg(any(
    feature = "billing-duration-property-schema",
    feature = "pending-schema-section"
))]
pub use r#billing_duration::*;
#[cfg(any(
    feature = "billing-increment-property-schema",
    feature = "general-schema-section"
))]
mod r#billing_increment;
#[cfg(any(
    feature = "billing-increment-property-schema",
    feature = "general-schema-section"
))]
pub use r#billing_increment::*;
#[cfg(any(
    feature = "billing-period-property-schema",
    feature = "general-schema-section"
))]
mod r#billing_period;
#[cfg(any(
    feature = "billing-period-property-schema",
    feature = "general-schema-section"
))]
pub use r#billing_period::*;
#[cfg(any(
    feature = "billing-start-property-schema",
    feature = "pending-schema-section"
))]
mod r#billing_start;
#[cfg(any(
    feature = "billing-start-property-schema",
    feature = "pending-schema-section"
))]
pub use r#billing_start::*;
#[cfg(any(
    feature = "bio-chem-interaction-property-schema",
    feature = "pending-schema-section"
))]
mod r#bio_chem_interaction;
#[cfg(any(
    feature = "bio-chem-interaction-property-schema",
    feature = "pending-schema-section"
))]
pub use r#bio_chem_interaction::*;
#[cfg(any(
    feature = "bio-chem-similarity-property-schema",
    feature = "pending-schema-section"
))]
mod r#bio_chem_similarity;
#[cfg(any(
    feature = "bio-chem-similarity-property-schema",
    feature = "pending-schema-section"
))]
pub use r#bio_chem_similarity::*;
#[cfg(any(
    feature = "biological-role-property-schema",
    feature = "pending-schema-section"
))]
mod r#biological_role;
#[cfg(any(
    feature = "biological-role-property-schema",
    feature = "pending-schema-section"
))]
pub use r#biological_role::*;
#[cfg(any(
    feature = "biomechnical-class-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#biomechnical_class;
#[cfg(any(
    feature = "biomechnical-class-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#biomechnical_class::*;
#[cfg(any(
    feature = "birth-date-property-schema",
    feature = "general-schema-section"
))]
mod r#birth_date;
#[cfg(any(
    feature = "birth-date-property-schema",
    feature = "general-schema-section"
))]
pub use r#birth_date::*;
#[cfg(any(
    feature = "birth-place-property-schema",
    feature = "general-schema-section"
))]
mod r#birth_place;
#[cfg(any(
    feature = "birth-place-property-schema",
    feature = "general-schema-section"
))]
pub use r#birth_place::*;
#[cfg(any(
    feature = "bitrate-property-schema",
    feature = "general-schema-section"
))]
mod r#bitrate;
#[cfg(any(
    feature = "bitrate-property-schema",
    feature = "general-schema-section"
))]
pub use r#bitrate::*;
#[cfg(any(
    feature = "blog-post-property-schema",
    feature = "general-schema-section"
))]
mod r#blog_post;
#[cfg(any(
    feature = "blog-post-property-schema",
    feature = "general-schema-section"
))]
pub use r#blog_post::*;
#[cfg(any(
    feature = "blog-posts-property-schema",
    feature = "general-schema-section"
))]
mod r#blog_posts;
#[cfg(any(
    feature = "blog-posts-property-schema",
    feature = "general-schema-section"
))]
pub use r#blog_posts::*;
#[cfg(any(
    feature = "blood-supply-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#blood_supply;
#[cfg(any(
    feature = "blood-supply-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#blood_supply::*;
#[cfg(any(
    feature = "boarding-group-property-schema",
    feature = "general-schema-section"
))]
mod r#boarding_group;
#[cfg(any(
    feature = "boarding-group-property-schema",
    feature = "general-schema-section"
))]
pub use r#boarding_group::*;
#[cfg(any(
    feature = "boarding-policy-property-schema",
    feature = "general-schema-section"
))]
mod r#boarding_policy;
#[cfg(any(
    feature = "boarding-policy-property-schema",
    feature = "general-schema-section"
))]
pub use r#boarding_policy::*;
#[cfg(any(
    feature = "body-location-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#body_location;
#[cfg(any(
    feature = "body-location-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#body_location::*;
#[cfg(any(feature = "body-type-property-schema", feature = "auto-schema-section"))]
mod r#body_type;
#[cfg(any(feature = "body-type-property-schema", feature = "auto-schema-section"))]
pub use r#body_type::*;
#[cfg(any(
    feature = "book-edition-property-schema",
    feature = "general-schema-section"
))]
mod r#book_edition;
#[cfg(any(
    feature = "book-edition-property-schema",
    feature = "general-schema-section"
))]
pub use r#book_edition::*;
#[cfg(any(
    feature = "book-format-property-schema",
    feature = "general-schema-section"
))]
mod r#book_format;
#[cfg(any(
    feature = "book-format-property-schema",
    feature = "general-schema-section"
))]
pub use r#book_format::*;
#[cfg(any(
    feature = "booking-agent-property-schema",
    feature = "general-schema-section"
))]
mod r#booking_agent;
#[cfg(any(
    feature = "booking-agent-property-schema",
    feature = "general-schema-section"
))]
pub use r#booking_agent::*;
#[cfg(any(
    feature = "booking-time-property-schema",
    feature = "general-schema-section"
))]
mod r#booking_time;
#[cfg(any(
    feature = "booking-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#booking_time::*;
#[cfg(any(
    feature = "borrower-property-schema",
    feature = "general-schema-section"
))]
mod r#borrower;
#[cfg(any(
    feature = "borrower-property-schema",
    feature = "general-schema-section"
))]
pub use r#borrower::*;
#[cfg(any(feature = "box-property-schema", feature = "general-schema-section"))]
mod r#box;
#[cfg(any(feature = "box-property-schema", feature = "general-schema-section"))]
pub use r#box::*;
#[cfg(any(
    feature = "branch-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#branch;
#[cfg(any(
    feature = "branch-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#branch::*;
#[cfg(any(
    feature = "branch-code-property-schema",
    feature = "general-schema-section"
))]
mod r#branch_code;
#[cfg(any(
    feature = "branch-code-property-schema",
    feature = "general-schema-section"
))]
pub use r#branch_code::*;
#[cfg(any(
    feature = "branch-of-property-schema",
    feature = "general-schema-section"
))]
mod r#branch_of;
#[cfg(any(
    feature = "branch-of-property-schema",
    feature = "general-schema-section"
))]
pub use r#branch_of::*;
#[cfg(any(feature = "brand-property-schema", feature = "general-schema-section"))]
mod r#brand;
#[cfg(any(feature = "brand-property-schema", feature = "general-schema-section"))]
pub use r#brand::*;
#[cfg(any(
    feature = "breadcrumb-property-schema",
    feature = "general-schema-section"
))]
mod r#breadcrumb;
#[cfg(any(
    feature = "breadcrumb-property-schema",
    feature = "general-schema-section"
))]
pub use r#breadcrumb::*;
#[cfg(any(
    feature = "breastfeeding-warning-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#breastfeeding_warning;
#[cfg(any(
    feature = "breastfeeding-warning-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#breastfeeding_warning::*;
#[cfg(any(
    feature = "broadcast-affiliate-of-property-schema",
    feature = "general-schema-section"
))]
mod r#broadcast_affiliate_of;
#[cfg(any(
    feature = "broadcast-affiliate-of-property-schema",
    feature = "general-schema-section"
))]
pub use r#broadcast_affiliate_of::*;
#[cfg(any(
    feature = "broadcast-channel-id-property-schema",
    feature = "general-schema-section"
))]
mod r#broadcast_channel_id;
#[cfg(any(
    feature = "broadcast-channel-id-property-schema",
    feature = "general-schema-section"
))]
pub use r#broadcast_channel_id::*;
#[cfg(any(
    feature = "broadcast-display-name-property-schema",
    feature = "general-schema-section"
))]
mod r#broadcast_display_name;
#[cfg(any(
    feature = "broadcast-display-name-property-schema",
    feature = "general-schema-section"
))]
pub use r#broadcast_display_name::*;
#[cfg(any(
    feature = "broadcast-frequency-property-schema",
    feature = "general-schema-section"
))]
mod r#broadcast_frequency;
#[cfg(any(
    feature = "broadcast-frequency-property-schema",
    feature = "general-schema-section"
))]
pub use r#broadcast_frequency::*;
#[cfg(any(
    feature = "broadcast-frequency-value-property-schema",
    feature = "general-schema-section"
))]
mod r#broadcast_frequency_value;
#[cfg(any(
    feature = "broadcast-frequency-value-property-schema",
    feature = "general-schema-section"
))]
pub use r#broadcast_frequency_value::*;
#[cfg(any(
    feature = "broadcast-of-event-property-schema",
    feature = "general-schema-section"
))]
mod r#broadcast_of_event;
#[cfg(any(
    feature = "broadcast-of-event-property-schema",
    feature = "general-schema-section"
))]
pub use r#broadcast_of_event::*;
#[cfg(any(
    feature = "broadcast-service-tier-property-schema",
    feature = "general-schema-section"
))]
mod r#broadcast_service_tier;
#[cfg(any(
    feature = "broadcast-service-tier-property-schema",
    feature = "general-schema-section"
))]
pub use r#broadcast_service_tier::*;
#[cfg(any(
    feature = "broadcast-signal-modulation-property-schema",
    feature = "pending-schema-section"
))]
mod r#broadcast_signal_modulation;
#[cfg(any(
    feature = "broadcast-signal-modulation-property-schema",
    feature = "pending-schema-section"
))]
pub use r#broadcast_signal_modulation::*;
#[cfg(any(
    feature = "broadcast-sub-channel-property-schema",
    feature = "pending-schema-section"
))]
mod r#broadcast_sub_channel;
#[cfg(any(
    feature = "broadcast-sub-channel-property-schema",
    feature = "pending-schema-section"
))]
pub use r#broadcast_sub_channel::*;
#[cfg(any(
    feature = "broadcast-timezone-property-schema",
    feature = "general-schema-section"
))]
mod r#broadcast_timezone;
#[cfg(any(
    feature = "broadcast-timezone-property-schema",
    feature = "general-schema-section"
))]
pub use r#broadcast_timezone::*;
#[cfg(any(
    feature = "broadcaster-property-schema",
    feature = "general-schema-section"
))]
mod r#broadcaster;
#[cfg(any(
    feature = "broadcaster-property-schema",
    feature = "general-schema-section"
))]
pub use r#broadcaster::*;
#[cfg(any(feature = "broker-property-schema", feature = "general-schema-section"))]
mod r#broker;
#[cfg(any(feature = "broker-property-schema", feature = "general-schema-section"))]
pub use r#broker::*;
#[cfg(any(
    feature = "browser-requirements-property-schema",
    feature = "general-schema-section"
))]
mod r#browser_requirements;
#[cfg(any(
    feature = "browser-requirements-property-schema",
    feature = "general-schema-section"
))]
pub use r#browser_requirements::*;
#[cfg(any(
    feature = "bus-name-property-schema",
    feature = "general-schema-section"
))]
mod r#bus_name;
#[cfg(any(
    feature = "bus-name-property-schema",
    feature = "general-schema-section"
))]
pub use r#bus_name::*;
#[cfg(any(
    feature = "bus-number-property-schema",
    feature = "general-schema-section"
))]
mod r#bus_number;
#[cfg(any(
    feature = "bus-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#bus_number::*;
#[cfg(any(
    feature = "business-days-property-schema",
    feature = "pending-schema-section"
))]
mod r#business_days;
#[cfg(any(
    feature = "business-days-property-schema",
    feature = "pending-schema-section"
))]
pub use r#business_days::*;
#[cfg(any(
    feature = "business-function-property-schema",
    feature = "general-schema-section"
))]
mod r#business_function;
#[cfg(any(
    feature = "business-function-property-schema",
    feature = "general-schema-section"
))]
pub use r#business_function::*;
#[cfg(any(feature = "buyer-property-schema", feature = "general-schema-section"))]
mod r#buyer;
#[cfg(any(feature = "buyer-property-schema", feature = "general-schema-section"))]
pub use r#buyer::*;
#[cfg(any(
    feature = "by-artist-property-schema",
    feature = "general-schema-section"
))]
mod r#by_artist;
#[cfg(any(
    feature = "by-artist-property-schema",
    feature = "general-schema-section"
))]
pub use r#by_artist::*;
#[cfg(any(feature = "by-day-property-schema", feature = "pending-schema-section"))]
mod r#by_day;
#[cfg(any(feature = "by-day-property-schema", feature = "pending-schema-section"))]
pub use r#by_day::*;
#[cfg(any(
    feature = "by-month-property-schema",
    feature = "pending-schema-section"
))]
mod r#by_month;
#[cfg(any(
    feature = "by-month-property-schema",
    feature = "pending-schema-section"
))]
pub use r#by_month::*;
#[cfg(any(
    feature = "by-month-day-property-schema",
    feature = "pending-schema-section"
))]
mod r#by_month_day;
#[cfg(any(
    feature = "by-month-day-property-schema",
    feature = "pending-schema-section"
))]
pub use r#by_month_day::*;
#[cfg(any(
    feature = "by-month-week-property-schema",
    feature = "pending-schema-section"
))]
mod r#by_month_week;
#[cfg(any(
    feature = "by-month-week-property-schema",
    feature = "pending-schema-section"
))]
pub use r#by_month_week::*;
#[cfg(any(
    feature = "call-sign-property-schema",
    feature = "pending-schema-section"
))]
mod r#call_sign;
#[cfg(any(
    feature = "call-sign-property-schema",
    feature = "pending-schema-section"
))]
pub use r#call_sign::*;
#[cfg(any(
    feature = "calories-property-schema",
    feature = "general-schema-section"
))]
mod r#calories;
#[cfg(any(
    feature = "calories-property-schema",
    feature = "general-schema-section"
))]
pub use r#calories::*;
#[cfg(any(
    feature = "candidate-property-schema",
    feature = "general-schema-section"
))]
mod r#candidate;
#[cfg(any(
    feature = "candidate-property-schema",
    feature = "general-schema-section"
))]
pub use r#candidate::*;
#[cfg(any(
    feature = "caption-property-schema",
    feature = "general-schema-section"
))]
mod r#caption;
#[cfg(any(
    feature = "caption-property-schema",
    feature = "general-schema-section"
))]
pub use r#caption::*;
#[cfg(any(
    feature = "carbohydrate-content-property-schema",
    feature = "general-schema-section"
))]
mod r#carbohydrate_content;
#[cfg(any(
    feature = "carbohydrate-content-property-schema",
    feature = "general-schema-section"
))]
pub use r#carbohydrate_content::*;
#[cfg(any(
    feature = "cargo-volume-property-schema",
    feature = "general-schema-section"
))]
mod r#cargo_volume;
#[cfg(any(
    feature = "cargo-volume-property-schema",
    feature = "general-schema-section"
))]
pub use r#cargo_volume::*;
#[cfg(any(
    feature = "carrier-property-schema",
    feature = "general-schema-section"
))]
mod r#carrier;
#[cfg(any(
    feature = "carrier-property-schema",
    feature = "general-schema-section"
))]
pub use r#carrier::*;
#[cfg(any(
    feature = "carrier-requirements-property-schema",
    feature = "general-schema-section"
))]
mod r#carrier_requirements;
#[cfg(any(
    feature = "carrier-requirements-property-schema",
    feature = "general-schema-section"
))]
pub use r#carrier_requirements::*;
#[cfg(any(
    feature = "cash-back-property-schema",
    feature = "pending-schema-section"
))]
mod r#cash_back;
#[cfg(any(
    feature = "cash-back-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cash_back::*;
#[cfg(any(
    feature = "catalog-property-schema",
    feature = "general-schema-section"
))]
mod r#catalog;
#[cfg(any(
    feature = "catalog-property-schema",
    feature = "general-schema-section"
))]
pub use r#catalog::*;
#[cfg(any(
    feature = "catalog-number-property-schema",
    feature = "general-schema-section"
))]
mod r#catalog_number;
#[cfg(any(
    feature = "catalog-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#catalog_number::*;
#[cfg(any(
    feature = "category-property-schema",
    feature = "general-schema-section"
))]
mod r#category;
#[cfg(any(
    feature = "category-property-schema",
    feature = "general-schema-section"
))]
pub use r#category::*;
#[cfg(any(
    feature = "cause-of-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#cause_of;
#[cfg(any(
    feature = "cause-of-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#cause_of::*;
#[cfg(any(
    feature = "cc-recipient-property-schema",
    feature = "general-schema-section"
))]
mod r#cc_recipient;
#[cfg(any(
    feature = "cc-recipient-property-schema",
    feature = "general-schema-section"
))]
pub use r#cc_recipient::*;
#[cfg(any(
    feature = "character-property-schema",
    feature = "general-schema-section"
))]
mod r#character;
#[cfg(any(
    feature = "character-property-schema",
    feature = "general-schema-section"
))]
pub use r#character::*;
#[cfg(any(
    feature = "character-attribute-property-schema",
    feature = "general-schema-section"
))]
mod r#character_attribute;
#[cfg(any(
    feature = "character-attribute-property-schema",
    feature = "general-schema-section"
))]
pub use r#character_attribute::*;
#[cfg(any(
    feature = "character-name-property-schema",
    feature = "general-schema-section"
))]
mod r#character_name;
#[cfg(any(
    feature = "character-name-property-schema",
    feature = "general-schema-section"
))]
pub use r#character_name::*;
#[cfg(any(
    feature = "cheat-code-property-schema",
    feature = "general-schema-section"
))]
mod r#cheat_code;
#[cfg(any(
    feature = "cheat-code-property-schema",
    feature = "general-schema-section"
))]
pub use r#cheat_code::*;
#[cfg(any(
    feature = "checkin-time-property-schema",
    feature = "general-schema-section"
))]
mod r#checkin_time;
#[cfg(any(
    feature = "checkin-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#checkin_time::*;
#[cfg(any(
    feature = "checkout-page-url-template-property-schema",
    feature = "pending-schema-section"
))]
mod r#checkout_page_url_template;
#[cfg(any(
    feature = "checkout-page-url-template-property-schema",
    feature = "pending-schema-section"
))]
pub use r#checkout_page_url_template::*;
#[cfg(any(
    feature = "checkout-time-property-schema",
    feature = "general-schema-section"
))]
mod r#checkout_time;
#[cfg(any(
    feature = "checkout-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#checkout_time::*;
#[cfg(any(
    feature = "chemical-composition-property-schema",
    feature = "pending-schema-section"
))]
mod r#chemical_composition;
#[cfg(any(
    feature = "chemical-composition-property-schema",
    feature = "pending-schema-section"
))]
pub use r#chemical_composition::*;
#[cfg(any(
    feature = "chemical-role-property-schema",
    feature = "pending-schema-section"
))]
mod r#chemical_role;
#[cfg(any(
    feature = "chemical-role-property-schema",
    feature = "pending-schema-section"
))]
pub use r#chemical_role::*;
#[cfg(any(
    feature = "child-max-age-property-schema",
    feature = "general-schema-section"
))]
mod r#child_max_age;
#[cfg(any(
    feature = "child-max-age-property-schema",
    feature = "general-schema-section"
))]
pub use r#child_max_age::*;
#[cfg(any(
    feature = "child-min-age-property-schema",
    feature = "general-schema-section"
))]
mod r#child_min_age;
#[cfg(any(
    feature = "child-min-age-property-schema",
    feature = "general-schema-section"
))]
pub use r#child_min_age::*;
#[cfg(any(
    feature = "child-taxon-property-schema",
    feature = "pending-schema-section"
))]
mod r#child_taxon;
#[cfg(any(
    feature = "child-taxon-property-schema",
    feature = "pending-schema-section"
))]
pub use r#child_taxon::*;
#[cfg(any(
    feature = "children-property-schema",
    feature = "general-schema-section"
))]
mod r#children;
#[cfg(any(
    feature = "children-property-schema",
    feature = "general-schema-section"
))]
pub use r#children::*;
#[cfg(any(
    feature = "cholesterol-content-property-schema",
    feature = "general-schema-section"
))]
mod r#cholesterol_content;
#[cfg(any(
    feature = "cholesterol-content-property-schema",
    feature = "general-schema-section"
))]
pub use r#cholesterol_content::*;
#[cfg(any(feature = "circle-property-schema", feature = "general-schema-section"))]
mod r#circle;
#[cfg(any(feature = "circle-property-schema", feature = "general-schema-section"))]
pub use r#circle::*;
#[cfg(any(
    feature = "citation-property-schema",
    feature = "general-schema-section"
))]
mod r#citation;
#[cfg(any(
    feature = "citation-property-schema",
    feature = "general-schema-section"
))]
pub use r#citation::*;
#[cfg(any(
    feature = "claim-interpreter-property-schema",
    feature = "pending-schema-section"
))]
mod r#claim_interpreter;
#[cfg(any(
    feature = "claim-interpreter-property-schema",
    feature = "pending-schema-section"
))]
pub use r#claim_interpreter::*;
#[cfg(any(
    feature = "claim-reviewed-property-schema",
    feature = "general-schema-section"
))]
mod r#claim_reviewed;
#[cfg(any(
    feature = "claim-reviewed-property-schema",
    feature = "general-schema-section"
))]
pub use r#claim_reviewed::*;
#[cfg(any(
    feature = "clincal-pharmacology-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#clincal_pharmacology;
#[cfg(any(
    feature = "clincal-pharmacology-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#clincal_pharmacology::*;
#[cfg(any(
    feature = "clinical-pharmacology-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#clinical_pharmacology;
#[cfg(any(
    feature = "clinical-pharmacology-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#clinical_pharmacology::*;
#[cfg(any(
    feature = "clip-number-property-schema",
    feature = "general-schema-section"
))]
mod r#clip_number;
#[cfg(any(
    feature = "clip-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#clip_number::*;
#[cfg(any(feature = "closes-property-schema", feature = "general-schema-section"))]
mod r#closes;
#[cfg(any(feature = "closes-property-schema", feature = "general-schema-section"))]
pub use r#closes::*;
#[cfg(any(feature = "coach-property-schema", feature = "general-schema-section"))]
mod r#coach;
#[cfg(any(feature = "coach-property-schema", feature = "general-schema-section"))]
pub use r#coach::*;
#[cfg(any(
    feature = "code-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#code;
#[cfg(any(
    feature = "code-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#code::*;
#[cfg(any(
    feature = "code-repository-property-schema",
    feature = "general-schema-section"
))]
mod r#code_repository;
#[cfg(any(
    feature = "code-repository-property-schema",
    feature = "general-schema-section"
))]
pub use r#code_repository::*;
#[cfg(any(
    feature = "code-sample-type-property-schema",
    feature = "general-schema-section"
))]
mod r#code_sample_type;
#[cfg(any(
    feature = "code-sample-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#code_sample_type::*;
#[cfg(any(
    feature = "code-value-property-schema",
    feature = "pending-schema-section"
))]
mod r#code_value;
#[cfg(any(
    feature = "code-value-property-schema",
    feature = "pending-schema-section"
))]
pub use r#code_value::*;
#[cfg(any(
    feature = "coding-system-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#coding_system;
#[cfg(any(
    feature = "coding-system-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#coding_system::*;
#[cfg(any(
    feature = "colleague-property-schema",
    feature = "general-schema-section"
))]
mod r#colleague;
#[cfg(any(
    feature = "colleague-property-schema",
    feature = "general-schema-section"
))]
pub use r#colleague::*;
#[cfg(any(
    feature = "colleagues-property-schema",
    feature = "general-schema-section"
))]
mod r#colleagues;
#[cfg(any(
    feature = "colleagues-property-schema",
    feature = "general-schema-section"
))]
pub use r#colleagues::*;
#[cfg(any(
    feature = "collection-property-schema",
    feature = "general-schema-section"
))]
mod r#collection;
#[cfg(any(
    feature = "collection-property-schema",
    feature = "general-schema-section"
))]
pub use r#collection::*;
#[cfg(any(
    feature = "collection-size-property-schema",
    feature = "pending-schema-section"
))]
mod r#collection_size;
#[cfg(any(
    feature = "collection-size-property-schema",
    feature = "pending-schema-section"
))]
pub use r#collection_size::*;
#[cfg(any(feature = "color-property-schema", feature = "general-schema-section"))]
mod r#color;
#[cfg(any(feature = "color-property-schema", feature = "general-schema-section"))]
pub use r#color::*;
#[cfg(any(feature = "colorist-property-schema", feature = "bib-schema-section"))]
mod r#colorist;
#[cfg(any(feature = "colorist-property-schema", feature = "bib-schema-section"))]
pub use r#colorist::*;
#[cfg(any(
    feature = "comment-property-schema",
    feature = "general-schema-section"
))]
mod r#comment;
#[cfg(any(
    feature = "comment-property-schema",
    feature = "general-schema-section"
))]
pub use r#comment::*;
#[cfg(any(
    feature = "comment-count-property-schema",
    feature = "general-schema-section"
))]
mod r#comment_count;
#[cfg(any(
    feature = "comment-count-property-schema",
    feature = "general-schema-section"
))]
pub use r#comment_count::*;
#[cfg(any(
    feature = "comment-text-property-schema",
    feature = "general-schema-section"
))]
mod r#comment_text;
#[cfg(any(
    feature = "comment-text-property-schema",
    feature = "general-schema-section"
))]
pub use r#comment_text::*;
#[cfg(any(
    feature = "comment-time-property-schema",
    feature = "general-schema-section"
))]
mod r#comment_time;
#[cfg(any(
    feature = "comment-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#comment_time::*;
#[cfg(any(
    feature = "competency-required-property-schema",
    feature = "pending-schema-section"
))]
mod r#competency_required;
#[cfg(any(
    feature = "competency-required-property-schema",
    feature = "pending-schema-section"
))]
pub use r#competency_required::*;
#[cfg(any(
    feature = "competitor-property-schema",
    feature = "general-schema-section"
))]
mod r#competitor;
#[cfg(any(
    feature = "competitor-property-schema",
    feature = "general-schema-section"
))]
pub use r#competitor::*;
#[cfg(any(
    feature = "composer-property-schema",
    feature = "general-schema-section"
))]
mod r#composer;
#[cfg(any(
    feature = "composer-property-schema",
    feature = "general-schema-section"
))]
pub use r#composer::*;
#[cfg(any(
    feature = "comprised-of-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#comprised_of;
#[cfg(any(
    feature = "comprised-of-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#comprised_of::*;
#[cfg(any(
    feature = "conditions-of-access-property-schema",
    feature = "pending-schema-section"
))]
mod r#conditions_of_access;
#[cfg(any(
    feature = "conditions-of-access-property-schema",
    feature = "pending-schema-section"
))]
pub use r#conditions_of_access::*;
#[cfg(any(
    feature = "confirmation-number-property-schema",
    feature = "general-schema-section"
))]
mod r#confirmation_number;
#[cfg(any(
    feature = "confirmation-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#confirmation_number::*;
#[cfg(any(
    feature = "connected-to-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#connected_to;
#[cfg(any(
    feature = "connected-to-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#connected_to::*;
#[cfg(any(
    feature = "constraint-property-property-schema",
    feature = "pending-schema-section"
))]
mod r#constraint_property;
#[cfg(any(
    feature = "constraint-property-property-schema",
    feature = "pending-schema-section"
))]
pub use r#constraint_property::*;
#[cfg(any(
    feature = "contact-option-property-schema",
    feature = "general-schema-section"
))]
mod r#contact_option;
#[cfg(any(
    feature = "contact-option-property-schema",
    feature = "general-schema-section"
))]
pub use r#contact_option::*;
#[cfg(any(
    feature = "contact-point-property-schema",
    feature = "general-schema-section"
))]
mod r#contact_point;
#[cfg(any(
    feature = "contact-point-property-schema",
    feature = "general-schema-section"
))]
pub use r#contact_point::*;
#[cfg(any(
    feature = "contact-points-property-schema",
    feature = "general-schema-section"
))]
mod r#contact_points;
#[cfg(any(
    feature = "contact-points-property-schema",
    feature = "general-schema-section"
))]
pub use r#contact_points::*;
#[cfg(any(
    feature = "contact-type-property-schema",
    feature = "general-schema-section"
))]
mod r#contact_type;
#[cfg(any(
    feature = "contact-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#contact_type::*;
#[cfg(any(
    feature = "contactless-payment-property-schema",
    feature = "pending-schema-section"
))]
mod r#contactless_payment;
#[cfg(any(
    feature = "contactless-payment-property-schema",
    feature = "pending-schema-section"
))]
pub use r#contactless_payment::*;
#[cfg(any(
    feature = "contained-in-property-schema",
    feature = "general-schema-section"
))]
mod r#contained_in;
#[cfg(any(
    feature = "contained-in-property-schema",
    feature = "general-schema-section"
))]
pub use r#contained_in::*;
#[cfg(any(
    feature = "contained-in-place-property-schema",
    feature = "general-schema-section"
))]
mod r#contained_in_place;
#[cfg(any(
    feature = "contained-in-place-property-schema",
    feature = "general-schema-section"
))]
pub use r#contained_in_place::*;
#[cfg(any(
    feature = "contains-place-property-schema",
    feature = "general-schema-section"
))]
mod r#contains_place;
#[cfg(any(
    feature = "contains-place-property-schema",
    feature = "general-schema-section"
))]
pub use r#contains_place::*;
#[cfg(any(
    feature = "contains-season-property-schema",
    feature = "general-schema-section"
))]
mod r#contains_season;
#[cfg(any(
    feature = "contains-season-property-schema",
    feature = "general-schema-section"
))]
pub use r#contains_season::*;
#[cfg(any(
    feature = "content-location-property-schema",
    feature = "general-schema-section"
))]
mod r#content_location;
#[cfg(any(
    feature = "content-location-property-schema",
    feature = "general-schema-section"
))]
pub use r#content_location::*;
#[cfg(any(
    feature = "content-rating-property-schema",
    feature = "general-schema-section"
))]
mod r#content_rating;
#[cfg(any(
    feature = "content-rating-property-schema",
    feature = "general-schema-section"
))]
pub use r#content_rating::*;
#[cfg(any(
    feature = "content-reference-time-property-schema",
    feature = "pending-schema-section"
))]
mod r#content_reference_time;
#[cfg(any(
    feature = "content-reference-time-property-schema",
    feature = "pending-schema-section"
))]
pub use r#content_reference_time::*;
#[cfg(any(
    feature = "content-size-property-schema",
    feature = "general-schema-section"
))]
mod r#content_size;
#[cfg(any(
    feature = "content-size-property-schema",
    feature = "general-schema-section"
))]
pub use r#content_size::*;
#[cfg(any(
    feature = "content-type-property-schema",
    feature = "general-schema-section"
))]
mod r#content_type;
#[cfg(any(
    feature = "content-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#content_type::*;
#[cfg(any(
    feature = "content-url-property-schema",
    feature = "general-schema-section"
))]
mod r#content_url;
#[cfg(any(
    feature = "content-url-property-schema",
    feature = "general-schema-section"
))]
pub use r#content_url::*;
#[cfg(any(
    feature = "contraindication-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#contraindication;
#[cfg(any(
    feature = "contraindication-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#contraindication::*;
#[cfg(any(
    feature = "contributor-property-schema",
    feature = "general-schema-section"
))]
mod r#contributor;
#[cfg(any(
    feature = "contributor-property-schema",
    feature = "general-schema-section"
))]
pub use r#contributor::*;
#[cfg(any(
    feature = "cook-time-property-schema",
    feature = "general-schema-section"
))]
mod r#cook_time;
#[cfg(any(
    feature = "cook-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#cook_time::*;
#[cfg(any(
    feature = "cooking-method-property-schema",
    feature = "general-schema-section"
))]
mod r#cooking_method;
#[cfg(any(
    feature = "cooking-method-property-schema",
    feature = "general-schema-section"
))]
pub use r#cooking_method::*;
#[cfg(any(
    feature = "copyright-holder-property-schema",
    feature = "general-schema-section"
))]
mod r#copyright_holder;
#[cfg(any(
    feature = "copyright-holder-property-schema",
    feature = "general-schema-section"
))]
pub use r#copyright_holder::*;
#[cfg(any(
    feature = "copyright-notice-property-schema",
    feature = "pending-schema-section"
))]
mod r#copyright_notice;
#[cfg(any(
    feature = "copyright-notice-property-schema",
    feature = "pending-schema-section"
))]
pub use r#copyright_notice::*;
#[cfg(any(
    feature = "copyright-year-property-schema",
    feature = "general-schema-section"
))]
mod r#copyright_year;
#[cfg(any(
    feature = "copyright-year-property-schema",
    feature = "general-schema-section"
))]
pub use r#copyright_year::*;
#[cfg(any(
    feature = "correction-property-schema",
    feature = "pending-schema-section"
))]
mod r#correction;
#[cfg(any(
    feature = "correction-property-schema",
    feature = "pending-schema-section"
))]
pub use r#correction::*;
#[cfg(any(
    feature = "corrections-policy-property-schema",
    feature = "pending-schema-section"
))]
mod r#corrections_policy;
#[cfg(any(
    feature = "corrections-policy-property-schema",
    feature = "pending-schema-section"
))]
pub use r#corrections_policy::*;
#[cfg(any(
    feature = "cost-category-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#cost_category;
#[cfg(any(
    feature = "cost-category-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#cost_category::*;
#[cfg(any(
    feature = "cost-currency-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#cost_currency;
#[cfg(any(
    feature = "cost-currency-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#cost_currency::*;
#[cfg(any(
    feature = "cost-origin-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#cost_origin;
#[cfg(any(
    feature = "cost-origin-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#cost_origin::*;
#[cfg(any(
    feature = "cost-per-unit-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#cost_per_unit;
#[cfg(any(
    feature = "cost-per-unit-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#cost_per_unit::*;
#[cfg(any(
    feature = "countries-not-supported-property-schema",
    feature = "general-schema-section"
))]
mod r#countries_not_supported;
#[cfg(any(
    feature = "countries-not-supported-property-schema",
    feature = "general-schema-section"
))]
pub use r#countries_not_supported::*;
#[cfg(any(
    feature = "countries-supported-property-schema",
    feature = "general-schema-section"
))]
mod r#countries_supported;
#[cfg(any(
    feature = "countries-supported-property-schema",
    feature = "general-schema-section"
))]
pub use r#countries_supported::*;
#[cfg(any(
    feature = "country-of-assembly-property-schema",
    feature = "pending-schema-section"
))]
mod r#country_of_assembly;
#[cfg(any(
    feature = "country-of-assembly-property-schema",
    feature = "pending-schema-section"
))]
pub use r#country_of_assembly::*;
#[cfg(any(
    feature = "country-of-last-processing-property-schema",
    feature = "pending-schema-section"
))]
mod r#country_of_last_processing;
#[cfg(any(
    feature = "country-of-last-processing-property-schema",
    feature = "pending-schema-section"
))]
pub use r#country_of_last_processing::*;
#[cfg(any(
    feature = "country-of-origin-property-schema",
    feature = "general-schema-section"
))]
mod r#country_of_origin;
#[cfg(any(
    feature = "country-of-origin-property-schema",
    feature = "general-schema-section"
))]
pub use r#country_of_origin::*;
#[cfg(any(feature = "course-property-schema", feature = "general-schema-section"))]
mod r#course;
#[cfg(any(feature = "course-property-schema", feature = "general-schema-section"))]
pub use r#course::*;
#[cfg(any(
    feature = "course-code-property-schema",
    feature = "general-schema-section"
))]
mod r#course_code;
#[cfg(any(
    feature = "course-code-property-schema",
    feature = "general-schema-section"
))]
pub use r#course_code::*;
#[cfg(any(
    feature = "course-mode-property-schema",
    feature = "general-schema-section"
))]
mod r#course_mode;
#[cfg(any(
    feature = "course-mode-property-schema",
    feature = "general-schema-section"
))]
pub use r#course_mode::*;
#[cfg(any(
    feature = "course-prerequisites-property-schema",
    feature = "general-schema-section"
))]
mod r#course_prerequisites;
#[cfg(any(
    feature = "course-prerequisites-property-schema",
    feature = "general-schema-section"
))]
pub use r#course_prerequisites::*;
#[cfg(any(
    feature = "course-schedule-property-schema",
    feature = "pending-schema-section"
))]
mod r#course_schedule;
#[cfg(any(
    feature = "course-schedule-property-schema",
    feature = "pending-schema-section"
))]
pub use r#course_schedule::*;
#[cfg(any(
    feature = "course-workload-property-schema",
    feature = "pending-schema-section"
))]
mod r#course_workload;
#[cfg(any(
    feature = "course-workload-property-schema",
    feature = "pending-schema-section"
))]
pub use r#course_workload::*;
#[cfg(any(
    feature = "coverage-end-time-property-schema",
    feature = "general-schema-section"
))]
mod r#coverage_end_time;
#[cfg(any(
    feature = "coverage-end-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#coverage_end_time::*;
#[cfg(any(
    feature = "coverage-start-time-property-schema",
    feature = "general-schema-section"
))]
mod r#coverage_start_time;
#[cfg(any(
    feature = "coverage-start-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#coverage_start_time::*;
#[cfg(any(
    feature = "creative-work-status-property-schema",
    feature = "pending-schema-section"
))]
mod r#creative_work_status;
#[cfg(any(
    feature = "creative-work-status-property-schema",
    feature = "pending-schema-section"
))]
pub use r#creative_work_status::*;
#[cfg(any(
    feature = "creator-property-schema",
    feature = "general-schema-section"
))]
mod r#creator;
#[cfg(any(
    feature = "creator-property-schema",
    feature = "general-schema-section"
))]
pub use r#creator::*;
#[cfg(any(
    feature = "credential-category-property-schema",
    feature = "pending-schema-section"
))]
mod r#credential_category;
#[cfg(any(
    feature = "credential-category-property-schema",
    feature = "pending-schema-section"
))]
pub use r#credential_category::*;
#[cfg(any(
    feature = "credit-text-property-schema",
    feature = "pending-schema-section"
))]
mod r#credit_text;
#[cfg(any(
    feature = "credit-text-property-schema",
    feature = "pending-schema-section"
))]
pub use r#credit_text::*;
#[cfg(any(
    feature = "credited-to-property-schema",
    feature = "general-schema-section"
))]
mod r#credited_to;
#[cfg(any(
    feature = "credited-to-property-schema",
    feature = "general-schema-section"
))]
pub use r#credited_to::*;
#[cfg(any(
    feature = "css-selector-property-schema",
    feature = "general-schema-section"
))]
mod r#css_selector;
#[cfg(any(
    feature = "css-selector-property-schema",
    feature = "general-schema-section"
))]
pub use r#css_selector::*;
#[cfg(any(
    feature = "currencies-accepted-property-schema",
    feature = "general-schema-section"
))]
mod r#currencies_accepted;
#[cfg(any(
    feature = "currencies-accepted-property-schema",
    feature = "general-schema-section"
))]
pub use r#currencies_accepted::*;
#[cfg(any(
    feature = "currency-property-schema",
    feature = "general-schema-section"
))]
mod r#currency;
#[cfg(any(
    feature = "currency-property-schema",
    feature = "general-schema-section"
))]
pub use r#currency::*;
#[cfg(any(
    feature = "current-exchange-rate-property-schema",
    feature = "pending-schema-section"
))]
mod r#current_exchange_rate;
#[cfg(any(
    feature = "current-exchange-rate-property-schema",
    feature = "pending-schema-section"
))]
pub use r#current_exchange_rate::*;
#[cfg(any(
    feature = "customer-property-schema",
    feature = "general-schema-section"
))]
mod r#customer;
#[cfg(any(
    feature = "customer-property-schema",
    feature = "general-schema-section"
))]
pub use r#customer::*;
#[cfg(any(
    feature = "customer-remorse-return-fees-property-schema",
    feature = "pending-schema-section"
))]
mod r#customer_remorse_return_fees;
#[cfg(any(
    feature = "customer-remorse-return-fees-property-schema",
    feature = "pending-schema-section"
))]
pub use r#customer_remorse_return_fees::*;
#[cfg(any(
    feature = "customer-remorse-return-label-source-property-schema",
    feature = "pending-schema-section"
))]
mod r#customer_remorse_return_label_source;
#[cfg(any(
    feature = "customer-remorse-return-label-source-property-schema",
    feature = "pending-schema-section"
))]
pub use r#customer_remorse_return_label_source::*;
#[cfg(any(
    feature = "customer-remorse-return-shipping-fees-amount-property-schema",
    feature = "pending-schema-section"
))]
mod r#customer_remorse_return_shipping_fees_amount;
#[cfg(any(
    feature = "customer-remorse-return-shipping-fees-amount-property-schema",
    feature = "pending-schema-section"
))]
pub use r#customer_remorse_return_shipping_fees_amount::*;
#[cfg(any(
    feature = "cutoff-time-property-schema",
    feature = "pending-schema-section"
))]
mod r#cutoff_time;
#[cfg(any(
    feature = "cutoff-time-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cutoff_time::*;
#[cfg(any(
    feature = "cvd-collection-date-property-schema",
    feature = "pending-schema-section"
))]
mod r#cvd_collection_date;
#[cfg(any(
    feature = "cvd-collection-date-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cvd_collection_date::*;
#[cfg(any(
    feature = "cvd-facility-county-property-schema",
    feature = "pending-schema-section"
))]
mod r#cvd_facility_county;
#[cfg(any(
    feature = "cvd-facility-county-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cvd_facility_county::*;
#[cfg(any(
    feature = "cvd-facility-id-property-schema",
    feature = "pending-schema-section"
))]
mod r#cvd_facility_id;
#[cfg(any(
    feature = "cvd-facility-id-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cvd_facility_id::*;
#[cfg(any(
    feature = "cvd-num-beds-property-schema",
    feature = "pending-schema-section"
))]
mod r#cvd_num_beds;
#[cfg(any(
    feature = "cvd-num-beds-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cvd_num_beds::*;
#[cfg(any(
    feature = "cvd-num-beds-occ-property-schema",
    feature = "pending-schema-section"
))]
mod r#cvd_num_beds_occ;
#[cfg(any(
    feature = "cvd-num-beds-occ-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cvd_num_beds_occ::*;
#[cfg(any(
    feature = "cvd-num-c-19-died-property-schema",
    feature = "pending-schema-section"
))]
mod r#cvd_num_c_19_died;
#[cfg(any(
    feature = "cvd-num-c-19-died-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cvd_num_c_19_died::*;
#[cfg(any(
    feature = "cvd-num-c-19-ho-pats-property-schema",
    feature = "pending-schema-section"
))]
mod r#cvd_num_c_19_ho_pats;
#[cfg(any(
    feature = "cvd-num-c-19-ho-pats-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cvd_num_c_19_ho_pats::*;
#[cfg(any(
    feature = "cvd-num-c-19-hosp-pats-property-schema",
    feature = "pending-schema-section"
))]
mod r#cvd_num_c_19_hosp_pats;
#[cfg(any(
    feature = "cvd-num-c-19-hosp-pats-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cvd_num_c_19_hosp_pats::*;
#[cfg(any(
    feature = "cvd-num-c-19-mech-vent-pats-property-schema",
    feature = "pending-schema-section"
))]
mod r#cvd_num_c_19_mech_vent_pats;
#[cfg(any(
    feature = "cvd-num-c-19-mech-vent-pats-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cvd_num_c_19_mech_vent_pats::*;
#[cfg(any(
    feature = "cvd-num-c-19-of-mech-vent-pats-property-schema",
    feature = "pending-schema-section"
))]
mod r#cvd_num_c_19_of_mech_vent_pats;
#[cfg(any(
    feature = "cvd-num-c-19-of-mech-vent-pats-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cvd_num_c_19_of_mech_vent_pats::*;
#[cfg(any(
    feature = "cvd-num-c-19-overflow-pats-property-schema",
    feature = "pending-schema-section"
))]
mod r#cvd_num_c_19_overflow_pats;
#[cfg(any(
    feature = "cvd-num-c-19-overflow-pats-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cvd_num_c_19_overflow_pats::*;
#[cfg(any(
    feature = "cvd-num-icu-beds-property-schema",
    feature = "pending-schema-section"
))]
mod r#cvd_num_icu_beds;
#[cfg(any(
    feature = "cvd-num-icu-beds-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cvd_num_icu_beds::*;
#[cfg(any(
    feature = "cvd-num-icu-beds-occ-property-schema",
    feature = "pending-schema-section"
))]
mod r#cvd_num_icu_beds_occ;
#[cfg(any(
    feature = "cvd-num-icu-beds-occ-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cvd_num_icu_beds_occ::*;
#[cfg(any(
    feature = "cvd-num-tot-beds-property-schema",
    feature = "pending-schema-section"
))]
mod r#cvd_num_tot_beds;
#[cfg(any(
    feature = "cvd-num-tot-beds-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cvd_num_tot_beds::*;
#[cfg(any(
    feature = "cvd-num-vent-property-schema",
    feature = "pending-schema-section"
))]
mod r#cvd_num_vent;
#[cfg(any(
    feature = "cvd-num-vent-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cvd_num_vent::*;
#[cfg(any(
    feature = "cvd-num-vent-use-property-schema",
    feature = "pending-schema-section"
))]
mod r#cvd_num_vent_use;
#[cfg(any(
    feature = "cvd-num-vent-use-property-schema",
    feature = "pending-schema-section"
))]
pub use r#cvd_num_vent_use::*;
#[cfg(any(
    feature = "data-feed-element-property-schema",
    feature = "general-schema-section"
))]
mod r#data_feed_element;
#[cfg(any(
    feature = "data-feed-element-property-schema",
    feature = "general-schema-section"
))]
pub use r#data_feed_element::*;
#[cfg(any(
    feature = "dataset-property-schema",
    feature = "general-schema-section"
))]
mod r#dataset;
#[cfg(any(
    feature = "dataset-property-schema",
    feature = "general-schema-section"
))]
pub use r#dataset::*;
#[cfg(any(
    feature = "dataset-time-interval-property-schema",
    feature = "general-schema-section"
))]
mod r#dataset_time_interval;
#[cfg(any(
    feature = "dataset-time-interval-property-schema",
    feature = "general-schema-section"
))]
pub use r#dataset_time_interval::*;
#[cfg(any(
    feature = "date-created-property-schema",
    feature = "general-schema-section"
))]
mod r#date_created;
#[cfg(any(
    feature = "date-created-property-schema",
    feature = "general-schema-section"
))]
pub use r#date_created::*;
#[cfg(any(
    feature = "date-deleted-property-schema",
    feature = "general-schema-section"
))]
mod r#date_deleted;
#[cfg(any(
    feature = "date-deleted-property-schema",
    feature = "general-schema-section"
))]
pub use r#date_deleted::*;
#[cfg(any(
    feature = "date-issued-property-schema",
    feature = "general-schema-section"
))]
mod r#date_issued;
#[cfg(any(
    feature = "date-issued-property-schema",
    feature = "general-schema-section"
))]
pub use r#date_issued::*;
#[cfg(any(
    feature = "date-modified-property-schema",
    feature = "general-schema-section"
))]
mod r#date_modified;
#[cfg(any(
    feature = "date-modified-property-schema",
    feature = "general-schema-section"
))]
pub use r#date_modified::*;
#[cfg(any(
    feature = "date-posted-property-schema",
    feature = "general-schema-section"
))]
mod r#date_posted;
#[cfg(any(
    feature = "date-posted-property-schema",
    feature = "general-schema-section"
))]
pub use r#date_posted::*;
#[cfg(any(
    feature = "date-published-property-schema",
    feature = "general-schema-section"
))]
mod r#date_published;
#[cfg(any(
    feature = "date-published-property-schema",
    feature = "general-schema-section"
))]
pub use r#date_published::*;
#[cfg(any(
    feature = "date-read-property-schema",
    feature = "general-schema-section"
))]
mod r#date_read;
#[cfg(any(
    feature = "date-read-property-schema",
    feature = "general-schema-section"
))]
pub use r#date_read::*;
#[cfg(any(
    feature = "date-received-property-schema",
    feature = "general-schema-section"
))]
mod r#date_received;
#[cfg(any(
    feature = "date-received-property-schema",
    feature = "general-schema-section"
))]
pub use r#date_received::*;
#[cfg(any(
    feature = "date-sent-property-schema",
    feature = "general-schema-section"
))]
mod r#date_sent;
#[cfg(any(
    feature = "date-sent-property-schema",
    feature = "general-schema-section"
))]
pub use r#date_sent::*;
#[cfg(any(
    feature = "date-vehicle-first-registered-property-schema",
    feature = "general-schema-section"
))]
mod r#date_vehicle_first_registered;
#[cfg(any(
    feature = "date-vehicle-first-registered-property-schema",
    feature = "general-schema-section"
))]
pub use r#date_vehicle_first_registered::*;
#[cfg(any(
    feature = "dateline-property-schema",
    feature = "general-schema-section"
))]
mod r#dateline;
#[cfg(any(
    feature = "dateline-property-schema",
    feature = "general-schema-section"
))]
pub use r#dateline::*;
#[cfg(any(
    feature = "day-of-week-property-schema",
    feature = "general-schema-section"
))]
mod r#day_of_week;
#[cfg(any(
    feature = "day-of-week-property-schema",
    feature = "general-schema-section"
))]
pub use r#day_of_week::*;
#[cfg(any(
    feature = "death-date-property-schema",
    feature = "general-schema-section"
))]
mod r#death_date;
#[cfg(any(
    feature = "death-date-property-schema",
    feature = "general-schema-section"
))]
pub use r#death_date::*;
#[cfg(any(
    feature = "death-place-property-schema",
    feature = "general-schema-section"
))]
mod r#death_place;
#[cfg(any(
    feature = "death-place-property-schema",
    feature = "general-schema-section"
))]
pub use r#death_place::*;
#[cfg(any(
    feature = "default-value-property-schema",
    feature = "general-schema-section"
))]
mod r#default_value;
#[cfg(any(
    feature = "default-value-property-schema",
    feature = "general-schema-section"
))]
pub use r#default_value::*;
#[cfg(any(
    feature = "delivery-address-property-schema",
    feature = "general-schema-section"
))]
mod r#delivery_address;
#[cfg(any(
    feature = "delivery-address-property-schema",
    feature = "general-schema-section"
))]
pub use r#delivery_address::*;
#[cfg(any(
    feature = "delivery-lead-time-property-schema",
    feature = "general-schema-section"
))]
mod r#delivery_lead_time;
#[cfg(any(
    feature = "delivery-lead-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#delivery_lead_time::*;
#[cfg(any(
    feature = "delivery-method-property-schema",
    feature = "general-schema-section"
))]
mod r#delivery_method;
#[cfg(any(
    feature = "delivery-method-property-schema",
    feature = "general-schema-section"
))]
pub use r#delivery_method::*;
#[cfg(any(
    feature = "delivery-status-property-schema",
    feature = "general-schema-section"
))]
mod r#delivery_status;
#[cfg(any(
    feature = "delivery-status-property-schema",
    feature = "general-schema-section"
))]
pub use r#delivery_status::*;
#[cfg(any(
    feature = "delivery-time-property-schema",
    feature = "pending-schema-section"
))]
mod r#delivery_time;
#[cfg(any(
    feature = "delivery-time-property-schema",
    feature = "pending-schema-section"
))]
pub use r#delivery_time::*;
#[cfg(any(
    feature = "department-property-schema",
    feature = "general-schema-section"
))]
mod r#department;
#[cfg(any(
    feature = "department-property-schema",
    feature = "general-schema-section"
))]
pub use r#department::*;
#[cfg(any(
    feature = "departure-airport-property-schema",
    feature = "general-schema-section"
))]
mod r#departure_airport;
#[cfg(any(
    feature = "departure-airport-property-schema",
    feature = "general-schema-section"
))]
pub use r#departure_airport::*;
#[cfg(any(
    feature = "departure-boat-terminal-property-schema",
    feature = "pending-schema-section"
))]
mod r#departure_boat_terminal;
#[cfg(any(
    feature = "departure-boat-terminal-property-schema",
    feature = "pending-schema-section"
))]
pub use r#departure_boat_terminal::*;
#[cfg(any(
    feature = "departure-bus-stop-property-schema",
    feature = "general-schema-section"
))]
mod r#departure_bus_stop;
#[cfg(any(
    feature = "departure-bus-stop-property-schema",
    feature = "general-schema-section"
))]
pub use r#departure_bus_stop::*;
#[cfg(any(
    feature = "departure-gate-property-schema",
    feature = "general-schema-section"
))]
mod r#departure_gate;
#[cfg(any(
    feature = "departure-gate-property-schema",
    feature = "general-schema-section"
))]
pub use r#departure_gate::*;
#[cfg(any(
    feature = "departure-platform-property-schema",
    feature = "general-schema-section"
))]
mod r#departure_platform;
#[cfg(any(
    feature = "departure-platform-property-schema",
    feature = "general-schema-section"
))]
pub use r#departure_platform::*;
#[cfg(any(
    feature = "departure-station-property-schema",
    feature = "general-schema-section"
))]
mod r#departure_station;
#[cfg(any(
    feature = "departure-station-property-schema",
    feature = "general-schema-section"
))]
pub use r#departure_station::*;
#[cfg(any(
    feature = "departure-terminal-property-schema",
    feature = "general-schema-section"
))]
mod r#departure_terminal;
#[cfg(any(
    feature = "departure-terminal-property-schema",
    feature = "general-schema-section"
))]
pub use r#departure_terminal::*;
#[cfg(any(
    feature = "departure-time-property-schema",
    feature = "general-schema-section"
))]
mod r#departure_time;
#[cfg(any(
    feature = "departure-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#departure_time::*;
#[cfg(any(
    feature = "dependencies-property-schema",
    feature = "general-schema-section"
))]
mod r#dependencies;
#[cfg(any(
    feature = "dependencies-property-schema",
    feature = "general-schema-section"
))]
pub use r#dependencies::*;
#[cfg(any(feature = "depth-property-schema", feature = "general-schema-section"))]
mod r#depth;
#[cfg(any(feature = "depth-property-schema", feature = "general-schema-section"))]
pub use r#depth::*;
#[cfg(any(
    feature = "description-property-schema",
    feature = "general-schema-section"
))]
mod r#description;
#[cfg(any(
    feature = "description-property-schema",
    feature = "general-schema-section"
))]
pub use r#description::*;
#[cfg(any(feature = "device-property-schema", feature = "general-schema-section"))]
mod r#device;
#[cfg(any(feature = "device-property-schema", feature = "general-schema-section"))]
pub use r#device::*;
#[cfg(any(
    feature = "diagnosis-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#diagnosis;
#[cfg(any(
    feature = "diagnosis-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#diagnosis::*;
#[cfg(any(
    feature = "diagram-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#diagram;
#[cfg(any(
    feature = "diagram-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#diagram::*;
#[cfg(any(
    feature = "diet-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#diet;
#[cfg(any(
    feature = "diet-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#diet::*;
#[cfg(any(
    feature = "diet-features-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#diet_features;
#[cfg(any(
    feature = "diet-features-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#diet_features::*;
#[cfg(any(
    feature = "differential-diagnosis-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#differential_diagnosis;
#[cfg(any(
    feature = "differential-diagnosis-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#differential_diagnosis::*;
#[cfg(any(
    feature = "direct-apply-property-schema",
    feature = "pending-schema-section"
))]
mod r#direct_apply;
#[cfg(any(
    feature = "direct-apply-property-schema",
    feature = "pending-schema-section"
))]
pub use r#direct_apply::*;
#[cfg(any(
    feature = "director-property-schema",
    feature = "general-schema-section"
))]
mod r#director;
#[cfg(any(
    feature = "director-property-schema",
    feature = "general-schema-section"
))]
pub use r#director::*;
#[cfg(any(
    feature = "directors-property-schema",
    feature = "general-schema-section"
))]
mod r#directors;
#[cfg(any(
    feature = "directors-property-schema",
    feature = "general-schema-section"
))]
pub use r#directors::*;
#[cfg(any(
    feature = "disambiguating-description-property-schema",
    feature = "general-schema-section"
))]
mod r#disambiguating_description;
#[cfg(any(
    feature = "disambiguating-description-property-schema",
    feature = "general-schema-section"
))]
pub use r#disambiguating_description::*;
#[cfg(any(
    feature = "discount-property-schema",
    feature = "general-schema-section"
))]
mod r#discount;
#[cfg(any(
    feature = "discount-property-schema",
    feature = "general-schema-section"
))]
pub use r#discount::*;
#[cfg(any(
    feature = "discount-code-property-schema",
    feature = "general-schema-section"
))]
mod r#discount_code;
#[cfg(any(
    feature = "discount-code-property-schema",
    feature = "general-schema-section"
))]
pub use r#discount_code::*;
#[cfg(any(
    feature = "discount-currency-property-schema",
    feature = "general-schema-section"
))]
mod r#discount_currency;
#[cfg(any(
    feature = "discount-currency-property-schema",
    feature = "general-schema-section"
))]
pub use r#discount_currency::*;
#[cfg(any(
    feature = "discusses-property-schema",
    feature = "general-schema-section"
))]
mod r#discusses;
#[cfg(any(
    feature = "discusses-property-schema",
    feature = "general-schema-section"
))]
pub use r#discusses::*;
#[cfg(any(
    feature = "discussion-url-property-schema",
    feature = "general-schema-section"
))]
mod r#discussion_url;
#[cfg(any(
    feature = "discussion-url-property-schema",
    feature = "general-schema-section"
))]
pub use r#discussion_url::*;
#[cfg(any(
    feature = "disease-prevention-info-property-schema",
    feature = "pending-schema-section"
))]
mod r#disease_prevention_info;
#[cfg(any(
    feature = "disease-prevention-info-property-schema",
    feature = "pending-schema-section"
))]
pub use r#disease_prevention_info::*;
#[cfg(any(
    feature = "disease-spread-statistics-property-schema",
    feature = "pending-schema-section"
))]
mod r#disease_spread_statistics;
#[cfg(any(
    feature = "disease-spread-statistics-property-schema",
    feature = "pending-schema-section"
))]
pub use r#disease_spread_statistics::*;
#[cfg(any(
    feature = "dissolution-date-property-schema",
    feature = "general-schema-section"
))]
mod r#dissolution_date;
#[cfg(any(
    feature = "dissolution-date-property-schema",
    feature = "general-schema-section"
))]
pub use r#dissolution_date::*;
#[cfg(any(
    feature = "distance-property-schema",
    feature = "general-schema-section"
))]
mod r#distance;
#[cfg(any(
    feature = "distance-property-schema",
    feature = "general-schema-section"
))]
pub use r#distance::*;
#[cfg(any(
    feature = "distinguishing-sign-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#distinguishing_sign;
#[cfg(any(
    feature = "distinguishing-sign-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#distinguishing_sign::*;
#[cfg(any(
    feature = "distribution-property-schema",
    feature = "general-schema-section"
))]
mod r#distribution;
#[cfg(any(
    feature = "distribution-property-schema",
    feature = "general-schema-section"
))]
pub use r#distribution::*;
#[cfg(any(
    feature = "diversity-policy-property-schema",
    feature = "pending-schema-section"
))]
mod r#diversity_policy;
#[cfg(any(
    feature = "diversity-policy-property-schema",
    feature = "pending-schema-section"
))]
pub use r#diversity_policy::*;
#[cfg(any(
    feature = "diversity-staffing-report-property-schema",
    feature = "pending-schema-section"
))]
mod r#diversity_staffing_report;
#[cfg(any(
    feature = "diversity-staffing-report-property-schema",
    feature = "pending-schema-section"
))]
pub use r#diversity_staffing_report::*;
#[cfg(any(
    feature = "documentation-property-schema",
    feature = "pending-schema-section"
))]
mod r#documentation;
#[cfg(any(
    feature = "documentation-property-schema",
    feature = "pending-schema-section"
))]
pub use r#documentation::*;
#[cfg(any(
    feature = "does-not-ship-property-schema",
    feature = "pending-schema-section"
))]
mod r#does_not_ship;
#[cfg(any(
    feature = "does-not-ship-property-schema",
    feature = "pending-schema-section"
))]
pub use r#does_not_ship::*;
#[cfg(any(
    feature = "domain-includes-property-schema",
    feature = "meta-schema-section"
))]
mod r#domain_includes;
#[cfg(any(
    feature = "domain-includes-property-schema",
    feature = "meta-schema-section"
))]
pub use r#domain_includes::*;
#[cfg(any(
    feature = "domiciled-mortgage-property-schema",
    feature = "pending-schema-section"
))]
mod r#domiciled_mortgage;
#[cfg(any(
    feature = "domiciled-mortgage-property-schema",
    feature = "pending-schema-section"
))]
pub use r#domiciled_mortgage::*;
#[cfg(any(
    feature = "door-time-property-schema",
    feature = "general-schema-section"
))]
mod r#door_time;
#[cfg(any(
    feature = "door-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#door_time::*;
#[cfg(any(
    feature = "dosage-form-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#dosage_form;
#[cfg(any(
    feature = "dosage-form-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#dosage_form::*;
#[cfg(any(
    feature = "dose-schedule-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#dose_schedule;
#[cfg(any(
    feature = "dose-schedule-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#dose_schedule::*;
#[cfg(any(
    feature = "dose-unit-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#dose_unit;
#[cfg(any(
    feature = "dose-unit-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#dose_unit::*;
#[cfg(any(
    feature = "dose-value-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#dose_value;
#[cfg(any(
    feature = "dose-value-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#dose_value::*;
#[cfg(any(
    feature = "down-payment-property-schema",
    feature = "pending-schema-section"
))]
mod r#down_payment;
#[cfg(any(
    feature = "down-payment-property-schema",
    feature = "pending-schema-section"
))]
pub use r#down_payment::*;
#[cfg(any(
    feature = "download-url-property-schema",
    feature = "general-schema-section"
))]
mod r#download_url;
#[cfg(any(
    feature = "download-url-property-schema",
    feature = "general-schema-section"
))]
pub use r#download_url::*;
#[cfg(any(
    feature = "downvote-count-property-schema",
    feature = "general-schema-section"
))]
mod r#downvote_count;
#[cfg(any(
    feature = "downvote-count-property-schema",
    feature = "general-schema-section"
))]
pub use r#downvote_count::*;
#[cfg(any(
    feature = "drains-to-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#drains_to;
#[cfg(any(
    feature = "drains-to-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#drains_to::*;
#[cfg(any(
    feature = "drive-wheel-configuration-property-schema",
    feature = "general-schema-section"
))]
mod r#drive_wheel_configuration;
#[cfg(any(
    feature = "drive-wheel-configuration-property-schema",
    feature = "general-schema-section"
))]
pub use r#drive_wheel_configuration::*;
#[cfg(any(
    feature = "dropoff-location-property-schema",
    feature = "general-schema-section"
))]
mod r#dropoff_location;
#[cfg(any(
    feature = "dropoff-location-property-schema",
    feature = "general-schema-section"
))]
pub use r#dropoff_location::*;
#[cfg(any(
    feature = "dropoff-time-property-schema",
    feature = "general-schema-section"
))]
mod r#dropoff_time;
#[cfg(any(
    feature = "dropoff-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#dropoff_time::*;
#[cfg(any(
    feature = "drug-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#drug;
#[cfg(any(
    feature = "drug-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#drug::*;
#[cfg(any(
    feature = "drug-class-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#drug_class;
#[cfg(any(
    feature = "drug-class-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#drug_class::*;
#[cfg(any(
    feature = "drug-unit-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#drug_unit;
#[cfg(any(
    feature = "drug-unit-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#drug_unit::*;
#[cfg(any(feature = "duns-property-schema", feature = "general-schema-section"))]
mod r#duns;
#[cfg(any(feature = "duns-property-schema", feature = "general-schema-section"))]
pub use r#duns::*;
#[cfg(any(
    feature = "duplicate-therapy-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#duplicate_therapy;
#[cfg(any(
    feature = "duplicate-therapy-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#duplicate_therapy::*;
#[cfg(any(
    feature = "duration-property-schema",
    feature = "general-schema-section"
))]
mod r#duration;
#[cfg(any(
    feature = "duration-property-schema",
    feature = "general-schema-section"
))]
pub use r#duration::*;
#[cfg(any(
    feature = "duration-of-warranty-property-schema",
    feature = "general-schema-section"
))]
mod r#duration_of_warranty;
#[cfg(any(
    feature = "duration-of-warranty-property-schema",
    feature = "general-schema-section"
))]
pub use r#duration_of_warranty::*;
#[cfg(any(
    feature = "during-media-property-schema",
    feature = "general-schema-section"
))]
mod r#during_media;
#[cfg(any(
    feature = "during-media-property-schema",
    feature = "general-schema-section"
))]
pub use r#during_media::*;
#[cfg(any(
    feature = "early-prepayment-penalty-property-schema",
    feature = "pending-schema-section"
))]
mod r#early_prepayment_penalty;
#[cfg(any(
    feature = "early-prepayment-penalty-property-schema",
    feature = "pending-schema-section"
))]
pub use r#early_prepayment_penalty::*;
#[cfg(any(
    feature = "edit-eidr-property-schema",
    feature = "pending-schema-section"
))]
mod r#edit_eidr;
#[cfg(any(
    feature = "edit-eidr-property-schema",
    feature = "pending-schema-section"
))]
pub use r#edit_eidr::*;
#[cfg(any(feature = "editor-property-schema", feature = "general-schema-section"))]
mod r#editor;
#[cfg(any(feature = "editor-property-schema", feature = "general-schema-section"))]
pub use r#editor::*;
#[cfg(any(
    feature = "edu-question-type-property-schema",
    feature = "pending-schema-section"
))]
mod r#edu_question_type;
#[cfg(any(
    feature = "edu-question-type-property-schema",
    feature = "pending-schema-section"
))]
pub use r#edu_question_type::*;
#[cfg(any(
    feature = "education-requirements-property-schema",
    feature = "pending-schema-section"
))]
mod r#education_requirements;
#[cfg(any(
    feature = "education-requirements-property-schema",
    feature = "pending-schema-section"
))]
pub use r#education_requirements::*;
#[cfg(any(
    feature = "educational-alignment-property-schema",
    feature = "general-schema-section"
))]
mod r#educational_alignment;
#[cfg(any(
    feature = "educational-alignment-property-schema",
    feature = "general-schema-section"
))]
pub use r#educational_alignment::*;
#[cfg(any(
    feature = "educational-credential-awarded-property-schema",
    feature = "general-schema-section"
))]
mod r#educational_credential_awarded;
#[cfg(any(
    feature = "educational-credential-awarded-property-schema",
    feature = "general-schema-section"
))]
pub use r#educational_credential_awarded::*;
#[cfg(any(
    feature = "educational-framework-property-schema",
    feature = "general-schema-section"
))]
mod r#educational_framework;
#[cfg(any(
    feature = "educational-framework-property-schema",
    feature = "general-schema-section"
))]
pub use r#educational_framework::*;
#[cfg(any(
    feature = "educational-level-property-schema",
    feature = "pending-schema-section"
))]
mod r#educational_level;
#[cfg(any(
    feature = "educational-level-property-schema",
    feature = "pending-schema-section"
))]
pub use r#educational_level::*;
#[cfg(any(
    feature = "educational-program-mode-property-schema",
    feature = "pending-schema-section"
))]
mod r#educational_program_mode;
#[cfg(any(
    feature = "educational-program-mode-property-schema",
    feature = "pending-schema-section"
))]
pub use r#educational_program_mode::*;
#[cfg(any(
    feature = "educational-role-property-schema",
    feature = "general-schema-section"
))]
mod r#educational_role;
#[cfg(any(
    feature = "educational-role-property-schema",
    feature = "general-schema-section"
))]
pub use r#educational_role::*;
#[cfg(any(
    feature = "educational-use-property-schema",
    feature = "general-schema-section"
))]
mod r#educational_use;
#[cfg(any(
    feature = "educational-use-property-schema",
    feature = "general-schema-section"
))]
pub use r#educational_use::*;
#[cfg(any(
    feature = "elevation-property-schema",
    feature = "general-schema-section"
))]
mod r#elevation;
#[cfg(any(
    feature = "elevation-property-schema",
    feature = "general-schema-section"
))]
pub use r#elevation::*;
#[cfg(any(
    feature = "eligibility-to-work-requirement-property-schema",
    feature = "pending-schema-section"
))]
mod r#eligibility_to_work_requirement;
#[cfg(any(
    feature = "eligibility-to-work-requirement-property-schema",
    feature = "pending-schema-section"
))]
pub use r#eligibility_to_work_requirement::*;
#[cfg(any(
    feature = "eligible-customer-type-property-schema",
    feature = "general-schema-section"
))]
mod r#eligible_customer_type;
#[cfg(any(
    feature = "eligible-customer-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#eligible_customer_type::*;
#[cfg(any(
    feature = "eligible-duration-property-schema",
    feature = "general-schema-section"
))]
mod r#eligible_duration;
#[cfg(any(
    feature = "eligible-duration-property-schema",
    feature = "general-schema-section"
))]
pub use r#eligible_duration::*;
#[cfg(any(
    feature = "eligible-quantity-property-schema",
    feature = "general-schema-section"
))]
mod r#eligible_quantity;
#[cfg(any(
    feature = "eligible-quantity-property-schema",
    feature = "general-schema-section"
))]
pub use r#eligible_quantity::*;
#[cfg(any(
    feature = "eligible-region-property-schema",
    feature = "general-schema-section"
))]
mod r#eligible_region;
#[cfg(any(
    feature = "eligible-region-property-schema",
    feature = "general-schema-section"
))]
pub use r#eligible_region::*;
#[cfg(any(
    feature = "eligible-transaction-volume-property-schema",
    feature = "general-schema-section"
))]
mod r#eligible_transaction_volume;
#[cfg(any(
    feature = "eligible-transaction-volume-property-schema",
    feature = "general-schema-section"
))]
pub use r#eligible_transaction_volume::*;
#[cfg(any(feature = "email-property-schema", feature = "general-schema-section"))]
mod r#email;
#[cfg(any(feature = "email-property-schema", feature = "general-schema-section"))]
pub use r#email::*;
#[cfg(any(
    feature = "embed-url-property-schema",
    feature = "general-schema-section"
))]
mod r#embed_url;
#[cfg(any(
    feature = "embed-url-property-schema",
    feature = "general-schema-section"
))]
pub use r#embed_url::*;
#[cfg(any(
    feature = "embedded-text-caption-property-schema",
    feature = "pending-schema-section"
))]
mod r#embedded_text_caption;
#[cfg(any(
    feature = "embedded-text-caption-property-schema",
    feature = "pending-schema-section"
))]
pub use r#embedded_text_caption::*;
#[cfg(any(
    feature = "emissions-co-2-property-schema",
    feature = "auto-schema-section"
))]
mod r#emissions_co_2;
#[cfg(any(
    feature = "emissions-co-2-property-schema",
    feature = "auto-schema-section"
))]
pub use r#emissions_co_2::*;
#[cfg(any(
    feature = "employee-property-schema",
    feature = "general-schema-section"
))]
mod r#employee;
#[cfg(any(
    feature = "employee-property-schema",
    feature = "general-schema-section"
))]
pub use r#employee::*;
#[cfg(any(
    feature = "employees-property-schema",
    feature = "general-schema-section"
))]
mod r#employees;
#[cfg(any(
    feature = "employees-property-schema",
    feature = "general-schema-section"
))]
pub use r#employees::*;
#[cfg(any(
    feature = "employer-overview-property-schema",
    feature = "pending-schema-section"
))]
mod r#employer_overview;
#[cfg(any(
    feature = "employer-overview-property-schema",
    feature = "pending-schema-section"
))]
pub use r#employer_overview::*;
#[cfg(any(
    feature = "employment-type-property-schema",
    feature = "general-schema-section"
))]
mod r#employment_type;
#[cfg(any(
    feature = "employment-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#employment_type::*;
#[cfg(any(
    feature = "employment-unit-property-schema",
    feature = "pending-schema-section"
))]
mod r#employment_unit;
#[cfg(any(
    feature = "employment-unit-property-schema",
    feature = "pending-schema-section"
))]
pub use r#employment_unit::*;
#[cfg(any(
    feature = "encodes-bio-chem-entity-property-schema",
    feature = "pending-schema-section"
))]
mod r#encodes_bio_chem_entity;
#[cfg(any(
    feature = "encodes-bio-chem-entity-property-schema",
    feature = "pending-schema-section"
))]
pub use r#encodes_bio_chem_entity::*;
#[cfg(any(
    feature = "encodes-creative-work-property-schema",
    feature = "general-schema-section"
))]
mod r#encodes_creative_work;
#[cfg(any(
    feature = "encodes-creative-work-property-schema",
    feature = "general-schema-section"
))]
pub use r#encodes_creative_work::*;
#[cfg(any(
    feature = "encoding-property-schema",
    feature = "general-schema-section"
))]
mod r#encoding;
#[cfg(any(
    feature = "encoding-property-schema",
    feature = "general-schema-section"
))]
pub use r#encoding::*;
#[cfg(any(
    feature = "encoding-format-property-schema",
    feature = "general-schema-section"
))]
mod r#encoding_format;
#[cfg(any(
    feature = "encoding-format-property-schema",
    feature = "general-schema-section"
))]
pub use r#encoding_format::*;
#[cfg(any(
    feature = "encoding-type-property-schema",
    feature = "general-schema-section"
))]
mod r#encoding_type;
#[cfg(any(
    feature = "encoding-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#encoding_type::*;
#[cfg(any(
    feature = "encodings-property-schema",
    feature = "general-schema-section"
))]
mod r#encodings;
#[cfg(any(
    feature = "encodings-property-schema",
    feature = "general-schema-section"
))]
pub use r#encodings::*;
#[cfg(any(
    feature = "end-date-property-schema",
    feature = "general-schema-section"
))]
mod r#end_date;
#[cfg(any(
    feature = "end-date-property-schema",
    feature = "general-schema-section"
))]
pub use r#end_date::*;
#[cfg(any(
    feature = "end-offset-property-schema",
    feature = "pending-schema-section"
))]
mod r#end_offset;
#[cfg(any(
    feature = "end-offset-property-schema",
    feature = "pending-schema-section"
))]
pub use r#end_offset::*;
#[cfg(any(
    feature = "end-time-property-schema",
    feature = "general-schema-section"
))]
mod r#end_time;
#[cfg(any(
    feature = "end-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#end_time::*;
#[cfg(any(
    feature = "endorsee-property-schema",
    feature = "general-schema-section"
))]
mod r#endorsee;
#[cfg(any(
    feature = "endorsee-property-schema",
    feature = "general-schema-section"
))]
pub use r#endorsee::*;
#[cfg(any(
    feature = "endorsers-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#endorsers;
#[cfg(any(
    feature = "endorsers-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#endorsers::*;
#[cfg(any(
    feature = "energy-efficiency-scale-max-property-schema",
    feature = "pending-schema-section"
))]
mod r#energy_efficiency_scale_max;
#[cfg(any(
    feature = "energy-efficiency-scale-max-property-schema",
    feature = "pending-schema-section"
))]
pub use r#energy_efficiency_scale_max::*;
#[cfg(any(
    feature = "energy-efficiency-scale-min-property-schema",
    feature = "pending-schema-section"
))]
mod r#energy_efficiency_scale_min;
#[cfg(any(
    feature = "energy-efficiency-scale-min-property-schema",
    feature = "pending-schema-section"
))]
pub use r#energy_efficiency_scale_min::*;
#[cfg(any(
    feature = "engine-displacement-property-schema",
    feature = "auto-schema-section"
))]
mod r#engine_displacement;
#[cfg(any(
    feature = "engine-displacement-property-schema",
    feature = "auto-schema-section"
))]
pub use r#engine_displacement::*;
#[cfg(any(
    feature = "engine-power-property-schema",
    feature = "auto-schema-section"
))]
mod r#engine_power;
#[cfg(any(
    feature = "engine-power-property-schema",
    feature = "auto-schema-section"
))]
pub use r#engine_power::*;
#[cfg(any(
    feature = "engine-type-property-schema",
    feature = "auto-schema-section"
))]
mod r#engine_type;
#[cfg(any(
    feature = "engine-type-property-schema",
    feature = "auto-schema-section"
))]
pub use r#engine_type::*;
#[cfg(any(
    feature = "entertainment-business-property-schema",
    feature = "general-schema-section"
))]
mod r#entertainment_business;
#[cfg(any(
    feature = "entertainment-business-property-schema",
    feature = "general-schema-section"
))]
pub use r#entertainment_business::*;
#[cfg(any(
    feature = "epidemiology-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#epidemiology;
#[cfg(any(
    feature = "epidemiology-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#epidemiology::*;
#[cfg(any(
    feature = "episode-property-schema",
    feature = "general-schema-section"
))]
mod r#episode;
#[cfg(any(
    feature = "episode-property-schema",
    feature = "general-schema-section"
))]
pub use r#episode::*;
#[cfg(any(
    feature = "episode-number-property-schema",
    feature = "general-schema-section"
))]
mod r#episode_number;
#[cfg(any(
    feature = "episode-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#episode_number::*;
#[cfg(any(
    feature = "episodes-property-schema",
    feature = "general-schema-section"
))]
mod r#episodes;
#[cfg(any(
    feature = "episodes-property-schema",
    feature = "general-schema-section"
))]
pub use r#episodes::*;
#[cfg(any(feature = "equal-property-schema", feature = "general-schema-section"))]
mod r#equal;
#[cfg(any(feature = "equal-property-schema", feature = "general-schema-section"))]
pub use r#equal::*;
#[cfg(any(feature = "error-property-schema", feature = "general-schema-section"))]
mod r#error;
#[cfg(any(feature = "error-property-schema", feature = "general-schema-section"))]
pub use r#error::*;
#[cfg(any(
    feature = "estimated-cost-property-schema",
    feature = "general-schema-section"
))]
mod r#estimated_cost;
#[cfg(any(
    feature = "estimated-cost-property-schema",
    feature = "general-schema-section"
))]
pub use r#estimated_cost::*;
#[cfg(any(
    feature = "estimated-flight-duration-property-schema",
    feature = "general-schema-section"
))]
mod r#estimated_flight_duration;
#[cfg(any(
    feature = "estimated-flight-duration-property-schema",
    feature = "general-schema-section"
))]
pub use r#estimated_flight_duration::*;
#[cfg(any(
    feature = "estimated-salary-property-schema",
    feature = "general-schema-section"
))]
mod r#estimated_salary;
#[cfg(any(
    feature = "estimated-salary-property-schema",
    feature = "general-schema-section"
))]
pub use r#estimated_salary::*;
#[cfg(any(
    feature = "estimates-risk-of-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#estimates_risk_of;
#[cfg(any(
    feature = "estimates-risk-of-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#estimates_risk_of::*;
#[cfg(any(
    feature = "ethics-policy-property-schema",
    feature = "pending-schema-section"
))]
mod r#ethics_policy;
#[cfg(any(
    feature = "ethics-policy-property-schema",
    feature = "pending-schema-section"
))]
pub use r#ethics_policy::*;
#[cfg(any(feature = "event-property-schema", feature = "general-schema-section"))]
mod r#event;
#[cfg(any(feature = "event-property-schema", feature = "general-schema-section"))]
pub use r#event::*;
#[cfg(any(
    feature = "event-attendance-mode-property-schema",
    feature = "pending-schema-section"
))]
mod r#event_attendance_mode;
#[cfg(any(
    feature = "event-attendance-mode-property-schema",
    feature = "pending-schema-section"
))]
pub use r#event_attendance_mode::*;
#[cfg(any(
    feature = "event-schedule-property-schema",
    feature = "pending-schema-section"
))]
mod r#event_schedule;
#[cfg(any(
    feature = "event-schedule-property-schema",
    feature = "pending-schema-section"
))]
pub use r#event_schedule::*;
#[cfg(any(
    feature = "event-status-property-schema",
    feature = "general-schema-section"
))]
mod r#event_status;
#[cfg(any(
    feature = "event-status-property-schema",
    feature = "general-schema-section"
))]
pub use r#event_status::*;
#[cfg(any(feature = "events-property-schema", feature = "general-schema-section"))]
mod r#events;
#[cfg(any(feature = "events-property-schema", feature = "general-schema-section"))]
pub use r#events::*;
#[cfg(any(
    feature = "evidence-level-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#evidence_level;
#[cfg(any(
    feature = "evidence-level-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#evidence_level::*;
#[cfg(any(
    feature = "evidence-origin-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#evidence_origin;
#[cfg(any(
    feature = "evidence-origin-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#evidence_origin::*;
#[cfg(any(
    feature = "example-of-work-property-schema",
    feature = "general-schema-section"
))]
mod r#example_of_work;
#[cfg(any(
    feature = "example-of-work-property-schema",
    feature = "general-schema-section"
))]
pub use r#example_of_work::*;
#[cfg(any(
    feature = "except-date-property-schema",
    feature = "pending-schema-section"
))]
mod r#except_date;
#[cfg(any(
    feature = "except-date-property-schema",
    feature = "pending-schema-section"
))]
pub use r#except_date::*;
#[cfg(any(
    feature = "exchange-rate-spread-property-schema",
    feature = "pending-schema-section"
))]
mod r#exchange_rate_spread;
#[cfg(any(
    feature = "exchange-rate-spread-property-schema",
    feature = "pending-schema-section"
))]
pub use r#exchange_rate_spread::*;
#[cfg(any(
    feature = "executable-library-name-property-schema",
    feature = "general-schema-section"
))]
mod r#executable_library_name;
#[cfg(any(
    feature = "executable-library-name-property-schema",
    feature = "general-schema-section"
))]
pub use r#executable_library_name::*;
#[cfg(any(
    feature = "exercise-course-property-schema",
    feature = "general-schema-section"
))]
mod r#exercise_course;
#[cfg(any(
    feature = "exercise-course-property-schema",
    feature = "general-schema-section"
))]
pub use r#exercise_course::*;
#[cfg(any(
    feature = "exercise-plan-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#exercise_plan;
#[cfg(any(
    feature = "exercise-plan-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#exercise_plan::*;
#[cfg(any(
    feature = "exercise-related-diet-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#exercise_related_diet;
#[cfg(any(
    feature = "exercise-related-diet-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#exercise_related_diet::*;
#[cfg(any(
    feature = "exercise-type-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#exercise_type;
#[cfg(any(
    feature = "exercise-type-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#exercise_type::*;
#[cfg(any(
    feature = "exif-data-property-schema",
    feature = "general-schema-section"
))]
mod r#exif_data;
#[cfg(any(
    feature = "exif-data-property-schema",
    feature = "general-schema-section"
))]
pub use r#exif_data::*;
#[cfg(any(
    feature = "expected-arrival-from-property-schema",
    feature = "general-schema-section"
))]
mod r#expected_arrival_from;
#[cfg(any(
    feature = "expected-arrival-from-property-schema",
    feature = "general-schema-section"
))]
pub use r#expected_arrival_from::*;
#[cfg(any(
    feature = "expected-arrival-until-property-schema",
    feature = "general-schema-section"
))]
mod r#expected_arrival_until;
#[cfg(any(
    feature = "expected-arrival-until-property-schema",
    feature = "general-schema-section"
))]
pub use r#expected_arrival_until::*;
#[cfg(any(
    feature = "expected-prognosis-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#expected_prognosis;
#[cfg(any(
    feature = "expected-prognosis-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#expected_prognosis::*;
#[cfg(any(
    feature = "expects-acceptance-of-property-schema",
    feature = "general-schema-section"
))]
mod r#expects_acceptance_of;
#[cfg(any(
    feature = "expects-acceptance-of-property-schema",
    feature = "general-schema-section"
))]
pub use r#expects_acceptance_of::*;
#[cfg(any(
    feature = "experience-in-place-of-education-property-schema",
    feature = "pending-schema-section"
))]
mod r#experience_in_place_of_education;
#[cfg(any(
    feature = "experience-in-place-of-education-property-schema",
    feature = "pending-schema-section"
))]
pub use r#experience_in_place_of_education::*;
#[cfg(any(
    feature = "experience-requirements-property-schema",
    feature = "general-schema-section"
))]
mod r#experience_requirements;
#[cfg(any(
    feature = "experience-requirements-property-schema",
    feature = "general-schema-section"
))]
pub use r#experience_requirements::*;
#[cfg(any(
    feature = "expert-considerations-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#expert_considerations;
#[cfg(any(
    feature = "expert-considerations-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#expert_considerations::*;
#[cfg(any(
    feature = "expires-property-schema",
    feature = "general-schema-section"
))]
mod r#expires;
#[cfg(any(
    feature = "expires-property-schema",
    feature = "general-schema-section"
))]
pub use r#expires::*;
#[cfg(any(
    feature = "expressed-in-property-schema",
    feature = "pending-schema-section"
))]
mod r#expressed_in;
#[cfg(any(
    feature = "expressed-in-property-schema",
    feature = "pending-schema-section"
))]
pub use r#expressed_in::*;
#[cfg(any(
    feature = "family-name-property-schema",
    feature = "general-schema-section"
))]
mod r#family_name;
#[cfg(any(
    feature = "family-name-property-schema",
    feature = "general-schema-section"
))]
pub use r#family_name::*;
#[cfg(any(
    feature = "fat-content-property-schema",
    feature = "general-schema-section"
))]
mod r#fat_content;
#[cfg(any(
    feature = "fat-content-property-schema",
    feature = "general-schema-section"
))]
pub use r#fat_content::*;
#[cfg(any(
    feature = "fax-number-property-schema",
    feature = "general-schema-section"
))]
mod r#fax_number;
#[cfg(any(
    feature = "fax-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#fax_number::*;
#[cfg(any(
    feature = "feature-list-property-schema",
    feature = "general-schema-section"
))]
mod r#feature_list;
#[cfg(any(
    feature = "feature-list-property-schema",
    feature = "general-schema-section"
))]
pub use r#feature_list::*;
#[cfg(any(
    feature = "fees-and-commissions-specification-property-schema",
    feature = "general-schema-section"
))]
mod r#fees_and_commissions_specification;
#[cfg(any(
    feature = "fees-and-commissions-specification-property-schema",
    feature = "general-schema-section"
))]
pub use r#fees_and_commissions_specification::*;
#[cfg(any(
    feature = "fiber-content-property-schema",
    feature = "general-schema-section"
))]
mod r#fiber_content;
#[cfg(any(
    feature = "fiber-content-property-schema",
    feature = "general-schema-section"
))]
pub use r#fiber_content::*;
#[cfg(any(
    feature = "file-format-property-schema",
    feature = "general-schema-section"
))]
mod r#file_format;
#[cfg(any(
    feature = "file-format-property-schema",
    feature = "general-schema-section"
))]
pub use r#file_format::*;
#[cfg(any(
    feature = "file-size-property-schema",
    feature = "general-schema-section"
))]
mod r#file_size;
#[cfg(any(
    feature = "file-size-property-schema",
    feature = "general-schema-section"
))]
pub use r#file_size::*;
#[cfg(any(
    feature = "financial-aid-eligible-property-schema",
    feature = "pending-schema-section"
))]
mod r#financial_aid_eligible;
#[cfg(any(
    feature = "financial-aid-eligible-property-schema",
    feature = "pending-schema-section"
))]
pub use r#financial_aid_eligible::*;
#[cfg(any(
    feature = "first-appearance-property-schema",
    feature = "pending-schema-section"
))]
mod r#first_appearance;
#[cfg(any(
    feature = "first-appearance-property-schema",
    feature = "pending-schema-section"
))]
pub use r#first_appearance::*;
#[cfg(any(
    feature = "first-performance-property-schema",
    feature = "general-schema-section"
))]
mod r#first_performance;
#[cfg(any(
    feature = "first-performance-property-schema",
    feature = "general-schema-section"
))]
pub use r#first_performance::*;
#[cfg(any(
    feature = "flight-distance-property-schema",
    feature = "general-schema-section"
))]
mod r#flight_distance;
#[cfg(any(
    feature = "flight-distance-property-schema",
    feature = "general-schema-section"
))]
pub use r#flight_distance::*;
#[cfg(any(
    feature = "flight-number-property-schema",
    feature = "general-schema-section"
))]
mod r#flight_number;
#[cfg(any(
    feature = "flight-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#flight_number::*;
#[cfg(any(
    feature = "floor-level-property-schema",
    feature = "pending-schema-section"
))]
mod r#floor_level;
#[cfg(any(
    feature = "floor-level-property-schema",
    feature = "pending-schema-section"
))]
pub use r#floor_level::*;
#[cfg(any(
    feature = "floor-limit-property-schema",
    feature = "pending-schema-section"
))]
mod r#floor_limit;
#[cfg(any(
    feature = "floor-limit-property-schema",
    feature = "pending-schema-section"
))]
pub use r#floor_limit::*;
#[cfg(any(
    feature = "floor-size-property-schema",
    feature = "general-schema-section"
))]
mod r#floor_size;
#[cfg(any(
    feature = "floor-size-property-schema",
    feature = "general-schema-section"
))]
pub use r#floor_size::*;
#[cfg(any(
    feature = "followee-property-schema",
    feature = "general-schema-section"
))]
mod r#followee;
#[cfg(any(
    feature = "followee-property-schema",
    feature = "general-schema-section"
))]
pub use r#followee::*;
#[cfg(any(
    feature = "follows-property-schema",
    feature = "general-schema-section"
))]
mod r#follows;
#[cfg(any(
    feature = "follows-property-schema",
    feature = "general-schema-section"
))]
pub use r#follows::*;
#[cfg(any(
    feature = "followup-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#followup;
#[cfg(any(
    feature = "followup-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#followup::*;
#[cfg(any(
    feature = "food-establishment-property-schema",
    feature = "general-schema-section"
))]
mod r#food_establishment;
#[cfg(any(
    feature = "food-establishment-property-schema",
    feature = "general-schema-section"
))]
pub use r#food_establishment::*;
#[cfg(any(
    feature = "food-event-property-schema",
    feature = "general-schema-section"
))]
mod r#food_event;
#[cfg(any(
    feature = "food-event-property-schema",
    feature = "general-schema-section"
))]
pub use r#food_event::*;
#[cfg(any(
    feature = "food-warning-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#food_warning;
#[cfg(any(
    feature = "food-warning-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#food_warning::*;
#[cfg(any(
    feature = "founder-property-schema",
    feature = "general-schema-section"
))]
mod r#founder;
#[cfg(any(
    feature = "founder-property-schema",
    feature = "general-schema-section"
))]
pub use r#founder::*;
#[cfg(any(
    feature = "founders-property-schema",
    feature = "general-schema-section"
))]
mod r#founders;
#[cfg(any(
    feature = "founders-property-schema",
    feature = "general-schema-section"
))]
pub use r#founders::*;
#[cfg(any(
    feature = "founding-date-property-schema",
    feature = "general-schema-section"
))]
mod r#founding_date;
#[cfg(any(
    feature = "founding-date-property-schema",
    feature = "general-schema-section"
))]
pub use r#founding_date::*;
#[cfg(any(
    feature = "founding-location-property-schema",
    feature = "general-schema-section"
))]
mod r#founding_location;
#[cfg(any(
    feature = "founding-location-property-schema",
    feature = "general-schema-section"
))]
pub use r#founding_location::*;
#[cfg(any(feature = "free-property-schema", feature = "general-schema-section"))]
mod r#free;
#[cfg(any(feature = "free-property-schema", feature = "general-schema-section"))]
pub use r#free::*;
#[cfg(any(
    feature = "free-shipping-threshold-property-schema",
    feature = "pending-schema-section"
))]
mod r#free_shipping_threshold;
#[cfg(any(
    feature = "free-shipping-threshold-property-schema",
    feature = "pending-schema-section"
))]
pub use r#free_shipping_threshold::*;
#[cfg(any(
    feature = "frequency-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#frequency;
#[cfg(any(
    feature = "frequency-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#frequency::*;
#[cfg(any(
    feature = "from-location-property-schema",
    feature = "general-schema-section"
))]
mod r#from_location;
#[cfg(any(
    feature = "from-location-property-schema",
    feature = "general-schema-section"
))]
pub use r#from_location::*;
#[cfg(any(
    feature = "fuel-capacity-property-schema",
    feature = "auto-schema-section"
))]
mod r#fuel_capacity;
#[cfg(any(
    feature = "fuel-capacity-property-schema",
    feature = "auto-schema-section"
))]
pub use r#fuel_capacity::*;
#[cfg(any(
    feature = "fuel-consumption-property-schema",
    feature = "general-schema-section"
))]
mod r#fuel_consumption;
#[cfg(any(
    feature = "fuel-consumption-property-schema",
    feature = "general-schema-section"
))]
pub use r#fuel_consumption::*;
#[cfg(any(
    feature = "fuel-efficiency-property-schema",
    feature = "general-schema-section"
))]
mod r#fuel_efficiency;
#[cfg(any(
    feature = "fuel-efficiency-property-schema",
    feature = "general-schema-section"
))]
pub use r#fuel_efficiency::*;
#[cfg(any(
    feature = "fuel-type-property-schema",
    feature = "general-schema-section"
))]
mod r#fuel_type;
#[cfg(any(
    feature = "fuel-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#fuel_type::*;
#[cfg(any(
    feature = "functional-class-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#functional_class;
#[cfg(any(
    feature = "functional-class-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#functional_class::*;
#[cfg(any(
    feature = "funded-item-property-schema",
    feature = "pending-schema-section"
))]
mod r#funded_item;
#[cfg(any(
    feature = "funded-item-property-schema",
    feature = "pending-schema-section"
))]
pub use r#funded_item::*;
#[cfg(any(feature = "funder-property-schema", feature = "general-schema-section"))]
mod r#funder;
#[cfg(any(feature = "funder-property-schema", feature = "general-schema-section"))]
pub use r#funder::*;
#[cfg(any(
    feature = "funding-property-schema",
    feature = "pending-schema-section"
))]
mod r#funding;
#[cfg(any(
    feature = "funding-property-schema",
    feature = "pending-schema-section"
))]
pub use r#funding::*;
#[cfg(any(feature = "game-property-schema", feature = "general-schema-section"))]
mod r#game;
#[cfg(any(feature = "game-property-schema", feature = "general-schema-section"))]
pub use r#game::*;
#[cfg(any(
    feature = "game-availability-type-property-schema",
    feature = "pending-schema-section"
))]
mod r#game_availability_type;
#[cfg(any(
    feature = "game-availability-type-property-schema",
    feature = "pending-schema-section"
))]
pub use r#game_availability_type::*;
#[cfg(any(
    feature = "game-edition-property-schema",
    feature = "general-schema-section"
))]
mod r#game_edition;
#[cfg(any(
    feature = "game-edition-property-schema",
    feature = "general-schema-section"
))]
pub use r#game_edition::*;
#[cfg(any(
    feature = "game-item-property-schema",
    feature = "general-schema-section"
))]
mod r#game_item;
#[cfg(any(
    feature = "game-item-property-schema",
    feature = "general-schema-section"
))]
pub use r#game_item::*;
#[cfg(any(
    feature = "game-location-property-schema",
    feature = "general-schema-section"
))]
mod r#game_location;
#[cfg(any(
    feature = "game-location-property-schema",
    feature = "general-schema-section"
))]
pub use r#game_location::*;
#[cfg(any(
    feature = "game-platform-property-schema",
    feature = "general-schema-section"
))]
mod r#game_platform;
#[cfg(any(
    feature = "game-platform-property-schema",
    feature = "general-schema-section"
))]
pub use r#game_platform::*;
#[cfg(any(
    feature = "game-server-property-schema",
    feature = "general-schema-section"
))]
mod r#game_server;
#[cfg(any(
    feature = "game-server-property-schema",
    feature = "general-schema-section"
))]
pub use r#game_server::*;
#[cfg(any(
    feature = "game-tip-property-schema",
    feature = "general-schema-section"
))]
mod r#game_tip;
#[cfg(any(
    feature = "game-tip-property-schema",
    feature = "general-schema-section"
))]
pub use r#game_tip::*;
#[cfg(any(feature = "gender-property-schema", feature = "pending-schema-section"))]
mod r#gender;
#[cfg(any(feature = "gender-property-schema", feature = "pending-schema-section"))]
pub use r#gender::*;
#[cfg(any(feature = "genre-property-schema", feature = "general-schema-section"))]
mod r#genre;
#[cfg(any(feature = "genre-property-schema", feature = "general-schema-section"))]
pub use r#genre::*;
#[cfg(any(feature = "geo-property-schema", feature = "general-schema-section"))]
mod r#geo;
#[cfg(any(feature = "geo-property-schema", feature = "general-schema-section"))]
pub use r#geo::*;
#[cfg(any(
    feature = "geo-contains-property-schema",
    feature = "general-schema-section"
))]
mod r#geo_contains;
#[cfg(any(
    feature = "geo-contains-property-schema",
    feature = "general-schema-section"
))]
pub use r#geo_contains::*;
#[cfg(any(
    feature = "geo-covered-by-property-schema",
    feature = "general-schema-section"
))]
mod r#geo_covered_by;
#[cfg(any(
    feature = "geo-covered-by-property-schema",
    feature = "general-schema-section"
))]
pub use r#geo_covered_by::*;
#[cfg(any(
    feature = "geo-covers-property-schema",
    feature = "general-schema-section"
))]
mod r#geo_covers;
#[cfg(any(
    feature = "geo-covers-property-schema",
    feature = "general-schema-section"
))]
pub use r#geo_covers::*;
#[cfg(any(
    feature = "geo-crosses-property-schema",
    feature = "general-schema-section"
))]
mod r#geo_crosses;
#[cfg(any(
    feature = "geo-crosses-property-schema",
    feature = "general-schema-section"
))]
pub use r#geo_crosses::*;
#[cfg(any(
    feature = "geo-disjoint-property-schema",
    feature = "general-schema-section"
))]
mod r#geo_disjoint;
#[cfg(any(
    feature = "geo-disjoint-property-schema",
    feature = "general-schema-section"
))]
pub use r#geo_disjoint::*;
#[cfg(any(
    feature = "geo-equals-property-schema",
    feature = "general-schema-section"
))]
mod r#geo_equals;
#[cfg(any(
    feature = "geo-equals-property-schema",
    feature = "general-schema-section"
))]
pub use r#geo_equals::*;
#[cfg(any(
    feature = "geo-intersects-property-schema",
    feature = "general-schema-section"
))]
mod r#geo_intersects;
#[cfg(any(
    feature = "geo-intersects-property-schema",
    feature = "general-schema-section"
))]
pub use r#geo_intersects::*;
#[cfg(any(
    feature = "geo-midpoint-property-schema",
    feature = "general-schema-section"
))]
mod r#geo_midpoint;
#[cfg(any(
    feature = "geo-midpoint-property-schema",
    feature = "general-schema-section"
))]
pub use r#geo_midpoint::*;
#[cfg(any(
    feature = "geo-overlaps-property-schema",
    feature = "general-schema-section"
))]
mod r#geo_overlaps;
#[cfg(any(
    feature = "geo-overlaps-property-schema",
    feature = "general-schema-section"
))]
pub use r#geo_overlaps::*;
#[cfg(any(
    feature = "geo-radius-property-schema",
    feature = "general-schema-section"
))]
mod r#geo_radius;
#[cfg(any(
    feature = "geo-radius-property-schema",
    feature = "general-schema-section"
))]
pub use r#geo_radius::*;
#[cfg(any(
    feature = "geo-touches-property-schema",
    feature = "general-schema-section"
))]
mod r#geo_touches;
#[cfg(any(
    feature = "geo-touches-property-schema",
    feature = "general-schema-section"
))]
pub use r#geo_touches::*;
#[cfg(any(
    feature = "geo-within-property-schema",
    feature = "general-schema-section"
))]
mod r#geo_within;
#[cfg(any(
    feature = "geo-within-property-schema",
    feature = "general-schema-section"
))]
pub use r#geo_within::*;
#[cfg(any(
    feature = "geographic-area-property-schema",
    feature = "general-schema-section"
))]
mod r#geographic_area;
#[cfg(any(
    feature = "geographic-area-property-schema",
    feature = "general-schema-section"
))]
pub use r#geographic_area::*;
#[cfg(any(
    feature = "getting-tested-info-property-schema",
    feature = "pending-schema-section"
))]
mod r#getting_tested_info;
#[cfg(any(
    feature = "getting-tested-info-property-schema",
    feature = "pending-schema-section"
))]
pub use r#getting_tested_info::*;
#[cfg(any(
    feature = "given-name-property-schema",
    feature = "general-schema-section"
))]
mod r#given_name;
#[cfg(any(
    feature = "given-name-property-schema",
    feature = "general-schema-section"
))]
pub use r#given_name::*;
#[cfg(any(
    feature = "global-location-number-property-schema",
    feature = "general-schema-section"
))]
mod r#global_location_number;
#[cfg(any(
    feature = "global-location-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#global_location_number::*;
#[cfg(any(
    feature = "government-benefits-info-property-schema",
    feature = "pending-schema-section"
))]
mod r#government_benefits_info;
#[cfg(any(
    feature = "government-benefits-info-property-schema",
    feature = "pending-schema-section"
))]
pub use r#government_benefits_info::*;
#[cfg(any(
    feature = "grace-period-property-schema",
    feature = "pending-schema-section"
))]
mod r#grace_period;
#[cfg(any(
    feature = "grace-period-property-schema",
    feature = "pending-schema-section"
))]
pub use r#grace_period::*;
#[cfg(any(
    feature = "grantee-property-schema",
    feature = "general-schema-section"
))]
mod r#grantee;
#[cfg(any(
    feature = "grantee-property-schema",
    feature = "general-schema-section"
))]
pub use r#grantee::*;
#[cfg(any(
    feature = "greater-property-schema",
    feature = "general-schema-section"
))]
mod r#greater;
#[cfg(any(
    feature = "greater-property-schema",
    feature = "general-schema-section"
))]
pub use r#greater::*;
#[cfg(any(
    feature = "greater-or-equal-property-schema",
    feature = "general-schema-section"
))]
mod r#greater_or_equal;
#[cfg(any(
    feature = "greater-or-equal-property-schema",
    feature = "general-schema-section"
))]
pub use r#greater_or_equal::*;
#[cfg(any(feature = "gtin-property-schema", feature = "pending-schema-section"))]
mod r#gtin;
#[cfg(any(feature = "gtin-property-schema", feature = "pending-schema-section"))]
pub use r#gtin::*;
#[cfg(any(
    feature = "gtin-12-property-schema",
    feature = "general-schema-section"
))]
mod r#gtin_12;
#[cfg(any(
    feature = "gtin-12-property-schema",
    feature = "general-schema-section"
))]
pub use r#gtin_12::*;
#[cfg(any(
    feature = "gtin-13-property-schema",
    feature = "general-schema-section"
))]
mod r#gtin_13;
#[cfg(any(
    feature = "gtin-13-property-schema",
    feature = "general-schema-section"
))]
pub use r#gtin_13::*;
#[cfg(any(
    feature = "gtin-14-property-schema",
    feature = "general-schema-section"
))]
mod r#gtin_14;
#[cfg(any(
    feature = "gtin-14-property-schema",
    feature = "general-schema-section"
))]
pub use r#gtin_14::*;
#[cfg(any(feature = "gtin-8-property-schema", feature = "general-schema-section"))]
mod r#gtin_8;
#[cfg(any(feature = "gtin-8-property-schema", feature = "general-schema-section"))]
pub use r#gtin_8::*;
#[cfg(any(
    feature = "guideline-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#guideline;
#[cfg(any(
    feature = "guideline-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#guideline::*;
#[cfg(any(
    feature = "guideline-date-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#guideline_date;
#[cfg(any(
    feature = "guideline-date-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#guideline_date::*;
#[cfg(any(
    feature = "guideline-subject-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#guideline_subject;
#[cfg(any(
    feature = "guideline-subject-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#guideline_subject::*;
#[cfg(any(
    feature = "handling-time-property-schema",
    feature = "pending-schema-section"
))]
mod r#handling_time;
#[cfg(any(
    feature = "handling-time-property-schema",
    feature = "pending-schema-section"
))]
pub use r#handling_time::*;
#[cfg(any(
    feature = "has-adult-consideration-property-schema",
    feature = "pending-schema-section"
))]
mod r#has_adult_consideration;
#[cfg(any(
    feature = "has-adult-consideration-property-schema",
    feature = "pending-schema-section"
))]
pub use r#has_adult_consideration::*;
#[cfg(any(
    feature = "has-bio-chem-entity-part-property-schema",
    feature = "pending-schema-section"
))]
mod r#has_bio_chem_entity_part;
#[cfg(any(
    feature = "has-bio-chem-entity-part-property-schema",
    feature = "pending-schema-section"
))]
pub use r#has_bio_chem_entity_part::*;
#[cfg(any(
    feature = "has-bio-polymer-sequence-property-schema",
    feature = "pending-schema-section"
))]
mod r#has_bio_polymer_sequence;
#[cfg(any(
    feature = "has-bio-polymer-sequence-property-schema",
    feature = "pending-schema-section"
))]
pub use r#has_bio_polymer_sequence::*;
#[cfg(any(
    feature = "has-broadcast-channel-property-schema",
    feature = "general-schema-section"
))]
mod r#has_broadcast_channel;
#[cfg(any(
    feature = "has-broadcast-channel-property-schema",
    feature = "general-schema-section"
))]
pub use r#has_broadcast_channel::*;
#[cfg(any(
    feature = "has-category-code-property-schema",
    feature = "pending-schema-section"
))]
mod r#has_category_code;
#[cfg(any(
    feature = "has-category-code-property-schema",
    feature = "pending-schema-section"
))]
pub use r#has_category_code::*;
#[cfg(any(
    feature = "has-course-property-schema",
    feature = "pending-schema-section"
))]
mod r#has_course;
#[cfg(any(
    feature = "has-course-property-schema",
    feature = "pending-schema-section"
))]
pub use r#has_course::*;
#[cfg(any(
    feature = "has-course-instance-property-schema",
    feature = "general-schema-section"
))]
mod r#has_course_instance;
#[cfg(any(
    feature = "has-course-instance-property-schema",
    feature = "general-schema-section"
))]
pub use r#has_course_instance::*;
#[cfg(any(
    feature = "has-credential-property-schema",
    feature = "pending-schema-section"
))]
mod r#has_credential;
#[cfg(any(
    feature = "has-credential-property-schema",
    feature = "pending-schema-section"
))]
pub use r#has_credential::*;
#[cfg(any(
    feature = "has-defined-term-property-schema",
    feature = "pending-schema-section"
))]
mod r#has_defined_term;
#[cfg(any(
    feature = "has-defined-term-property-schema",
    feature = "pending-schema-section"
))]
pub use r#has_defined_term::*;
#[cfg(any(
    feature = "has-delivery-method-property-schema",
    feature = "general-schema-section"
))]
mod r#has_delivery_method;
#[cfg(any(
    feature = "has-delivery-method-property-schema",
    feature = "general-schema-section"
))]
pub use r#has_delivery_method::*;
#[cfg(any(
    feature = "has-digital-document-permission-property-schema",
    feature = "general-schema-section"
))]
mod r#has_digital_document_permission;
#[cfg(any(
    feature = "has-digital-document-permission-property-schema",
    feature = "general-schema-section"
))]
pub use r#has_digital_document_permission::*;
#[cfg(any(
    feature = "has-drive-through-service-property-schema",
    feature = "pending-schema-section"
))]
mod r#has_drive_through_service;
#[cfg(any(
    feature = "has-drive-through-service-property-schema",
    feature = "pending-schema-section"
))]
pub use r#has_drive_through_service::*;
#[cfg(any(
    feature = "has-energy-consumption-details-property-schema",
    feature = "pending-schema-section"
))]
mod r#has_energy_consumption_details;
#[cfg(any(
    feature = "has-energy-consumption-details-property-schema",
    feature = "pending-schema-section"
))]
pub use r#has_energy_consumption_details::*;
#[cfg(any(
    feature = "has-energy-efficiency-category-property-schema",
    feature = "pending-schema-section"
))]
mod r#has_energy_efficiency_category;
#[cfg(any(
    feature = "has-energy-efficiency-category-property-schema",
    feature = "pending-schema-section"
))]
pub use r#has_energy_efficiency_category::*;
#[cfg(any(
    feature = "has-health-aspect-property-schema",
    feature = "pending-schema-section"
))]
mod r#has_health_aspect;
#[cfg(any(
    feature = "has-health-aspect-property-schema",
    feature = "pending-schema-section"
))]
pub use r#has_health_aspect::*;
#[cfg(any(
    feature = "has-map-property-schema",
    feature = "general-schema-section"
))]
mod r#has_map;
#[cfg(any(
    feature = "has-map-property-schema",
    feature = "general-schema-section"
))]
pub use r#has_map::*;
#[cfg(any(
    feature = "has-measurement-property-schema",
    feature = "pending-schema-section"
))]
mod r#has_measurement;
#[cfg(any(
    feature = "has-measurement-property-schema",
    feature = "pending-schema-section"
))]
pub use r#has_measurement::*;
#[cfg(any(
    feature = "has-menu-property-schema",
    feature = "general-schema-section"
))]
mod r#has_menu;
#[cfg(any(
    feature = "has-menu-property-schema",
    feature = "general-schema-section"
))]
pub use r#has_menu::*;
#[cfg(any(
    feature = "has-menu-item-property-schema",
    feature = "general-schema-section"
))]
mod r#has_menu_item;
#[cfg(any(
    feature = "has-menu-item-property-schema",
    feature = "general-schema-section"
))]
pub use r#has_menu_item::*;
#[cfg(any(
    feature = "has-menu-section-property-schema",
    feature = "general-schema-section"
))]
mod r#has_menu_section;
#[cfg(any(
    feature = "has-menu-section-property-schema",
    feature = "general-schema-section"
))]
pub use r#has_menu_section::*;
#[cfg(any(
    feature = "has-merchant-return-policy-property-schema",
    feature = "pending-schema-section"
))]
mod r#has_merchant_return_policy;
#[cfg(any(
    feature = "has-merchant-return-policy-property-schema",
    feature = "pending-schema-section"
))]
pub use r#has_merchant_return_policy::*;
#[cfg(any(
    feature = "has-molecular-function-property-schema",
    feature = "pending-schema-section"
))]
mod r#has_molecular_function;
#[cfg(any(
    feature = "has-molecular-function-property-schema",
    feature = "pending-schema-section"
))]
pub use r#has_molecular_function::*;
#[cfg(any(
    feature = "has-occupation-property-schema",
    feature = "general-schema-section"
))]
mod r#has_occupation;
#[cfg(any(
    feature = "has-occupation-property-schema",
    feature = "general-schema-section"
))]
pub use r#has_occupation::*;
#[cfg(any(
    feature = "has-offer-catalog-property-schema",
    feature = "general-schema-section"
))]
mod r#has_offer_catalog;
#[cfg(any(
    feature = "has-offer-catalog-property-schema",
    feature = "general-schema-section"
))]
pub use r#has_offer_catalog::*;
#[cfg(any(
    feature = "has-pos-property-schema",
    feature = "general-schema-section"
))]
mod r#has_pos;
#[cfg(any(
    feature = "has-pos-property-schema",
    feature = "general-schema-section"
))]
pub use r#has_pos::*;
#[cfg(any(
    feature = "has-part-property-schema",
    feature = "general-schema-section"
))]
mod r#has_part;
#[cfg(any(
    feature = "has-part-property-schema",
    feature = "general-schema-section"
))]
pub use r#has_part::*;
#[cfg(any(
    feature = "has-product-return-policy-property-schema",
    feature = "attic-schema-section"
))]
mod r#has_product_return_policy;
#[cfg(any(
    feature = "has-product-return-policy-property-schema",
    feature = "attic-schema-section"
))]
pub use r#has_product_return_policy::*;
#[cfg(any(
    feature = "has-representation-property-schema",
    feature = "pending-schema-section"
))]
mod r#has_representation;
#[cfg(any(
    feature = "has-representation-property-schema",
    feature = "pending-schema-section"
))]
pub use r#has_representation::*;
#[cfg(any(
    feature = "has-variant-property-schema",
    feature = "pending-schema-section"
))]
mod r#has_variant;
#[cfg(any(
    feature = "has-variant-property-schema",
    feature = "pending-schema-section"
))]
pub use r#has_variant::*;
#[cfg(any(
    feature = "headline-property-schema",
    feature = "general-schema-section"
))]
mod r#headline;
#[cfg(any(
    feature = "headline-property-schema",
    feature = "general-schema-section"
))]
pub use r#headline::*;
#[cfg(any(
    feature = "health-condition-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#health_condition;
#[cfg(any(
    feature = "health-condition-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#health_condition::*;
#[cfg(any(
    feature = "health-plan-coinsurance-option-property-schema",
    feature = "pending-schema-section"
))]
mod r#health_plan_coinsurance_option;
#[cfg(any(
    feature = "health-plan-coinsurance-option-property-schema",
    feature = "pending-schema-section"
))]
pub use r#health_plan_coinsurance_option::*;
#[cfg(any(
    feature = "health-plan-coinsurance-rate-property-schema",
    feature = "pending-schema-section"
))]
mod r#health_plan_coinsurance_rate;
#[cfg(any(
    feature = "health-plan-coinsurance-rate-property-schema",
    feature = "pending-schema-section"
))]
pub use r#health_plan_coinsurance_rate::*;
#[cfg(any(
    feature = "health-plan-copay-property-schema",
    feature = "pending-schema-section"
))]
mod r#health_plan_copay;
#[cfg(any(
    feature = "health-plan-copay-property-schema",
    feature = "pending-schema-section"
))]
pub use r#health_plan_copay::*;
#[cfg(any(
    feature = "health-plan-copay-option-property-schema",
    feature = "pending-schema-section"
))]
mod r#health_plan_copay_option;
#[cfg(any(
    feature = "health-plan-copay-option-property-schema",
    feature = "pending-schema-section"
))]
pub use r#health_plan_copay_option::*;
#[cfg(any(
    feature = "health-plan-cost-sharing-property-schema",
    feature = "pending-schema-section"
))]
mod r#health_plan_cost_sharing;
#[cfg(any(
    feature = "health-plan-cost-sharing-property-schema",
    feature = "pending-schema-section"
))]
pub use r#health_plan_cost_sharing::*;
#[cfg(any(
    feature = "health-plan-drug-option-property-schema",
    feature = "pending-schema-section"
))]
mod r#health_plan_drug_option;
#[cfg(any(
    feature = "health-plan-drug-option-property-schema",
    feature = "pending-schema-section"
))]
pub use r#health_plan_drug_option::*;
#[cfg(any(
    feature = "health-plan-drug-tier-property-schema",
    feature = "pending-schema-section"
))]
mod r#health_plan_drug_tier;
#[cfg(any(
    feature = "health-plan-drug-tier-property-schema",
    feature = "pending-schema-section"
))]
pub use r#health_plan_drug_tier::*;
#[cfg(any(
    feature = "health-plan-id-property-schema",
    feature = "pending-schema-section"
))]
mod r#health_plan_id;
#[cfg(any(
    feature = "health-plan-id-property-schema",
    feature = "pending-schema-section"
))]
pub use r#health_plan_id::*;
#[cfg(any(
    feature = "health-plan-marketing-url-property-schema",
    feature = "pending-schema-section"
))]
mod r#health_plan_marketing_url;
#[cfg(any(
    feature = "health-plan-marketing-url-property-schema",
    feature = "pending-schema-section"
))]
pub use r#health_plan_marketing_url::*;
#[cfg(any(
    feature = "health-plan-network-id-property-schema",
    feature = "pending-schema-section"
))]
mod r#health_plan_network_id;
#[cfg(any(
    feature = "health-plan-network-id-property-schema",
    feature = "pending-schema-section"
))]
pub use r#health_plan_network_id::*;
#[cfg(any(
    feature = "health-plan-network-tier-property-schema",
    feature = "pending-schema-section"
))]
mod r#health_plan_network_tier;
#[cfg(any(
    feature = "health-plan-network-tier-property-schema",
    feature = "pending-schema-section"
))]
pub use r#health_plan_network_tier::*;
#[cfg(any(
    feature = "health-plan-pharmacy-category-property-schema",
    feature = "pending-schema-section"
))]
mod r#health_plan_pharmacy_category;
#[cfg(any(
    feature = "health-plan-pharmacy-category-property-schema",
    feature = "pending-schema-section"
))]
pub use r#health_plan_pharmacy_category::*;
#[cfg(any(
    feature = "healthcare-reporting-data-property-schema",
    feature = "pending-schema-section"
))]
mod r#healthcare_reporting_data;
#[cfg(any(
    feature = "healthcare-reporting-data-property-schema",
    feature = "pending-schema-section"
))]
pub use r#healthcare_reporting_data::*;
#[cfg(any(feature = "height-property-schema", feature = "general-schema-section"))]
mod r#height;
#[cfg(any(feature = "height-property-schema", feature = "general-schema-section"))]
pub use r#height::*;
#[cfg(any(
    feature = "high-price-property-schema",
    feature = "general-schema-section"
))]
mod r#high_price;
#[cfg(any(
    feature = "high-price-property-schema",
    feature = "general-schema-section"
))]
pub use r#high_price::*;
#[cfg(any(
    feature = "hiring-organization-property-schema",
    feature = "general-schema-section"
))]
mod r#hiring_organization;
#[cfg(any(
    feature = "hiring-organization-property-schema",
    feature = "general-schema-section"
))]
pub use r#hiring_organization::*;
#[cfg(any(
    feature = "holding-archive-property-schema",
    feature = "pending-schema-section"
))]
mod r#holding_archive;
#[cfg(any(
    feature = "holding-archive-property-schema",
    feature = "pending-schema-section"
))]
pub use r#holding_archive::*;
#[cfg(any(
    feature = "home-location-property-schema",
    feature = "general-schema-section"
))]
mod r#home_location;
#[cfg(any(
    feature = "home-location-property-schema",
    feature = "general-schema-section"
))]
pub use r#home_location::*;
#[cfg(any(
    feature = "home-team-property-schema",
    feature = "general-schema-section"
))]
mod r#home_team;
#[cfg(any(
    feature = "home-team-property-schema",
    feature = "general-schema-section"
))]
pub use r#home_team::*;
#[cfg(any(
    feature = "honorific-prefix-property-schema",
    feature = "general-schema-section"
))]
mod r#honorific_prefix;
#[cfg(any(
    feature = "honorific-prefix-property-schema",
    feature = "general-schema-section"
))]
pub use r#honorific_prefix::*;
#[cfg(any(
    feature = "honorific-suffix-property-schema",
    feature = "general-schema-section"
))]
mod r#honorific_suffix;
#[cfg(any(
    feature = "honorific-suffix-property-schema",
    feature = "general-schema-section"
))]
pub use r#honorific_suffix::*;
#[cfg(any(
    feature = "hospital-affiliation-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#hospital_affiliation;
#[cfg(any(
    feature = "hospital-affiliation-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#hospital_affiliation::*;
#[cfg(any(
    feature = "hosting-organization-property-schema",
    feature = "general-schema-section"
))]
mod r#hosting_organization;
#[cfg(any(
    feature = "hosting-organization-property-schema",
    feature = "general-schema-section"
))]
pub use r#hosting_organization::*;
#[cfg(any(
    feature = "hours-available-property-schema",
    feature = "general-schema-section"
))]
mod r#hours_available;
#[cfg(any(
    feature = "hours-available-property-schema",
    feature = "general-schema-section"
))]
pub use r#hours_available::*;
#[cfg(any(
    feature = "how-performed-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#how_performed;
#[cfg(any(
    feature = "how-performed-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#how_performed::*;
#[cfg(any(
    feature = "http-method-property-schema",
    feature = "general-schema-section"
))]
mod r#http_method;
#[cfg(any(
    feature = "http-method-property-schema",
    feature = "general-schema-section"
))]
pub use r#http_method::*;
#[cfg(any(
    feature = "iata-code-property-schema",
    feature = "general-schema-section"
))]
mod r#iata_code;
#[cfg(any(
    feature = "iata-code-property-schema",
    feature = "general-schema-section"
))]
pub use r#iata_code::*;
#[cfg(any(
    feature = "icao-code-property-schema",
    feature = "general-schema-section"
))]
mod r#icao_code;
#[cfg(any(
    feature = "icao-code-property-schema",
    feature = "general-schema-section"
))]
pub use r#icao_code::*;
#[cfg(any(
    feature = "identifier-property-schema",
    feature = "general-schema-section"
))]
mod r#identifier;
#[cfg(any(
    feature = "identifier-property-schema",
    feature = "general-schema-section"
))]
pub use r#identifier::*;
#[cfg(any(
    feature = "identifying-exam-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#identifying_exam;
#[cfg(any(
    feature = "identifying-exam-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#identifying_exam::*;
#[cfg(any(
    feature = "identifying-test-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#identifying_test;
#[cfg(any(
    feature = "identifying-test-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#identifying_test::*;
#[cfg(any(
    feature = "illustrator-property-schema",
    feature = "general-schema-section"
))]
mod r#illustrator;
#[cfg(any(
    feature = "illustrator-property-schema",
    feature = "general-schema-section"
))]
pub use r#illustrator::*;
#[cfg(any(feature = "image-property-schema", feature = "general-schema-section"))]
mod r#image;
#[cfg(any(feature = "image-property-schema", feature = "general-schema-section"))]
pub use r#image::*;
#[cfg(any(
    feature = "imaging-technique-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#imaging_technique;
#[cfg(any(
    feature = "imaging-technique-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#imaging_technique::*;
#[cfg(any(
    feature = "in-album-property-schema",
    feature = "general-schema-section"
))]
mod r#in_album;
#[cfg(any(
    feature = "in-album-property-schema",
    feature = "general-schema-section"
))]
pub use r#in_album::*;
#[cfg(any(
    feature = "in-broadcast-lineup-property-schema",
    feature = "general-schema-section"
))]
mod r#in_broadcast_lineup;
#[cfg(any(
    feature = "in-broadcast-lineup-property-schema",
    feature = "general-schema-section"
))]
pub use r#in_broadcast_lineup::*;
#[cfg(any(
    feature = "in-ch-i-property-schema",
    feature = "pending-schema-section"
))]
mod r#in_ch_i;
#[cfg(any(
    feature = "in-ch-i-property-schema",
    feature = "pending-schema-section"
))]
pub use r#in_ch_i::*;
#[cfg(any(
    feature = "in-ch-i-key-property-schema",
    feature = "pending-schema-section"
))]
mod r#in_ch_i_key;
#[cfg(any(
    feature = "in-ch-i-key-property-schema",
    feature = "pending-schema-section"
))]
pub use r#in_ch_i_key::*;
#[cfg(any(
    feature = "in-code-set-property-schema",
    feature = "pending-schema-section"
))]
mod r#in_code_set;
#[cfg(any(
    feature = "in-code-set-property-schema",
    feature = "pending-schema-section"
))]
pub use r#in_code_set::*;
#[cfg(any(
    feature = "in-defined-term-set-property-schema",
    feature = "pending-schema-section"
))]
mod r#in_defined_term_set;
#[cfg(any(
    feature = "in-defined-term-set-property-schema",
    feature = "pending-schema-section"
))]
pub use r#in_defined_term_set::*;
#[cfg(any(
    feature = "in-language-property-schema",
    feature = "general-schema-section"
))]
mod r#in_language;
#[cfg(any(
    feature = "in-language-property-schema",
    feature = "general-schema-section"
))]
pub use r#in_language::*;
#[cfg(any(
    feature = "in-playlist-property-schema",
    feature = "general-schema-section"
))]
mod r#in_playlist;
#[cfg(any(
    feature = "in-playlist-property-schema",
    feature = "general-schema-section"
))]
pub use r#in_playlist::*;
#[cfg(any(
    feature = "in-product-group-with-id-property-schema",
    feature = "pending-schema-section"
))]
mod r#in_product_group_with_id;
#[cfg(any(
    feature = "in-product-group-with-id-property-schema",
    feature = "pending-schema-section"
))]
pub use r#in_product_group_with_id::*;
#[cfg(any(
    feature = "in-store-returns-offered-property-schema",
    feature = "pending-schema-section"
))]
mod r#in_store_returns_offered;
#[cfg(any(
    feature = "in-store-returns-offered-property-schema",
    feature = "pending-schema-section"
))]
pub use r#in_store_returns_offered::*;
#[cfg(any(
    feature = "in-support-of-property-schema",
    feature = "bib-schema-section"
))]
mod r#in_support_of;
#[cfg(any(
    feature = "in-support-of-property-schema",
    feature = "bib-schema-section"
))]
pub use r#in_support_of::*;
#[cfg(any(
    feature = "incentive-compensation-property-schema",
    feature = "general-schema-section"
))]
mod r#incentive_compensation;
#[cfg(any(
    feature = "incentive-compensation-property-schema",
    feature = "general-schema-section"
))]
pub use r#incentive_compensation::*;
#[cfg(any(
    feature = "incentives-property-schema",
    feature = "general-schema-section"
))]
mod r#incentives;
#[cfg(any(
    feature = "incentives-property-schema",
    feature = "general-schema-section"
))]
pub use r#incentives::*;
#[cfg(any(
    feature = "included-composition-property-schema",
    feature = "general-schema-section"
))]
mod r#included_composition;
#[cfg(any(
    feature = "included-composition-property-schema",
    feature = "general-schema-section"
))]
pub use r#included_composition::*;
#[cfg(any(
    feature = "included-data-catalog-property-schema",
    feature = "general-schema-section"
))]
mod r#included_data_catalog;
#[cfg(any(
    feature = "included-data-catalog-property-schema",
    feature = "general-schema-section"
))]
pub use r#included_data_catalog::*;
#[cfg(any(
    feature = "included-in-data-catalog-property-schema",
    feature = "general-schema-section"
))]
mod r#included_in_data_catalog;
#[cfg(any(
    feature = "included-in-data-catalog-property-schema",
    feature = "general-schema-section"
))]
pub use r#included_in_data_catalog::*;
#[cfg(any(
    feature = "included-in-health-insurance-plan-property-schema",
    feature = "pending-schema-section"
))]
mod r#included_in_health_insurance_plan;
#[cfg(any(
    feature = "included-in-health-insurance-plan-property-schema",
    feature = "pending-schema-section"
))]
pub use r#included_in_health_insurance_plan::*;
#[cfg(any(
    feature = "included-risk-factor-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#included_risk_factor;
#[cfg(any(
    feature = "included-risk-factor-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#included_risk_factor::*;
#[cfg(any(
    feature = "includes-attraction-property-schema",
    feature = "pending-schema-section"
))]
mod r#includes_attraction;
#[cfg(any(
    feature = "includes-attraction-property-schema",
    feature = "pending-schema-section"
))]
pub use r#includes_attraction::*;
#[cfg(any(
    feature = "includes-health-plan-formulary-property-schema",
    feature = "pending-schema-section"
))]
mod r#includes_health_plan_formulary;
#[cfg(any(
    feature = "includes-health-plan-formulary-property-schema",
    feature = "pending-schema-section"
))]
pub use r#includes_health_plan_formulary::*;
#[cfg(any(
    feature = "includes-health-plan-network-property-schema",
    feature = "pending-schema-section"
))]
mod r#includes_health_plan_network;
#[cfg(any(
    feature = "includes-health-plan-network-property-schema",
    feature = "pending-schema-section"
))]
pub use r#includes_health_plan_network::*;
#[cfg(any(
    feature = "includes-object-property-schema",
    feature = "general-schema-section"
))]
mod r#includes_object;
#[cfg(any(
    feature = "includes-object-property-schema",
    feature = "general-schema-section"
))]
pub use r#includes_object::*;
#[cfg(any(
    feature = "increases-risk-of-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#increases_risk_of;
#[cfg(any(
    feature = "increases-risk-of-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#increases_risk_of::*;
#[cfg(any(
    feature = "industry-property-schema",
    feature = "general-schema-section"
))]
mod r#industry;
#[cfg(any(
    feature = "industry-property-schema",
    feature = "general-schema-section"
))]
pub use r#industry::*;
#[cfg(any(
    feature = "ineligible-region-property-schema",
    feature = "pending-schema-section"
))]
mod r#ineligible_region;
#[cfg(any(
    feature = "ineligible-region-property-schema",
    feature = "pending-schema-section"
))]
pub use r#ineligible_region::*;
#[cfg(any(
    feature = "infectious-agent-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#infectious_agent;
#[cfg(any(
    feature = "infectious-agent-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#infectious_agent::*;
#[cfg(any(
    feature = "infectious-agent-class-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#infectious_agent_class;
#[cfg(any(
    feature = "infectious-agent-class-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#infectious_agent_class::*;
#[cfg(any(
    feature = "ingredients-property-schema",
    feature = "general-schema-section"
))]
mod r#ingredients;
#[cfg(any(
    feature = "ingredients-property-schema",
    feature = "general-schema-section"
))]
pub use r#ingredients::*;
#[cfg(any(feature = "inker-property-schema", feature = "bib-schema-section"))]
mod r#inker;
#[cfg(any(feature = "inker-property-schema", feature = "bib-schema-section"))]
pub use r#inker::*;
#[cfg(any(
    feature = "insertion-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#insertion;
#[cfg(any(
    feature = "insertion-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#insertion::*;
#[cfg(any(
    feature = "install-url-property-schema",
    feature = "general-schema-section"
))]
mod r#install_url;
#[cfg(any(
    feature = "install-url-property-schema",
    feature = "general-schema-section"
))]
pub use r#install_url::*;
#[cfg(any(
    feature = "instructor-property-schema",
    feature = "general-schema-section"
))]
mod r#instructor;
#[cfg(any(
    feature = "instructor-property-schema",
    feature = "general-schema-section"
))]
pub use r#instructor::*;
#[cfg(any(
    feature = "instrument-property-schema",
    feature = "general-schema-section"
))]
mod r#instrument;
#[cfg(any(
    feature = "instrument-property-schema",
    feature = "general-schema-section"
))]
pub use r#instrument::*;
#[cfg(any(
    feature = "intensity-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#intensity;
#[cfg(any(
    feature = "intensity-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#intensity::*;
#[cfg(any(
    feature = "interacting-drug-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#interacting_drug;
#[cfg(any(
    feature = "interacting-drug-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#interacting_drug::*;
#[cfg(any(
    feature = "interaction-count-property-schema",
    feature = "general-schema-section"
))]
mod r#interaction_count;
#[cfg(any(
    feature = "interaction-count-property-schema",
    feature = "general-schema-section"
))]
pub use r#interaction_count::*;
#[cfg(any(
    feature = "interaction-service-property-schema",
    feature = "general-schema-section"
))]
mod r#interaction_service;
#[cfg(any(
    feature = "interaction-service-property-schema",
    feature = "general-schema-section"
))]
pub use r#interaction_service::*;
#[cfg(any(
    feature = "interaction-statistic-property-schema",
    feature = "general-schema-section"
))]
mod r#interaction_statistic;
#[cfg(any(
    feature = "interaction-statistic-property-schema",
    feature = "general-schema-section"
))]
pub use r#interaction_statistic::*;
#[cfg(any(
    feature = "interaction-type-property-schema",
    feature = "general-schema-section"
))]
mod r#interaction_type;
#[cfg(any(
    feature = "interaction-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#interaction_type::*;
#[cfg(any(
    feature = "interactivity-type-property-schema",
    feature = "general-schema-section"
))]
mod r#interactivity_type;
#[cfg(any(
    feature = "interactivity-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#interactivity_type::*;
#[cfg(any(
    feature = "interest-rate-property-schema",
    feature = "general-schema-section"
))]
mod r#interest_rate;
#[cfg(any(
    feature = "interest-rate-property-schema",
    feature = "general-schema-section"
))]
pub use r#interest_rate::*;
#[cfg(any(
    feature = "interpreted-as-claim-property-schema",
    feature = "pending-schema-section"
))]
mod r#interpreted_as_claim;
#[cfg(any(
    feature = "interpreted-as-claim-property-schema",
    feature = "pending-schema-section"
))]
pub use r#interpreted_as_claim::*;
#[cfg(any(
    feature = "inventory-level-property-schema",
    feature = "general-schema-section"
))]
mod r#inventory_level;
#[cfg(any(
    feature = "inventory-level-property-schema",
    feature = "general-schema-section"
))]
pub use r#inventory_level::*;
#[cfg(any(
    feature = "inverse-of-property-schema",
    feature = "meta-schema-section"
))]
mod r#inverse_of;
#[cfg(any(
    feature = "inverse-of-property-schema",
    feature = "meta-schema-section"
))]
pub use r#inverse_of::*;
#[cfg(any(
    feature = "is-accepting-new-patients-property-schema",
    feature = "pending-schema-section"
))]
mod r#is_accepting_new_patients;
#[cfg(any(
    feature = "is-accepting-new-patients-property-schema",
    feature = "pending-schema-section"
))]
pub use r#is_accepting_new_patients::*;
#[cfg(any(
    feature = "is-accessible-for-free-property-schema",
    feature = "general-schema-section"
))]
mod r#is_accessible_for_free;
#[cfg(any(
    feature = "is-accessible-for-free-property-schema",
    feature = "general-schema-section"
))]
pub use r#is_accessible_for_free::*;
#[cfg(any(
    feature = "is-accessory-or-spare-part-for-property-schema",
    feature = "general-schema-section"
))]
mod r#is_accessory_or_spare_part_for;
#[cfg(any(
    feature = "is-accessory-or-spare-part-for-property-schema",
    feature = "general-schema-section"
))]
pub use r#is_accessory_or_spare_part_for::*;
#[cfg(any(
    feature = "is-available-generically-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#is_available_generically;
#[cfg(any(
    feature = "is-available-generically-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#is_available_generically::*;
#[cfg(any(
    feature = "is-based-on-property-schema",
    feature = "general-schema-section"
))]
mod r#is_based_on;
#[cfg(any(
    feature = "is-based-on-property-schema",
    feature = "general-schema-section"
))]
pub use r#is_based_on::*;
#[cfg(any(
    feature = "is-based-on-url-property-schema",
    feature = "general-schema-section"
))]
mod r#is_based_on_url;
#[cfg(any(
    feature = "is-based-on-url-property-schema",
    feature = "general-schema-section"
))]
pub use r#is_based_on_url::*;
#[cfg(any(
    feature = "is-consumable-for-property-schema",
    feature = "general-schema-section"
))]
mod r#is_consumable_for;
#[cfg(any(
    feature = "is-consumable-for-property-schema",
    feature = "general-schema-section"
))]
pub use r#is_consumable_for::*;
#[cfg(any(
    feature = "is-encoded-by-bio-chem-entity-property-schema",
    feature = "pending-schema-section"
))]
mod r#is_encoded_by_bio_chem_entity;
#[cfg(any(
    feature = "is-encoded-by-bio-chem-entity-property-schema",
    feature = "pending-schema-section"
))]
pub use r#is_encoded_by_bio_chem_entity::*;
#[cfg(any(
    feature = "is-family-friendly-property-schema",
    feature = "general-schema-section"
))]
mod r#is_family_friendly;
#[cfg(any(
    feature = "is-family-friendly-property-schema",
    feature = "general-schema-section"
))]
pub use r#is_family_friendly::*;
#[cfg(any(
    feature = "is-gift-property-schema",
    feature = "general-schema-section"
))]
mod r#is_gift;
#[cfg(any(
    feature = "is-gift-property-schema",
    feature = "general-schema-section"
))]
pub use r#is_gift::*;
#[cfg(any(
    feature = "is-involved-in-biological-process-property-schema",
    feature = "pending-schema-section"
))]
mod r#is_involved_in_biological_process;
#[cfg(any(
    feature = "is-involved-in-biological-process-property-schema",
    feature = "pending-schema-section"
))]
pub use r#is_involved_in_biological_process::*;
#[cfg(any(
    feature = "is-live-broadcast-property-schema",
    feature = "general-schema-section"
))]
mod r#is_live_broadcast;
#[cfg(any(
    feature = "is-live-broadcast-property-schema",
    feature = "general-schema-section"
))]
pub use r#is_live_broadcast::*;
#[cfg(any(
    feature = "is-located-in-subcellular-location-property-schema",
    feature = "pending-schema-section"
))]
mod r#is_located_in_subcellular_location;
#[cfg(any(
    feature = "is-located-in-subcellular-location-property-schema",
    feature = "pending-schema-section"
))]
pub use r#is_located_in_subcellular_location::*;
#[cfg(any(
    feature = "is-part-of-property-schema",
    feature = "general-schema-section"
))]
mod r#is_part_of;
#[cfg(any(
    feature = "is-part-of-property-schema",
    feature = "general-schema-section"
))]
pub use r#is_part_of::*;
#[cfg(any(
    feature = "is-part-of-bio-chem-entity-property-schema",
    feature = "pending-schema-section"
))]
mod r#is_part_of_bio_chem_entity;
#[cfg(any(
    feature = "is-part-of-bio-chem-entity-property-schema",
    feature = "pending-schema-section"
))]
pub use r#is_part_of_bio_chem_entity::*;
#[cfg(any(
    feature = "is-plan-for-apartment-property-schema",
    feature = "pending-schema-section"
))]
mod r#is_plan_for_apartment;
#[cfg(any(
    feature = "is-plan-for-apartment-property-schema",
    feature = "pending-schema-section"
))]
pub use r#is_plan_for_apartment::*;
#[cfg(any(
    feature = "is-proprietary-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#is_proprietary;
#[cfg(any(
    feature = "is-proprietary-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#is_proprietary::*;
#[cfg(any(
    feature = "is-related-to-property-schema",
    feature = "general-schema-section"
))]
mod r#is_related_to;
#[cfg(any(
    feature = "is-related-to-property-schema",
    feature = "general-schema-section"
))]
pub use r#is_related_to::*;
#[cfg(any(
    feature = "is-resizable-property-schema",
    feature = "pending-schema-section"
))]
mod r#is_resizable;
#[cfg(any(
    feature = "is-resizable-property-schema",
    feature = "pending-schema-section"
))]
pub use r#is_resizable::*;
#[cfg(any(
    feature = "is-similar-to-property-schema",
    feature = "general-schema-section"
))]
mod r#is_similar_to;
#[cfg(any(
    feature = "is-similar-to-property-schema",
    feature = "general-schema-section"
))]
pub use r#is_similar_to::*;
#[cfg(any(
    feature = "is-unlabelled-fallback-property-schema",
    feature = "pending-schema-section"
))]
mod r#is_unlabelled_fallback;
#[cfg(any(
    feature = "is-unlabelled-fallback-property-schema",
    feature = "pending-schema-section"
))]
pub use r#is_unlabelled_fallback::*;
#[cfg(any(
    feature = "is-variant-of-property-schema",
    feature = "general-schema-section"
))]
mod r#is_variant_of;
#[cfg(any(
    feature = "is-variant-of-property-schema",
    feature = "general-schema-section"
))]
pub use r#is_variant_of::*;
#[cfg(any(feature = "isbn-property-schema", feature = "general-schema-section"))]
mod r#isbn;
#[cfg(any(feature = "isbn-property-schema", feature = "general-schema-section"))]
pub use r#isbn::*;
#[cfg(any(
    feature = "isic-v-4-property-schema",
    feature = "general-schema-section"
))]
mod r#isic_v_4;
#[cfg(any(
    feature = "isic-v-4-property-schema",
    feature = "general-schema-section"
))]
pub use r#isic_v_4::*;
#[cfg(any(
    feature = "iso-6523-code-property-schema",
    feature = "pending-schema-section"
))]
mod r#iso_6523_code;
#[cfg(any(
    feature = "iso-6523-code-property-schema",
    feature = "pending-schema-section"
))]
pub use r#iso_6523_code::*;
#[cfg(any(
    feature = "isrc-code-property-schema",
    feature = "general-schema-section"
))]
mod r#isrc_code;
#[cfg(any(
    feature = "isrc-code-property-schema",
    feature = "general-schema-section"
))]
pub use r#isrc_code::*;
#[cfg(any(feature = "issn-property-schema", feature = "general-schema-section"))]
mod r#issn;
#[cfg(any(feature = "issn-property-schema", feature = "general-schema-section"))]
pub use r#issn::*;
#[cfg(any(
    feature = "issue-number-property-schema",
    feature = "general-schema-section"
))]
mod r#issue_number;
#[cfg(any(
    feature = "issue-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#issue_number::*;
#[cfg(any(
    feature = "issued-by-property-schema",
    feature = "general-schema-section"
))]
mod r#issued_by;
#[cfg(any(
    feature = "issued-by-property-schema",
    feature = "general-schema-section"
))]
pub use r#issued_by::*;
#[cfg(any(
    feature = "issued-through-property-schema",
    feature = "general-schema-section"
))]
mod r#issued_through;
#[cfg(any(
    feature = "issued-through-property-schema",
    feature = "general-schema-section"
))]
pub use r#issued_through::*;
#[cfg(any(
    feature = "iswc-code-property-schema",
    feature = "general-schema-section"
))]
mod r#iswc_code;
#[cfg(any(
    feature = "iswc-code-property-schema",
    feature = "general-schema-section"
))]
pub use r#iswc_code::*;
#[cfg(any(feature = "item-property-schema", feature = "general-schema-section"))]
mod r#item;
#[cfg(any(feature = "item-property-schema", feature = "general-schema-section"))]
pub use r#item::*;
#[cfg(any(
    feature = "item-condition-property-schema",
    feature = "general-schema-section"
))]
mod r#item_condition;
#[cfg(any(
    feature = "item-condition-property-schema",
    feature = "general-schema-section"
))]
pub use r#item_condition::*;
#[cfg(any(
    feature = "item-defect-return-fees-property-schema",
    feature = "pending-schema-section"
))]
mod r#item_defect_return_fees;
#[cfg(any(
    feature = "item-defect-return-fees-property-schema",
    feature = "pending-schema-section"
))]
pub use r#item_defect_return_fees::*;
#[cfg(any(
    feature = "item-defect-return-label-source-property-schema",
    feature = "pending-schema-section"
))]
mod r#item_defect_return_label_source;
#[cfg(any(
    feature = "item-defect-return-label-source-property-schema",
    feature = "pending-schema-section"
))]
pub use r#item_defect_return_label_source::*;
#[cfg(any(
    feature = "item-defect-return-shipping-fees-amount-property-schema",
    feature = "pending-schema-section"
))]
mod r#item_defect_return_shipping_fees_amount;
#[cfg(any(
    feature = "item-defect-return-shipping-fees-amount-property-schema",
    feature = "pending-schema-section"
))]
pub use r#item_defect_return_shipping_fees_amount::*;
#[cfg(any(
    feature = "item-list-element-property-schema",
    feature = "general-schema-section"
))]
mod r#item_list_element;
#[cfg(any(
    feature = "item-list-element-property-schema",
    feature = "general-schema-section"
))]
pub use r#item_list_element::*;
#[cfg(any(
    feature = "item-list-order-property-schema",
    feature = "general-schema-section"
))]
mod r#item_list_order;
#[cfg(any(
    feature = "item-list-order-property-schema",
    feature = "general-schema-section"
))]
pub use r#item_list_order::*;
#[cfg(any(
    feature = "item-location-property-schema",
    feature = "pending-schema-section"
))]
mod r#item_location;
#[cfg(any(
    feature = "item-location-property-schema",
    feature = "pending-schema-section"
))]
pub use r#item_location::*;
#[cfg(any(
    feature = "item-offered-property-schema",
    feature = "general-schema-section"
))]
mod r#item_offered;
#[cfg(any(
    feature = "item-offered-property-schema",
    feature = "general-schema-section"
))]
pub use r#item_offered::*;
#[cfg(any(
    feature = "item-reviewed-property-schema",
    feature = "general-schema-section"
))]
mod r#item_reviewed;
#[cfg(any(
    feature = "item-reviewed-property-schema",
    feature = "general-schema-section"
))]
pub use r#item_reviewed::*;
#[cfg(any(
    feature = "item-shipped-property-schema",
    feature = "general-schema-section"
))]
mod r#item_shipped;
#[cfg(any(
    feature = "item-shipped-property-schema",
    feature = "general-schema-section"
))]
pub use r#item_shipped::*;
#[cfg(any(
    feature = "itinerary-property-schema",
    feature = "pending-schema-section"
))]
mod r#itinerary;
#[cfg(any(
    feature = "itinerary-property-schema",
    feature = "pending-schema-section"
))]
pub use r#itinerary::*;
#[cfg(any(
    feature = "iupac-name-property-schema",
    feature = "pending-schema-section"
))]
mod r#iupac_name;
#[cfg(any(
    feature = "iupac-name-property-schema",
    feature = "pending-schema-section"
))]
pub use r#iupac_name::*;
#[cfg(any(
    feature = "job-benefits-property-schema",
    feature = "general-schema-section"
))]
mod r#job_benefits;
#[cfg(any(
    feature = "job-benefits-property-schema",
    feature = "general-schema-section"
))]
pub use r#job_benefits::*;
#[cfg(any(
    feature = "job-immediate-start-property-schema",
    feature = "pending-schema-section"
))]
mod r#job_immediate_start;
#[cfg(any(
    feature = "job-immediate-start-property-schema",
    feature = "pending-schema-section"
))]
pub use r#job_immediate_start::*;
#[cfg(any(
    feature = "job-location-property-schema",
    feature = "general-schema-section"
))]
mod r#job_location;
#[cfg(any(
    feature = "job-location-property-schema",
    feature = "general-schema-section"
))]
pub use r#job_location::*;
#[cfg(any(
    feature = "job-location-type-property-schema",
    feature = "pending-schema-section"
))]
mod r#job_location_type;
#[cfg(any(
    feature = "job-location-type-property-schema",
    feature = "pending-schema-section"
))]
pub use r#job_location_type::*;
#[cfg(any(
    feature = "job-start-date-property-schema",
    feature = "pending-schema-section"
))]
mod r#job_start_date;
#[cfg(any(
    feature = "job-start-date-property-schema",
    feature = "pending-schema-section"
))]
pub use r#job_start_date::*;
#[cfg(any(
    feature = "job-title-property-schema",
    feature = "pending-schema-section"
))]
mod r#job_title;
#[cfg(any(
    feature = "job-title-property-schema",
    feature = "pending-schema-section"
))]
pub use r#job_title::*;
#[cfg(any(
    feature = "jurisdiction-property-schema",
    feature = "pending-schema-section"
))]
mod r#jurisdiction;
#[cfg(any(
    feature = "jurisdiction-property-schema",
    feature = "pending-schema-section"
))]
pub use r#jurisdiction::*;
#[cfg(any(
    feature = "keywords-property-schema",
    feature = "general-schema-section"
))]
mod r#keywords;
#[cfg(any(
    feature = "keywords-property-schema",
    feature = "general-schema-section"
))]
pub use r#keywords::*;
#[cfg(any(
    feature = "known-vehicle-damages-property-schema",
    feature = "general-schema-section"
))]
mod r#known_vehicle_damages;
#[cfg(any(
    feature = "known-vehicle-damages-property-schema",
    feature = "general-schema-section"
))]
pub use r#known_vehicle_damages::*;
#[cfg(any(feature = "knows-property-schema", feature = "general-schema-section"))]
mod r#knows;
#[cfg(any(feature = "knows-property-schema", feature = "general-schema-section"))]
pub use r#knows::*;
#[cfg(any(
    feature = "knows-about-property-schema",
    feature = "pending-schema-section"
))]
mod r#knows_about;
#[cfg(any(
    feature = "knows-about-property-schema",
    feature = "pending-schema-section"
))]
pub use r#knows_about::*;
#[cfg(any(
    feature = "knows-language-property-schema",
    feature = "pending-schema-section"
))]
mod r#knows_language;
#[cfg(any(
    feature = "knows-language-property-schema",
    feature = "pending-schema-section"
))]
pub use r#knows_language::*;
#[cfg(any(
    feature = "label-details-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#label_details;
#[cfg(any(
    feature = "label-details-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#label_details::*;
#[cfg(any(
    feature = "landlord-property-schema",
    feature = "general-schema-section"
))]
mod r#landlord;
#[cfg(any(
    feature = "landlord-property-schema",
    feature = "general-schema-section"
))]
pub use r#landlord::*;
#[cfg(any(
    feature = "language-property-schema",
    feature = "general-schema-section"
))]
mod r#language;
#[cfg(any(
    feature = "language-property-schema",
    feature = "general-schema-section"
))]
pub use r#language::*;
#[cfg(any(
    feature = "last-reviewed-property-schema",
    feature = "general-schema-section"
))]
mod r#last_reviewed;
#[cfg(any(
    feature = "last-reviewed-property-schema",
    feature = "general-schema-section"
))]
pub use r#last_reviewed::*;
#[cfg(any(
    feature = "latitude-property-schema",
    feature = "general-schema-section"
))]
mod r#latitude;
#[cfg(any(
    feature = "latitude-property-schema",
    feature = "general-schema-section"
))]
pub use r#latitude::*;
#[cfg(any(
    feature = "layout-image-property-schema",
    feature = "pending-schema-section"
))]
mod r#layout_image;
#[cfg(any(
    feature = "layout-image-property-schema",
    feature = "pending-schema-section"
))]
pub use r#layout_image::*;
#[cfg(any(
    feature = "learning-resource-type-property-schema",
    feature = "general-schema-section"
))]
mod r#learning_resource_type;
#[cfg(any(
    feature = "learning-resource-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#learning_resource_type::*;
#[cfg(any(
    feature = "lease-length-property-schema",
    feature = "pending-schema-section"
))]
mod r#lease_length;
#[cfg(any(
    feature = "lease-length-property-schema",
    feature = "pending-schema-section"
))]
pub use r#lease_length::*;
#[cfg(any(
    feature = "legal-name-property-schema",
    feature = "general-schema-section"
))]
mod r#legal_name;
#[cfg(any(
    feature = "legal-name-property-schema",
    feature = "general-schema-section"
))]
pub use r#legal_name::*;
#[cfg(any(
    feature = "legal-status-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#legal_status;
#[cfg(any(
    feature = "legal-status-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#legal_status::*;
#[cfg(any(
    feature = "legislation-applies-property-schema",
    feature = "pending-schema-section"
))]
mod r#legislation_applies;
#[cfg(any(
    feature = "legislation-applies-property-schema",
    feature = "pending-schema-section"
))]
pub use r#legislation_applies::*;
#[cfg(any(
    feature = "legislation-changes-property-schema",
    feature = "pending-schema-section"
))]
mod r#legislation_changes;
#[cfg(any(
    feature = "legislation-changes-property-schema",
    feature = "pending-schema-section"
))]
pub use r#legislation_changes::*;
#[cfg(any(
    feature = "legislation-consolidates-property-schema",
    feature = "pending-schema-section"
))]
mod r#legislation_consolidates;
#[cfg(any(
    feature = "legislation-consolidates-property-schema",
    feature = "pending-schema-section"
))]
pub use r#legislation_consolidates::*;
#[cfg(any(
    feature = "legislation-date-property-schema",
    feature = "pending-schema-section"
))]
mod r#legislation_date;
#[cfg(any(
    feature = "legislation-date-property-schema",
    feature = "pending-schema-section"
))]
pub use r#legislation_date::*;
#[cfg(any(
    feature = "legislation-date-version-property-schema",
    feature = "pending-schema-section"
))]
mod r#legislation_date_version;
#[cfg(any(
    feature = "legislation-date-version-property-schema",
    feature = "pending-schema-section"
))]
pub use r#legislation_date_version::*;
#[cfg(any(
    feature = "legislation-identifier-property-schema",
    feature = "pending-schema-section"
))]
mod r#legislation_identifier;
#[cfg(any(
    feature = "legislation-identifier-property-schema",
    feature = "pending-schema-section"
))]
pub use r#legislation_identifier::*;
#[cfg(any(
    feature = "legislation-jurisdiction-property-schema",
    feature = "pending-schema-section"
))]
mod r#legislation_jurisdiction;
#[cfg(any(
    feature = "legislation-jurisdiction-property-schema",
    feature = "pending-schema-section"
))]
pub use r#legislation_jurisdiction::*;
#[cfg(any(
    feature = "legislation-legal-force-property-schema",
    feature = "pending-schema-section"
))]
mod r#legislation_legal_force;
#[cfg(any(
    feature = "legislation-legal-force-property-schema",
    feature = "pending-schema-section"
))]
pub use r#legislation_legal_force::*;
#[cfg(any(
    feature = "legislation-legal-value-property-schema",
    feature = "pending-schema-section"
))]
mod r#legislation_legal_value;
#[cfg(any(
    feature = "legislation-legal-value-property-schema",
    feature = "pending-schema-section"
))]
pub use r#legislation_legal_value::*;
#[cfg(any(
    feature = "legislation-passed-by-property-schema",
    feature = "pending-schema-section"
))]
mod r#legislation_passed_by;
#[cfg(any(
    feature = "legislation-passed-by-property-schema",
    feature = "pending-schema-section"
))]
pub use r#legislation_passed_by::*;
#[cfg(any(
    feature = "legislation-responsible-property-schema",
    feature = "pending-schema-section"
))]
mod r#legislation_responsible;
#[cfg(any(
    feature = "legislation-responsible-property-schema",
    feature = "pending-schema-section"
))]
pub use r#legislation_responsible::*;
#[cfg(any(
    feature = "legislation-transposes-property-schema",
    feature = "pending-schema-section"
))]
mod r#legislation_transposes;
#[cfg(any(
    feature = "legislation-transposes-property-schema",
    feature = "pending-schema-section"
))]
pub use r#legislation_transposes::*;
#[cfg(any(
    feature = "legislation-type-property-schema",
    feature = "pending-schema-section"
))]
mod r#legislation_type;
#[cfg(any(
    feature = "legislation-type-property-schema",
    feature = "pending-schema-section"
))]
pub use r#legislation_type::*;
#[cfg(any(
    feature = "lei-code-property-schema",
    feature = "general-schema-section"
))]
mod r#lei_code;
#[cfg(any(
    feature = "lei-code-property-schema",
    feature = "general-schema-section"
))]
pub use r#lei_code::*;
#[cfg(any(feature = "lender-property-schema", feature = "general-schema-section"))]
mod r#lender;
#[cfg(any(feature = "lender-property-schema", feature = "general-schema-section"))]
pub use r#lender::*;
#[cfg(any(feature = "lesser-property-schema", feature = "general-schema-section"))]
mod r#lesser;
#[cfg(any(feature = "lesser-property-schema", feature = "general-schema-section"))]
pub use r#lesser::*;
#[cfg(any(
    feature = "lesser-or-equal-property-schema",
    feature = "general-schema-section"
))]
mod r#lesser_or_equal;
#[cfg(any(
    feature = "lesser-or-equal-property-schema",
    feature = "general-schema-section"
))]
pub use r#lesser_or_equal::*;
#[cfg(any(feature = "letterer-property-schema", feature = "bib-schema-section"))]
mod r#letterer;
#[cfg(any(feature = "letterer-property-schema", feature = "bib-schema-section"))]
pub use r#letterer::*;
#[cfg(any(
    feature = "license-property-schema",
    feature = "general-schema-section"
))]
mod r#license;
#[cfg(any(
    feature = "license-property-schema",
    feature = "general-schema-section"
))]
pub use r#license::*;
#[cfg(any(feature = "line-property-schema", feature = "general-schema-section"))]
mod r#line;
#[cfg(any(feature = "line-property-schema", feature = "general-schema-section"))]
pub use r#line::*;
#[cfg(any(
    feature = "link-relationship-property-schema",
    feature = "pending-schema-section"
))]
mod r#link_relationship;
#[cfg(any(
    feature = "link-relationship-property-schema",
    feature = "pending-schema-section"
))]
pub use r#link_relationship::*;
#[cfg(any(
    feature = "live-blog-update-property-schema",
    feature = "general-schema-section"
))]
mod r#live_blog_update;
#[cfg(any(
    feature = "live-blog-update-property-schema",
    feature = "general-schema-section"
))]
pub use r#live_blog_update::*;
#[cfg(any(
    feature = "loan-mortgage-mandate-amount-property-schema",
    feature = "pending-schema-section"
))]
mod r#loan_mortgage_mandate_amount;
#[cfg(any(
    feature = "loan-mortgage-mandate-amount-property-schema",
    feature = "pending-schema-section"
))]
pub use r#loan_mortgage_mandate_amount::*;
#[cfg(any(
    feature = "loan-payment-amount-property-schema",
    feature = "pending-schema-section"
))]
mod r#loan_payment_amount;
#[cfg(any(
    feature = "loan-payment-amount-property-schema",
    feature = "pending-schema-section"
))]
pub use r#loan_payment_amount::*;
#[cfg(any(
    feature = "loan-payment-frequency-property-schema",
    feature = "pending-schema-section"
))]
mod r#loan_payment_frequency;
#[cfg(any(
    feature = "loan-payment-frequency-property-schema",
    feature = "pending-schema-section"
))]
pub use r#loan_payment_frequency::*;
#[cfg(any(
    feature = "loan-repayment-form-property-schema",
    feature = "pending-schema-section"
))]
mod r#loan_repayment_form;
#[cfg(any(
    feature = "loan-repayment-form-property-schema",
    feature = "pending-schema-section"
))]
pub use r#loan_repayment_form::*;
#[cfg(any(
    feature = "loan-term-property-schema",
    feature = "general-schema-section"
))]
mod r#loan_term;
#[cfg(any(
    feature = "loan-term-property-schema",
    feature = "general-schema-section"
))]
pub use r#loan_term::*;
#[cfg(any(
    feature = "loan-type-property-schema",
    feature = "pending-schema-section"
))]
mod r#loan_type;
#[cfg(any(
    feature = "loan-type-property-schema",
    feature = "pending-schema-section"
))]
pub use r#loan_type::*;
#[cfg(any(
    feature = "location-property-schema",
    feature = "general-schema-section"
))]
mod r#location;
#[cfg(any(
    feature = "location-property-schema",
    feature = "general-schema-section"
))]
pub use r#location::*;
#[cfg(any(
    feature = "location-created-property-schema",
    feature = "general-schema-section"
))]
mod r#location_created;
#[cfg(any(
    feature = "location-created-property-schema",
    feature = "general-schema-section"
))]
pub use r#location_created::*;
#[cfg(any(
    feature = "lodging-unit-description-property-schema",
    feature = "general-schema-section"
))]
mod r#lodging_unit_description;
#[cfg(any(
    feature = "lodging-unit-description-property-schema",
    feature = "general-schema-section"
))]
pub use r#lodging_unit_description::*;
#[cfg(any(
    feature = "lodging-unit-type-property-schema",
    feature = "general-schema-section"
))]
mod r#lodging_unit_type;
#[cfg(any(
    feature = "lodging-unit-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#lodging_unit_type::*;
#[cfg(any(feature = "logo-property-schema", feature = "general-schema-section"))]
mod r#logo;
#[cfg(any(feature = "logo-property-schema", feature = "general-schema-section"))]
pub use r#logo::*;
#[cfg(any(
    feature = "longitude-property-schema",
    feature = "general-schema-section"
))]
mod r#longitude;
#[cfg(any(
    feature = "longitude-property-schema",
    feature = "general-schema-section"
))]
pub use r#longitude::*;
#[cfg(any(feature = "loser-property-schema", feature = "general-schema-section"))]
mod r#loser;
#[cfg(any(feature = "loser-property-schema", feature = "general-schema-section"))]
pub use r#loser::*;
#[cfg(any(
    feature = "low-price-property-schema",
    feature = "general-schema-section"
))]
mod r#low_price;
#[cfg(any(
    feature = "low-price-property-schema",
    feature = "general-schema-section"
))]
pub use r#low_price::*;
#[cfg(any(
    feature = "lyricist-property-schema",
    feature = "general-schema-section"
))]
mod r#lyricist;
#[cfg(any(
    feature = "lyricist-property-schema",
    feature = "general-schema-section"
))]
pub use r#lyricist::*;
#[cfg(any(feature = "lyrics-property-schema", feature = "general-schema-section"))]
mod r#lyrics;
#[cfg(any(feature = "lyrics-property-schema", feature = "general-schema-section"))]
pub use r#lyrics::*;
#[cfg(any(
    feature = "main-content-of-page-property-schema",
    feature = "general-schema-section"
))]
mod r#main_content_of_page;
#[cfg(any(
    feature = "main-content-of-page-property-schema",
    feature = "general-schema-section"
))]
pub use r#main_content_of_page::*;
#[cfg(any(
    feature = "main-entity-property-schema",
    feature = "general-schema-section"
))]
mod r#main_entity;
#[cfg(any(
    feature = "main-entity-property-schema",
    feature = "general-schema-section"
))]
pub use r#main_entity::*;
#[cfg(any(
    feature = "main-entity-of-page-property-schema",
    feature = "general-schema-section"
))]
mod r#main_entity_of_page;
#[cfg(any(
    feature = "main-entity-of-page-property-schema",
    feature = "general-schema-section"
))]
pub use r#main_entity_of_page::*;
#[cfg(any(
    feature = "maintainer-property-schema",
    feature = "pending-schema-section"
))]
mod r#maintainer;
#[cfg(any(
    feature = "maintainer-property-schema",
    feature = "pending-schema-section"
))]
pub use r#maintainer::*;
#[cfg(any(
    feature = "makes-offer-property-schema",
    feature = "general-schema-section"
))]
mod r#makes_offer;
#[cfg(any(
    feature = "makes-offer-property-schema",
    feature = "general-schema-section"
))]
pub use r#makes_offer::*;
#[cfg(any(
    feature = "manufacturer-property-schema",
    feature = "general-schema-section"
))]
mod r#manufacturer;
#[cfg(any(
    feature = "manufacturer-property-schema",
    feature = "general-schema-section"
))]
pub use r#manufacturer::*;
#[cfg(any(feature = "map-property-schema", feature = "general-schema-section"))]
mod r#map;
#[cfg(any(feature = "map-property-schema", feature = "general-schema-section"))]
pub use r#map::*;
#[cfg(any(
    feature = "map-type-property-schema",
    feature = "general-schema-section"
))]
mod r#map_type;
#[cfg(any(
    feature = "map-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#map_type::*;
#[cfg(any(feature = "maps-property-schema", feature = "general-schema-section"))]
mod r#maps;
#[cfg(any(feature = "maps-property-schema", feature = "general-schema-section"))]
pub use r#maps::*;
#[cfg(any(
    feature = "margin-of-error-property-schema",
    feature = "pending-schema-section"
))]
mod r#margin_of_error;
#[cfg(any(
    feature = "margin-of-error-property-schema",
    feature = "pending-schema-section"
))]
pub use r#margin_of_error::*;
#[cfg(any(
    feature = "masthead-property-schema",
    feature = "pending-schema-section"
))]
mod r#masthead;
#[cfg(any(
    feature = "masthead-property-schema",
    feature = "pending-schema-section"
))]
pub use r#masthead::*;
#[cfg(any(
    feature = "material-property-schema",
    feature = "general-schema-section"
))]
mod r#material;
#[cfg(any(
    feature = "material-property-schema",
    feature = "general-schema-section"
))]
pub use r#material::*;
#[cfg(any(
    feature = "material-extent-property-schema",
    feature = "pending-schema-section"
))]
mod r#material_extent;
#[cfg(any(
    feature = "material-extent-property-schema",
    feature = "pending-schema-section"
))]
pub use r#material_extent::*;
#[cfg(any(
    feature = "math-expression-property-schema",
    feature = "pending-schema-section"
))]
mod r#math_expression;
#[cfg(any(
    feature = "math-expression-property-schema",
    feature = "pending-schema-section"
))]
pub use r#math_expression::*;
#[cfg(any(
    feature = "max-price-property-schema",
    feature = "general-schema-section"
))]
mod r#max_price;
#[cfg(any(
    feature = "max-price-property-schema",
    feature = "general-schema-section"
))]
pub use r#max_price::*;
#[cfg(any(
    feature = "max-value-property-schema",
    feature = "general-schema-section"
))]
mod r#max_value;
#[cfg(any(
    feature = "max-value-property-schema",
    feature = "general-schema-section"
))]
pub use r#max_value::*;
#[cfg(any(
    feature = "maximum-attendee-capacity-property-schema",
    feature = "general-schema-section"
))]
mod r#maximum_attendee_capacity;
#[cfg(any(
    feature = "maximum-attendee-capacity-property-schema",
    feature = "general-schema-section"
))]
pub use r#maximum_attendee_capacity::*;
#[cfg(any(
    feature = "maximum-enrollment-property-schema",
    feature = "pending-schema-section"
))]
mod r#maximum_enrollment;
#[cfg(any(
    feature = "maximum-enrollment-property-schema",
    feature = "pending-schema-section"
))]
pub use r#maximum_enrollment::*;
#[cfg(any(
    feature = "maximum-intake-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#maximum_intake;
#[cfg(any(
    feature = "maximum-intake-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#maximum_intake::*;
#[cfg(any(
    feature = "maximum-physical-attendee-capacity-property-schema",
    feature = "pending-schema-section"
))]
mod r#maximum_physical_attendee_capacity;
#[cfg(any(
    feature = "maximum-physical-attendee-capacity-property-schema",
    feature = "pending-schema-section"
))]
pub use r#maximum_physical_attendee_capacity::*;
#[cfg(any(
    feature = "maximum-virtual-attendee-capacity-property-schema",
    feature = "pending-schema-section"
))]
mod r#maximum_virtual_attendee_capacity;
#[cfg(any(
    feature = "maximum-virtual-attendee-capacity-property-schema",
    feature = "pending-schema-section"
))]
pub use r#maximum_virtual_attendee_capacity::*;
#[cfg(any(
    feature = "meal-service-property-schema",
    feature = "general-schema-section"
))]
mod r#meal_service;
#[cfg(any(
    feature = "meal-service-property-schema",
    feature = "general-schema-section"
))]
pub use r#meal_service::*;
#[cfg(any(
    feature = "measured-property-property-schema",
    feature = "pending-schema-section"
))]
mod r#measured_property;
#[cfg(any(
    feature = "measured-property-property-schema",
    feature = "pending-schema-section"
))]
pub use r#measured_property::*;
#[cfg(any(
    feature = "measurement-denominator-property-schema",
    feature = "pending-schema-section"
))]
mod r#measurement_denominator;
#[cfg(any(
    feature = "measurement-denominator-property-schema",
    feature = "pending-schema-section"
))]
pub use r#measurement_denominator::*;
#[cfg(any(
    feature = "measurement-method-property-schema",
    feature = "pending-schema-section"
))]
mod r#measurement_method;
#[cfg(any(
    feature = "measurement-method-property-schema",
    feature = "pending-schema-section"
))]
pub use r#measurement_method::*;
#[cfg(any(
    feature = "measurement-qualifier-property-schema",
    feature = "pending-schema-section"
))]
mod r#measurement_qualifier;
#[cfg(any(
    feature = "measurement-qualifier-property-schema",
    feature = "pending-schema-section"
))]
pub use r#measurement_qualifier::*;
#[cfg(any(
    feature = "measurement-technique-property-schema",
    feature = "pending-schema-section"
))]
mod r#measurement_technique;
#[cfg(any(
    feature = "measurement-technique-property-schema",
    feature = "pending-schema-section"
))]
pub use r#measurement_technique::*;
#[cfg(any(
    feature = "mechanism-of-action-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#mechanism_of_action;
#[cfg(any(
    feature = "mechanism-of-action-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#mechanism_of_action::*;
#[cfg(any(
    feature = "media-authenticity-category-property-schema",
    feature = "pending-schema-section"
))]
mod r#media_authenticity_category;
#[cfg(any(
    feature = "media-authenticity-category-property-schema",
    feature = "pending-schema-section"
))]
pub use r#media_authenticity_category::*;
#[cfg(any(
    feature = "media-item-appearance-property-schema",
    feature = "pending-schema-section"
))]
mod r#media_item_appearance;
#[cfg(any(
    feature = "media-item-appearance-property-schema",
    feature = "pending-schema-section"
))]
pub use r#media_item_appearance::*;
#[cfg(any(feature = "median-property-schema", feature = "general-schema-section"))]
mod r#median;
#[cfg(any(feature = "median-property-schema", feature = "general-schema-section"))]
pub use r#median::*;
#[cfg(any(
    feature = "medical-audience-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#medical_audience;
#[cfg(any(
    feature = "medical-audience-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#medical_audience::*;
#[cfg(any(
    feature = "medical-specialty-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#medical_specialty;
#[cfg(any(
    feature = "medical-specialty-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#medical_specialty::*;
#[cfg(any(
    feature = "medicine-system-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#medicine_system;
#[cfg(any(
    feature = "medicine-system-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#medicine_system::*;
#[cfg(any(
    feature = "meets-emission-standard-property-schema",
    feature = "auto-schema-section"
))]
mod r#meets_emission_standard;
#[cfg(any(
    feature = "meets-emission-standard-property-schema",
    feature = "auto-schema-section"
))]
pub use r#meets_emission_standard::*;
#[cfg(any(feature = "member-property-schema", feature = "general-schema-section"))]
mod r#member;
#[cfg(any(feature = "member-property-schema", feature = "general-schema-section"))]
pub use r#member::*;
#[cfg(any(
    feature = "member-of-property-schema",
    feature = "general-schema-section"
))]
mod r#member_of;
#[cfg(any(
    feature = "member-of-property-schema",
    feature = "general-schema-section"
))]
pub use r#member_of::*;
#[cfg(any(
    feature = "members-property-schema",
    feature = "general-schema-section"
))]
mod r#members;
#[cfg(any(
    feature = "members-property-schema",
    feature = "general-schema-section"
))]
pub use r#members::*;
#[cfg(any(
    feature = "membership-number-property-schema",
    feature = "general-schema-section"
))]
mod r#membership_number;
#[cfg(any(
    feature = "membership-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#membership_number::*;
#[cfg(any(
    feature = "membership-points-earned-property-schema",
    feature = "pending-schema-section"
))]
mod r#membership_points_earned;
#[cfg(any(
    feature = "membership-points-earned-property-schema",
    feature = "pending-schema-section"
))]
pub use r#membership_points_earned::*;
#[cfg(any(
    feature = "memory-requirements-property-schema",
    feature = "general-schema-section"
))]
mod r#memory_requirements;
#[cfg(any(
    feature = "memory-requirements-property-schema",
    feature = "general-schema-section"
))]
pub use r#memory_requirements::*;
#[cfg(any(
    feature = "mentions-property-schema",
    feature = "general-schema-section"
))]
mod r#mentions;
#[cfg(any(
    feature = "mentions-property-schema",
    feature = "general-schema-section"
))]
pub use r#mentions::*;
#[cfg(any(feature = "menu-property-schema", feature = "general-schema-section"))]
mod r#menu;
#[cfg(any(feature = "menu-property-schema", feature = "general-schema-section"))]
pub use r#menu::*;
#[cfg(any(
    feature = "menu-add-on-property-schema",
    feature = "general-schema-section"
))]
mod r#menu_add_on;
#[cfg(any(
    feature = "menu-add-on-property-schema",
    feature = "general-schema-section"
))]
pub use r#menu_add_on::*;
#[cfg(any(
    feature = "merchant-property-schema",
    feature = "general-schema-section"
))]
mod r#merchant;
#[cfg(any(
    feature = "merchant-property-schema",
    feature = "general-schema-section"
))]
pub use r#merchant::*;
#[cfg(any(
    feature = "merchant-return-days-property-schema",
    feature = "pending-schema-section"
))]
mod r#merchant_return_days;
#[cfg(any(
    feature = "merchant-return-days-property-schema",
    feature = "pending-schema-section"
))]
pub use r#merchant_return_days::*;
#[cfg(any(
    feature = "merchant-return-link-property-schema",
    feature = "pending-schema-section"
))]
mod r#merchant_return_link;
#[cfg(any(
    feature = "merchant-return-link-property-schema",
    feature = "pending-schema-section"
))]
pub use r#merchant_return_link::*;
#[cfg(any(
    feature = "message-attachment-property-schema",
    feature = "general-schema-section"
))]
mod r#message_attachment;
#[cfg(any(
    feature = "message-attachment-property-schema",
    feature = "general-schema-section"
))]
pub use r#message_attachment::*;
#[cfg(any(
    feature = "mileage-from-odometer-property-schema",
    feature = "general-schema-section"
))]
mod r#mileage_from_odometer;
#[cfg(any(
    feature = "mileage-from-odometer-property-schema",
    feature = "general-schema-section"
))]
pub use r#mileage_from_odometer::*;
#[cfg(any(
    feature = "min-price-property-schema",
    feature = "general-schema-section"
))]
mod r#min_price;
#[cfg(any(
    feature = "min-price-property-schema",
    feature = "general-schema-section"
))]
pub use r#min_price::*;
#[cfg(any(
    feature = "min-value-property-schema",
    feature = "general-schema-section"
))]
mod r#min_value;
#[cfg(any(
    feature = "min-value-property-schema",
    feature = "general-schema-section"
))]
pub use r#min_value::*;
#[cfg(any(
    feature = "minimum-payment-due-property-schema",
    feature = "general-schema-section"
))]
mod r#minimum_payment_due;
#[cfg(any(
    feature = "minimum-payment-due-property-schema",
    feature = "general-schema-section"
))]
pub use r#minimum_payment_due::*;
#[cfg(any(
    feature = "mission-coverage-priorities-policy-property-schema",
    feature = "pending-schema-section"
))]
mod r#mission_coverage_priorities_policy;
#[cfg(any(
    feature = "mission-coverage-priorities-policy-property-schema",
    feature = "pending-schema-section"
))]
pub use r#mission_coverage_priorities_policy::*;
#[cfg(any(
    feature = "mobile-url-property-schema",
    feature = "pending-schema-section"
))]
mod r#mobile_url;
#[cfg(any(
    feature = "mobile-url-property-schema",
    feature = "pending-schema-section"
))]
pub use r#mobile_url::*;
#[cfg(any(feature = "model-property-schema", feature = "general-schema-section"))]
mod r#model;
#[cfg(any(feature = "model-property-schema", feature = "general-schema-section"))]
pub use r#model::*;
#[cfg(any(
    feature = "model-date-property-schema",
    feature = "auto-schema-section"
))]
mod r#model_date;
#[cfg(any(
    feature = "model-date-property-schema",
    feature = "auto-schema-section"
))]
pub use r#model_date::*;
#[cfg(any(
    feature = "modified-time-property-schema",
    feature = "general-schema-section"
))]
mod r#modified_time;
#[cfg(any(
    feature = "modified-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#modified_time::*;
#[cfg(any(
    feature = "molecular-formula-property-schema",
    feature = "pending-schema-section"
))]
mod r#molecular_formula;
#[cfg(any(
    feature = "molecular-formula-property-schema",
    feature = "pending-schema-section"
))]
pub use r#molecular_formula::*;
#[cfg(any(
    feature = "molecular-weight-property-schema",
    feature = "pending-schema-section"
))]
mod r#molecular_weight;
#[cfg(any(
    feature = "molecular-weight-property-schema",
    feature = "pending-schema-section"
))]
pub use r#molecular_weight::*;
#[cfg(any(
    feature = "monoisotopic-molecular-weight-property-schema",
    feature = "pending-schema-section"
))]
mod r#monoisotopic_molecular_weight;
#[cfg(any(
    feature = "monoisotopic-molecular-weight-property-schema",
    feature = "pending-schema-section"
))]
pub use r#monoisotopic_molecular_weight::*;
#[cfg(any(
    feature = "monthly-minimum-repayment-amount-property-schema",
    feature = "pending-schema-section"
))]
mod r#monthly_minimum_repayment_amount;
#[cfg(any(
    feature = "monthly-minimum-repayment-amount-property-schema",
    feature = "pending-schema-section"
))]
pub use r#monthly_minimum_repayment_amount::*;
#[cfg(any(
    feature = "months-of-experience-property-schema",
    feature = "pending-schema-section"
))]
mod r#months_of_experience;
#[cfg(any(
    feature = "months-of-experience-property-schema",
    feature = "pending-schema-section"
))]
pub use r#months_of_experience::*;
#[cfg(any(feature = "mpn-property-schema", feature = "general-schema-section"))]
mod r#mpn;
#[cfg(any(feature = "mpn-property-schema", feature = "general-schema-section"))]
pub use r#mpn::*;
#[cfg(any(
    feature = "multiple-values-property-schema",
    feature = "general-schema-section"
))]
mod r#multiple_values;
#[cfg(any(
    feature = "multiple-values-property-schema",
    feature = "general-schema-section"
))]
pub use r#multiple_values::*;
#[cfg(any(
    feature = "muscle-action-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#muscle_action;
#[cfg(any(
    feature = "muscle-action-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#muscle_action::*;
#[cfg(any(
    feature = "music-arrangement-property-schema",
    feature = "general-schema-section"
))]
mod r#music_arrangement;
#[cfg(any(
    feature = "music-arrangement-property-schema",
    feature = "general-schema-section"
))]
pub use r#music_arrangement::*;
#[cfg(any(
    feature = "music-by-property-schema",
    feature = "general-schema-section"
))]
mod r#music_by;
#[cfg(any(
    feature = "music-by-property-schema",
    feature = "general-schema-section"
))]
pub use r#music_by::*;
#[cfg(any(
    feature = "music-composition-form-property-schema",
    feature = "general-schema-section"
))]
mod r#music_composition_form;
#[cfg(any(
    feature = "music-composition-form-property-schema",
    feature = "general-schema-section"
))]
pub use r#music_composition_form::*;
#[cfg(any(
    feature = "music-group-member-property-schema",
    feature = "general-schema-section"
))]
mod r#music_group_member;
#[cfg(any(
    feature = "music-group-member-property-schema",
    feature = "general-schema-section"
))]
pub use r#music_group_member::*;
#[cfg(any(
    feature = "music-release-format-property-schema",
    feature = "general-schema-section"
))]
mod r#music_release_format;
#[cfg(any(
    feature = "music-release-format-property-schema",
    feature = "general-schema-section"
))]
pub use r#music_release_format::*;
#[cfg(any(
    feature = "musical-key-property-schema",
    feature = "general-schema-section"
))]
mod r#musical_key;
#[cfg(any(
    feature = "musical-key-property-schema",
    feature = "general-schema-section"
))]
pub use r#musical_key::*;
#[cfg(any(feature = "naics-property-schema", feature = "general-schema-section"))]
mod r#naics;
#[cfg(any(feature = "naics-property-schema", feature = "general-schema-section"))]
pub use r#naics::*;
#[cfg(any(feature = "name-property-schema", feature = "general-schema-section"))]
mod r#name;
#[cfg(any(feature = "name-property-schema", feature = "general-schema-section"))]
pub use r#name::*;
#[cfg(any(
    feature = "named-position-property-schema",
    feature = "general-schema-section"
))]
mod r#named_position;
#[cfg(any(
    feature = "named-position-property-schema",
    feature = "general-schema-section"
))]
pub use r#named_position::*;
#[cfg(any(
    feature = "nationality-property-schema",
    feature = "general-schema-section"
))]
mod r#nationality;
#[cfg(any(
    feature = "nationality-property-schema",
    feature = "general-schema-section"
))]
pub use r#nationality::*;
#[cfg(any(
    feature = "natural-progression-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#natural_progression;
#[cfg(any(
    feature = "natural-progression-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#natural_progression::*;
#[cfg(any(
    feature = "negative-notes-property-schema",
    feature = "pending-schema-section"
))]
mod r#negative_notes;
#[cfg(any(
    feature = "negative-notes-property-schema",
    feature = "pending-schema-section"
))]
pub use r#negative_notes::*;
#[cfg(any(
    feature = "nerve-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#nerve;
#[cfg(any(
    feature = "nerve-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#nerve::*;
#[cfg(any(
    feature = "nerve-motor-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#nerve_motor;
#[cfg(any(
    feature = "nerve-motor-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#nerve_motor::*;
#[cfg(any(
    feature = "net-worth-property-schema",
    feature = "general-schema-section"
))]
mod r#net_worth;
#[cfg(any(
    feature = "net-worth-property-schema",
    feature = "general-schema-section"
))]
pub use r#net_worth::*;
#[cfg(any(
    feature = "news-updates-and-guidelines-property-schema",
    feature = "pending-schema-section"
))]
mod r#news_updates_and_guidelines;
#[cfg(any(
    feature = "news-updates-and-guidelines-property-schema",
    feature = "pending-schema-section"
))]
pub use r#news_updates_and_guidelines::*;
#[cfg(any(
    feature = "next-item-property-schema",
    feature = "general-schema-section"
))]
mod r#next_item;
#[cfg(any(
    feature = "next-item-property-schema",
    feature = "general-schema-section"
))]
pub use r#next_item::*;
#[cfg(any(
    feature = "no-bylines-policy-property-schema",
    feature = "pending-schema-section"
))]
mod r#no_bylines_policy;
#[cfg(any(
    feature = "no-bylines-policy-property-schema",
    feature = "pending-schema-section"
))]
pub use r#no_bylines_policy::*;
#[cfg(any(
    feature = "non-equal-property-schema",
    feature = "general-schema-section"
))]
mod r#non_equal;
#[cfg(any(
    feature = "non-equal-property-schema",
    feature = "general-schema-section"
))]
pub use r#non_equal::*;
#[cfg(any(
    feature = "non-proprietary-name-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#non_proprietary_name;
#[cfg(any(
    feature = "non-proprietary-name-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#non_proprietary_name::*;
#[cfg(any(
    feature = "nonprofit-status-property-schema",
    feature = "pending-schema-section"
))]
mod r#nonprofit_status;
#[cfg(any(
    feature = "nonprofit-status-property-schema",
    feature = "pending-schema-section"
))]
pub use r#nonprofit_status::*;
#[cfg(any(
    feature = "normal-range-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#normal_range;
#[cfg(any(
    feature = "normal-range-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#normal_range::*;
#[cfg(any(feature = "nsn-property-schema", feature = "pending-schema-section"))]
mod r#nsn;
#[cfg(any(feature = "nsn-property-schema", feature = "pending-schema-section"))]
pub use r#nsn::*;
#[cfg(any(
    feature = "num-adults-property-schema",
    feature = "general-schema-section"
))]
mod r#num_adults;
#[cfg(any(
    feature = "num-adults-property-schema",
    feature = "general-schema-section"
))]
pub use r#num_adults::*;
#[cfg(any(
    feature = "num-children-property-schema",
    feature = "general-schema-section"
))]
mod r#num_children;
#[cfg(any(
    feature = "num-children-property-schema",
    feature = "general-schema-section"
))]
pub use r#num_children::*;
#[cfg(any(
    feature = "num-constraints-property-schema",
    feature = "pending-schema-section"
))]
mod r#num_constraints;
#[cfg(any(
    feature = "num-constraints-property-schema",
    feature = "pending-schema-section"
))]
pub use r#num_constraints::*;
#[cfg(any(
    feature = "num-tracks-property-schema",
    feature = "general-schema-section"
))]
mod r#num_tracks;
#[cfg(any(
    feature = "num-tracks-property-schema",
    feature = "general-schema-section"
))]
pub use r#num_tracks::*;
#[cfg(any(
    feature = "number-of-accommodation-units-property-schema",
    feature = "pending-schema-section"
))]
mod r#number_of_accommodation_units;
#[cfg(any(
    feature = "number-of-accommodation-units-property-schema",
    feature = "pending-schema-section"
))]
pub use r#number_of_accommodation_units::*;
#[cfg(any(
    feature = "number-of-airbags-property-schema",
    feature = "general-schema-section"
))]
mod r#number_of_airbags;
#[cfg(any(
    feature = "number-of-airbags-property-schema",
    feature = "general-schema-section"
))]
pub use r#number_of_airbags::*;
#[cfg(any(
    feature = "number-of-available-accommodation-units-property-schema",
    feature = "pending-schema-section"
))]
mod r#number_of_available_accommodation_units;
#[cfg(any(
    feature = "number-of-available-accommodation-units-property-schema",
    feature = "pending-schema-section"
))]
pub use r#number_of_available_accommodation_units::*;
#[cfg(any(
    feature = "number-of-axles-property-schema",
    feature = "general-schema-section"
))]
mod r#number_of_axles;
#[cfg(any(
    feature = "number-of-axles-property-schema",
    feature = "general-schema-section"
))]
pub use r#number_of_axles::*;
#[cfg(any(
    feature = "number-of-bathrooms-total-property-schema",
    feature = "pending-schema-section"
))]
mod r#number_of_bathrooms_total;
#[cfg(any(
    feature = "number-of-bathrooms-total-property-schema",
    feature = "pending-schema-section"
))]
pub use r#number_of_bathrooms_total::*;
#[cfg(any(
    feature = "number-of-bedrooms-property-schema",
    feature = "pending-schema-section"
))]
mod r#number_of_bedrooms;
#[cfg(any(
    feature = "number-of-bedrooms-property-schema",
    feature = "pending-schema-section"
))]
pub use r#number_of_bedrooms::*;
#[cfg(any(
    feature = "number-of-beds-property-schema",
    feature = "general-schema-section"
))]
mod r#number_of_beds;
#[cfg(any(
    feature = "number-of-beds-property-schema",
    feature = "general-schema-section"
))]
pub use r#number_of_beds::*;
#[cfg(any(
    feature = "number-of-credits-property-schema",
    feature = "pending-schema-section"
))]
mod r#number_of_credits;
#[cfg(any(
    feature = "number-of-credits-property-schema",
    feature = "pending-schema-section"
))]
pub use r#number_of_credits::*;
#[cfg(any(
    feature = "number-of-doors-property-schema",
    feature = "general-schema-section"
))]
mod r#number_of_doors;
#[cfg(any(
    feature = "number-of-doors-property-schema",
    feature = "general-schema-section"
))]
pub use r#number_of_doors::*;
#[cfg(any(
    feature = "number-of-employees-property-schema",
    feature = "general-schema-section"
))]
mod r#number_of_employees;
#[cfg(any(
    feature = "number-of-employees-property-schema",
    feature = "general-schema-section"
))]
pub use r#number_of_employees::*;
#[cfg(any(
    feature = "number-of-episodes-property-schema",
    feature = "general-schema-section"
))]
mod r#number_of_episodes;
#[cfg(any(
    feature = "number-of-episodes-property-schema",
    feature = "general-schema-section"
))]
pub use r#number_of_episodes::*;
#[cfg(any(
    feature = "number-of-forward-gears-property-schema",
    feature = "general-schema-section"
))]
mod r#number_of_forward_gears;
#[cfg(any(
    feature = "number-of-forward-gears-property-schema",
    feature = "general-schema-section"
))]
pub use r#number_of_forward_gears::*;
#[cfg(any(
    feature = "number-of-full-bathrooms-property-schema",
    feature = "pending-schema-section"
))]
mod r#number_of_full_bathrooms;
#[cfg(any(
    feature = "number-of-full-bathrooms-property-schema",
    feature = "pending-schema-section"
))]
pub use r#number_of_full_bathrooms::*;
#[cfg(any(
    feature = "number-of-items-property-schema",
    feature = "general-schema-section"
))]
mod r#number_of_items;
#[cfg(any(
    feature = "number-of-items-property-schema",
    feature = "general-schema-section"
))]
pub use r#number_of_items::*;
#[cfg(any(
    feature = "number-of-loan-payments-property-schema",
    feature = "pending-schema-section"
))]
mod r#number_of_loan_payments;
#[cfg(any(
    feature = "number-of-loan-payments-property-schema",
    feature = "pending-schema-section"
))]
pub use r#number_of_loan_payments::*;
#[cfg(any(
    feature = "number-of-pages-property-schema",
    feature = "general-schema-section"
))]
mod r#number_of_pages;
#[cfg(any(
    feature = "number-of-pages-property-schema",
    feature = "general-schema-section"
))]
pub use r#number_of_pages::*;
#[cfg(any(
    feature = "number-of-partial-bathrooms-property-schema",
    feature = "pending-schema-section"
))]
mod r#number_of_partial_bathrooms;
#[cfg(any(
    feature = "number-of-partial-bathrooms-property-schema",
    feature = "pending-schema-section"
))]
pub use r#number_of_partial_bathrooms::*;
#[cfg(any(
    feature = "number-of-players-property-schema",
    feature = "general-schema-section"
))]
mod r#number_of_players;
#[cfg(any(
    feature = "number-of-players-property-schema",
    feature = "general-schema-section"
))]
pub use r#number_of_players::*;
#[cfg(any(
    feature = "number-of-previous-owners-property-schema",
    feature = "general-schema-section"
))]
mod r#number_of_previous_owners;
#[cfg(any(
    feature = "number-of-previous-owners-property-schema",
    feature = "general-schema-section"
))]
pub use r#number_of_previous_owners::*;
#[cfg(any(
    feature = "number-of-rooms-property-schema",
    feature = "general-schema-section"
))]
mod r#number_of_rooms;
#[cfg(any(
    feature = "number-of-rooms-property-schema",
    feature = "general-schema-section"
))]
pub use r#number_of_rooms::*;
#[cfg(any(
    feature = "number-of-seasons-property-schema",
    feature = "general-schema-section"
))]
mod r#number_of_seasons;
#[cfg(any(
    feature = "number-of-seasons-property-schema",
    feature = "general-schema-section"
))]
pub use r#number_of_seasons::*;
#[cfg(any(
    feature = "numbered-position-property-schema",
    feature = "general-schema-section"
))]
mod r#numbered_position;
#[cfg(any(
    feature = "numbered-position-property-schema",
    feature = "general-schema-section"
))]
pub use r#numbered_position::*;
#[cfg(any(
    feature = "nutrition-property-schema",
    feature = "general-schema-section"
))]
mod r#nutrition;
#[cfg(any(
    feature = "nutrition-property-schema",
    feature = "general-schema-section"
))]
pub use r#nutrition::*;
#[cfg(any(feature = "object-property-schema", feature = "general-schema-section"))]
mod r#object;
#[cfg(any(feature = "object-property-schema", feature = "general-schema-section"))]
pub use r#object::*;
#[cfg(any(
    feature = "observation-about-property-schema",
    feature = "pending-schema-section"
))]
mod r#observation_about;
#[cfg(any(
    feature = "observation-about-property-schema",
    feature = "pending-schema-section"
))]
pub use r#observation_about::*;
#[cfg(any(
    feature = "observation-date-property-schema",
    feature = "pending-schema-section"
))]
mod r#observation_date;
#[cfg(any(
    feature = "observation-date-property-schema",
    feature = "pending-schema-section"
))]
pub use r#observation_date::*;
#[cfg(any(
    feature = "observation-period-property-schema",
    feature = "pending-schema-section"
))]
mod r#observation_period;
#[cfg(any(
    feature = "observation-period-property-schema",
    feature = "pending-schema-section"
))]
pub use r#observation_period::*;
#[cfg(any(
    feature = "occupancy-property-schema",
    feature = "general-schema-section"
))]
mod r#occupancy;
#[cfg(any(
    feature = "occupancy-property-schema",
    feature = "general-schema-section"
))]
pub use r#occupancy::*;
#[cfg(any(
    feature = "occupation-location-property-schema",
    feature = "general-schema-section"
))]
mod r#occupation_location;
#[cfg(any(
    feature = "occupation-location-property-schema",
    feature = "general-schema-section"
))]
pub use r#occupation_location::*;
#[cfg(any(
    feature = "occupational-category-property-schema",
    feature = "pending-schema-section"
))]
mod r#occupational_category;
#[cfg(any(
    feature = "occupational-category-property-schema",
    feature = "pending-schema-section"
))]
pub use r#occupational_category::*;
#[cfg(any(
    feature = "occupational-credential-awarded-property-schema",
    feature = "pending-schema-section"
))]
mod r#occupational_credential_awarded;
#[cfg(any(
    feature = "occupational-credential-awarded-property-schema",
    feature = "pending-schema-section"
))]
pub use r#occupational_credential_awarded::*;
#[cfg(any(
    feature = "offer-count-property-schema",
    feature = "general-schema-section"
))]
mod r#offer_count;
#[cfg(any(
    feature = "offer-count-property-schema",
    feature = "general-schema-section"
))]
pub use r#offer_count::*;
#[cfg(any(
    feature = "offered-by-property-schema",
    feature = "general-schema-section"
))]
mod r#offered_by;
#[cfg(any(
    feature = "offered-by-property-schema",
    feature = "general-schema-section"
))]
pub use r#offered_by::*;
#[cfg(any(feature = "offers-property-schema", feature = "general-schema-section"))]
mod r#offers;
#[cfg(any(feature = "offers-property-schema", feature = "general-schema-section"))]
pub use r#offers::*;
#[cfg(any(
    feature = "offers-prescription-by-mail-property-schema",
    feature = "pending-schema-section"
))]
mod r#offers_prescription_by_mail;
#[cfg(any(
    feature = "offers-prescription-by-mail-property-schema",
    feature = "pending-schema-section"
))]
pub use r#offers_prescription_by_mail::*;
#[cfg(any(
    feature = "opening-hours-property-schema",
    feature = "general-schema-section"
))]
mod r#opening_hours;
#[cfg(any(
    feature = "opening-hours-property-schema",
    feature = "general-schema-section"
))]
pub use r#opening_hours::*;
#[cfg(any(
    feature = "opening-hours-specification-property-schema",
    feature = "general-schema-section"
))]
mod r#opening_hours_specification;
#[cfg(any(
    feature = "opening-hours-specification-property-schema",
    feature = "general-schema-section"
))]
pub use r#opening_hours_specification::*;
#[cfg(any(feature = "opens-property-schema", feature = "general-schema-section"))]
mod r#opens;
#[cfg(any(feature = "opens-property-schema", feature = "general-schema-section"))]
pub use r#opens::*;
#[cfg(any(
    feature = "operating-system-property-schema",
    feature = "general-schema-section"
))]
mod r#operating_system;
#[cfg(any(
    feature = "operating-system-property-schema",
    feature = "general-schema-section"
))]
pub use r#operating_system::*;
#[cfg(any(
    feature = "opponent-property-schema",
    feature = "general-schema-section"
))]
mod r#opponent;
#[cfg(any(
    feature = "opponent-property-schema",
    feature = "general-schema-section"
))]
pub use r#opponent::*;
#[cfg(any(feature = "option-property-schema", feature = "general-schema-section"))]
mod r#option;
#[cfg(any(feature = "option-property-schema", feature = "general-schema-section"))]
pub use r#option::*;
#[cfg(any(
    feature = "order-date-property-schema",
    feature = "general-schema-section"
))]
mod r#order_date;
#[cfg(any(
    feature = "order-date-property-schema",
    feature = "general-schema-section"
))]
pub use r#order_date::*;
#[cfg(any(
    feature = "order-delivery-property-schema",
    feature = "general-schema-section"
))]
mod r#order_delivery;
#[cfg(any(
    feature = "order-delivery-property-schema",
    feature = "general-schema-section"
))]
pub use r#order_delivery::*;
#[cfg(any(
    feature = "order-item-number-property-schema",
    feature = "general-schema-section"
))]
mod r#order_item_number;
#[cfg(any(
    feature = "order-item-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#order_item_number::*;
#[cfg(any(
    feature = "order-item-status-property-schema",
    feature = "general-schema-section"
))]
mod r#order_item_status;
#[cfg(any(
    feature = "order-item-status-property-schema",
    feature = "general-schema-section"
))]
pub use r#order_item_status::*;
#[cfg(any(
    feature = "order-number-property-schema",
    feature = "general-schema-section"
))]
mod r#order_number;
#[cfg(any(
    feature = "order-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#order_number::*;
#[cfg(any(
    feature = "order-quantity-property-schema",
    feature = "general-schema-section"
))]
mod r#order_quantity;
#[cfg(any(
    feature = "order-quantity-property-schema",
    feature = "general-schema-section"
))]
pub use r#order_quantity::*;
#[cfg(any(
    feature = "order-status-property-schema",
    feature = "general-schema-section"
))]
mod r#order_status;
#[cfg(any(
    feature = "order-status-property-schema",
    feature = "general-schema-section"
))]
pub use r#order_status::*;
#[cfg(any(
    feature = "ordered-item-property-schema",
    feature = "general-schema-section"
))]
mod r#ordered_item;
#[cfg(any(
    feature = "ordered-item-property-schema",
    feature = "general-schema-section"
))]
pub use r#ordered_item::*;
#[cfg(any(
    feature = "organizer-property-schema",
    feature = "general-schema-section"
))]
mod r#organizer;
#[cfg(any(
    feature = "organizer-property-schema",
    feature = "general-schema-section"
))]
pub use r#organizer::*;
#[cfg(any(
    feature = "origin-address-property-schema",
    feature = "general-schema-section"
))]
mod r#origin_address;
#[cfg(any(
    feature = "origin-address-property-schema",
    feature = "general-schema-section"
))]
pub use r#origin_address::*;
#[cfg(any(
    feature = "original-media-context-description-property-schema",
    feature = "pending-schema-section"
))]
mod r#original_media_context_description;
#[cfg(any(
    feature = "original-media-context-description-property-schema",
    feature = "pending-schema-section"
))]
pub use r#original_media_context_description::*;
#[cfg(any(
    feature = "original-media-link-property-schema",
    feature = "pending-schema-section"
))]
mod r#original_media_link;
#[cfg(any(
    feature = "original-media-link-property-schema",
    feature = "pending-schema-section"
))]
pub use r#original_media_link::*;
#[cfg(any(
    feature = "originates-from-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#originates_from;
#[cfg(any(
    feature = "originates-from-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#originates_from::*;
#[cfg(any(
    feature = "overdosage-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#overdosage;
#[cfg(any(
    feature = "overdosage-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#overdosage::*;
#[cfg(any(
    feature = "owned-from-property-schema",
    feature = "general-schema-section"
))]
mod r#owned_from;
#[cfg(any(
    feature = "owned-from-property-schema",
    feature = "general-schema-section"
))]
pub use r#owned_from::*;
#[cfg(any(
    feature = "owned-through-property-schema",
    feature = "general-schema-section"
))]
mod r#owned_through;
#[cfg(any(
    feature = "owned-through-property-schema",
    feature = "general-schema-section"
))]
pub use r#owned_through::*;
#[cfg(any(
    feature = "ownership-funding-info-property-schema",
    feature = "pending-schema-section"
))]
mod r#ownership_funding_info;
#[cfg(any(
    feature = "ownership-funding-info-property-schema",
    feature = "pending-schema-section"
))]
pub use r#ownership_funding_info::*;
#[cfg(any(feature = "owns-property-schema", feature = "general-schema-section"))]
mod r#owns;
#[cfg(any(feature = "owns-property-schema", feature = "general-schema-section"))]
pub use r#owns::*;
#[cfg(any(
    feature = "page-end-property-schema",
    feature = "general-schema-section"
))]
mod r#page_end;
#[cfg(any(
    feature = "page-end-property-schema",
    feature = "general-schema-section"
))]
pub use r#page_end::*;
#[cfg(any(
    feature = "page-start-property-schema",
    feature = "general-schema-section"
))]
mod r#page_start;
#[cfg(any(
    feature = "page-start-property-schema",
    feature = "general-schema-section"
))]
pub use r#page_start::*;
#[cfg(any(
    feature = "pagination-property-schema",
    feature = "general-schema-section"
))]
mod r#pagination;
#[cfg(any(
    feature = "pagination-property-schema",
    feature = "general-schema-section"
))]
pub use r#pagination::*;
#[cfg(any(feature = "parent-property-schema", feature = "general-schema-section"))]
mod r#parent;
#[cfg(any(feature = "parent-property-schema", feature = "general-schema-section"))]
pub use r#parent::*;
#[cfg(any(
    feature = "parent-item-property-schema",
    feature = "general-schema-section"
))]
mod r#parent_item;
#[cfg(any(
    feature = "parent-item-property-schema",
    feature = "general-schema-section"
))]
pub use r#parent_item::*;
#[cfg(any(
    feature = "parent-organization-property-schema",
    feature = "general-schema-section"
))]
mod r#parent_organization;
#[cfg(any(
    feature = "parent-organization-property-schema",
    feature = "general-schema-section"
))]
pub use r#parent_organization::*;
#[cfg(any(
    feature = "parent-service-property-schema",
    feature = "general-schema-section"
))]
mod r#parent_service;
#[cfg(any(
    feature = "parent-service-property-schema",
    feature = "general-schema-section"
))]
pub use r#parent_service::*;
#[cfg(any(
    feature = "parent-taxon-property-schema",
    feature = "pending-schema-section"
))]
mod r#parent_taxon;
#[cfg(any(
    feature = "parent-taxon-property-schema",
    feature = "pending-schema-section"
))]
pub use r#parent_taxon::*;
#[cfg(any(
    feature = "parents-property-schema",
    feature = "general-schema-section"
))]
mod r#parents;
#[cfg(any(
    feature = "parents-property-schema",
    feature = "general-schema-section"
))]
pub use r#parents::*;
#[cfg(any(
    feature = "part-of-episode-property-schema",
    feature = "general-schema-section"
))]
mod r#part_of_episode;
#[cfg(any(
    feature = "part-of-episode-property-schema",
    feature = "general-schema-section"
))]
pub use r#part_of_episode::*;
#[cfg(any(
    feature = "part-of-invoice-property-schema",
    feature = "general-schema-section"
))]
mod r#part_of_invoice;
#[cfg(any(
    feature = "part-of-invoice-property-schema",
    feature = "general-schema-section"
))]
pub use r#part_of_invoice::*;
#[cfg(any(
    feature = "part-of-order-property-schema",
    feature = "general-schema-section"
))]
mod r#part_of_order;
#[cfg(any(
    feature = "part-of-order-property-schema",
    feature = "general-schema-section"
))]
pub use r#part_of_order::*;
#[cfg(any(
    feature = "part-of-season-property-schema",
    feature = "general-schema-section"
))]
mod r#part_of_season;
#[cfg(any(
    feature = "part-of-season-property-schema",
    feature = "general-schema-section"
))]
pub use r#part_of_season::*;
#[cfg(any(
    feature = "part-of-series-property-schema",
    feature = "general-schema-section"
))]
mod r#part_of_series;
#[cfg(any(
    feature = "part-of-series-property-schema",
    feature = "general-schema-section"
))]
pub use r#part_of_series::*;
#[cfg(any(
    feature = "part-of-system-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#part_of_system;
#[cfg(any(
    feature = "part-of-system-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#part_of_system::*;
#[cfg(any(
    feature = "part-of-tv-series-property-schema",
    feature = "general-schema-section"
))]
mod r#part_of_tv_series;
#[cfg(any(
    feature = "part-of-tv-series-property-schema",
    feature = "general-schema-section"
))]
pub use r#part_of_tv_series::*;
#[cfg(any(
    feature = "part-of-trip-property-schema",
    feature = "pending-schema-section"
))]
mod r#part_of_trip;
#[cfg(any(
    feature = "part-of-trip-property-schema",
    feature = "pending-schema-section"
))]
pub use r#part_of_trip::*;
#[cfg(any(
    feature = "participant-property-schema",
    feature = "general-schema-section"
))]
mod r#participant;
#[cfg(any(
    feature = "participant-property-schema",
    feature = "general-schema-section"
))]
pub use r#participant::*;
#[cfg(any(
    feature = "party-size-property-schema",
    feature = "general-schema-section"
))]
mod r#party_size;
#[cfg(any(
    feature = "party-size-property-schema",
    feature = "general-schema-section"
))]
pub use r#party_size::*;
#[cfg(any(
    feature = "passenger-priority-status-property-schema",
    feature = "general-schema-section"
))]
mod r#passenger_priority_status;
#[cfg(any(
    feature = "passenger-priority-status-property-schema",
    feature = "general-schema-section"
))]
pub use r#passenger_priority_status::*;
#[cfg(any(
    feature = "passenger-sequence-number-property-schema",
    feature = "general-schema-section"
))]
mod r#passenger_sequence_number;
#[cfg(any(
    feature = "passenger-sequence-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#passenger_sequence_number::*;
#[cfg(any(
    feature = "pathophysiology-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#pathophysiology;
#[cfg(any(
    feature = "pathophysiology-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#pathophysiology::*;
#[cfg(any(
    feature = "pattern-property-schema",
    feature = "pending-schema-section"
))]
mod r#pattern;
#[cfg(any(
    feature = "pattern-property-schema",
    feature = "pending-schema-section"
))]
pub use r#pattern::*;
#[cfg(any(feature = "payload-property-schema", feature = "auto-schema-section"))]
mod r#payload;
#[cfg(any(feature = "payload-property-schema", feature = "auto-schema-section"))]
pub use r#payload::*;
#[cfg(any(
    feature = "payment-accepted-property-schema",
    feature = "general-schema-section"
))]
mod r#payment_accepted;
#[cfg(any(
    feature = "payment-accepted-property-schema",
    feature = "general-schema-section"
))]
pub use r#payment_accepted::*;
#[cfg(any(
    feature = "payment-due-property-schema",
    feature = "general-schema-section"
))]
mod r#payment_due;
#[cfg(any(
    feature = "payment-due-property-schema",
    feature = "general-schema-section"
))]
pub use r#payment_due::*;
#[cfg(any(
    feature = "payment-due-date-property-schema",
    feature = "general-schema-section"
))]
mod r#payment_due_date;
#[cfg(any(
    feature = "payment-due-date-property-schema",
    feature = "general-schema-section"
))]
pub use r#payment_due_date::*;
#[cfg(any(
    feature = "payment-method-property-schema",
    feature = "general-schema-section"
))]
mod r#payment_method;
#[cfg(any(
    feature = "payment-method-property-schema",
    feature = "general-schema-section"
))]
pub use r#payment_method::*;
#[cfg(any(
    feature = "payment-method-id-property-schema",
    feature = "general-schema-section"
))]
mod r#payment_method_id;
#[cfg(any(
    feature = "payment-method-id-property-schema",
    feature = "general-schema-section"
))]
pub use r#payment_method_id::*;
#[cfg(any(
    feature = "payment-status-property-schema",
    feature = "general-schema-section"
))]
mod r#payment_status;
#[cfg(any(
    feature = "payment-status-property-schema",
    feature = "general-schema-section"
))]
pub use r#payment_status::*;
#[cfg(any(
    feature = "payment-url-property-schema",
    feature = "general-schema-section"
))]
mod r#payment_url;
#[cfg(any(
    feature = "payment-url-property-schema",
    feature = "general-schema-section"
))]
pub use r#payment_url::*;
#[cfg(any(feature = "penciler-property-schema", feature = "bib-schema-section"))]
mod r#penciler;
#[cfg(any(feature = "penciler-property-schema", feature = "bib-schema-section"))]
pub use r#penciler::*;
#[cfg(any(
    feature = "percentile-10-property-schema",
    feature = "general-schema-section"
))]
mod r#percentile_10;
#[cfg(any(
    feature = "percentile-10-property-schema",
    feature = "general-schema-section"
))]
pub use r#percentile_10::*;
#[cfg(any(
    feature = "percentile-25-property-schema",
    feature = "general-schema-section"
))]
mod r#percentile_25;
#[cfg(any(
    feature = "percentile-25-property-schema",
    feature = "general-schema-section"
))]
pub use r#percentile_25::*;
#[cfg(any(
    feature = "percentile-75-property-schema",
    feature = "general-schema-section"
))]
mod r#percentile_75;
#[cfg(any(
    feature = "percentile-75-property-schema",
    feature = "general-schema-section"
))]
pub use r#percentile_75::*;
#[cfg(any(
    feature = "percentile-90-property-schema",
    feature = "general-schema-section"
))]
mod r#percentile_90;
#[cfg(any(
    feature = "percentile-90-property-schema",
    feature = "general-schema-section"
))]
pub use r#percentile_90::*;
#[cfg(any(
    feature = "perform-time-property-schema",
    feature = "general-schema-section"
))]
mod r#perform_time;
#[cfg(any(
    feature = "perform-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#perform_time::*;
#[cfg(any(
    feature = "performer-property-schema",
    feature = "general-schema-section"
))]
mod r#performer;
#[cfg(any(
    feature = "performer-property-schema",
    feature = "general-schema-section"
))]
pub use r#performer::*;
#[cfg(any(
    feature = "performer-in-property-schema",
    feature = "general-schema-section"
))]
mod r#performer_in;
#[cfg(any(
    feature = "performer-in-property-schema",
    feature = "general-schema-section"
))]
pub use r#performer_in::*;
#[cfg(any(
    feature = "performers-property-schema",
    feature = "general-schema-section"
))]
mod r#performers;
#[cfg(any(
    feature = "performers-property-schema",
    feature = "general-schema-section"
))]
pub use r#performers::*;
#[cfg(any(
    feature = "permission-type-property-schema",
    feature = "general-schema-section"
))]
mod r#permission_type;
#[cfg(any(
    feature = "permission-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#permission_type::*;
#[cfg(any(
    feature = "permissions-property-schema",
    feature = "general-schema-section"
))]
mod r#permissions;
#[cfg(any(
    feature = "permissions-property-schema",
    feature = "general-schema-section"
))]
pub use r#permissions::*;
#[cfg(any(
    feature = "permit-audience-property-schema",
    feature = "general-schema-section"
))]
mod r#permit_audience;
#[cfg(any(
    feature = "permit-audience-property-schema",
    feature = "general-schema-section"
))]
pub use r#permit_audience::*;
#[cfg(any(
    feature = "permitted-usage-property-schema",
    feature = "general-schema-section"
))]
mod r#permitted_usage;
#[cfg(any(
    feature = "permitted-usage-property-schema",
    feature = "general-schema-section"
))]
pub use r#permitted_usage::*;
#[cfg(any(
    feature = "pets-allowed-property-schema",
    feature = "general-schema-section"
))]
mod r#pets_allowed;
#[cfg(any(
    feature = "pets-allowed-property-schema",
    feature = "general-schema-section"
))]
pub use r#pets_allowed::*;
#[cfg(any(
    feature = "phonetic-text-property-schema",
    feature = "pending-schema-section"
))]
mod r#phonetic_text;
#[cfg(any(
    feature = "phonetic-text-property-schema",
    feature = "pending-schema-section"
))]
pub use r#phonetic_text::*;
#[cfg(any(feature = "photo-property-schema", feature = "general-schema-section"))]
mod r#photo;
#[cfg(any(feature = "photo-property-schema", feature = "general-schema-section"))]
pub use r#photo::*;
#[cfg(any(feature = "photos-property-schema", feature = "general-schema-section"))]
mod r#photos;
#[cfg(any(feature = "photos-property-schema", feature = "general-schema-section"))]
pub use r#photos::*;
#[cfg(any(
    feature = "physical-requirement-property-schema",
    feature = "pending-schema-section"
))]
mod r#physical_requirement;
#[cfg(any(
    feature = "physical-requirement-property-schema",
    feature = "pending-schema-section"
))]
pub use r#physical_requirement::*;
#[cfg(any(
    feature = "physiological-benefits-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#physiological_benefits;
#[cfg(any(
    feature = "physiological-benefits-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#physiological_benefits::*;
#[cfg(any(
    feature = "pickup-location-property-schema",
    feature = "general-schema-section"
))]
mod r#pickup_location;
#[cfg(any(
    feature = "pickup-location-property-schema",
    feature = "general-schema-section"
))]
pub use r#pickup_location::*;
#[cfg(any(
    feature = "pickup-time-property-schema",
    feature = "general-schema-section"
))]
mod r#pickup_time;
#[cfg(any(
    feature = "pickup-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#pickup_time::*;
#[cfg(any(
    feature = "play-mode-property-schema",
    feature = "general-schema-section"
))]
mod r#play_mode;
#[cfg(any(
    feature = "play-mode-property-schema",
    feature = "general-schema-section"
))]
pub use r#play_mode::*;
#[cfg(any(
    feature = "player-type-property-schema",
    feature = "general-schema-section"
))]
mod r#player_type;
#[cfg(any(
    feature = "player-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#player_type::*;
#[cfg(any(
    feature = "players-online-property-schema",
    feature = "general-schema-section"
))]
mod r#players_online;
#[cfg(any(
    feature = "players-online-property-schema",
    feature = "general-schema-section"
))]
pub use r#players_online::*;
#[cfg(any(
    feature = "polygon-property-schema",
    feature = "general-schema-section"
))]
mod r#polygon;
#[cfg(any(
    feature = "polygon-property-schema",
    feature = "general-schema-section"
))]
pub use r#polygon::*;
#[cfg(any(
    feature = "population-type-property-schema",
    feature = "pending-schema-section"
))]
mod r#population_type;
#[cfg(any(
    feature = "population-type-property-schema",
    feature = "pending-schema-section"
))]
pub use r#population_type::*;
#[cfg(any(
    feature = "position-property-schema",
    feature = "general-schema-section"
))]
mod r#position;
#[cfg(any(
    feature = "position-property-schema",
    feature = "general-schema-section"
))]
pub use r#position::*;
#[cfg(any(
    feature = "positive-notes-property-schema",
    feature = "pending-schema-section"
))]
mod r#positive_notes;
#[cfg(any(
    feature = "positive-notes-property-schema",
    feature = "pending-schema-section"
))]
pub use r#positive_notes::*;
#[cfg(any(
    feature = "possible-complication-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#possible_complication;
#[cfg(any(
    feature = "possible-complication-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#possible_complication::*;
#[cfg(any(
    feature = "possible-treatment-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#possible_treatment;
#[cfg(any(
    feature = "possible-treatment-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#possible_treatment::*;
#[cfg(any(
    feature = "post-office-box-number-property-schema",
    feature = "general-schema-section"
))]
mod r#post_office_box_number;
#[cfg(any(
    feature = "post-office-box-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#post_office_box_number::*;
#[cfg(any(
    feature = "post-op-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#post_op;
#[cfg(any(
    feature = "post-op-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#post_op::*;
#[cfg(any(
    feature = "postal-code-property-schema",
    feature = "general-schema-section"
))]
mod r#postal_code;
#[cfg(any(
    feature = "postal-code-property-schema",
    feature = "general-schema-section"
))]
pub use r#postal_code::*;
#[cfg(any(
    feature = "postal-code-begin-property-schema",
    feature = "pending-schema-section"
))]
mod r#postal_code_begin;
#[cfg(any(
    feature = "postal-code-begin-property-schema",
    feature = "pending-schema-section"
))]
pub use r#postal_code_begin::*;
#[cfg(any(
    feature = "postal-code-end-property-schema",
    feature = "pending-schema-section"
))]
mod r#postal_code_end;
#[cfg(any(
    feature = "postal-code-end-property-schema",
    feature = "pending-schema-section"
))]
pub use r#postal_code_end::*;
#[cfg(any(
    feature = "postal-code-prefix-property-schema",
    feature = "pending-schema-section"
))]
mod r#postal_code_prefix;
#[cfg(any(
    feature = "postal-code-prefix-property-schema",
    feature = "pending-schema-section"
))]
pub use r#postal_code_prefix::*;
#[cfg(any(
    feature = "postal-code-range-property-schema",
    feature = "pending-schema-section"
))]
mod r#postal_code_range;
#[cfg(any(
    feature = "postal-code-range-property-schema",
    feature = "pending-schema-section"
))]
pub use r#postal_code_range::*;
#[cfg(any(
    feature = "potential-action-property-schema",
    feature = "general-schema-section"
))]
mod r#potential_action;
#[cfg(any(
    feature = "potential-action-property-schema",
    feature = "general-schema-section"
))]
pub use r#potential_action::*;
#[cfg(any(
    feature = "potential-use-property-schema",
    feature = "pending-schema-section"
))]
mod r#potential_use;
#[cfg(any(
    feature = "potential-use-property-schema",
    feature = "pending-schema-section"
))]
pub use r#potential_use::*;
#[cfg(any(
    feature = "pre-op-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#pre_op;
#[cfg(any(
    feature = "pre-op-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#pre_op::*;
#[cfg(any(
    feature = "predecessor-of-property-schema",
    feature = "general-schema-section"
))]
mod r#predecessor_of;
#[cfg(any(
    feature = "predecessor-of-property-schema",
    feature = "general-schema-section"
))]
pub use r#predecessor_of::*;
#[cfg(any(
    feature = "pregnancy-category-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#pregnancy_category;
#[cfg(any(
    feature = "pregnancy-category-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#pregnancy_category::*;
#[cfg(any(
    feature = "pregnancy-warning-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#pregnancy_warning;
#[cfg(any(
    feature = "pregnancy-warning-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#pregnancy_warning::*;
#[cfg(any(
    feature = "prep-time-property-schema",
    feature = "general-schema-section"
))]
mod r#prep_time;
#[cfg(any(
    feature = "prep-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#prep_time::*;
#[cfg(any(
    feature = "preparation-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#preparation;
#[cfg(any(
    feature = "preparation-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#preparation::*;
#[cfg(any(
    feature = "prescribing-info-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#prescribing_info;
#[cfg(any(
    feature = "prescribing-info-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#prescribing_info::*;
#[cfg(any(
    feature = "prescription-status-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#prescription_status;
#[cfg(any(
    feature = "prescription-status-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#prescription_status::*;
#[cfg(any(
    feature = "previous-item-property-schema",
    feature = "general-schema-section"
))]
mod r#previous_item;
#[cfg(any(
    feature = "previous-item-property-schema",
    feature = "general-schema-section"
))]
pub use r#previous_item::*;
#[cfg(any(
    feature = "previous-start-date-property-schema",
    feature = "general-schema-section"
))]
mod r#previous_start_date;
#[cfg(any(
    feature = "previous-start-date-property-schema",
    feature = "general-schema-section"
))]
pub use r#previous_start_date::*;
#[cfg(any(feature = "price-property-schema", feature = "general-schema-section"))]
mod r#price;
#[cfg(any(feature = "price-property-schema", feature = "general-schema-section"))]
pub use r#price::*;
#[cfg(any(
    feature = "price-component-property-schema",
    feature = "general-schema-section"
))]
mod r#price_component;
#[cfg(any(
    feature = "price-component-property-schema",
    feature = "general-schema-section"
))]
pub use r#price_component::*;
#[cfg(any(
    feature = "price-component-type-property-schema",
    feature = "pending-schema-section"
))]
mod r#price_component_type;
#[cfg(any(
    feature = "price-component-type-property-schema",
    feature = "pending-schema-section"
))]
pub use r#price_component_type::*;
#[cfg(any(
    feature = "price-currency-property-schema",
    feature = "general-schema-section"
))]
mod r#price_currency;
#[cfg(any(
    feature = "price-currency-property-schema",
    feature = "general-schema-section"
))]
pub use r#price_currency::*;
#[cfg(any(
    feature = "price-range-property-schema",
    feature = "general-schema-section"
))]
mod r#price_range;
#[cfg(any(
    feature = "price-range-property-schema",
    feature = "general-schema-section"
))]
pub use r#price_range::*;
#[cfg(any(
    feature = "price-specification-property-schema",
    feature = "general-schema-section"
))]
mod r#price_specification;
#[cfg(any(
    feature = "price-specification-property-schema",
    feature = "general-schema-section"
))]
pub use r#price_specification::*;
#[cfg(any(
    feature = "price-type-property-schema",
    feature = "general-schema-section"
))]
mod r#price_type;
#[cfg(any(
    feature = "price-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#price_type::*;
#[cfg(any(
    feature = "price-valid-until-property-schema",
    feature = "general-schema-section"
))]
mod r#price_valid_until;
#[cfg(any(
    feature = "price-valid-until-property-schema",
    feature = "general-schema-section"
))]
pub use r#price_valid_until::*;
#[cfg(any(
    feature = "primary-image-of-page-property-schema",
    feature = "general-schema-section"
))]
mod r#primary_image_of_page;
#[cfg(any(
    feature = "primary-image-of-page-property-schema",
    feature = "general-schema-section"
))]
pub use r#primary_image_of_page::*;
#[cfg(any(
    feature = "primary-prevention-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#primary_prevention;
#[cfg(any(
    feature = "primary-prevention-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#primary_prevention::*;
#[cfg(any(
    feature = "print-column-property-schema",
    feature = "general-schema-section"
))]
mod r#print_column;
#[cfg(any(
    feature = "print-column-property-schema",
    feature = "general-schema-section"
))]
pub use r#print_column::*;
#[cfg(any(
    feature = "print-edition-property-schema",
    feature = "general-schema-section"
))]
mod r#print_edition;
#[cfg(any(
    feature = "print-edition-property-schema",
    feature = "general-schema-section"
))]
pub use r#print_edition::*;
#[cfg(any(
    feature = "print-page-property-schema",
    feature = "general-schema-section"
))]
mod r#print_page;
#[cfg(any(
    feature = "print-page-property-schema",
    feature = "general-schema-section"
))]
pub use r#print_page::*;
#[cfg(any(
    feature = "print-section-property-schema",
    feature = "general-schema-section"
))]
mod r#print_section;
#[cfg(any(
    feature = "print-section-property-schema",
    feature = "general-schema-section"
))]
pub use r#print_section::*;
#[cfg(any(
    feature = "procedure-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#procedure;
#[cfg(any(
    feature = "procedure-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#procedure::*;
#[cfg(any(
    feature = "procedure-type-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#procedure_type;
#[cfg(any(
    feature = "procedure-type-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#procedure_type::*;
#[cfg(any(
    feature = "processing-time-property-schema",
    feature = "general-schema-section"
))]
mod r#processing_time;
#[cfg(any(
    feature = "processing-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#processing_time::*;
#[cfg(any(
    feature = "processor-requirements-property-schema",
    feature = "general-schema-section"
))]
mod r#processor_requirements;
#[cfg(any(
    feature = "processor-requirements-property-schema",
    feature = "general-schema-section"
))]
pub use r#processor_requirements::*;
#[cfg(any(
    feature = "producer-property-schema",
    feature = "general-schema-section"
))]
mod r#producer;
#[cfg(any(
    feature = "producer-property-schema",
    feature = "general-schema-section"
))]
pub use r#producer::*;
#[cfg(any(
    feature = "produces-property-schema",
    feature = "general-schema-section"
))]
mod r#produces;
#[cfg(any(
    feature = "produces-property-schema",
    feature = "general-schema-section"
))]
pub use r#produces::*;
#[cfg(any(
    feature = "product-group-id-property-schema",
    feature = "pending-schema-section"
))]
mod r#product_group_id;
#[cfg(any(
    feature = "product-group-id-property-schema",
    feature = "pending-schema-section"
))]
pub use r#product_group_id::*;
#[cfg(any(
    feature = "product-id-property-schema",
    feature = "general-schema-section"
))]
mod r#product_id;
#[cfg(any(
    feature = "product-id-property-schema",
    feature = "general-schema-section"
))]
pub use r#product_id::*;
#[cfg(any(
    feature = "product-return-days-property-schema",
    feature = "attic-schema-section"
))]
mod r#product_return_days;
#[cfg(any(
    feature = "product-return-days-property-schema",
    feature = "attic-schema-section"
))]
pub use r#product_return_days::*;
#[cfg(any(
    feature = "product-return-link-property-schema",
    feature = "attic-schema-section"
))]
mod r#product_return_link;
#[cfg(any(
    feature = "product-return-link-property-schema",
    feature = "attic-schema-section"
))]
pub use r#product_return_link::*;
#[cfg(any(
    feature = "product-supported-property-schema",
    feature = "general-schema-section"
))]
mod r#product_supported;
#[cfg(any(
    feature = "product-supported-property-schema",
    feature = "general-schema-section"
))]
pub use r#product_supported::*;
#[cfg(any(
    feature = "production-company-property-schema",
    feature = "general-schema-section"
))]
mod r#production_company;
#[cfg(any(
    feature = "production-company-property-schema",
    feature = "general-schema-section"
))]
pub use r#production_company::*;
#[cfg(any(
    feature = "production-date-property-schema",
    feature = "general-schema-section"
))]
mod r#production_date;
#[cfg(any(
    feature = "production-date-property-schema",
    feature = "general-schema-section"
))]
pub use r#production_date::*;
#[cfg(any(
    feature = "proficiency-level-property-schema",
    feature = "general-schema-section"
))]
mod r#proficiency_level;
#[cfg(any(
    feature = "proficiency-level-property-schema",
    feature = "general-schema-section"
))]
pub use r#proficiency_level::*;
#[cfg(any(
    feature = "program-membership-used-property-schema",
    feature = "general-schema-section"
))]
mod r#program_membership_used;
#[cfg(any(
    feature = "program-membership-used-property-schema",
    feature = "general-schema-section"
))]
pub use r#program_membership_used::*;
#[cfg(any(
    feature = "program-name-property-schema",
    feature = "general-schema-section"
))]
mod r#program_name;
#[cfg(any(
    feature = "program-name-property-schema",
    feature = "general-schema-section"
))]
pub use r#program_name::*;
#[cfg(any(
    feature = "program-prerequisites-property-schema",
    feature = "pending-schema-section"
))]
mod r#program_prerequisites;
#[cfg(any(
    feature = "program-prerequisites-property-schema",
    feature = "pending-schema-section"
))]
pub use r#program_prerequisites::*;
#[cfg(any(
    feature = "program-type-property-schema",
    feature = "pending-schema-section"
))]
mod r#program_type;
#[cfg(any(
    feature = "program-type-property-schema",
    feature = "pending-schema-section"
))]
pub use r#program_type::*;
#[cfg(any(
    feature = "programming-language-property-schema",
    feature = "general-schema-section"
))]
mod r#programming_language;
#[cfg(any(
    feature = "programming-language-property-schema",
    feature = "general-schema-section"
))]
pub use r#programming_language::*;
#[cfg(any(
    feature = "programming-model-property-schema",
    feature = "general-schema-section"
))]
mod r#programming_model;
#[cfg(any(
    feature = "programming-model-property-schema",
    feature = "general-schema-section"
))]
pub use r#programming_model::*;
#[cfg(any(
    feature = "property-id-property-schema",
    feature = "general-schema-section"
))]
mod r#property_id;
#[cfg(any(
    feature = "property-id-property-schema",
    feature = "general-schema-section"
))]
pub use r#property_id::*;
#[cfg(any(
    feature = "proprietary-name-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#proprietary_name;
#[cfg(any(
    feature = "proprietary-name-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#proprietary_name::*;
#[cfg(any(
    feature = "protein-content-property-schema",
    feature = "general-schema-section"
))]
mod r#protein_content;
#[cfg(any(
    feature = "protein-content-property-schema",
    feature = "general-schema-section"
))]
pub use r#protein_content::*;
#[cfg(any(
    feature = "provider-property-schema",
    feature = "pending-schema-section"
))]
mod r#provider;
#[cfg(any(
    feature = "provider-property-schema",
    feature = "pending-schema-section"
))]
pub use r#provider::*;
#[cfg(any(
    feature = "provider-mobility-property-schema",
    feature = "general-schema-section"
))]
mod r#provider_mobility;
#[cfg(any(
    feature = "provider-mobility-property-schema",
    feature = "general-schema-section"
))]
pub use r#provider_mobility::*;
#[cfg(any(
    feature = "provides-broadcast-service-property-schema",
    feature = "general-schema-section"
))]
mod r#provides_broadcast_service;
#[cfg(any(
    feature = "provides-broadcast-service-property-schema",
    feature = "general-schema-section"
))]
pub use r#provides_broadcast_service::*;
#[cfg(any(
    feature = "provides-service-property-schema",
    feature = "general-schema-section"
))]
mod r#provides_service;
#[cfg(any(
    feature = "provides-service-property-schema",
    feature = "general-schema-section"
))]
pub use r#provides_service::*;
#[cfg(any(
    feature = "public-access-property-schema",
    feature = "general-schema-section"
))]
mod r#public_access;
#[cfg(any(
    feature = "public-access-property-schema",
    feature = "general-schema-section"
))]
pub use r#public_access::*;
#[cfg(any(
    feature = "public-transport-closures-info-property-schema",
    feature = "pending-schema-section"
))]
mod r#public_transport_closures_info;
#[cfg(any(
    feature = "public-transport-closures-info-property-schema",
    feature = "pending-schema-section"
))]
pub use r#public_transport_closures_info::*;
#[cfg(any(
    feature = "publication-property-schema",
    feature = "general-schema-section"
))]
mod r#publication;
#[cfg(any(
    feature = "publication-property-schema",
    feature = "general-schema-section"
))]
pub use r#publication::*;
#[cfg(any(
    feature = "publication-type-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#publication_type;
#[cfg(any(
    feature = "publication-type-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#publication_type::*;
#[cfg(any(
    feature = "published-by-property-schema",
    feature = "bib-schema-section"
))]
mod r#published_by;
#[cfg(any(
    feature = "published-by-property-schema",
    feature = "bib-schema-section"
))]
pub use r#published_by::*;
#[cfg(any(
    feature = "published-on-property-schema",
    feature = "general-schema-section"
))]
mod r#published_on;
#[cfg(any(
    feature = "published-on-property-schema",
    feature = "general-schema-section"
))]
pub use r#published_on::*;
#[cfg(any(
    feature = "publisher-property-schema",
    feature = "general-schema-section"
))]
mod r#publisher;
#[cfg(any(
    feature = "publisher-property-schema",
    feature = "general-schema-section"
))]
pub use r#publisher::*;
#[cfg(any(
    feature = "publisher-imprint-property-schema",
    feature = "bib-schema-section"
))]
mod r#publisher_imprint;
#[cfg(any(
    feature = "publisher-imprint-property-schema",
    feature = "bib-schema-section"
))]
pub use r#publisher_imprint::*;
#[cfg(any(
    feature = "publishing-principles-property-schema",
    feature = "general-schema-section"
))]
mod r#publishing_principles;
#[cfg(any(
    feature = "publishing-principles-property-schema",
    feature = "general-schema-section"
))]
pub use r#publishing_principles::*;
#[cfg(any(
    feature = "purchase-date-property-schema",
    feature = "general-schema-section"
))]
mod r#purchase_date;
#[cfg(any(
    feature = "purchase-date-property-schema",
    feature = "general-schema-section"
))]
pub use r#purchase_date::*;
#[cfg(any(
    feature = "qualifications-property-schema",
    feature = "pending-schema-section"
))]
mod r#qualifications;
#[cfg(any(
    feature = "qualifications-property-schema",
    feature = "pending-schema-section"
))]
pub use r#qualifications::*;
#[cfg(any(
    feature = "quarantine-guidelines-property-schema",
    feature = "pending-schema-section"
))]
mod r#quarantine_guidelines;
#[cfg(any(
    feature = "quarantine-guidelines-property-schema",
    feature = "pending-schema-section"
))]
pub use r#quarantine_guidelines::*;
#[cfg(any(feature = "query-property-schema", feature = "general-schema-section"))]
mod r#query;
#[cfg(any(feature = "query-property-schema", feature = "general-schema-section"))]
pub use r#query::*;
#[cfg(any(feature = "quest-property-schema", feature = "general-schema-section"))]
mod r#quest;
#[cfg(any(feature = "quest-property-schema", feature = "general-schema-section"))]
pub use r#quest::*;
#[cfg(any(
    feature = "question-property-schema",
    feature = "general-schema-section"
))]
mod r#question;
#[cfg(any(
    feature = "question-property-schema",
    feature = "general-schema-section"
))]
pub use r#question::*;
#[cfg(any(
    feature = "range-includes-property-schema",
    feature = "meta-schema-section"
))]
mod r#range_includes;
#[cfg(any(
    feature = "range-includes-property-schema",
    feature = "meta-schema-section"
))]
pub use r#range_includes::*;
#[cfg(any(
    feature = "rating-count-property-schema",
    feature = "general-schema-section"
))]
mod r#rating_count;
#[cfg(any(
    feature = "rating-count-property-schema",
    feature = "general-schema-section"
))]
pub use r#rating_count::*;
#[cfg(any(
    feature = "rating-explanation-property-schema",
    feature = "pending-schema-section"
))]
mod r#rating_explanation;
#[cfg(any(
    feature = "rating-explanation-property-schema",
    feature = "pending-schema-section"
))]
pub use r#rating_explanation::*;
#[cfg(any(
    feature = "rating-value-property-schema",
    feature = "general-schema-section"
))]
mod r#rating_value;
#[cfg(any(
    feature = "rating-value-property-schema",
    feature = "general-schema-section"
))]
pub use r#rating_value::*;
#[cfg(any(feature = "read-by-property-schema", feature = "bib-schema-section"))]
mod r#read_by;
#[cfg(any(feature = "read-by-property-schema", feature = "bib-schema-section"))]
pub use r#read_by::*;
#[cfg(any(
    feature = "readonly-value-property-schema",
    feature = "general-schema-section"
))]
mod r#readonly_value;
#[cfg(any(
    feature = "readonly-value-property-schema",
    feature = "general-schema-section"
))]
pub use r#readonly_value::*;
#[cfg(any(
    feature = "real-estate-agent-property-schema",
    feature = "general-schema-section"
))]
mod r#real_estate_agent;
#[cfg(any(
    feature = "real-estate-agent-property-schema",
    feature = "general-schema-section"
))]
pub use r#real_estate_agent::*;
#[cfg(any(feature = "recipe-property-schema", feature = "general-schema-section"))]
mod r#recipe;
#[cfg(any(feature = "recipe-property-schema", feature = "general-schema-section"))]
pub use r#recipe::*;
#[cfg(any(
    feature = "recipe-category-property-schema",
    feature = "general-schema-section"
))]
mod r#recipe_category;
#[cfg(any(
    feature = "recipe-category-property-schema",
    feature = "general-schema-section"
))]
pub use r#recipe_category::*;
#[cfg(any(
    feature = "recipe-cuisine-property-schema",
    feature = "general-schema-section"
))]
mod r#recipe_cuisine;
#[cfg(any(
    feature = "recipe-cuisine-property-schema",
    feature = "general-schema-section"
))]
pub use r#recipe_cuisine::*;
#[cfg(any(
    feature = "recipe-ingredient-property-schema",
    feature = "general-schema-section"
))]
mod r#recipe_ingredient;
#[cfg(any(
    feature = "recipe-ingredient-property-schema",
    feature = "general-schema-section"
))]
pub use r#recipe_ingredient::*;
#[cfg(any(
    feature = "recipe-instructions-property-schema",
    feature = "general-schema-section"
))]
mod r#recipe_instructions;
#[cfg(any(
    feature = "recipe-instructions-property-schema",
    feature = "general-schema-section"
))]
pub use r#recipe_instructions::*;
#[cfg(any(
    feature = "recipe-yield-property-schema",
    feature = "general-schema-section"
))]
mod r#recipe_yield;
#[cfg(any(
    feature = "recipe-yield-property-schema",
    feature = "general-schema-section"
))]
pub use r#recipe_yield::*;
#[cfg(any(
    feature = "recipient-property-schema",
    feature = "general-schema-section"
))]
mod r#recipient;
#[cfg(any(
    feature = "recipient-property-schema",
    feature = "general-schema-section"
))]
pub use r#recipient::*;
#[cfg(any(
    feature = "recognized-by-property-schema",
    feature = "pending-schema-section"
))]
mod r#recognized_by;
#[cfg(any(
    feature = "recognized-by-property-schema",
    feature = "pending-schema-section"
))]
pub use r#recognized_by::*;
#[cfg(any(
    feature = "recognizing-authority-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#recognizing_authority;
#[cfg(any(
    feature = "recognizing-authority-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#recognizing_authority::*;
#[cfg(any(
    feature = "recommendation-strength-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#recommendation_strength;
#[cfg(any(
    feature = "recommendation-strength-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#recommendation_strength::*;
#[cfg(any(
    feature = "recommended-intake-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#recommended_intake;
#[cfg(any(
    feature = "recommended-intake-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#recommended_intake::*;
#[cfg(any(
    feature = "record-label-property-schema",
    feature = "general-schema-section"
))]
mod r#record_label;
#[cfg(any(
    feature = "record-label-property-schema",
    feature = "general-schema-section"
))]
pub use r#record_label::*;
#[cfg(any(
    feature = "recorded-as-property-schema",
    feature = "general-schema-section"
))]
mod r#recorded_as;
#[cfg(any(
    feature = "recorded-as-property-schema",
    feature = "general-schema-section"
))]
pub use r#recorded_as::*;
#[cfg(any(
    feature = "recorded-at-property-schema",
    feature = "general-schema-section"
))]
mod r#recorded_at;
#[cfg(any(
    feature = "recorded-at-property-schema",
    feature = "general-schema-section"
))]
pub use r#recorded_at::*;
#[cfg(any(
    feature = "recorded-in-property-schema",
    feature = "general-schema-section"
))]
mod r#recorded_in;
#[cfg(any(
    feature = "recorded-in-property-schema",
    feature = "general-schema-section"
))]
pub use r#recorded_in::*;
#[cfg(any(
    feature = "recording-of-property-schema",
    feature = "general-schema-section"
))]
mod r#recording_of;
#[cfg(any(
    feature = "recording-of-property-schema",
    feature = "general-schema-section"
))]
pub use r#recording_of::*;
#[cfg(any(
    feature = "recourse-loan-property-schema",
    feature = "pending-schema-section"
))]
mod r#recourse_loan;
#[cfg(any(
    feature = "recourse-loan-property-schema",
    feature = "pending-schema-section"
))]
pub use r#recourse_loan::*;
#[cfg(any(
    feature = "reference-quantity-property-schema",
    feature = "general-schema-section"
))]
mod r#reference_quantity;
#[cfg(any(
    feature = "reference-quantity-property-schema",
    feature = "general-schema-section"
))]
pub use r#reference_quantity::*;
#[cfg(any(
    feature = "references-order-property-schema",
    feature = "general-schema-section"
))]
mod r#references_order;
#[cfg(any(
    feature = "references-order-property-schema",
    feature = "general-schema-section"
))]
pub use r#references_order::*;
#[cfg(any(
    feature = "refund-type-property-schema",
    feature = "pending-schema-section"
))]
mod r#refund_type;
#[cfg(any(
    feature = "refund-type-property-schema",
    feature = "pending-schema-section"
))]
pub use r#refund_type::*;
#[cfg(any(
    feature = "region-drained-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#region_drained;
#[cfg(any(
    feature = "region-drained-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#region_drained::*;
#[cfg(any(
    feature = "regions-allowed-property-schema",
    feature = "general-schema-section"
))]
mod r#regions_allowed;
#[cfg(any(
    feature = "regions-allowed-property-schema",
    feature = "general-schema-section"
))]
pub use r#regions_allowed::*;
#[cfg(any(
    feature = "related-anatomy-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#related_anatomy;
#[cfg(any(
    feature = "related-anatomy-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#related_anatomy::*;
#[cfg(any(
    feature = "related-condition-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#related_condition;
#[cfg(any(
    feature = "related-condition-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#related_condition::*;
#[cfg(any(
    feature = "related-drug-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#related_drug;
#[cfg(any(
    feature = "related-drug-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#related_drug::*;
#[cfg(any(
    feature = "related-link-property-schema",
    feature = "general-schema-section"
))]
mod r#related_link;
#[cfg(any(
    feature = "related-link-property-schema",
    feature = "general-schema-section"
))]
pub use r#related_link::*;
#[cfg(any(
    feature = "related-structure-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#related_structure;
#[cfg(any(
    feature = "related-structure-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#related_structure::*;
#[cfg(any(
    feature = "related-therapy-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#related_therapy;
#[cfg(any(
    feature = "related-therapy-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#related_therapy::*;
#[cfg(any(
    feature = "related-to-property-schema",
    feature = "general-schema-section"
))]
mod r#related_to;
#[cfg(any(
    feature = "related-to-property-schema",
    feature = "general-schema-section"
))]
pub use r#related_to::*;
#[cfg(any(
    feature = "release-date-property-schema",
    feature = "general-schema-section"
))]
mod r#release_date;
#[cfg(any(
    feature = "release-date-property-schema",
    feature = "general-schema-section"
))]
pub use r#release_date::*;
#[cfg(any(
    feature = "release-notes-property-schema",
    feature = "general-schema-section"
))]
mod r#release_notes;
#[cfg(any(
    feature = "release-notes-property-schema",
    feature = "general-schema-section"
))]
pub use r#release_notes::*;
#[cfg(any(
    feature = "release-of-property-schema",
    feature = "general-schema-section"
))]
mod r#release_of;
#[cfg(any(
    feature = "release-of-property-schema",
    feature = "general-schema-section"
))]
pub use r#release_of::*;
#[cfg(any(
    feature = "released-event-property-schema",
    feature = "general-schema-section"
))]
mod r#released_event;
#[cfg(any(
    feature = "released-event-property-schema",
    feature = "general-schema-section"
))]
pub use r#released_event::*;
#[cfg(any(
    feature = "relevant-occupation-property-schema",
    feature = "general-schema-section"
))]
mod r#relevant_occupation;
#[cfg(any(
    feature = "relevant-occupation-property-schema",
    feature = "general-schema-section"
))]
pub use r#relevant_occupation::*;
#[cfg(any(
    feature = "relevant-specialty-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#relevant_specialty;
#[cfg(any(
    feature = "relevant-specialty-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#relevant_specialty::*;
#[cfg(any(
    feature = "remaining-attendee-capacity-property-schema",
    feature = "general-schema-section"
))]
mod r#remaining_attendee_capacity;
#[cfg(any(
    feature = "remaining-attendee-capacity-property-schema",
    feature = "general-schema-section"
))]
pub use r#remaining_attendee_capacity::*;
#[cfg(any(
    feature = "renegotiable-loan-property-schema",
    feature = "pending-schema-section"
))]
mod r#renegotiable_loan;
#[cfg(any(
    feature = "renegotiable-loan-property-schema",
    feature = "pending-schema-section"
))]
pub use r#renegotiable_loan::*;
#[cfg(any(
    feature = "repeat-count-property-schema",
    feature = "pending-schema-section"
))]
mod r#repeat_count;
#[cfg(any(
    feature = "repeat-count-property-schema",
    feature = "pending-schema-section"
))]
pub use r#repeat_count::*;
#[cfg(any(
    feature = "repeat-frequency-property-schema",
    feature = "pending-schema-section"
))]
mod r#repeat_frequency;
#[cfg(any(
    feature = "repeat-frequency-property-schema",
    feature = "pending-schema-section"
))]
pub use r#repeat_frequency::*;
#[cfg(any(
    feature = "repetitions-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#repetitions;
#[cfg(any(
    feature = "repetitions-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#repetitions::*;
#[cfg(any(
    feature = "replacee-property-schema",
    feature = "general-schema-section"
))]
mod r#replacee;
#[cfg(any(
    feature = "replacee-property-schema",
    feature = "general-schema-section"
))]
pub use r#replacee::*;
#[cfg(any(
    feature = "replacer-property-schema",
    feature = "general-schema-section"
))]
mod r#replacer;
#[cfg(any(
    feature = "replacer-property-schema",
    feature = "general-schema-section"
))]
pub use r#replacer::*;
#[cfg(any(
    feature = "reply-to-url-property-schema",
    feature = "general-schema-section"
))]
mod r#reply_to_url;
#[cfg(any(
    feature = "reply-to-url-property-schema",
    feature = "general-schema-section"
))]
pub use r#reply_to_url::*;
#[cfg(any(
    feature = "report-number-property-schema",
    feature = "general-schema-section"
))]
mod r#report_number;
#[cfg(any(
    feature = "report-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#report_number::*;
#[cfg(any(
    feature = "representative-of-page-property-schema",
    feature = "general-schema-section"
))]
mod r#representative_of_page;
#[cfg(any(
    feature = "representative-of-page-property-schema",
    feature = "general-schema-section"
))]
pub use r#representative_of_page::*;
#[cfg(any(
    feature = "required-collateral-property-schema",
    feature = "general-schema-section"
))]
mod r#required_collateral;
#[cfg(any(
    feature = "required-collateral-property-schema",
    feature = "general-schema-section"
))]
pub use r#required_collateral::*;
#[cfg(any(
    feature = "required-gender-property-schema",
    feature = "general-schema-section"
))]
mod r#required_gender;
#[cfg(any(
    feature = "required-gender-property-schema",
    feature = "general-schema-section"
))]
pub use r#required_gender::*;
#[cfg(any(
    feature = "required-max-age-property-schema",
    feature = "general-schema-section"
))]
mod r#required_max_age;
#[cfg(any(
    feature = "required-max-age-property-schema",
    feature = "general-schema-section"
))]
pub use r#required_max_age::*;
#[cfg(any(
    feature = "required-min-age-property-schema",
    feature = "general-schema-section"
))]
mod r#required_min_age;
#[cfg(any(
    feature = "required-min-age-property-schema",
    feature = "general-schema-section"
))]
pub use r#required_min_age::*;
#[cfg(any(
    feature = "required-quantity-property-schema",
    feature = "general-schema-section"
))]
mod r#required_quantity;
#[cfg(any(
    feature = "required-quantity-property-schema",
    feature = "general-schema-section"
))]
pub use r#required_quantity::*;
#[cfg(any(
    feature = "requirements-property-schema",
    feature = "general-schema-section"
))]
mod r#requirements;
#[cfg(any(
    feature = "requirements-property-schema",
    feature = "general-schema-section"
))]
pub use r#requirements::*;
#[cfg(any(
    feature = "requires-subscription-property-schema",
    feature = "general-schema-section"
))]
mod r#requires_subscription;
#[cfg(any(
    feature = "requires-subscription-property-schema",
    feature = "general-schema-section"
))]
pub use r#requires_subscription::*;
#[cfg(any(
    feature = "reservation-for-property-schema",
    feature = "general-schema-section"
))]
mod r#reservation_for;
#[cfg(any(
    feature = "reservation-for-property-schema",
    feature = "general-schema-section"
))]
pub use r#reservation_for::*;
#[cfg(any(
    feature = "reservation-id-property-schema",
    feature = "general-schema-section"
))]
mod r#reservation_id;
#[cfg(any(
    feature = "reservation-id-property-schema",
    feature = "general-schema-section"
))]
pub use r#reservation_id::*;
#[cfg(any(
    feature = "reservation-status-property-schema",
    feature = "general-schema-section"
))]
mod r#reservation_status;
#[cfg(any(
    feature = "reservation-status-property-schema",
    feature = "general-schema-section"
))]
pub use r#reservation_status::*;
#[cfg(any(
    feature = "reserved-ticket-property-schema",
    feature = "general-schema-section"
))]
mod r#reserved_ticket;
#[cfg(any(
    feature = "reserved-ticket-property-schema",
    feature = "general-schema-section"
))]
pub use r#reserved_ticket::*;
#[cfg(any(
    feature = "responsibilities-property-schema",
    feature = "general-schema-section"
))]
mod r#responsibilities;
#[cfg(any(
    feature = "responsibilities-property-schema",
    feature = "general-schema-section"
))]
pub use r#responsibilities::*;
#[cfg(any(
    feature = "rest-periods-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#rest_periods;
#[cfg(any(
    feature = "rest-periods-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#rest_periods::*;
#[cfg(any(
    feature = "restocking-fee-property-schema",
    feature = "pending-schema-section"
))]
mod r#restocking_fee;
#[cfg(any(
    feature = "restocking-fee-property-schema",
    feature = "pending-schema-section"
))]
pub use r#restocking_fee::*;
#[cfg(any(feature = "result-property-schema", feature = "general-schema-section"))]
mod r#result;
#[cfg(any(feature = "result-property-schema", feature = "general-schema-section"))]
pub use r#result::*;
#[cfg(any(
    feature = "result-comment-property-schema",
    feature = "general-schema-section"
))]
mod r#result_comment;
#[cfg(any(
    feature = "result-comment-property-schema",
    feature = "general-schema-section"
))]
pub use r#result_comment::*;
#[cfg(any(
    feature = "result-review-property-schema",
    feature = "general-schema-section"
))]
mod r#result_review;
#[cfg(any(
    feature = "result-review-property-schema",
    feature = "general-schema-section"
))]
pub use r#result_review::*;
#[cfg(any(
    feature = "return-fees-property-schema",
    feature = "pending-schema-section"
))]
mod r#return_fees;
#[cfg(any(
    feature = "return-fees-property-schema",
    feature = "pending-schema-section"
))]
pub use r#return_fees::*;
#[cfg(any(
    feature = "return-label-source-property-schema",
    feature = "pending-schema-section"
))]
mod r#return_label_source;
#[cfg(any(
    feature = "return-label-source-property-schema",
    feature = "pending-schema-section"
))]
pub use r#return_label_source::*;
#[cfg(any(
    feature = "return-method-property-schema",
    feature = "pending-schema-section"
))]
mod r#return_method;
#[cfg(any(
    feature = "return-method-property-schema",
    feature = "pending-schema-section"
))]
pub use r#return_method::*;
#[cfg(any(
    feature = "return-policy-category-property-schema",
    feature = "pending-schema-section"
))]
mod r#return_policy_category;
#[cfg(any(
    feature = "return-policy-category-property-schema",
    feature = "pending-schema-section"
))]
pub use r#return_policy_category::*;
#[cfg(any(
    feature = "return-policy-country-property-schema",
    feature = "pending-schema-section"
))]
mod r#return_policy_country;
#[cfg(any(
    feature = "return-policy-country-property-schema",
    feature = "pending-schema-section"
))]
pub use r#return_policy_country::*;
#[cfg(any(
    feature = "return-policy-seasonal-override-property-schema",
    feature = "pending-schema-section"
))]
mod r#return_policy_seasonal_override;
#[cfg(any(
    feature = "return-policy-seasonal-override-property-schema",
    feature = "pending-schema-section"
))]
pub use r#return_policy_seasonal_override::*;
#[cfg(any(
    feature = "return-shipping-fees-amount-property-schema",
    feature = "pending-schema-section"
))]
mod r#return_shipping_fees_amount;
#[cfg(any(
    feature = "return-shipping-fees-amount-property-schema",
    feature = "pending-schema-section"
))]
pub use r#return_shipping_fees_amount::*;
#[cfg(any(feature = "review-property-schema", feature = "general-schema-section"))]
mod r#review;
#[cfg(any(feature = "review-property-schema", feature = "general-schema-section"))]
pub use r#review::*;
#[cfg(any(
    feature = "review-aspect-property-schema",
    feature = "general-schema-section"
))]
mod r#review_aspect;
#[cfg(any(
    feature = "review-aspect-property-schema",
    feature = "general-schema-section"
))]
pub use r#review_aspect::*;
#[cfg(any(
    feature = "review-body-property-schema",
    feature = "general-schema-section"
))]
mod r#review_body;
#[cfg(any(
    feature = "review-body-property-schema",
    feature = "general-schema-section"
))]
pub use r#review_body::*;
#[cfg(any(
    feature = "review-count-property-schema",
    feature = "general-schema-section"
))]
mod r#review_count;
#[cfg(any(
    feature = "review-count-property-schema",
    feature = "general-schema-section"
))]
pub use r#review_count::*;
#[cfg(any(
    feature = "review-rating-property-schema",
    feature = "general-schema-section"
))]
mod r#review_rating;
#[cfg(any(
    feature = "review-rating-property-schema",
    feature = "general-schema-section"
))]
pub use r#review_rating::*;
#[cfg(any(
    feature = "reviewed-by-property-schema",
    feature = "general-schema-section"
))]
mod r#reviewed_by;
#[cfg(any(
    feature = "reviewed-by-property-schema",
    feature = "general-schema-section"
))]
pub use r#reviewed_by::*;
#[cfg(any(
    feature = "reviews-property-schema",
    feature = "general-schema-section"
))]
mod r#reviews;
#[cfg(any(
    feature = "reviews-property-schema",
    feature = "general-schema-section"
))]
pub use r#reviews::*;
#[cfg(any(
    feature = "risk-factor-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#risk_factor;
#[cfg(any(
    feature = "risk-factor-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#risk_factor::*;
#[cfg(any(
    feature = "risks-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#risks;
#[cfg(any(
    feature = "risks-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#risks::*;
#[cfg(any(
    feature = "role-name-property-schema",
    feature = "general-schema-section"
))]
mod r#role_name;
#[cfg(any(
    feature = "role-name-property-schema",
    feature = "general-schema-section"
))]
pub use r#role_name::*;
#[cfg(any(feature = "roof-load-property-schema", feature = "auto-schema-section"))]
mod r#roof_load;
#[cfg(any(feature = "roof-load-property-schema", feature = "auto-schema-section"))]
pub use r#roof_load::*;
#[cfg(any(
    feature = "rsvp-response-property-schema",
    feature = "general-schema-section"
))]
mod r#rsvp_response;
#[cfg(any(
    feature = "rsvp-response-property-schema",
    feature = "general-schema-section"
))]
pub use r#rsvp_response::*;
#[cfg(any(
    feature = "runs-to-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#runs_to;
#[cfg(any(
    feature = "runs-to-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#runs_to::*;
#[cfg(any(
    feature = "runtime-property-schema",
    feature = "general-schema-section"
))]
mod r#runtime;
#[cfg(any(
    feature = "runtime-property-schema",
    feature = "general-schema-section"
))]
pub use r#runtime::*;
#[cfg(any(
    feature = "runtime-platform-property-schema",
    feature = "general-schema-section"
))]
mod r#runtime_platform;
#[cfg(any(
    feature = "runtime-platform-property-schema",
    feature = "general-schema-section"
))]
pub use r#runtime_platform::*;
#[cfg(any(feature = "rxcui-property-schema", feature = "pending-schema-section"))]
mod r#rxcui;
#[cfg(any(feature = "rxcui-property-schema", feature = "pending-schema-section"))]
pub use r#rxcui::*;
#[cfg(any(
    feature = "safety-consideration-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#safety_consideration;
#[cfg(any(
    feature = "safety-consideration-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#safety_consideration::*;
#[cfg(any(
    feature = "salary-currency-property-schema",
    feature = "general-schema-section"
))]
mod r#salary_currency;
#[cfg(any(
    feature = "salary-currency-property-schema",
    feature = "general-schema-section"
))]
pub use r#salary_currency::*;
#[cfg(any(
    feature = "salary-upon-completion-property-schema",
    feature = "pending-schema-section"
))]
mod r#salary_upon_completion;
#[cfg(any(
    feature = "salary-upon-completion-property-schema",
    feature = "pending-schema-section"
))]
pub use r#salary_upon_completion::*;
#[cfg(any(
    feature = "same-as-property-schema",
    feature = "general-schema-section"
))]
mod r#same_as;
#[cfg(any(
    feature = "same-as-property-schema",
    feature = "general-schema-section"
))]
pub use r#same_as::*;
#[cfg(any(
    feature = "sample-type-property-schema",
    feature = "general-schema-section"
))]
mod r#sample_type;
#[cfg(any(
    feature = "sample-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#sample_type::*;
#[cfg(any(
    feature = "saturated-fat-content-property-schema",
    feature = "general-schema-section"
))]
mod r#saturated_fat_content;
#[cfg(any(
    feature = "saturated-fat-content-property-schema",
    feature = "general-schema-section"
))]
pub use r#saturated_fat_content::*;
#[cfg(any(
    feature = "schedule-timezone-property-schema",
    feature = "pending-schema-section"
))]
mod r#schedule_timezone;
#[cfg(any(
    feature = "schedule-timezone-property-schema",
    feature = "pending-schema-section"
))]
pub use r#schedule_timezone::*;
#[cfg(any(
    feature = "scheduled-payment-date-property-schema",
    feature = "general-schema-section"
))]
mod r#scheduled_payment_date;
#[cfg(any(
    feature = "scheduled-payment-date-property-schema",
    feature = "general-schema-section"
))]
pub use r#scheduled_payment_date::*;
#[cfg(any(
    feature = "scheduled-time-property-schema",
    feature = "general-schema-section"
))]
mod r#scheduled_time;
#[cfg(any(
    feature = "scheduled-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#scheduled_time::*;
#[cfg(any(
    feature = "schema-version-property-schema",
    feature = "general-schema-section"
))]
mod r#schema_version;
#[cfg(any(
    feature = "schema-version-property-schema",
    feature = "general-schema-section"
))]
pub use r#schema_version::*;
#[cfg(any(
    feature = "school-closures-info-property-schema",
    feature = "pending-schema-section"
))]
mod r#school_closures_info;
#[cfg(any(
    feature = "school-closures-info-property-schema",
    feature = "pending-schema-section"
))]
pub use r#school_closures_info::*;
#[cfg(any(
    feature = "screen-count-property-schema",
    feature = "general-schema-section"
))]
mod r#screen_count;
#[cfg(any(
    feature = "screen-count-property-schema",
    feature = "general-schema-section"
))]
pub use r#screen_count::*;
#[cfg(any(
    feature = "screenshot-property-schema",
    feature = "general-schema-section"
))]
mod r#screenshot;
#[cfg(any(
    feature = "screenshot-property-schema",
    feature = "general-schema-section"
))]
pub use r#screenshot::*;
#[cfg(any(
    feature = "sd-date-published-property-schema",
    feature = "pending-schema-section"
))]
mod r#sd_date_published;
#[cfg(any(
    feature = "sd-date-published-property-schema",
    feature = "pending-schema-section"
))]
pub use r#sd_date_published::*;
#[cfg(any(
    feature = "sd-license-property-schema",
    feature = "pending-schema-section"
))]
mod r#sd_license;
#[cfg(any(
    feature = "sd-license-property-schema",
    feature = "pending-schema-section"
))]
pub use r#sd_license::*;
#[cfg(any(
    feature = "sd-publisher-property-schema",
    feature = "pending-schema-section"
))]
mod r#sd_publisher;
#[cfg(any(
    feature = "sd-publisher-property-schema",
    feature = "pending-schema-section"
))]
pub use r#sd_publisher::*;
#[cfg(any(feature = "season-property-schema", feature = "general-schema-section"))]
mod r#season;
#[cfg(any(feature = "season-property-schema", feature = "general-schema-section"))]
pub use r#season::*;
#[cfg(any(
    feature = "season-number-property-schema",
    feature = "general-schema-section"
))]
mod r#season_number;
#[cfg(any(
    feature = "season-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#season_number::*;
#[cfg(any(
    feature = "seasons-property-schema",
    feature = "general-schema-section"
))]
mod r#seasons;
#[cfg(any(
    feature = "seasons-property-schema",
    feature = "general-schema-section"
))]
pub use r#seasons::*;
#[cfg(any(
    feature = "seat-number-property-schema",
    feature = "general-schema-section"
))]
mod r#seat_number;
#[cfg(any(
    feature = "seat-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#seat_number::*;
#[cfg(any(
    feature = "seat-row-property-schema",
    feature = "general-schema-section"
))]
mod r#seat_row;
#[cfg(any(
    feature = "seat-row-property-schema",
    feature = "general-schema-section"
))]
pub use r#seat_row::*;
#[cfg(any(
    feature = "seat-section-property-schema",
    feature = "general-schema-section"
))]
mod r#seat_section;
#[cfg(any(
    feature = "seat-section-property-schema",
    feature = "general-schema-section"
))]
pub use r#seat_section::*;
#[cfg(any(
    feature = "seating-capacity-property-schema",
    feature = "auto-schema-section"
))]
mod r#seating_capacity;
#[cfg(any(
    feature = "seating-capacity-property-schema",
    feature = "auto-schema-section"
))]
pub use r#seating_capacity::*;
#[cfg(any(
    feature = "seating-type-property-schema",
    feature = "general-schema-section"
))]
mod r#seating_type;
#[cfg(any(
    feature = "seating-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#seating_type::*;
#[cfg(any(
    feature = "secondary-prevention-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#secondary_prevention;
#[cfg(any(
    feature = "secondary-prevention-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#secondary_prevention::*;
#[cfg(any(
    feature = "security-clearance-requirement-property-schema",
    feature = "pending-schema-section"
))]
mod r#security_clearance_requirement;
#[cfg(any(
    feature = "security-clearance-requirement-property-schema",
    feature = "pending-schema-section"
))]
pub use r#security_clearance_requirement::*;
#[cfg(any(
    feature = "security-screening-property-schema",
    feature = "general-schema-section"
))]
mod r#security_screening;
#[cfg(any(
    feature = "security-screening-property-schema",
    feature = "general-schema-section"
))]
pub use r#security_screening::*;
#[cfg(any(feature = "seeks-property-schema", feature = "general-schema-section"))]
mod r#seeks;
#[cfg(any(feature = "seeks-property-schema", feature = "general-schema-section"))]
pub use r#seeks::*;
#[cfg(any(feature = "seller-property-schema", feature = "general-schema-section"))]
mod r#seller;
#[cfg(any(feature = "seller-property-schema", feature = "general-schema-section"))]
pub use r#seller::*;
#[cfg(any(feature = "sender-property-schema", feature = "general-schema-section"))]
mod r#sender;
#[cfg(any(feature = "sender-property-schema", feature = "general-schema-section"))]
pub use r#sender::*;
#[cfg(any(
    feature = "sensory-requirement-property-schema",
    feature = "pending-schema-section"
))]
mod r#sensory_requirement;
#[cfg(any(
    feature = "sensory-requirement-property-schema",
    feature = "pending-schema-section"
))]
pub use r#sensory_requirement::*;
#[cfg(any(
    feature = "sensory-unit-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#sensory_unit;
#[cfg(any(
    feature = "sensory-unit-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#sensory_unit::*;
#[cfg(any(
    feature = "serial-number-property-schema",
    feature = "general-schema-section"
))]
mod r#serial_number;
#[cfg(any(
    feature = "serial-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#serial_number::*;
#[cfg(any(
    feature = "serious-adverse-outcome-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#serious_adverse_outcome;
#[cfg(any(
    feature = "serious-adverse-outcome-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#serious_adverse_outcome::*;
#[cfg(any(
    feature = "server-status-property-schema",
    feature = "general-schema-section"
))]
mod r#server_status;
#[cfg(any(
    feature = "server-status-property-schema",
    feature = "general-schema-section"
))]
pub use r#server_status::*;
#[cfg(any(
    feature = "serves-cuisine-property-schema",
    feature = "general-schema-section"
))]
mod r#serves_cuisine;
#[cfg(any(
    feature = "serves-cuisine-property-schema",
    feature = "general-schema-section"
))]
pub use r#serves_cuisine::*;
#[cfg(any(
    feature = "service-area-property-schema",
    feature = "general-schema-section"
))]
mod r#service_area;
#[cfg(any(
    feature = "service-area-property-schema",
    feature = "general-schema-section"
))]
pub use r#service_area::*;
#[cfg(any(
    feature = "service-audience-property-schema",
    feature = "general-schema-section"
))]
mod r#service_audience;
#[cfg(any(
    feature = "service-audience-property-schema",
    feature = "general-schema-section"
))]
pub use r#service_audience::*;
#[cfg(any(
    feature = "service-location-property-schema",
    feature = "general-schema-section"
))]
mod r#service_location;
#[cfg(any(
    feature = "service-location-property-schema",
    feature = "general-schema-section"
))]
pub use r#service_location::*;
#[cfg(any(
    feature = "service-operator-property-schema",
    feature = "general-schema-section"
))]
mod r#service_operator;
#[cfg(any(
    feature = "service-operator-property-schema",
    feature = "general-schema-section"
))]
pub use r#service_operator::*;
#[cfg(any(
    feature = "service-output-property-schema",
    feature = "general-schema-section"
))]
mod r#service_output;
#[cfg(any(
    feature = "service-output-property-schema",
    feature = "general-schema-section"
))]
pub use r#service_output::*;
#[cfg(any(
    feature = "service-phone-property-schema",
    feature = "general-schema-section"
))]
mod r#service_phone;
#[cfg(any(
    feature = "service-phone-property-schema",
    feature = "general-schema-section"
))]
pub use r#service_phone::*;
#[cfg(any(
    feature = "service-postal-address-property-schema",
    feature = "general-schema-section"
))]
mod r#service_postal_address;
#[cfg(any(
    feature = "service-postal-address-property-schema",
    feature = "general-schema-section"
))]
pub use r#service_postal_address::*;
#[cfg(any(
    feature = "service-sms-number-property-schema",
    feature = "general-schema-section"
))]
mod r#service_sms_number;
#[cfg(any(
    feature = "service-sms-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#service_sms_number::*;
#[cfg(any(
    feature = "service-type-property-schema",
    feature = "general-schema-section"
))]
mod r#service_type;
#[cfg(any(
    feature = "service-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#service_type::*;
#[cfg(any(
    feature = "service-url-property-schema",
    feature = "general-schema-section"
))]
mod r#service_url;
#[cfg(any(
    feature = "service-url-property-schema",
    feature = "general-schema-section"
))]
pub use r#service_url::*;
#[cfg(any(
    feature = "serving-size-property-schema",
    feature = "general-schema-section"
))]
mod r#serving_size;
#[cfg(any(
    feature = "serving-size-property-schema",
    feature = "general-schema-section"
))]
pub use r#serving_size::*;
#[cfg(any(
    feature = "sha-256-property-schema",
    feature = "pending-schema-section"
))]
mod r#sha_256;
#[cfg(any(
    feature = "sha-256-property-schema",
    feature = "pending-schema-section"
))]
pub use r#sha_256::*;
#[cfg(any(
    feature = "shared-content-property-schema",
    feature = "general-schema-section"
))]
mod r#shared_content;
#[cfg(any(
    feature = "shared-content-property-schema",
    feature = "general-schema-section"
))]
pub use r#shared_content::*;
#[cfg(any(
    feature = "shipping-destination-property-schema",
    feature = "pending-schema-section"
))]
mod r#shipping_destination;
#[cfg(any(
    feature = "shipping-destination-property-schema",
    feature = "pending-schema-section"
))]
pub use r#shipping_destination::*;
#[cfg(any(
    feature = "shipping-details-property-schema",
    feature = "pending-schema-section"
))]
mod r#shipping_details;
#[cfg(any(
    feature = "shipping-details-property-schema",
    feature = "pending-schema-section"
))]
pub use r#shipping_details::*;
#[cfg(any(
    feature = "shipping-label-property-schema",
    feature = "pending-schema-section"
))]
mod r#shipping_label;
#[cfg(any(
    feature = "shipping-label-property-schema",
    feature = "pending-schema-section"
))]
pub use r#shipping_label::*;
#[cfg(any(
    feature = "shipping-origin-property-schema",
    feature = "pending-schema-section"
))]
mod r#shipping_origin;
#[cfg(any(
    feature = "shipping-origin-property-schema",
    feature = "pending-schema-section"
))]
pub use r#shipping_origin::*;
#[cfg(any(
    feature = "shipping-rate-property-schema",
    feature = "pending-schema-section"
))]
mod r#shipping_rate;
#[cfg(any(
    feature = "shipping-rate-property-schema",
    feature = "pending-schema-section"
))]
pub use r#shipping_rate::*;
#[cfg(any(
    feature = "shipping-settings-link-property-schema",
    feature = "pending-schema-section"
))]
mod r#shipping_settings_link;
#[cfg(any(
    feature = "shipping-settings-link-property-schema",
    feature = "pending-schema-section"
))]
pub use r#shipping_settings_link::*;
#[cfg(any(
    feature = "sibling-property-schema",
    feature = "general-schema-section"
))]
mod r#sibling;
#[cfg(any(
    feature = "sibling-property-schema",
    feature = "general-schema-section"
))]
pub use r#sibling::*;
#[cfg(any(
    feature = "siblings-property-schema",
    feature = "general-schema-section"
))]
mod r#siblings;
#[cfg(any(
    feature = "siblings-property-schema",
    feature = "general-schema-section"
))]
pub use r#siblings::*;
#[cfg(any(
    feature = "sign-detected-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#sign_detected;
#[cfg(any(
    feature = "sign-detected-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#sign_detected::*;
#[cfg(any(
    feature = "sign-or-symptom-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#sign_or_symptom;
#[cfg(any(
    feature = "sign-or-symptom-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#sign_or_symptom::*;
#[cfg(any(
    feature = "significance-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#significance;
#[cfg(any(
    feature = "significance-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#significance::*;
#[cfg(any(
    feature = "significant-link-property-schema",
    feature = "general-schema-section"
))]
mod r#significant_link;
#[cfg(any(
    feature = "significant-link-property-schema",
    feature = "general-schema-section"
))]
pub use r#significant_link::*;
#[cfg(any(
    feature = "significant-links-property-schema",
    feature = "general-schema-section"
))]
mod r#significant_links;
#[cfg(any(
    feature = "significant-links-property-schema",
    feature = "general-schema-section"
))]
pub use r#significant_links::*;
#[cfg(any(feature = "size-property-schema", feature = "pending-schema-section"))]
mod r#size;
#[cfg(any(feature = "size-property-schema", feature = "pending-schema-section"))]
pub use r#size::*;
#[cfg(any(
    feature = "size-group-property-schema",
    feature = "pending-schema-section"
))]
mod r#size_group;
#[cfg(any(
    feature = "size-group-property-schema",
    feature = "pending-schema-section"
))]
pub use r#size_group::*;
#[cfg(any(
    feature = "size-system-property-schema",
    feature = "pending-schema-section"
))]
mod r#size_system;
#[cfg(any(
    feature = "size-system-property-schema",
    feature = "pending-schema-section"
))]
pub use r#size_system::*;
#[cfg(any(feature = "skills-property-schema", feature = "general-schema-section"))]
mod r#skills;
#[cfg(any(feature = "skills-property-schema", feature = "general-schema-section"))]
pub use r#skills::*;
#[cfg(any(feature = "sku-property-schema", feature = "general-schema-section"))]
mod r#sku;
#[cfg(any(feature = "sku-property-schema", feature = "general-schema-section"))]
pub use r#sku::*;
#[cfg(any(feature = "slogan-property-schema", feature = "general-schema-section"))]
mod r#slogan;
#[cfg(any(feature = "slogan-property-schema", feature = "general-schema-section"))]
pub use r#slogan::*;
#[cfg(any(feature = "smiles-property-schema", feature = "pending-schema-section"))]
mod r#smiles;
#[cfg(any(feature = "smiles-property-schema", feature = "pending-schema-section"))]
pub use r#smiles::*;
#[cfg(any(
    feature = "smoking-allowed-property-schema",
    feature = "general-schema-section"
))]
mod r#smoking_allowed;
#[cfg(any(
    feature = "smoking-allowed-property-schema",
    feature = "general-schema-section"
))]
pub use r#smoking_allowed::*;
#[cfg(any(
    feature = "sodium-content-property-schema",
    feature = "general-schema-section"
))]
mod r#sodium_content;
#[cfg(any(
    feature = "sodium-content-property-schema",
    feature = "general-schema-section"
))]
pub use r#sodium_content::*;
#[cfg(any(
    feature = "software-add-on-property-schema",
    feature = "general-schema-section"
))]
mod r#software_add_on;
#[cfg(any(
    feature = "software-add-on-property-schema",
    feature = "general-schema-section"
))]
pub use r#software_add_on::*;
#[cfg(any(
    feature = "software-help-property-schema",
    feature = "general-schema-section"
))]
mod r#software_help;
#[cfg(any(
    feature = "software-help-property-schema",
    feature = "general-schema-section"
))]
pub use r#software_help::*;
#[cfg(any(
    feature = "software-requirements-property-schema",
    feature = "general-schema-section"
))]
mod r#software_requirements;
#[cfg(any(
    feature = "software-requirements-property-schema",
    feature = "general-schema-section"
))]
pub use r#software_requirements::*;
#[cfg(any(
    feature = "software-version-property-schema",
    feature = "general-schema-section"
))]
mod r#software_version;
#[cfg(any(
    feature = "software-version-property-schema",
    feature = "general-schema-section"
))]
pub use r#software_version::*;
#[cfg(any(
    feature = "source-organization-property-schema",
    feature = "general-schema-section"
))]
mod r#source_organization;
#[cfg(any(
    feature = "source-organization-property-schema",
    feature = "general-schema-section"
))]
pub use r#source_organization::*;
#[cfg(any(
    feature = "sourced-from-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#sourced_from;
#[cfg(any(
    feature = "sourced-from-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#sourced_from::*;
#[cfg(any(
    feature = "spatial-property-schema",
    feature = "general-schema-section"
))]
mod r#spatial;
#[cfg(any(
    feature = "spatial-property-schema",
    feature = "general-schema-section"
))]
pub use r#spatial::*;
#[cfg(any(
    feature = "spatial-coverage-property-schema",
    feature = "general-schema-section"
))]
mod r#spatial_coverage;
#[cfg(any(
    feature = "spatial-coverage-property-schema",
    feature = "general-schema-section"
))]
pub use r#spatial_coverage::*;
#[cfg(any(
    feature = "speakable-property-schema",
    feature = "general-schema-section"
))]
mod r#speakable;
#[cfg(any(
    feature = "speakable-property-schema",
    feature = "general-schema-section"
))]
pub use r#speakable::*;
#[cfg(any(
    feature = "special-commitments-property-schema",
    feature = "general-schema-section"
))]
mod r#special_commitments;
#[cfg(any(
    feature = "special-commitments-property-schema",
    feature = "general-schema-section"
))]
pub use r#special_commitments::*;
#[cfg(any(
    feature = "special-opening-hours-specification-property-schema",
    feature = "general-schema-section"
))]
mod r#special_opening_hours_specification;
#[cfg(any(
    feature = "special-opening-hours-specification-property-schema",
    feature = "general-schema-section"
))]
pub use r#special_opening_hours_specification::*;
#[cfg(any(
    feature = "specialty-property-schema",
    feature = "general-schema-section"
))]
mod r#specialty;
#[cfg(any(
    feature = "specialty-property-schema",
    feature = "general-schema-section"
))]
pub use r#specialty::*;
#[cfg(any(
    feature = "speech-to-text-markup-property-schema",
    feature = "pending-schema-section"
))]
mod r#speech_to_text_markup;
#[cfg(any(
    feature = "speech-to-text-markup-property-schema",
    feature = "pending-schema-section"
))]
pub use r#speech_to_text_markup::*;
#[cfg(any(feature = "speed-property-schema", feature = "auto-schema-section"))]
mod r#speed;
#[cfg(any(feature = "speed-property-schema", feature = "auto-schema-section"))]
pub use r#speed::*;
#[cfg(any(
    feature = "spoken-by-character-property-schema",
    feature = "pending-schema-section"
))]
mod r#spoken_by_character;
#[cfg(any(
    feature = "spoken-by-character-property-schema",
    feature = "pending-schema-section"
))]
pub use r#spoken_by_character::*;
#[cfg(any(
    feature = "sponsor-property-schema",
    feature = "general-schema-section"
))]
mod r#sponsor;
#[cfg(any(
    feature = "sponsor-property-schema",
    feature = "general-schema-section"
))]
pub use r#sponsor::*;
#[cfg(any(feature = "sport-property-schema", feature = "pending-schema-section"))]
mod r#sport;
#[cfg(any(feature = "sport-property-schema", feature = "pending-schema-section"))]
pub use r#sport::*;
#[cfg(any(
    feature = "sports-activity-location-property-schema",
    feature = "general-schema-section"
))]
mod r#sports_activity_location;
#[cfg(any(
    feature = "sports-activity-location-property-schema",
    feature = "general-schema-section"
))]
pub use r#sports_activity_location::*;
#[cfg(any(
    feature = "sports-event-property-schema",
    feature = "general-schema-section"
))]
mod r#sports_event;
#[cfg(any(
    feature = "sports-event-property-schema",
    feature = "general-schema-section"
))]
pub use r#sports_event::*;
#[cfg(any(
    feature = "sports-team-property-schema",
    feature = "general-schema-section"
))]
mod r#sports_team;
#[cfg(any(
    feature = "sports-team-property-schema",
    feature = "general-schema-section"
))]
pub use r#sports_team::*;
#[cfg(any(feature = "spouse-property-schema", feature = "general-schema-section"))]
mod r#spouse;
#[cfg(any(feature = "spouse-property-schema", feature = "general-schema-section"))]
pub use r#spouse::*;
#[cfg(any(
    feature = "stage-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#stage;
#[cfg(any(
    feature = "stage-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#stage::*;
#[cfg(any(
    feature = "stage-as-number-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#stage_as_number;
#[cfg(any(
    feature = "stage-as-number-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#stage_as_number::*;
#[cfg(any(
    feature = "star-rating-property-schema",
    feature = "general-schema-section"
))]
mod r#star_rating;
#[cfg(any(
    feature = "star-rating-property-schema",
    feature = "general-schema-section"
))]
pub use r#star_rating::*;
#[cfg(any(
    feature = "start-date-property-schema",
    feature = "general-schema-section"
))]
mod r#start_date;
#[cfg(any(
    feature = "start-date-property-schema",
    feature = "general-schema-section"
))]
pub use r#start_date::*;
#[cfg(any(
    feature = "start-offset-property-schema",
    feature = "pending-schema-section"
))]
mod r#start_offset;
#[cfg(any(
    feature = "start-offset-property-schema",
    feature = "pending-schema-section"
))]
pub use r#start_offset::*;
#[cfg(any(
    feature = "start-time-property-schema",
    feature = "general-schema-section"
))]
mod r#start_time;
#[cfg(any(
    feature = "start-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#start_time::*;
#[cfg(any(
    feature = "stat-type-property-schema",
    feature = "pending-schema-section"
))]
mod r#stat_type;
#[cfg(any(
    feature = "stat-type-property-schema",
    feature = "pending-schema-section"
))]
pub use r#stat_type::*;
#[cfg(any(
    feature = "status-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#status;
#[cfg(any(
    feature = "status-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#status::*;
#[cfg(any(
    feature = "steering-position-property-schema",
    feature = "general-schema-section"
))]
mod r#steering_position;
#[cfg(any(
    feature = "steering-position-property-schema",
    feature = "general-schema-section"
))]
pub use r#steering_position::*;
#[cfg(any(feature = "step-property-schema", feature = "general-schema-section"))]
mod r#step;
#[cfg(any(feature = "step-property-schema", feature = "general-schema-section"))]
pub use r#step::*;
#[cfg(any(
    feature = "step-value-property-schema",
    feature = "general-schema-section"
))]
mod r#step_value;
#[cfg(any(
    feature = "step-value-property-schema",
    feature = "general-schema-section"
))]
pub use r#step_value::*;
#[cfg(any(feature = "steps-property-schema", feature = "general-schema-section"))]
mod r#steps;
#[cfg(any(feature = "steps-property-schema", feature = "general-schema-section"))]
pub use r#steps::*;
#[cfg(any(
    feature = "storage-requirements-property-schema",
    feature = "general-schema-section"
))]
mod r#storage_requirements;
#[cfg(any(
    feature = "storage-requirements-property-schema",
    feature = "general-schema-section"
))]
pub use r#storage_requirements::*;
#[cfg(any(
    feature = "street-address-property-schema",
    feature = "general-schema-section"
))]
mod r#street_address;
#[cfg(any(
    feature = "street-address-property-schema",
    feature = "general-schema-section"
))]
pub use r#street_address::*;
#[cfg(any(
    feature = "strength-unit-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#strength_unit;
#[cfg(any(
    feature = "strength-unit-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#strength_unit::*;
#[cfg(any(
    feature = "strength-value-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#strength_value;
#[cfg(any(
    feature = "strength-value-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#strength_value::*;
#[cfg(any(
    feature = "structural-class-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#structural_class;
#[cfg(any(
    feature = "structural-class-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#structural_class::*;
#[cfg(any(
    feature = "study-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#study;
#[cfg(any(
    feature = "study-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#study::*;
#[cfg(any(
    feature = "study-design-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#study_design;
#[cfg(any(
    feature = "study-design-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#study_design::*;
#[cfg(any(
    feature = "study-location-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#study_location;
#[cfg(any(
    feature = "study-location-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#study_location::*;
#[cfg(any(
    feature = "study-subject-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#study_subject;
#[cfg(any(
    feature = "study-subject-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#study_subject::*;
#[cfg(any(
    feature = "stupid-property-property-schema",
    feature = "attic-schema-section"
))]
mod r#stupid_property;
#[cfg(any(
    feature = "stupid-property-property-schema",
    feature = "attic-schema-section"
))]
pub use r#stupid_property::*;
#[cfg(any(
    feature = "sub-event-property-schema",
    feature = "general-schema-section"
))]
mod r#sub_event;
#[cfg(any(
    feature = "sub-event-property-schema",
    feature = "general-schema-section"
))]
pub use r#sub_event::*;
#[cfg(any(
    feature = "sub-events-property-schema",
    feature = "general-schema-section"
))]
mod r#sub_events;
#[cfg(any(
    feature = "sub-events-property-schema",
    feature = "general-schema-section"
))]
pub use r#sub_events::*;
#[cfg(any(
    feature = "sub-organization-property-schema",
    feature = "general-schema-section"
))]
mod r#sub_organization;
#[cfg(any(
    feature = "sub-organization-property-schema",
    feature = "general-schema-section"
))]
pub use r#sub_organization::*;
#[cfg(any(
    feature = "sub-reservation-property-schema",
    feature = "general-schema-section"
))]
mod r#sub_reservation;
#[cfg(any(
    feature = "sub-reservation-property-schema",
    feature = "general-schema-section"
))]
pub use r#sub_reservation::*;
#[cfg(any(
    feature = "sub-stage-suffix-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#sub_stage_suffix;
#[cfg(any(
    feature = "sub-stage-suffix-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#sub_stage_suffix::*;
#[cfg(any(
    feature = "sub-structure-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#sub_structure;
#[cfg(any(
    feature = "sub-structure-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#sub_structure::*;
#[cfg(any(
    feature = "sub-test-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#sub_test;
#[cfg(any(
    feature = "sub-test-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#sub_test::*;
#[cfg(any(
    feature = "sub-trip-property-schema",
    feature = "pending-schema-section"
))]
mod r#sub_trip;
#[cfg(any(
    feature = "sub-trip-property-schema",
    feature = "pending-schema-section"
))]
pub use r#sub_trip::*;
#[cfg(any(
    feature = "subject-of-property-schema",
    feature = "general-schema-section"
))]
mod r#subject_of;
#[cfg(any(
    feature = "subject-of-property-schema",
    feature = "general-schema-section"
))]
pub use r#subject_of::*;
#[cfg(any(
    feature = "subtitle-language-property-schema",
    feature = "pending-schema-section"
))]
mod r#subtitle_language;
#[cfg(any(
    feature = "subtitle-language-property-schema",
    feature = "pending-schema-section"
))]
pub use r#subtitle_language::*;
#[cfg(any(
    feature = "successor-of-property-schema",
    feature = "general-schema-section"
))]
mod r#successor_of;
#[cfg(any(
    feature = "successor-of-property-schema",
    feature = "general-schema-section"
))]
pub use r#successor_of::*;
#[cfg(any(
    feature = "sugar-content-property-schema",
    feature = "general-schema-section"
))]
mod r#sugar_content;
#[cfg(any(
    feature = "sugar-content-property-schema",
    feature = "general-schema-section"
))]
pub use r#sugar_content::*;
#[cfg(any(
    feature = "suggested-age-property-schema",
    feature = "pending-schema-section"
))]
mod r#suggested_age;
#[cfg(any(
    feature = "suggested-age-property-schema",
    feature = "pending-schema-section"
))]
pub use r#suggested_age::*;
#[cfg(any(
    feature = "suggested-answer-property-schema",
    feature = "general-schema-section"
))]
mod r#suggested_answer;
#[cfg(any(
    feature = "suggested-answer-property-schema",
    feature = "general-schema-section"
))]
pub use r#suggested_answer::*;
#[cfg(any(
    feature = "suggested-gender-property-schema",
    feature = "general-schema-section"
))]
mod r#suggested_gender;
#[cfg(any(
    feature = "suggested-gender-property-schema",
    feature = "general-schema-section"
))]
pub use r#suggested_gender::*;
#[cfg(any(
    feature = "suggested-max-age-property-schema",
    feature = "general-schema-section"
))]
mod r#suggested_max_age;
#[cfg(any(
    feature = "suggested-max-age-property-schema",
    feature = "general-schema-section"
))]
pub use r#suggested_max_age::*;
#[cfg(any(
    feature = "suggested-measurement-property-schema",
    feature = "pending-schema-section"
))]
mod r#suggested_measurement;
#[cfg(any(
    feature = "suggested-measurement-property-schema",
    feature = "pending-schema-section"
))]
pub use r#suggested_measurement::*;
#[cfg(any(
    feature = "suggested-min-age-property-schema",
    feature = "general-schema-section"
))]
mod r#suggested_min_age;
#[cfg(any(
    feature = "suggested-min-age-property-schema",
    feature = "general-schema-section"
))]
pub use r#suggested_min_age::*;
#[cfg(any(
    feature = "suitable-for-diet-property-schema",
    feature = "general-schema-section"
))]
mod r#suitable_for_diet;
#[cfg(any(
    feature = "suitable-for-diet-property-schema",
    feature = "general-schema-section"
))]
pub use r#suitable_for_diet::*;
#[cfg(any(
    feature = "super-event-property-schema",
    feature = "general-schema-section"
))]
mod r#super_event;
#[cfg(any(
    feature = "super-event-property-schema",
    feature = "general-schema-section"
))]
pub use r#super_event::*;
#[cfg(any(
    feature = "superseded-by-property-schema",
    feature = "meta-schema-section"
))]
mod r#superseded_by;
#[cfg(any(
    feature = "superseded-by-property-schema",
    feature = "meta-schema-section"
))]
pub use r#superseded_by::*;
#[cfg(any(feature = "supply-property-schema", feature = "general-schema-section"))]
mod r#supply;
#[cfg(any(feature = "supply-property-schema", feature = "general-schema-section"))]
pub use r#supply::*;
#[cfg(any(
    feature = "supply-to-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#supply_to;
#[cfg(any(
    feature = "supply-to-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#supply_to::*;
#[cfg(any(
    feature = "supporting-data-property-schema",
    feature = "general-schema-section"
))]
mod r#supporting_data;
#[cfg(any(
    feature = "supporting-data-property-schema",
    feature = "general-schema-section"
))]
pub use r#supporting_data::*;
#[cfg(any(
    feature = "surface-property-schema",
    feature = "general-schema-section"
))]
mod r#surface;
#[cfg(any(
    feature = "surface-property-schema",
    feature = "general-schema-section"
))]
pub use r#surface::*;
#[cfg(any(
    feature = "syllabus-sections-property-schema",
    feature = "pending-schema-section"
))]
mod r#syllabus_sections;
#[cfg(any(
    feature = "syllabus-sections-property-schema",
    feature = "pending-schema-section"
))]
pub use r#syllabus_sections::*;
#[cfg(any(feature = "target-property-schema", feature = "general-schema-section"))]
mod r#target;
#[cfg(any(feature = "target-property-schema", feature = "general-schema-section"))]
pub use r#target::*;
#[cfg(any(
    feature = "target-collection-property-schema",
    feature = "general-schema-section"
))]
mod r#target_collection;
#[cfg(any(
    feature = "target-collection-property-schema",
    feature = "general-schema-section"
))]
pub use r#target_collection::*;
#[cfg(any(
    feature = "target-description-property-schema",
    feature = "general-schema-section"
))]
mod r#target_description;
#[cfg(any(
    feature = "target-description-property-schema",
    feature = "general-schema-section"
))]
pub use r#target_description::*;
#[cfg(any(
    feature = "target-name-property-schema",
    feature = "general-schema-section"
))]
mod r#target_name;
#[cfg(any(
    feature = "target-name-property-schema",
    feature = "general-schema-section"
))]
pub use r#target_name::*;
#[cfg(any(
    feature = "target-platform-property-schema",
    feature = "general-schema-section"
))]
mod r#target_platform;
#[cfg(any(
    feature = "target-platform-property-schema",
    feature = "general-schema-section"
))]
pub use r#target_platform::*;
#[cfg(any(
    feature = "target-population-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#target_population;
#[cfg(any(
    feature = "target-population-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#target_population::*;
#[cfg(any(
    feature = "target-product-property-schema",
    feature = "general-schema-section"
))]
mod r#target_product;
#[cfg(any(
    feature = "target-product-property-schema",
    feature = "general-schema-section"
))]
pub use r#target_product::*;
#[cfg(any(
    feature = "target-url-property-schema",
    feature = "general-schema-section"
))]
mod r#target_url;
#[cfg(any(
    feature = "target-url-property-schema",
    feature = "general-schema-section"
))]
pub use r#target_url::*;
#[cfg(any(feature = "tax-id-property-schema", feature = "general-schema-section"))]
mod r#tax_id;
#[cfg(any(feature = "tax-id-property-schema", feature = "general-schema-section"))]
pub use r#tax_id::*;
#[cfg(any(
    feature = "taxon-rank-property-schema",
    feature = "pending-schema-section"
))]
mod r#taxon_rank;
#[cfg(any(
    feature = "taxon-rank-property-schema",
    feature = "pending-schema-section"
))]
pub use r#taxon_rank::*;
#[cfg(any(
    feature = "taxonomic-range-property-schema",
    feature = "pending-schema-section"
))]
mod r#taxonomic_range;
#[cfg(any(
    feature = "taxonomic-range-property-schema",
    feature = "pending-schema-section"
))]
pub use r#taxonomic_range::*;
#[cfg(any(
    feature = "teaches-property-schema",
    feature = "pending-schema-section"
))]
mod r#teaches;
#[cfg(any(
    feature = "teaches-property-schema",
    feature = "pending-schema-section"
))]
pub use r#teaches::*;
#[cfg(any(
    feature = "telephone-property-schema",
    feature = "general-schema-section"
))]
mod r#telephone;
#[cfg(any(
    feature = "telephone-property-schema",
    feature = "general-schema-section"
))]
pub use r#telephone::*;
#[cfg(any(
    feature = "temporal-property-schema",
    feature = "general-schema-section"
))]
mod r#temporal;
#[cfg(any(
    feature = "temporal-property-schema",
    feature = "general-schema-section"
))]
pub use r#temporal::*;
#[cfg(any(
    feature = "temporal-coverage-property-schema",
    feature = "general-schema-section"
))]
mod r#temporal_coverage;
#[cfg(any(
    feature = "temporal-coverage-property-schema",
    feature = "general-schema-section"
))]
pub use r#temporal_coverage::*;
#[cfg(any(
    feature = "term-code-property-schema",
    feature = "pending-schema-section"
))]
mod r#term_code;
#[cfg(any(
    feature = "term-code-property-schema",
    feature = "pending-schema-section"
))]
pub use r#term_code::*;
#[cfg(any(
    feature = "term-duration-property-schema",
    feature = "pending-schema-section"
))]
mod r#term_duration;
#[cfg(any(
    feature = "term-duration-property-schema",
    feature = "pending-schema-section"
))]
pub use r#term_duration::*;
#[cfg(any(
    feature = "terms-of-service-property-schema",
    feature = "pending-schema-section"
))]
mod r#terms_of_service;
#[cfg(any(
    feature = "terms-of-service-property-schema",
    feature = "pending-schema-section"
))]
pub use r#terms_of_service::*;
#[cfg(any(
    feature = "terms-per-year-property-schema",
    feature = "pending-schema-section"
))]
mod r#terms_per_year;
#[cfg(any(
    feature = "terms-per-year-property-schema",
    feature = "pending-schema-section"
))]
pub use r#terms_per_year::*;
#[cfg(any(feature = "text-property-schema", feature = "general-schema-section"))]
mod r#text;
#[cfg(any(feature = "text-property-schema", feature = "general-schema-section"))]
pub use r#text::*;
#[cfg(any(
    feature = "text-value-property-schema",
    feature = "pending-schema-section"
))]
mod r#text_value;
#[cfg(any(
    feature = "text-value-property-schema",
    feature = "pending-schema-section"
))]
pub use r#text_value::*;
#[cfg(any(
    feature = "thumbnail-property-schema",
    feature = "general-schema-section"
))]
mod r#thumbnail;
#[cfg(any(
    feature = "thumbnail-property-schema",
    feature = "general-schema-section"
))]
pub use r#thumbnail::*;
#[cfg(any(
    feature = "thumbnail-url-property-schema",
    feature = "general-schema-section"
))]
mod r#thumbnail_url;
#[cfg(any(
    feature = "thumbnail-url-property-schema",
    feature = "general-schema-section"
))]
pub use r#thumbnail_url::*;
#[cfg(any(
    feature = "ticker-symbol-property-schema",
    feature = "general-schema-section"
))]
mod r#ticker_symbol;
#[cfg(any(
    feature = "ticker-symbol-property-schema",
    feature = "general-schema-section"
))]
pub use r#ticker_symbol::*;
#[cfg(any(
    feature = "ticket-number-property-schema",
    feature = "general-schema-section"
))]
mod r#ticket_number;
#[cfg(any(
    feature = "ticket-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#ticket_number::*;
#[cfg(any(
    feature = "ticket-token-property-schema",
    feature = "general-schema-section"
))]
mod r#ticket_token;
#[cfg(any(
    feature = "ticket-token-property-schema",
    feature = "general-schema-section"
))]
pub use r#ticket_token::*;
#[cfg(any(
    feature = "ticketed-seat-property-schema",
    feature = "general-schema-section"
))]
mod r#ticketed_seat;
#[cfg(any(
    feature = "ticketed-seat-property-schema",
    feature = "general-schema-section"
))]
pub use r#ticketed_seat::*;
#[cfg(any(
    feature = "time-of-day-property-schema",
    feature = "pending-schema-section"
))]
mod r#time_of_day;
#[cfg(any(
    feature = "time-of-day-property-schema",
    feature = "pending-schema-section"
))]
pub use r#time_of_day::*;
#[cfg(any(
    feature = "time-required-property-schema",
    feature = "general-schema-section"
))]
mod r#time_required;
#[cfg(any(
    feature = "time-required-property-schema",
    feature = "general-schema-section"
))]
pub use r#time_required::*;
#[cfg(any(
    feature = "time-to-complete-property-schema",
    feature = "pending-schema-section"
))]
mod r#time_to_complete;
#[cfg(any(
    feature = "time-to-complete-property-schema",
    feature = "pending-schema-section"
))]
pub use r#time_to_complete::*;
#[cfg(any(
    feature = "tissue-sample-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#tissue_sample;
#[cfg(any(
    feature = "tissue-sample-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#tissue_sample::*;
#[cfg(any(feature = "title-property-schema", feature = "general-schema-section"))]
mod r#title;
#[cfg(any(feature = "title-property-schema", feature = "general-schema-section"))]
pub use r#title::*;
#[cfg(any(
    feature = "title-eidr-property-schema",
    feature = "pending-schema-section"
))]
mod r#title_eidr;
#[cfg(any(
    feature = "title-eidr-property-schema",
    feature = "pending-schema-section"
))]
pub use r#title_eidr::*;
#[cfg(any(
    feature = "to-location-property-schema",
    feature = "general-schema-section"
))]
mod r#to_location;
#[cfg(any(
    feature = "to-location-property-schema",
    feature = "general-schema-section"
))]
pub use r#to_location::*;
#[cfg(any(
    feature = "to-recipient-property-schema",
    feature = "general-schema-section"
))]
mod r#to_recipient;
#[cfg(any(
    feature = "to-recipient-property-schema",
    feature = "general-schema-section"
))]
pub use r#to_recipient::*;
#[cfg(any(
    feature = "toc-continuation-property-schema",
    feature = "pending-schema-section"
))]
mod r#toc_continuation;
#[cfg(any(
    feature = "toc-continuation-property-schema",
    feature = "pending-schema-section"
))]
pub use r#toc_continuation::*;
#[cfg(any(
    feature = "toc-entry-property-schema",
    feature = "pending-schema-section"
))]
mod r#toc_entry;
#[cfg(any(
    feature = "toc-entry-property-schema",
    feature = "pending-schema-section"
))]
pub use r#toc_entry::*;
#[cfg(any(
    feature = "tongue-weight-property-schema",
    feature = "auto-schema-section"
))]
mod r#tongue_weight;
#[cfg(any(
    feature = "tongue-weight-property-schema",
    feature = "auto-schema-section"
))]
pub use r#tongue_weight::*;
#[cfg(any(feature = "tool-property-schema", feature = "general-schema-section"))]
mod r#tool;
#[cfg(any(feature = "tool-property-schema", feature = "general-schema-section"))]
pub use r#tool::*;
#[cfg(any(feature = "torque-property-schema", feature = "auto-schema-section"))]
mod r#torque;
#[cfg(any(feature = "torque-property-schema", feature = "auto-schema-section"))]
pub use r#torque::*;
#[cfg(any(
    feature = "total-historical-enrollment-property-schema",
    feature = "pending-schema-section"
))]
mod r#total_historical_enrollment;
#[cfg(any(
    feature = "total-historical-enrollment-property-schema",
    feature = "pending-schema-section"
))]
pub use r#total_historical_enrollment::*;
#[cfg(any(
    feature = "total-job-openings-property-schema",
    feature = "pending-schema-section"
))]
mod r#total_job_openings;
#[cfg(any(
    feature = "total-job-openings-property-schema",
    feature = "pending-schema-section"
))]
pub use r#total_job_openings::*;
#[cfg(any(
    feature = "total-payment-due-property-schema",
    feature = "general-schema-section"
))]
mod r#total_payment_due;
#[cfg(any(
    feature = "total-payment-due-property-schema",
    feature = "general-schema-section"
))]
pub use r#total_payment_due::*;
#[cfg(any(
    feature = "total-price-property-schema",
    feature = "general-schema-section"
))]
mod r#total_price;
#[cfg(any(
    feature = "total-price-property-schema",
    feature = "general-schema-section"
))]
pub use r#total_price::*;
#[cfg(any(
    feature = "total-time-property-schema",
    feature = "general-schema-section"
))]
mod r#total_time;
#[cfg(any(
    feature = "total-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#total_time::*;
#[cfg(any(
    feature = "tour-booking-page-property-schema",
    feature = "pending-schema-section"
))]
mod r#tour_booking_page;
#[cfg(any(
    feature = "tour-booking-page-property-schema",
    feature = "pending-schema-section"
))]
pub use r#tour_booking_page::*;
#[cfg(any(
    feature = "tourist-type-property-schema",
    feature = "general-schema-section"
))]
mod r#tourist_type;
#[cfg(any(
    feature = "tourist-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#tourist_type::*;
#[cfg(any(feature = "track-property-schema", feature = "general-schema-section"))]
mod r#track;
#[cfg(any(feature = "track-property-schema", feature = "general-schema-section"))]
pub use r#track::*;
#[cfg(any(
    feature = "tracking-number-property-schema",
    feature = "general-schema-section"
))]
mod r#tracking_number;
#[cfg(any(
    feature = "tracking-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#tracking_number::*;
#[cfg(any(
    feature = "tracking-url-property-schema",
    feature = "general-schema-section"
))]
mod r#tracking_url;
#[cfg(any(
    feature = "tracking-url-property-schema",
    feature = "general-schema-section"
))]
pub use r#tracking_url::*;
#[cfg(any(feature = "tracks-property-schema", feature = "general-schema-section"))]
mod r#tracks;
#[cfg(any(feature = "tracks-property-schema", feature = "general-schema-section"))]
pub use r#tracks::*;
#[cfg(any(
    feature = "trailer-property-schema",
    feature = "general-schema-section"
))]
mod r#trailer;
#[cfg(any(
    feature = "trailer-property-schema",
    feature = "general-schema-section"
))]
pub use r#trailer::*;
#[cfg(any(
    feature = "trailer-weight-property-schema",
    feature = "auto-schema-section"
))]
mod r#trailer_weight;
#[cfg(any(
    feature = "trailer-weight-property-schema",
    feature = "auto-schema-section"
))]
pub use r#trailer_weight::*;
#[cfg(any(
    feature = "train-name-property-schema",
    feature = "general-schema-section"
))]
mod r#train_name;
#[cfg(any(
    feature = "train-name-property-schema",
    feature = "general-schema-section"
))]
pub use r#train_name::*;
#[cfg(any(
    feature = "train-number-property-schema",
    feature = "general-schema-section"
))]
mod r#train_number;
#[cfg(any(
    feature = "train-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#train_number::*;
#[cfg(any(
    feature = "training-salary-property-schema",
    feature = "pending-schema-section"
))]
mod r#training_salary;
#[cfg(any(
    feature = "training-salary-property-schema",
    feature = "pending-schema-section"
))]
pub use r#training_salary::*;
#[cfg(any(
    feature = "trans-fat-content-property-schema",
    feature = "general-schema-section"
))]
mod r#trans_fat_content;
#[cfg(any(
    feature = "trans-fat-content-property-schema",
    feature = "general-schema-section"
))]
pub use r#trans_fat_content::*;
#[cfg(any(
    feature = "transcript-property-schema",
    feature = "general-schema-section"
))]
mod r#transcript;
#[cfg(any(
    feature = "transcript-property-schema",
    feature = "general-schema-section"
))]
pub use r#transcript::*;
#[cfg(any(
    feature = "transit-time-property-schema",
    feature = "pending-schema-section"
))]
mod r#transit_time;
#[cfg(any(
    feature = "transit-time-property-schema",
    feature = "pending-schema-section"
))]
pub use r#transit_time::*;
#[cfg(any(
    feature = "transit-time-label-property-schema",
    feature = "pending-schema-section"
))]
mod r#transit_time_label;
#[cfg(any(
    feature = "transit-time-label-property-schema",
    feature = "pending-schema-section"
))]
pub use r#transit_time_label::*;
#[cfg(any(
    feature = "translation-of-work-property-schema",
    feature = "bib-schema-section"
))]
mod r#translation_of_work;
#[cfg(any(
    feature = "translation-of-work-property-schema",
    feature = "bib-schema-section"
))]
pub use r#translation_of_work::*;
#[cfg(any(
    feature = "translator-property-schema",
    feature = "general-schema-section"
))]
mod r#translator;
#[cfg(any(
    feature = "translator-property-schema",
    feature = "general-schema-section"
))]
pub use r#translator::*;
#[cfg(any(
    feature = "transmission-method-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#transmission_method;
#[cfg(any(
    feature = "transmission-method-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#transmission_method::*;
#[cfg(any(
    feature = "travel-bans-property-schema",
    feature = "pending-schema-section"
))]
mod r#travel_bans;
#[cfg(any(
    feature = "travel-bans-property-schema",
    feature = "pending-schema-section"
))]
pub use r#travel_bans::*;
#[cfg(any(
    feature = "trial-design-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#trial_design;
#[cfg(any(
    feature = "trial-design-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#trial_design::*;
#[cfg(any(
    feature = "tributary-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#tributary;
#[cfg(any(
    feature = "tributary-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#tributary::*;
#[cfg(any(
    feature = "trip-origin-property-schema",
    feature = "general-schema-section"
))]
mod r#trip_origin;
#[cfg(any(
    feature = "trip-origin-property-schema",
    feature = "general-schema-section"
))]
pub use r#trip_origin::*;
#[cfg(any(
    feature = "type-of-bed-property-schema",
    feature = "general-schema-section"
))]
mod r#type_of_bed;
#[cfg(any(
    feature = "type-of-bed-property-schema",
    feature = "general-schema-section"
))]
pub use r#type_of_bed::*;
#[cfg(any(
    feature = "type-of-good-property-schema",
    feature = "general-schema-section"
))]
mod r#type_of_good;
#[cfg(any(
    feature = "type-of-good-property-schema",
    feature = "general-schema-section"
))]
pub use r#type_of_good::*;
#[cfg(any(
    feature = "typical-age-range-property-schema",
    feature = "general-schema-section"
))]
mod r#typical_age_range;
#[cfg(any(
    feature = "typical-age-range-property-schema",
    feature = "general-schema-section"
))]
pub use r#typical_age_range::*;
#[cfg(any(
    feature = "typical-credits-per-term-property-schema",
    feature = "pending-schema-section"
))]
mod r#typical_credits_per_term;
#[cfg(any(
    feature = "typical-credits-per-term-property-schema",
    feature = "pending-schema-section"
))]
pub use r#typical_credits_per_term::*;
#[cfg(any(
    feature = "typical-test-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#typical_test;
#[cfg(any(
    feature = "typical-test-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#typical_test::*;
#[cfg(any(
    feature = "under-name-property-schema",
    feature = "general-schema-section"
))]
mod r#under_name;
#[cfg(any(
    feature = "under-name-property-schema",
    feature = "general-schema-section"
))]
pub use r#under_name::*;
#[cfg(any(
    feature = "unit-code-property-schema",
    feature = "general-schema-section"
))]
mod r#unit_code;
#[cfg(any(
    feature = "unit-code-property-schema",
    feature = "general-schema-section"
))]
pub use r#unit_code::*;
#[cfg(any(
    feature = "unit-text-property-schema",
    feature = "general-schema-section"
))]
mod r#unit_text;
#[cfg(any(
    feature = "unit-text-property-schema",
    feature = "general-schema-section"
))]
pub use r#unit_text::*;
#[cfg(any(
    feature = "unnamed-sources-policy-property-schema",
    feature = "pending-schema-section"
))]
mod r#unnamed_sources_policy;
#[cfg(any(
    feature = "unnamed-sources-policy-property-schema",
    feature = "pending-schema-section"
))]
pub use r#unnamed_sources_policy::*;
#[cfg(any(
    feature = "unsaturated-fat-content-property-schema",
    feature = "general-schema-section"
))]
mod r#unsaturated_fat_content;
#[cfg(any(
    feature = "unsaturated-fat-content-property-schema",
    feature = "general-schema-section"
))]
pub use r#unsaturated_fat_content::*;
#[cfg(any(
    feature = "upload-date-property-schema",
    feature = "general-schema-section"
))]
mod r#upload_date;
#[cfg(any(
    feature = "upload-date-property-schema",
    feature = "general-schema-section"
))]
pub use r#upload_date::*;
#[cfg(any(
    feature = "upvote-count-property-schema",
    feature = "general-schema-section"
))]
mod r#upvote_count;
#[cfg(any(
    feature = "upvote-count-property-schema",
    feature = "general-schema-section"
))]
pub use r#upvote_count::*;
#[cfg(any(feature = "url-property-schema", feature = "general-schema-section"))]
mod r#url;
#[cfg(any(feature = "url-property-schema", feature = "general-schema-section"))]
pub use r#url::*;
#[cfg(any(
    feature = "url-template-property-schema",
    feature = "general-schema-section"
))]
mod r#url_template;
#[cfg(any(
    feature = "url-template-property-schema",
    feature = "general-schema-section"
))]
pub use r#url_template::*;
#[cfg(any(
    feature = "usage-info-property-schema",
    feature = "pending-schema-section"
))]
mod r#usage_info;
#[cfg(any(
    feature = "usage-info-property-schema",
    feature = "pending-schema-section"
))]
pub use r#usage_info::*;
#[cfg(any(
    feature = "used-to-diagnose-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#used_to_diagnose;
#[cfg(any(
    feature = "used-to-diagnose-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#used_to_diagnose::*;
#[cfg(any(
    feature = "user-interaction-count-property-schema",
    feature = "general-schema-section"
))]
mod r#user_interaction_count;
#[cfg(any(
    feature = "user-interaction-count-property-schema",
    feature = "general-schema-section"
))]
pub use r#user_interaction_count::*;
#[cfg(any(
    feature = "uses-device-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#uses_device;
#[cfg(any(
    feature = "uses-device-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#uses_device::*;
#[cfg(any(
    feature = "uses-health-plan-id-standard-property-schema",
    feature = "pending-schema-section"
))]
mod r#uses_health_plan_id_standard;
#[cfg(any(
    feature = "uses-health-plan-id-standard-property-schema",
    feature = "pending-schema-section"
))]
pub use r#uses_health_plan_id_standard::*;
#[cfg(any(
    feature = "utterances-property-schema",
    feature = "pending-schema-section"
))]
mod r#utterances;
#[cfg(any(
    feature = "utterances-property-schema",
    feature = "pending-schema-section"
))]
pub use r#utterances::*;
#[cfg(any(
    feature = "valid-for-property-schema",
    feature = "general-schema-section"
))]
mod r#valid_for;
#[cfg(any(
    feature = "valid-for-property-schema",
    feature = "general-schema-section"
))]
pub use r#valid_for::*;
#[cfg(any(
    feature = "valid-from-property-schema",
    feature = "general-schema-section"
))]
mod r#valid_from;
#[cfg(any(
    feature = "valid-from-property-schema",
    feature = "general-schema-section"
))]
pub use r#valid_from::*;
#[cfg(any(
    feature = "valid-in-property-schema",
    feature = "general-schema-section"
))]
mod r#valid_in;
#[cfg(any(
    feature = "valid-in-property-schema",
    feature = "general-schema-section"
))]
pub use r#valid_in::*;
#[cfg(any(
    feature = "valid-through-property-schema",
    feature = "general-schema-section"
))]
mod r#valid_through;
#[cfg(any(
    feature = "valid-through-property-schema",
    feature = "general-schema-section"
))]
pub use r#valid_through::*;
#[cfg(any(
    feature = "valid-until-property-schema",
    feature = "general-schema-section"
))]
mod r#valid_until;
#[cfg(any(
    feature = "valid-until-property-schema",
    feature = "general-schema-section"
))]
pub use r#valid_until::*;
#[cfg(any(feature = "value-property-schema", feature = "general-schema-section"))]
mod r#value;
#[cfg(any(feature = "value-property-schema", feature = "general-schema-section"))]
pub use r#value::*;
#[cfg(any(
    feature = "value-added-tax-included-property-schema",
    feature = "general-schema-section"
))]
mod r#value_added_tax_included;
#[cfg(any(
    feature = "value-added-tax-included-property-schema",
    feature = "general-schema-section"
))]
pub use r#value_added_tax_included::*;
#[cfg(any(
    feature = "value-max-length-property-schema",
    feature = "general-schema-section"
))]
mod r#value_max_length;
#[cfg(any(
    feature = "value-max-length-property-schema",
    feature = "general-schema-section"
))]
pub use r#value_max_length::*;
#[cfg(any(
    feature = "value-min-length-property-schema",
    feature = "general-schema-section"
))]
mod r#value_min_length;
#[cfg(any(
    feature = "value-min-length-property-schema",
    feature = "general-schema-section"
))]
pub use r#value_min_length::*;
#[cfg(any(
    feature = "value-name-property-schema",
    feature = "general-schema-section"
))]
mod r#value_name;
#[cfg(any(
    feature = "value-name-property-schema",
    feature = "general-schema-section"
))]
pub use r#value_name::*;
#[cfg(any(
    feature = "value-pattern-property-schema",
    feature = "general-schema-section"
))]
mod r#value_pattern;
#[cfg(any(
    feature = "value-pattern-property-schema",
    feature = "general-schema-section"
))]
pub use r#value_pattern::*;
#[cfg(any(
    feature = "value-reference-property-schema",
    feature = "general-schema-section"
))]
mod r#value_reference;
#[cfg(any(
    feature = "value-reference-property-schema",
    feature = "general-schema-section"
))]
pub use r#value_reference::*;
#[cfg(any(
    feature = "value-required-property-schema",
    feature = "general-schema-section"
))]
mod r#value_required;
#[cfg(any(
    feature = "value-required-property-schema",
    feature = "general-schema-section"
))]
pub use r#value_required::*;
#[cfg(any(
    feature = "variable-measured-property-schema",
    feature = "pending-schema-section"
))]
mod r#variable_measured;
#[cfg(any(
    feature = "variable-measured-property-schema",
    feature = "pending-schema-section"
))]
pub use r#variable_measured::*;
#[cfg(any(
    feature = "variables-measured-property-schema",
    feature = "attic-schema-section"
))]
mod r#variables_measured;
#[cfg(any(
    feature = "variables-measured-property-schema",
    feature = "attic-schema-section"
))]
pub use r#variables_measured::*;
#[cfg(any(
    feature = "variant-cover-property-schema",
    feature = "bib-schema-section"
))]
mod r#variant_cover;
#[cfg(any(
    feature = "variant-cover-property-schema",
    feature = "bib-schema-section"
))]
pub use r#variant_cover::*;
#[cfg(any(
    feature = "varies-by-property-schema",
    feature = "pending-schema-section"
))]
mod r#varies_by;
#[cfg(any(
    feature = "varies-by-property-schema",
    feature = "pending-schema-section"
))]
pub use r#varies_by::*;
#[cfg(any(feature = "vat-id-property-schema", feature = "general-schema-section"))]
mod r#vat_id;
#[cfg(any(feature = "vat-id-property-schema", feature = "general-schema-section"))]
pub use r#vat_id::*;
#[cfg(any(
    feature = "vehicle-configuration-property-schema",
    feature = "general-schema-section"
))]
mod r#vehicle_configuration;
#[cfg(any(
    feature = "vehicle-configuration-property-schema",
    feature = "general-schema-section"
))]
pub use r#vehicle_configuration::*;
#[cfg(any(
    feature = "vehicle-engine-property-schema",
    feature = "general-schema-section"
))]
mod r#vehicle_engine;
#[cfg(any(
    feature = "vehicle-engine-property-schema",
    feature = "general-schema-section"
))]
pub use r#vehicle_engine::*;
#[cfg(any(
    feature = "vehicle-identification-number-property-schema",
    feature = "general-schema-section"
))]
mod r#vehicle_identification_number;
#[cfg(any(
    feature = "vehicle-identification-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#vehicle_identification_number::*;
#[cfg(any(
    feature = "vehicle-interior-color-property-schema",
    feature = "general-schema-section"
))]
mod r#vehicle_interior_color;
#[cfg(any(
    feature = "vehicle-interior-color-property-schema",
    feature = "general-schema-section"
))]
pub use r#vehicle_interior_color::*;
#[cfg(any(
    feature = "vehicle-interior-type-property-schema",
    feature = "general-schema-section"
))]
mod r#vehicle_interior_type;
#[cfg(any(
    feature = "vehicle-interior-type-property-schema",
    feature = "general-schema-section"
))]
pub use r#vehicle_interior_type::*;
#[cfg(any(
    feature = "vehicle-model-date-property-schema",
    feature = "general-schema-section"
))]
mod r#vehicle_model_date;
#[cfg(any(
    feature = "vehicle-model-date-property-schema",
    feature = "general-schema-section"
))]
pub use r#vehicle_model_date::*;
#[cfg(any(
    feature = "vehicle-seating-capacity-property-schema",
    feature = "general-schema-section"
))]
mod r#vehicle_seating_capacity;
#[cfg(any(
    feature = "vehicle-seating-capacity-property-schema",
    feature = "general-schema-section"
))]
pub use r#vehicle_seating_capacity::*;
#[cfg(any(
    feature = "vehicle-special-usage-property-schema",
    feature = "auto-schema-section"
))]
mod r#vehicle_special_usage;
#[cfg(any(
    feature = "vehicle-special-usage-property-schema",
    feature = "auto-schema-section"
))]
pub use r#vehicle_special_usage::*;
#[cfg(any(
    feature = "vehicle-transmission-property-schema",
    feature = "general-schema-section"
))]
mod r#vehicle_transmission;
#[cfg(any(
    feature = "vehicle-transmission-property-schema",
    feature = "general-schema-section"
))]
pub use r#vehicle_transmission::*;
#[cfg(any(feature = "vendor-property-schema", feature = "general-schema-section"))]
mod r#vendor;
#[cfg(any(feature = "vendor-property-schema", feature = "general-schema-section"))]
pub use r#vendor::*;
#[cfg(any(
    feature = "verification-fact-checking-policy-property-schema",
    feature = "pending-schema-section"
))]
mod r#verification_fact_checking_policy;
#[cfg(any(
    feature = "verification-fact-checking-policy-property-schema",
    feature = "pending-schema-section"
))]
pub use r#verification_fact_checking_policy::*;
#[cfg(any(
    feature = "version-property-schema",
    feature = "general-schema-section"
))]
mod r#version;
#[cfg(any(
    feature = "version-property-schema",
    feature = "general-schema-section"
))]
pub use r#version::*;
#[cfg(any(feature = "video-property-schema", feature = "general-schema-section"))]
mod r#video;
#[cfg(any(feature = "video-property-schema", feature = "general-schema-section"))]
pub use r#video::*;
#[cfg(any(
    feature = "video-format-property-schema",
    feature = "general-schema-section"
))]
mod r#video_format;
#[cfg(any(
    feature = "video-format-property-schema",
    feature = "general-schema-section"
))]
pub use r#video_format::*;
#[cfg(any(
    feature = "video-frame-size-property-schema",
    feature = "general-schema-section"
))]
mod r#video_frame_size;
#[cfg(any(
    feature = "video-frame-size-property-schema",
    feature = "general-schema-section"
))]
pub use r#video_frame_size::*;
#[cfg(any(
    feature = "video-quality-property-schema",
    feature = "general-schema-section"
))]
mod r#video_quality;
#[cfg(any(
    feature = "video-quality-property-schema",
    feature = "general-schema-section"
))]
pub use r#video_quality::*;
#[cfg(any(
    feature = "volume-number-property-schema",
    feature = "general-schema-section"
))]
mod r#volume_number;
#[cfg(any(
    feature = "volume-number-property-schema",
    feature = "general-schema-section"
))]
pub use r#volume_number::*;
#[cfg(any(
    feature = "warning-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#warning;
#[cfg(any(
    feature = "warning-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#warning::*;
#[cfg(any(
    feature = "warranty-property-schema",
    feature = "general-schema-section"
))]
mod r#warranty;
#[cfg(any(
    feature = "warranty-property-schema",
    feature = "general-schema-section"
))]
pub use r#warranty::*;
#[cfg(any(
    feature = "warranty-promise-property-schema",
    feature = "general-schema-section"
))]
mod r#warranty_promise;
#[cfg(any(
    feature = "warranty-promise-property-schema",
    feature = "general-schema-section"
))]
pub use r#warranty_promise::*;
#[cfg(any(
    feature = "warranty-scope-property-schema",
    feature = "general-schema-section"
))]
mod r#warranty_scope;
#[cfg(any(
    feature = "warranty-scope-property-schema",
    feature = "general-schema-section"
))]
pub use r#warranty_scope::*;
#[cfg(any(
    feature = "web-checkin-time-property-schema",
    feature = "general-schema-section"
))]
mod r#web_checkin_time;
#[cfg(any(
    feature = "web-checkin-time-property-schema",
    feature = "general-schema-section"
))]
pub use r#web_checkin_time::*;
#[cfg(any(
    feature = "web-feed-property-schema",
    feature = "pending-schema-section"
))]
mod r#web_feed;
#[cfg(any(
    feature = "web-feed-property-schema",
    feature = "pending-schema-section"
))]
pub use r#web_feed::*;
#[cfg(any(feature = "weight-property-schema", feature = "general-schema-section"))]
mod r#weight;
#[cfg(any(feature = "weight-property-schema", feature = "general-schema-section"))]
pub use r#weight::*;
#[cfg(any(
    feature = "weight-total-property-schema",
    feature = "auto-schema-section"
))]
mod r#weight_total;
#[cfg(any(
    feature = "weight-total-property-schema",
    feature = "auto-schema-section"
))]
pub use r#weight_total::*;
#[cfg(any(feature = "wheelbase-property-schema", feature = "auto-schema-section"))]
mod r#wheelbase;
#[cfg(any(feature = "wheelbase-property-schema", feature = "auto-schema-section"))]
pub use r#wheelbase::*;
#[cfg(any(feature = "width-property-schema", feature = "general-schema-section"))]
mod r#width;
#[cfg(any(feature = "width-property-schema", feature = "general-schema-section"))]
pub use r#width::*;
#[cfg(any(feature = "winner-property-schema", feature = "general-schema-section"))]
mod r#winner;
#[cfg(any(feature = "winner-property-schema", feature = "general-schema-section"))]
pub use r#winner::*;
#[cfg(any(
    feature = "word-count-property-schema",
    feature = "general-schema-section"
))]
mod r#word_count;
#[cfg(any(
    feature = "word-count-property-schema",
    feature = "general-schema-section"
))]
pub use r#word_count::*;
#[cfg(any(
    feature = "work-example-property-schema",
    feature = "general-schema-section"
))]
mod r#work_example;
#[cfg(any(
    feature = "work-example-property-schema",
    feature = "general-schema-section"
))]
pub use r#work_example::*;
#[cfg(any(
    feature = "work-featured-property-schema",
    feature = "general-schema-section"
))]
mod r#work_featured;
#[cfg(any(
    feature = "work-featured-property-schema",
    feature = "general-schema-section"
))]
pub use r#work_featured::*;
#[cfg(any(
    feature = "work-hours-property-schema",
    feature = "general-schema-section"
))]
mod r#work_hours;
#[cfg(any(
    feature = "work-hours-property-schema",
    feature = "general-schema-section"
))]
pub use r#work_hours::*;
#[cfg(any(
    feature = "work-location-property-schema",
    feature = "general-schema-section"
))]
mod r#work_location;
#[cfg(any(
    feature = "work-location-property-schema",
    feature = "general-schema-section"
))]
pub use r#work_location::*;
#[cfg(any(
    feature = "work-performed-property-schema",
    feature = "general-schema-section"
))]
mod r#work_performed;
#[cfg(any(
    feature = "work-performed-property-schema",
    feature = "general-schema-section"
))]
pub use r#work_performed::*;
#[cfg(any(
    feature = "work-presented-property-schema",
    feature = "general-schema-section"
))]
mod r#work_presented;
#[cfg(any(
    feature = "work-presented-property-schema",
    feature = "general-schema-section"
))]
pub use r#work_presented::*;
#[cfg(any(
    feature = "work-translation-property-schema",
    feature = "bib-schema-section"
))]
mod r#work_translation;
#[cfg(any(
    feature = "work-translation-property-schema",
    feature = "bib-schema-section"
))]
pub use r#work_translation::*;
#[cfg(any(
    feature = "workload-property-schema",
    feature = "health-lifesci-schema-section"
))]
mod r#workload;
#[cfg(any(
    feature = "workload-property-schema",
    feature = "health-lifesci-schema-section"
))]
pub use r#workload::*;
#[cfg(any(
    feature = "works-for-property-schema",
    feature = "general-schema-section"
))]
mod r#works_for;
#[cfg(any(
    feature = "works-for-property-schema",
    feature = "general-schema-section"
))]
pub use r#works_for::*;
#[cfg(any(
    feature = "worst-rating-property-schema",
    feature = "general-schema-section"
))]
mod r#worst_rating;
#[cfg(any(
    feature = "worst-rating-property-schema",
    feature = "general-schema-section"
))]
pub use r#worst_rating::*;
#[cfg(any(feature = "xpath-property-schema", feature = "general-schema-section"))]
mod r#xpath;
#[cfg(any(feature = "xpath-property-schema", feature = "general-schema-section"))]
pub use r#xpath::*;
#[cfg(any(
    feature = "year-built-property-schema",
    feature = "pending-schema-section"
))]
mod r#year_built;
#[cfg(any(
    feature = "year-built-property-schema",
    feature = "pending-schema-section"
))]
pub use r#year_built::*;
#[cfg(any(
    feature = "yearly-revenue-property-schema",
    feature = "general-schema-section"
))]
mod r#yearly_revenue;
#[cfg(any(
    feature = "yearly-revenue-property-schema",
    feature = "general-schema-section"
))]
pub use r#yearly_revenue::*;
#[cfg(any(
    feature = "years-in-operation-property-schema",
    feature = "general-schema-section"
))]
mod r#years_in_operation;
#[cfg(any(
    feature = "years-in-operation-property-schema",
    feature = "general-schema-section"
))]
pub use r#years_in_operation::*;
#[cfg(any(feature = "yield-property-schema", feature = "general-schema-section"))]
mod r#yield;
#[cfg(any(feature = "yield-property-schema", feature = "general-schema-section"))]
pub use r#yield::*;
