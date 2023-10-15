use super::*;
#[cfg(any(
    any(feature = "about-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#about;
#[cfg(any(
    any(feature = "about-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#about::*;
#[cfg(any(
    any(feature = "abridged-property-schema", feature = "bib-schema-section"),
    doc
))]
mod r#abridged;
#[cfg(any(
    any(feature = "abridged-property-schema", feature = "bib-schema-section"),
    doc
))]
pub use r#abridged::*;
#[cfg(any(
    any(
        feature = "abstract-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#abstract;
#[cfg(any(
    any(
        feature = "abstract-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#abstract::*;
#[cfg(any(
    any(
        feature = "acceleration-time-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
mod r#acceleration_time;
#[cfg(any(
    any(
        feature = "acceleration-time-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
pub use r#acceleration_time::*;
#[cfg(any(
    any(
        feature = "accepted-answer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#accepted_answer;
#[cfg(any(
    any(
        feature = "accepted-answer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#accepted_answer::*;
#[cfg(any(
    any(
        feature = "accepted-offer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#accepted_offer;
#[cfg(any(
    any(
        feature = "accepted-offer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#accepted_offer::*;
#[cfg(any(
    any(
        feature = "accepted-payment-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#accepted_payment_method;
#[cfg(any(
    any(
        feature = "accepted-payment-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#accepted_payment_method::*;
#[cfg(any(
    any(
        feature = "accepts-reservations-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#accepts_reservations;
#[cfg(any(
    any(
        feature = "accepts-reservations-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#accepts_reservations::*;
#[cfg(any(
    any(
        feature = "access-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#access_code;
#[cfg(any(
    any(
        feature = "access-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#access_code::*;
#[cfg(any(
    any(
        feature = "access-mode-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#access_mode;
#[cfg(any(
    any(
        feature = "access-mode-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#access_mode::*;
#[cfg(any(
    any(
        feature = "access-mode-sufficient-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#access_mode_sufficient;
#[cfg(any(
    any(
        feature = "access-mode-sufficient-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#access_mode_sufficient::*;
#[cfg(any(
    any(
        feature = "accessibility-api-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#accessibility_api;
#[cfg(any(
    any(
        feature = "accessibility-api-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#accessibility_api::*;
#[cfg(any(
    any(
        feature = "accessibility-control-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#accessibility_control;
#[cfg(any(
    any(
        feature = "accessibility-control-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#accessibility_control::*;
#[cfg(any(
    any(
        feature = "accessibility-feature-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#accessibility_feature;
#[cfg(any(
    any(
        feature = "accessibility-feature-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#accessibility_feature::*;
#[cfg(any(
    any(
        feature = "accessibility-hazard-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#accessibility_hazard;
#[cfg(any(
    any(
        feature = "accessibility-hazard-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#accessibility_hazard::*;
#[cfg(any(
    any(
        feature = "accessibility-summary-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#accessibility_summary;
#[cfg(any(
    any(
        feature = "accessibility-summary-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#accessibility_summary::*;
#[cfg(any(
    any(
        feature = "accommodation-category-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#accommodation_category;
#[cfg(any(
    any(
        feature = "accommodation-category-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#accommodation_category::*;
#[cfg(any(
    any(
        feature = "accommodation-floor-plan-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#accommodation_floor_plan;
#[cfg(any(
    any(
        feature = "accommodation-floor-plan-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#accommodation_floor_plan::*;
#[cfg(any(
    any(
        feature = "account-id-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#account_id;
#[cfg(any(
    any(
        feature = "account-id-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#account_id::*;
#[cfg(any(
    any(
        feature = "account-minimum-inflow-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#account_minimum_inflow;
#[cfg(any(
    any(
        feature = "account-minimum-inflow-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#account_minimum_inflow::*;
#[cfg(any(
    any(
        feature = "account-overdraft-limit-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#account_overdraft_limit;
#[cfg(any(
    any(
        feature = "account-overdraft-limit-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#account_overdraft_limit::*;
#[cfg(any(
    any(
        feature = "accountable-person-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#accountable_person;
#[cfg(any(
    any(
        feature = "accountable-person-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#accountable_person::*;
#[cfg(any(
    any(
        feature = "acquire-license-page-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#acquire_license_page;
#[cfg(any(
    any(
        feature = "acquire-license-page-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#acquire_license_page::*;
#[cfg(any(
    any(
        feature = "acquired-from-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#acquired_from;
#[cfg(any(
    any(
        feature = "acquired-from-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#acquired_from::*;
#[cfg(any(
    any(
        feature = "acriss-code-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
mod r#acriss_code;
#[cfg(any(
    any(
        feature = "acriss-code-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
pub use r#acriss_code::*;
#[cfg(any(
    any(
        feature = "action-accessibility-requirement-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#action_accessibility_requirement;
#[cfg(any(
    any(
        feature = "action-accessibility-requirement-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#action_accessibility_requirement::*;
#[cfg(any(
    any(
        feature = "action-application-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#action_application;
#[cfg(any(
    any(
        feature = "action-application-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#action_application::*;
#[cfg(any(
    any(
        feature = "action-option-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#action_option;
#[cfg(any(
    any(
        feature = "action-option-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#action_option::*;
#[cfg(any(
    any(
        feature = "action-platform-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#action_platform;
#[cfg(any(
    any(
        feature = "action-platform-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#action_platform::*;
#[cfg(any(
    any(
        feature = "action-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#action_status;
#[cfg(any(
    any(
        feature = "action-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#action_status::*;
#[cfg(any(
    any(
        feature = "actionable-feedback-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#actionable_feedback_policy;
#[cfg(any(
    any(
        feature = "actionable-feedback-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#actionable_feedback_policy::*;
#[cfg(any(
    any(
        feature = "active-ingredient-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#active_ingredient;
#[cfg(any(
    any(
        feature = "active-ingredient-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#active_ingredient::*;
#[cfg(any(
    any(
        feature = "activity-duration-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#activity_duration;
#[cfg(any(
    any(
        feature = "activity-duration-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#activity_duration::*;
#[cfg(any(
    any(
        feature = "activity-frequency-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#activity_frequency;
#[cfg(any(
    any(
        feature = "activity-frequency-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#activity_frequency::*;
#[cfg(any(
    any(feature = "actor-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#actor;
#[cfg(any(
    any(feature = "actor-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#actor::*;
#[cfg(any(
    any(feature = "actors-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#actors;
#[cfg(any(
    any(feature = "actors-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#actors::*;
#[cfg(any(
    any(feature = "add-on-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#add_on;
#[cfg(any(
    any(feature = "add-on-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#add_on::*;
#[cfg(any(
    any(
        feature = "additional-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#additional_name;
#[cfg(any(
    any(
        feature = "additional-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#additional_name::*;
#[cfg(any(
    any(
        feature = "additional-number-of-guests-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#additional_number_of_guests;
#[cfg(any(
    any(
        feature = "additional-number-of-guests-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#additional_number_of_guests::*;
#[cfg(any(
    any(
        feature = "additional-property-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#additional_property;
#[cfg(any(
    any(
        feature = "additional-property-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#additional_property::*;
#[cfg(any(
    any(
        feature = "additional-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#additional_type;
#[cfg(any(
    any(
        feature = "additional-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#additional_type::*;
#[cfg(any(
    any(
        feature = "additional-variable-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#additional_variable;
#[cfg(any(
    any(
        feature = "additional-variable-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#additional_variable::*;
#[cfg(any(
    any(
        feature = "address-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#address;
#[cfg(any(
    any(
        feature = "address-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#address::*;
#[cfg(any(
    any(
        feature = "address-country-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#address_country;
#[cfg(any(
    any(
        feature = "address-country-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#address_country::*;
#[cfg(any(
    any(
        feature = "address-locality-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#address_locality;
#[cfg(any(
    any(
        feature = "address-locality-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#address_locality::*;
#[cfg(any(
    any(
        feature = "address-region-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#address_region;
#[cfg(any(
    any(
        feature = "address-region-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#address_region::*;
#[cfg(any(
    any(
        feature = "administration-route-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#administration_route;
#[cfg(any(
    any(
        feature = "administration-route-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#administration_route::*;
#[cfg(any(
    any(
        feature = "advance-booking-requirement-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#advance_booking_requirement;
#[cfg(any(
    any(
        feature = "advance-booking-requirement-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#advance_booking_requirement::*;
#[cfg(any(
    any(
        feature = "adverse-outcome-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#adverse_outcome;
#[cfg(any(
    any(
        feature = "adverse-outcome-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#adverse_outcome::*;
#[cfg(any(
    any(
        feature = "affected-by-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#affected_by;
#[cfg(any(
    any(
        feature = "affected-by-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#affected_by::*;
#[cfg(any(
    any(
        feature = "affiliation-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#affiliation;
#[cfg(any(
    any(
        feature = "affiliation-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#affiliation::*;
#[cfg(any(
    any(
        feature = "after-media-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#after_media;
#[cfg(any(
    any(
        feature = "after-media-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#after_media::*;
#[cfg(any(
    any(feature = "agent-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#agent;
#[cfg(any(
    any(feature = "agent-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#agent::*;
#[cfg(any(
    any(
        feature = "aggregate-rating-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#aggregate_rating;
#[cfg(any(
    any(
        feature = "aggregate-rating-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#aggregate_rating::*;
#[cfg(any(
    any(
        feature = "aircraft-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#aircraft;
#[cfg(any(
    any(
        feature = "aircraft-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#aircraft::*;
#[cfg(any(
    any(feature = "album-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#album;
#[cfg(any(
    any(feature = "album-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#album::*;
#[cfg(any(
    any(
        feature = "album-production-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#album_production_type;
#[cfg(any(
    any(
        feature = "album-production-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#album_production_type::*;
#[cfg(any(
    any(
        feature = "album-release-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#album_release;
#[cfg(any(
    any(
        feature = "album-release-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#album_release::*;
#[cfg(any(
    any(
        feature = "album-release-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#album_release_type;
#[cfg(any(
    any(
        feature = "album-release-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#album_release_type::*;
#[cfg(any(
    any(feature = "albums-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#albums;
#[cfg(any(
    any(feature = "albums-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#albums::*;
#[cfg(any(
    any(
        feature = "alcohol-warning-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#alcohol_warning;
#[cfg(any(
    any(
        feature = "alcohol-warning-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#alcohol_warning::*;
#[cfg(any(
    any(
        feature = "algorithm-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#algorithm;
#[cfg(any(
    any(
        feature = "algorithm-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#algorithm::*;
#[cfg(any(
    any(
        feature = "alignment-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#alignment_type;
#[cfg(any(
    any(
        feature = "alignment-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#alignment_type::*;
#[cfg(any(
    any(
        feature = "alternate-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#alternate_name;
#[cfg(any(
    any(
        feature = "alternate-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#alternate_name::*;
#[cfg(any(
    any(
        feature = "alternative-headline-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#alternative_headline;
#[cfg(any(
    any(
        feature = "alternative-headline-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#alternative_headline::*;
#[cfg(any(
    any(
        feature = "alternative-of-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#alternative_of;
#[cfg(any(
    any(
        feature = "alternative-of-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#alternative_of::*;
#[cfg(any(
    any(feature = "alumni-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#alumni;
#[cfg(any(
    any(feature = "alumni-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#alumni::*;
#[cfg(any(
    any(
        feature = "alumni-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#alumni_of;
#[cfg(any(
    any(
        feature = "alumni-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#alumni_of::*;
#[cfg(any(
    any(
        feature = "amenity-feature-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#amenity_feature;
#[cfg(any(
    any(
        feature = "amenity-feature-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#amenity_feature::*;
#[cfg(any(
    any(feature = "amount-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#amount;
#[cfg(any(
    any(feature = "amount-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#amount::*;
#[cfg(any(
    any(
        feature = "amount-of-this-good-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#amount_of_this_good;
#[cfg(any(
    any(
        feature = "amount-of-this-good-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#amount_of_this_good::*;
#[cfg(any(
    any(
        feature = "announcement-location-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#announcement_location;
#[cfg(any(
    any(
        feature = "announcement-location-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#announcement_location::*;
#[cfg(any(
    any(
        feature = "annual-percentage-rate-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#annual_percentage_rate;
#[cfg(any(
    any(
        feature = "annual-percentage-rate-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#annual_percentage_rate::*;
#[cfg(any(
    any(
        feature = "answer-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#answer_count;
#[cfg(any(
    any(
        feature = "answer-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#answer_count::*;
#[cfg(any(
    any(
        feature = "answer-explanation-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#answer_explanation;
#[cfg(any(
    any(
        feature = "answer-explanation-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#answer_explanation::*;
#[cfg(any(
    any(
        feature = "antagonist-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#antagonist;
#[cfg(any(
    any(
        feature = "antagonist-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#antagonist::*;
#[cfg(any(
    any(
        feature = "appearance-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#appearance;
#[cfg(any(
    any(
        feature = "appearance-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#appearance::*;
#[cfg(any(
    any(
        feature = "applicable-country-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#applicable_country;
#[cfg(any(
    any(
        feature = "applicable-country-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#applicable_country::*;
#[cfg(any(
    any(
        feature = "applicable-location-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#applicable_location;
#[cfg(any(
    any(
        feature = "applicable-location-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#applicable_location::*;
#[cfg(any(
    any(
        feature = "applicant-location-requirements-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#applicant_location_requirements;
#[cfg(any(
    any(
        feature = "applicant-location-requirements-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#applicant_location_requirements::*;
#[cfg(any(
    any(
        feature = "application-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#application;
#[cfg(any(
    any(
        feature = "application-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#application::*;
#[cfg(any(
    any(
        feature = "application-category-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#application_category;
#[cfg(any(
    any(
        feature = "application-category-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#application_category::*;
#[cfg(any(
    any(
        feature = "application-contact-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#application_contact;
#[cfg(any(
    any(
        feature = "application-contact-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#application_contact::*;
#[cfg(any(
    any(
        feature = "application-deadline-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#application_deadline;
#[cfg(any(
    any(
        feature = "application-deadline-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#application_deadline::*;
#[cfg(any(
    any(
        feature = "application-start-date-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#application_start_date;
#[cfg(any(
    any(
        feature = "application-start-date-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#application_start_date::*;
#[cfg(any(
    any(
        feature = "application-sub-category-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#application_sub_category;
#[cfg(any(
    any(
        feature = "application-sub-category-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#application_sub_category::*;
#[cfg(any(
    any(
        feature = "application-suite-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#application_suite;
#[cfg(any(
    any(
        feature = "application-suite-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#application_suite::*;
#[cfg(any(
    any(
        feature = "applies-to-delivery-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#applies_to_delivery_method;
#[cfg(any(
    any(
        feature = "applies-to-delivery-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#applies_to_delivery_method::*;
#[cfg(any(
    any(
        feature = "applies-to-payment-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#applies_to_payment_method;
#[cfg(any(
    any(
        feature = "applies-to-payment-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#applies_to_payment_method::*;
#[cfg(any(
    any(
        feature = "archive-held-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#archive_held;
#[cfg(any(
    any(
        feature = "archive-held-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#archive_held::*;
#[cfg(any(
    any(
        feature = "archived-at-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#archived_at;
#[cfg(any(
    any(
        feature = "archived-at-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#archived_at::*;
#[cfg(any(
    any(feature = "area-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#area;
#[cfg(any(
    any(feature = "area-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#area::*;
#[cfg(any(
    any(
        feature = "area-served-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#area_served;
#[cfg(any(
    any(
        feature = "area-served-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#area_served::*;
#[cfg(any(
    any(
        feature = "arrival-airport-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#arrival_airport;
#[cfg(any(
    any(
        feature = "arrival-airport-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#arrival_airport::*;
#[cfg(any(
    any(
        feature = "arrival-boat-terminal-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#arrival_boat_terminal;
#[cfg(any(
    any(
        feature = "arrival-boat-terminal-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#arrival_boat_terminal::*;
#[cfg(any(
    any(
        feature = "arrival-bus-stop-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#arrival_bus_stop;
#[cfg(any(
    any(
        feature = "arrival-bus-stop-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#arrival_bus_stop::*;
#[cfg(any(
    any(
        feature = "arrival-gate-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#arrival_gate;
#[cfg(any(
    any(
        feature = "arrival-gate-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#arrival_gate::*;
#[cfg(any(
    any(
        feature = "arrival-platform-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#arrival_platform;
#[cfg(any(
    any(
        feature = "arrival-platform-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#arrival_platform::*;
#[cfg(any(
    any(
        feature = "arrival-station-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#arrival_station;
#[cfg(any(
    any(
        feature = "arrival-station-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#arrival_station::*;
#[cfg(any(
    any(
        feature = "arrival-terminal-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#arrival_terminal;
#[cfg(any(
    any(
        feature = "arrival-terminal-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#arrival_terminal::*;
#[cfg(any(
    any(
        feature = "arrival-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#arrival_time;
#[cfg(any(
    any(
        feature = "arrival-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#arrival_time::*;
#[cfg(any(
    any(
        feature = "art-edition-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#art_edition;
#[cfg(any(
    any(
        feature = "art-edition-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#art_edition::*;
#[cfg(any(
    any(
        feature = "art-medium-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#art_medium;
#[cfg(any(
    any(
        feature = "art-medium-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#art_medium::*;
#[cfg(any(
    any(
        feature = "arterial-branch-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#arterial_branch;
#[cfg(any(
    any(
        feature = "arterial-branch-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#arterial_branch::*;
#[cfg(any(
    any(
        feature = "artform-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#artform;
#[cfg(any(
    any(
        feature = "artform-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#artform::*;
#[cfg(any(
    any(
        feature = "article-body-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#article_body;
#[cfg(any(
    any(
        feature = "article-body-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#article_body::*;
#[cfg(any(
    any(
        feature = "article-section-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#article_section;
#[cfg(any(
    any(
        feature = "article-section-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#article_section::*;
#[cfg(any(
    any(feature = "artist-property-schema", feature = "bib-schema-section"),
    doc
))]
mod r#artist;
#[cfg(any(
    any(feature = "artist-property-schema", feature = "bib-schema-section"),
    doc
))]
pub use r#artist::*;
#[cfg(any(
    any(
        feature = "artwork-surface-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#artwork_surface;
#[cfg(any(
    any(
        feature = "artwork-surface-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#artwork_surface::*;
#[cfg(any(
    any(feature = "asin-property-schema", feature = "pending-schema-section"),
    doc
))]
mod r#asin;
#[cfg(any(
    any(feature = "asin-property-schema", feature = "pending-schema-section"),
    doc
))]
pub use r#asin::*;
#[cfg(any(
    any(
        feature = "aspect-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#aspect;
#[cfg(any(
    any(
        feature = "aspect-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#aspect::*;
#[cfg(any(
    any(
        feature = "assembly-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#assembly;
#[cfg(any(
    any(
        feature = "assembly-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#assembly::*;
#[cfg(any(
    any(
        feature = "assembly-version-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#assembly_version;
#[cfg(any(
    any(
        feature = "assembly-version-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#assembly_version::*;
#[cfg(any(
    any(
        feature = "assesses-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#assesses;
#[cfg(any(
    any(
        feature = "assesses-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#assesses::*;
#[cfg(any(
    any(
        feature = "associated-anatomy-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#associated_anatomy;
#[cfg(any(
    any(
        feature = "associated-anatomy-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#associated_anatomy::*;
#[cfg(any(
    any(
        feature = "associated-article-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#associated_article;
#[cfg(any(
    any(
        feature = "associated-article-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#associated_article::*;
#[cfg(any(
    any(
        feature = "associated-claim-review-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#associated_claim_review;
#[cfg(any(
    any(
        feature = "associated-claim-review-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#associated_claim_review::*;
#[cfg(any(
    any(
        feature = "associated-disease-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#associated_disease;
#[cfg(any(
    any(
        feature = "associated-disease-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#associated_disease::*;
#[cfg(any(
    any(
        feature = "associated-media-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#associated_media;
#[cfg(any(
    any(
        feature = "associated-media-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#associated_media::*;
#[cfg(any(
    any(
        feature = "associated-media-review-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#associated_media_review;
#[cfg(any(
    any(
        feature = "associated-media-review-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#associated_media_review::*;
#[cfg(any(
    any(
        feature = "associated-pathophysiology-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#associated_pathophysiology;
#[cfg(any(
    any(
        feature = "associated-pathophysiology-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#associated_pathophysiology::*;
#[cfg(any(
    any(
        feature = "associated-review-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#associated_review;
#[cfg(any(
    any(
        feature = "associated-review-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#associated_review::*;
#[cfg(any(
    any(
        feature = "athlete-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#athlete;
#[cfg(any(
    any(
        feature = "athlete-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#athlete::*;
#[cfg(any(
    any(
        feature = "attendee-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#attendee;
#[cfg(any(
    any(
        feature = "attendee-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#attendee::*;
#[cfg(any(
    any(
        feature = "attendees-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#attendees;
#[cfg(any(
    any(
        feature = "attendees-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#attendees::*;
#[cfg(any(
    any(
        feature = "audience-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#audience;
#[cfg(any(
    any(
        feature = "audience-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#audience::*;
#[cfg(any(
    any(
        feature = "audience-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#audience_type;
#[cfg(any(
    any(
        feature = "audience-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#audience_type::*;
#[cfg(any(
    any(feature = "audio-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#audio;
#[cfg(any(
    any(feature = "audio-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#audio::*;
#[cfg(any(
    any(
        feature = "authenticator-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#authenticator;
#[cfg(any(
    any(
        feature = "authenticator-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#authenticator::*;
#[cfg(any(
    any(feature = "author-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#author;
#[cfg(any(
    any(feature = "author-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#author::*;
#[cfg(any(
    any(
        feature = "availability-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#availability;
#[cfg(any(
    any(
        feature = "availability-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#availability::*;
#[cfg(any(
    any(
        feature = "availability-ends-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#availability_ends;
#[cfg(any(
    any(
        feature = "availability-ends-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#availability_ends::*;
#[cfg(any(
    any(
        feature = "availability-starts-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#availability_starts;
#[cfg(any(
    any(
        feature = "availability-starts-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#availability_starts::*;
#[cfg(any(
    any(
        feature = "available-at-or-from-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#available_at_or_from;
#[cfg(any(
    any(
        feature = "available-at-or-from-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#available_at_or_from::*;
#[cfg(any(
    any(
        feature = "available-channel-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#available_channel;
#[cfg(any(
    any(
        feature = "available-channel-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#available_channel::*;
#[cfg(any(
    any(
        feature = "available-delivery-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#available_delivery_method;
#[cfg(any(
    any(
        feature = "available-delivery-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#available_delivery_method::*;
#[cfg(any(
    any(
        feature = "available-from-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#available_from;
#[cfg(any(
    any(
        feature = "available-from-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#available_from::*;
#[cfg(any(
    any(
        feature = "available-in-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#available_in;
#[cfg(any(
    any(
        feature = "available-in-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#available_in::*;
#[cfg(any(
    any(
        feature = "available-language-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#available_language;
#[cfg(any(
    any(
        feature = "available-language-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#available_language::*;
#[cfg(any(
    any(
        feature = "available-on-device-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#available_on_device;
#[cfg(any(
    any(
        feature = "available-on-device-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#available_on_device::*;
#[cfg(any(
    any(
        feature = "available-service-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#available_service;
#[cfg(any(
    any(
        feature = "available-service-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#available_service::*;
#[cfg(any(
    any(
        feature = "available-strength-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#available_strength;
#[cfg(any(
    any(
        feature = "available-strength-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#available_strength::*;
#[cfg(any(
    any(
        feature = "available-test-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#available_test;
#[cfg(any(
    any(
        feature = "available-test-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#available_test::*;
#[cfg(any(
    any(
        feature = "available-through-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#available_through;
#[cfg(any(
    any(
        feature = "available-through-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#available_through::*;
#[cfg(any(
    any(feature = "award-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#award;
#[cfg(any(
    any(feature = "award-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#award::*;
#[cfg(any(
    any(feature = "awards-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#awards;
#[cfg(any(
    any(feature = "awards-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#awards::*;
#[cfg(any(
    any(
        feature = "away-team-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#away_team;
#[cfg(any(
    any(
        feature = "away-team-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#away_team::*;
#[cfg(any(
    any(
        feature = "backstory-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#backstory;
#[cfg(any(
    any(
        feature = "backstory-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#backstory::*;
#[cfg(any(
    any(
        feature = "bank-account-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#bank_account_type;
#[cfg(any(
    any(
        feature = "bank-account-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#bank_account_type::*;
#[cfg(any(
    any(
        feature = "base-salary-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#base_salary;
#[cfg(any(
    any(
        feature = "base-salary-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#base_salary::*;
#[cfg(any(
    any(
        feature = "bcc-recipient-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#bcc_recipient;
#[cfg(any(
    any(
        feature = "bcc-recipient-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#bcc_recipient::*;
#[cfg(any(
    any(feature = "bed-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#bed;
#[cfg(any(
    any(feature = "bed-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#bed::*;
#[cfg(any(
    any(
        feature = "before-media-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#before_media;
#[cfg(any(
    any(
        feature = "before-media-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#before_media::*;
#[cfg(any(
    any(
        feature = "beneficiary-bank-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#beneficiary_bank;
#[cfg(any(
    any(
        feature = "beneficiary-bank-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#beneficiary_bank::*;
#[cfg(any(
    any(
        feature = "benefits-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#benefits;
#[cfg(any(
    any(
        feature = "benefits-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#benefits::*;
#[cfg(any(
    any(
        feature = "benefits-summary-url-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#benefits_summary_url;
#[cfg(any(
    any(
        feature = "benefits-summary-url-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#benefits_summary_url::*;
#[cfg(any(
    any(
        feature = "best-rating-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#best_rating;
#[cfg(any(
    any(
        feature = "best-rating-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#best_rating::*;
#[cfg(any(
    any(
        feature = "billing-address-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#billing_address;
#[cfg(any(
    any(
        feature = "billing-address-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#billing_address::*;
#[cfg(any(
    any(
        feature = "billing-duration-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#billing_duration;
#[cfg(any(
    any(
        feature = "billing-duration-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#billing_duration::*;
#[cfg(any(
    any(
        feature = "billing-increment-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#billing_increment;
#[cfg(any(
    any(
        feature = "billing-increment-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#billing_increment::*;
#[cfg(any(
    any(
        feature = "billing-period-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#billing_period;
#[cfg(any(
    any(
        feature = "billing-period-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#billing_period::*;
#[cfg(any(
    any(
        feature = "billing-start-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#billing_start;
#[cfg(any(
    any(
        feature = "billing-start-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#billing_start::*;
#[cfg(any(
    any(
        feature = "bio-chem-interaction-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#bio_chem_interaction;
#[cfg(any(
    any(
        feature = "bio-chem-interaction-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#bio_chem_interaction::*;
#[cfg(any(
    any(
        feature = "bio-chem-similarity-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#bio_chem_similarity;
#[cfg(any(
    any(
        feature = "bio-chem-similarity-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#bio_chem_similarity::*;
#[cfg(any(
    any(
        feature = "biological-role-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#biological_role;
#[cfg(any(
    any(
        feature = "biological-role-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#biological_role::*;
#[cfg(any(
    any(
        feature = "biomechnical-class-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#biomechnical_class;
#[cfg(any(
    any(
        feature = "biomechnical-class-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#biomechnical_class::*;
#[cfg(any(
    any(
        feature = "birth-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#birth_date;
#[cfg(any(
    any(
        feature = "birth-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#birth_date::*;
#[cfg(any(
    any(
        feature = "birth-place-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#birth_place;
#[cfg(any(
    any(
        feature = "birth-place-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#birth_place::*;
#[cfg(any(
    any(
        feature = "bitrate-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#bitrate;
#[cfg(any(
    any(
        feature = "bitrate-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#bitrate::*;
#[cfg(any(
    any(
        feature = "blog-post-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#blog_post;
#[cfg(any(
    any(
        feature = "blog-post-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#blog_post::*;
#[cfg(any(
    any(
        feature = "blog-posts-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#blog_posts;
#[cfg(any(
    any(
        feature = "blog-posts-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#blog_posts::*;
#[cfg(any(
    any(
        feature = "blood-supply-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#blood_supply;
#[cfg(any(
    any(
        feature = "blood-supply-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#blood_supply::*;
#[cfg(any(
    any(
        feature = "boarding-group-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#boarding_group;
#[cfg(any(
    any(
        feature = "boarding-group-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#boarding_group::*;
#[cfg(any(
    any(
        feature = "boarding-policy-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#boarding_policy;
#[cfg(any(
    any(
        feature = "boarding-policy-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#boarding_policy::*;
#[cfg(any(
    any(
        feature = "body-location-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#body_location;
#[cfg(any(
    any(
        feature = "body-location-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#body_location::*;
#[cfg(any(
    any(feature = "body-type-property-schema", feature = "auto-schema-section"),
    doc
))]
mod r#body_type;
#[cfg(any(
    any(feature = "body-type-property-schema", feature = "auto-schema-section"),
    doc
))]
pub use r#body_type::*;
#[cfg(any(
    any(
        feature = "book-edition-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#book_edition;
#[cfg(any(
    any(
        feature = "book-edition-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#book_edition::*;
#[cfg(any(
    any(
        feature = "book-format-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#book_format;
#[cfg(any(
    any(
        feature = "book-format-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#book_format::*;
#[cfg(any(
    any(
        feature = "booking-agent-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#booking_agent;
#[cfg(any(
    any(
        feature = "booking-agent-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#booking_agent::*;
#[cfg(any(
    any(
        feature = "booking-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#booking_time;
#[cfg(any(
    any(
        feature = "booking-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#booking_time::*;
#[cfg(any(
    any(
        feature = "borrower-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#borrower;
#[cfg(any(
    any(
        feature = "borrower-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#borrower::*;
#[cfg(any(
    any(feature = "box-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#box;
#[cfg(any(
    any(feature = "box-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#box::*;
#[cfg(any(
    any(
        feature = "branch-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#branch;
#[cfg(any(
    any(
        feature = "branch-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#branch::*;
#[cfg(any(
    any(
        feature = "branch-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#branch_code;
#[cfg(any(
    any(
        feature = "branch-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#branch_code::*;
#[cfg(any(
    any(
        feature = "branch-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#branch_of;
#[cfg(any(
    any(
        feature = "branch-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#branch_of::*;
#[cfg(any(
    any(feature = "brand-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#brand;
#[cfg(any(
    any(feature = "brand-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#brand::*;
#[cfg(any(
    any(
        feature = "breadcrumb-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#breadcrumb;
#[cfg(any(
    any(
        feature = "breadcrumb-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#breadcrumb::*;
#[cfg(any(
    any(
        feature = "breastfeeding-warning-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#breastfeeding_warning;
#[cfg(any(
    any(
        feature = "breastfeeding-warning-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#breastfeeding_warning::*;
#[cfg(any(
    any(
        feature = "broadcast-affiliate-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#broadcast_affiliate_of;
#[cfg(any(
    any(
        feature = "broadcast-affiliate-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#broadcast_affiliate_of::*;
#[cfg(any(
    any(
        feature = "broadcast-channel-id-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#broadcast_channel_id;
#[cfg(any(
    any(
        feature = "broadcast-channel-id-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#broadcast_channel_id::*;
#[cfg(any(
    any(
        feature = "broadcast-display-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#broadcast_display_name;
#[cfg(any(
    any(
        feature = "broadcast-display-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#broadcast_display_name::*;
#[cfg(any(
    any(
        feature = "broadcast-frequency-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#broadcast_frequency;
#[cfg(any(
    any(
        feature = "broadcast-frequency-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#broadcast_frequency::*;
#[cfg(any(
    any(
        feature = "broadcast-frequency-value-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#broadcast_frequency_value;
#[cfg(any(
    any(
        feature = "broadcast-frequency-value-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#broadcast_frequency_value::*;
#[cfg(any(
    any(
        feature = "broadcast-of-event-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#broadcast_of_event;
#[cfg(any(
    any(
        feature = "broadcast-of-event-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#broadcast_of_event::*;
#[cfg(any(
    any(
        feature = "broadcast-service-tier-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#broadcast_service_tier;
#[cfg(any(
    any(
        feature = "broadcast-service-tier-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#broadcast_service_tier::*;
#[cfg(any(
    any(
        feature = "broadcast-signal-modulation-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#broadcast_signal_modulation;
#[cfg(any(
    any(
        feature = "broadcast-signal-modulation-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#broadcast_signal_modulation::*;
#[cfg(any(
    any(
        feature = "broadcast-sub-channel-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#broadcast_sub_channel;
#[cfg(any(
    any(
        feature = "broadcast-sub-channel-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#broadcast_sub_channel::*;
#[cfg(any(
    any(
        feature = "broadcast-timezone-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#broadcast_timezone;
#[cfg(any(
    any(
        feature = "broadcast-timezone-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#broadcast_timezone::*;
#[cfg(any(
    any(
        feature = "broadcaster-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#broadcaster;
#[cfg(any(
    any(
        feature = "broadcaster-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#broadcaster::*;
#[cfg(any(
    any(feature = "broker-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#broker;
#[cfg(any(
    any(feature = "broker-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#broker::*;
#[cfg(any(
    any(
        feature = "browser-requirements-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#browser_requirements;
#[cfg(any(
    any(
        feature = "browser-requirements-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#browser_requirements::*;
#[cfg(any(
    any(
        feature = "bus-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#bus_name;
#[cfg(any(
    any(
        feature = "bus-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#bus_name::*;
#[cfg(any(
    any(
        feature = "bus-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#bus_number;
#[cfg(any(
    any(
        feature = "bus-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#bus_number::*;
#[cfg(any(
    any(
        feature = "business-days-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#business_days;
#[cfg(any(
    any(
        feature = "business-days-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#business_days::*;
#[cfg(any(
    any(
        feature = "business-function-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#business_function;
#[cfg(any(
    any(
        feature = "business-function-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#business_function::*;
#[cfg(any(
    any(feature = "buyer-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#buyer;
#[cfg(any(
    any(feature = "buyer-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#buyer::*;
#[cfg(any(
    any(
        feature = "by-artist-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#by_artist;
#[cfg(any(
    any(
        feature = "by-artist-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#by_artist::*;
#[cfg(any(
    any(feature = "by-day-property-schema", feature = "pending-schema-section"),
    doc
))]
mod r#by_day;
#[cfg(any(
    any(feature = "by-day-property-schema", feature = "pending-schema-section"),
    doc
))]
pub use r#by_day::*;
#[cfg(any(
    any(
        feature = "by-month-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#by_month;
#[cfg(any(
    any(
        feature = "by-month-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#by_month::*;
#[cfg(any(
    any(
        feature = "by-month-day-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#by_month_day;
#[cfg(any(
    any(
        feature = "by-month-day-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#by_month_day::*;
#[cfg(any(
    any(
        feature = "by-month-week-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#by_month_week;
#[cfg(any(
    any(
        feature = "by-month-week-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#by_month_week::*;
#[cfg(any(
    any(
        feature = "call-sign-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#call_sign;
#[cfg(any(
    any(
        feature = "call-sign-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#call_sign::*;
#[cfg(any(
    any(
        feature = "calories-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#calories;
#[cfg(any(
    any(
        feature = "calories-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#calories::*;
#[cfg(any(
    any(
        feature = "candidate-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#candidate;
#[cfg(any(
    any(
        feature = "candidate-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#candidate::*;
#[cfg(any(
    any(
        feature = "caption-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#caption;
#[cfg(any(
    any(
        feature = "caption-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#caption::*;
#[cfg(any(
    any(
        feature = "carbohydrate-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#carbohydrate_content;
#[cfg(any(
    any(
        feature = "carbohydrate-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#carbohydrate_content::*;
#[cfg(any(
    any(
        feature = "cargo-volume-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#cargo_volume;
#[cfg(any(
    any(
        feature = "cargo-volume-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#cargo_volume::*;
#[cfg(any(
    any(
        feature = "carrier-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#carrier;
#[cfg(any(
    any(
        feature = "carrier-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#carrier::*;
#[cfg(any(
    any(
        feature = "carrier-requirements-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#carrier_requirements;
#[cfg(any(
    any(
        feature = "carrier-requirements-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#carrier_requirements::*;
#[cfg(any(
    any(
        feature = "cash-back-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cash_back;
#[cfg(any(
    any(
        feature = "cash-back-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cash_back::*;
#[cfg(any(
    any(
        feature = "catalog-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#catalog;
#[cfg(any(
    any(
        feature = "catalog-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#catalog::*;
#[cfg(any(
    any(
        feature = "catalog-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#catalog_number;
#[cfg(any(
    any(
        feature = "catalog-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#catalog_number::*;
#[cfg(any(
    any(
        feature = "category-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#category;
#[cfg(any(
    any(
        feature = "category-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#category::*;
#[cfg(any(
    any(
        feature = "cause-of-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#cause_of;
#[cfg(any(
    any(
        feature = "cause-of-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#cause_of::*;
#[cfg(any(
    any(
        feature = "cc-recipient-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#cc_recipient;
#[cfg(any(
    any(
        feature = "cc-recipient-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#cc_recipient::*;
#[cfg(any(
    any(
        feature = "character-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#character;
#[cfg(any(
    any(
        feature = "character-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#character::*;
#[cfg(any(
    any(
        feature = "character-attribute-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#character_attribute;
#[cfg(any(
    any(
        feature = "character-attribute-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#character_attribute::*;
#[cfg(any(
    any(
        feature = "character-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#character_name;
#[cfg(any(
    any(
        feature = "character-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#character_name::*;
#[cfg(any(
    any(
        feature = "cheat-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#cheat_code;
#[cfg(any(
    any(
        feature = "cheat-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#cheat_code::*;
#[cfg(any(
    any(
        feature = "checkin-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#checkin_time;
#[cfg(any(
    any(
        feature = "checkin-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#checkin_time::*;
#[cfg(any(
    any(
        feature = "checkout-page-url-template-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#checkout_page_url_template;
#[cfg(any(
    any(
        feature = "checkout-page-url-template-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#checkout_page_url_template::*;
#[cfg(any(
    any(
        feature = "checkout-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#checkout_time;
#[cfg(any(
    any(
        feature = "checkout-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#checkout_time::*;
#[cfg(any(
    any(
        feature = "chemical-composition-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#chemical_composition;
#[cfg(any(
    any(
        feature = "chemical-composition-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#chemical_composition::*;
#[cfg(any(
    any(
        feature = "chemical-role-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#chemical_role;
#[cfg(any(
    any(
        feature = "chemical-role-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#chemical_role::*;
#[cfg(any(
    any(
        feature = "child-max-age-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#child_max_age;
#[cfg(any(
    any(
        feature = "child-max-age-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#child_max_age::*;
#[cfg(any(
    any(
        feature = "child-min-age-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#child_min_age;
#[cfg(any(
    any(
        feature = "child-min-age-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#child_min_age::*;
#[cfg(any(
    any(
        feature = "child-taxon-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#child_taxon;
#[cfg(any(
    any(
        feature = "child-taxon-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#child_taxon::*;
#[cfg(any(
    any(
        feature = "children-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#children;
#[cfg(any(
    any(
        feature = "children-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#children::*;
#[cfg(any(
    any(
        feature = "cholesterol-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#cholesterol_content;
#[cfg(any(
    any(
        feature = "cholesterol-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#cholesterol_content::*;
#[cfg(any(
    any(feature = "circle-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#circle;
#[cfg(any(
    any(feature = "circle-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#circle::*;
#[cfg(any(
    any(
        feature = "citation-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#citation;
#[cfg(any(
    any(
        feature = "citation-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#citation::*;
#[cfg(any(
    any(
        feature = "claim-interpreter-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#claim_interpreter;
#[cfg(any(
    any(
        feature = "claim-interpreter-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#claim_interpreter::*;
#[cfg(any(
    any(
        feature = "claim-reviewed-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#claim_reviewed;
#[cfg(any(
    any(
        feature = "claim-reviewed-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#claim_reviewed::*;
#[cfg(any(
    any(
        feature = "clincal-pharmacology-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#clincal_pharmacology;
#[cfg(any(
    any(
        feature = "clincal-pharmacology-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#clincal_pharmacology::*;
#[cfg(any(
    any(
        feature = "clinical-pharmacology-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#clinical_pharmacology;
#[cfg(any(
    any(
        feature = "clinical-pharmacology-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#clinical_pharmacology::*;
#[cfg(any(
    any(
        feature = "clip-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#clip_number;
#[cfg(any(
    any(
        feature = "clip-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#clip_number::*;
#[cfg(any(
    any(feature = "closes-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#closes;
#[cfg(any(
    any(feature = "closes-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#closes::*;
#[cfg(any(
    any(feature = "coach-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#coach;
#[cfg(any(
    any(feature = "coach-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#coach::*;
#[cfg(any(
    any(
        feature = "code-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#code;
#[cfg(any(
    any(
        feature = "code-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#code::*;
#[cfg(any(
    any(
        feature = "code-repository-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#code_repository;
#[cfg(any(
    any(
        feature = "code-repository-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#code_repository::*;
#[cfg(any(
    any(
        feature = "code-sample-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#code_sample_type;
#[cfg(any(
    any(
        feature = "code-sample-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#code_sample_type::*;
#[cfg(any(
    any(
        feature = "code-value-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#code_value;
#[cfg(any(
    any(
        feature = "code-value-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#code_value::*;
#[cfg(any(
    any(
        feature = "coding-system-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#coding_system;
#[cfg(any(
    any(
        feature = "coding-system-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#coding_system::*;
#[cfg(any(
    any(
        feature = "colleague-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#colleague;
#[cfg(any(
    any(
        feature = "colleague-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#colleague::*;
#[cfg(any(
    any(
        feature = "colleagues-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#colleagues;
#[cfg(any(
    any(
        feature = "colleagues-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#colleagues::*;
#[cfg(any(
    any(
        feature = "collection-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#collection;
#[cfg(any(
    any(
        feature = "collection-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#collection::*;
#[cfg(any(
    any(
        feature = "collection-size-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#collection_size;
#[cfg(any(
    any(
        feature = "collection-size-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#collection_size::*;
#[cfg(any(
    any(feature = "color-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#color;
#[cfg(any(
    any(feature = "color-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#color::*;
#[cfg(any(
    any(feature = "colorist-property-schema", feature = "bib-schema-section"),
    doc
))]
mod r#colorist;
#[cfg(any(
    any(feature = "colorist-property-schema", feature = "bib-schema-section"),
    doc
))]
pub use r#colorist::*;
#[cfg(any(
    any(
        feature = "comment-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#comment;
#[cfg(any(
    any(
        feature = "comment-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#comment::*;
#[cfg(any(
    any(
        feature = "comment-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#comment_count;
#[cfg(any(
    any(
        feature = "comment-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#comment_count::*;
#[cfg(any(
    any(
        feature = "comment-text-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#comment_text;
#[cfg(any(
    any(
        feature = "comment-text-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#comment_text::*;
#[cfg(any(
    any(
        feature = "comment-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#comment_time;
#[cfg(any(
    any(
        feature = "comment-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#comment_time::*;
#[cfg(any(
    any(
        feature = "competency-required-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#competency_required;
#[cfg(any(
    any(
        feature = "competency-required-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#competency_required::*;
#[cfg(any(
    any(
        feature = "competitor-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#competitor;
#[cfg(any(
    any(
        feature = "competitor-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#competitor::*;
#[cfg(any(
    any(
        feature = "composer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#composer;
#[cfg(any(
    any(
        feature = "composer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#composer::*;
#[cfg(any(
    any(
        feature = "comprised-of-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#comprised_of;
#[cfg(any(
    any(
        feature = "comprised-of-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#comprised_of::*;
#[cfg(any(
    any(
        feature = "conditions-of-access-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#conditions_of_access;
#[cfg(any(
    any(
        feature = "conditions-of-access-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#conditions_of_access::*;
#[cfg(any(
    any(
        feature = "confirmation-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#confirmation_number;
#[cfg(any(
    any(
        feature = "confirmation-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#confirmation_number::*;
#[cfg(any(
    any(
        feature = "connected-to-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#connected_to;
#[cfg(any(
    any(
        feature = "connected-to-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#connected_to::*;
#[cfg(any(
    any(
        feature = "constraint-property-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#constraint_property;
#[cfg(any(
    any(
        feature = "constraint-property-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#constraint_property::*;
#[cfg(any(
    any(
        feature = "contact-option-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#contact_option;
#[cfg(any(
    any(
        feature = "contact-option-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#contact_option::*;
#[cfg(any(
    any(
        feature = "contact-point-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#contact_point;
#[cfg(any(
    any(
        feature = "contact-point-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#contact_point::*;
#[cfg(any(
    any(
        feature = "contact-points-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#contact_points;
#[cfg(any(
    any(
        feature = "contact-points-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#contact_points::*;
#[cfg(any(
    any(
        feature = "contact-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#contact_type;
#[cfg(any(
    any(
        feature = "contact-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#contact_type::*;
#[cfg(any(
    any(
        feature = "contactless-payment-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#contactless_payment;
#[cfg(any(
    any(
        feature = "contactless-payment-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#contactless_payment::*;
#[cfg(any(
    any(
        feature = "contained-in-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#contained_in;
#[cfg(any(
    any(
        feature = "contained-in-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#contained_in::*;
#[cfg(any(
    any(
        feature = "contained-in-place-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#contained_in_place;
#[cfg(any(
    any(
        feature = "contained-in-place-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#contained_in_place::*;
#[cfg(any(
    any(
        feature = "contains-place-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#contains_place;
#[cfg(any(
    any(
        feature = "contains-place-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#contains_place::*;
#[cfg(any(
    any(
        feature = "contains-season-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#contains_season;
#[cfg(any(
    any(
        feature = "contains-season-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#contains_season::*;
#[cfg(any(
    any(
        feature = "content-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#content_location;
#[cfg(any(
    any(
        feature = "content-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#content_location::*;
#[cfg(any(
    any(
        feature = "content-rating-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#content_rating;
#[cfg(any(
    any(
        feature = "content-rating-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#content_rating::*;
#[cfg(any(
    any(
        feature = "content-reference-time-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#content_reference_time;
#[cfg(any(
    any(
        feature = "content-reference-time-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#content_reference_time::*;
#[cfg(any(
    any(
        feature = "content-size-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#content_size;
#[cfg(any(
    any(
        feature = "content-size-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#content_size::*;
#[cfg(any(
    any(
        feature = "content-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#content_type;
#[cfg(any(
    any(
        feature = "content-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#content_type::*;
#[cfg(any(
    any(
        feature = "content-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#content_url;
#[cfg(any(
    any(
        feature = "content-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#content_url::*;
#[cfg(any(
    any(
        feature = "contraindication-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#contraindication;
#[cfg(any(
    any(
        feature = "contraindication-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#contraindication::*;
#[cfg(any(
    any(
        feature = "contributor-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#contributor;
#[cfg(any(
    any(
        feature = "contributor-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#contributor::*;
#[cfg(any(
    any(
        feature = "cook-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#cook_time;
#[cfg(any(
    any(
        feature = "cook-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#cook_time::*;
#[cfg(any(
    any(
        feature = "cooking-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#cooking_method;
#[cfg(any(
    any(
        feature = "cooking-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#cooking_method::*;
#[cfg(any(
    any(
        feature = "copyright-holder-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#copyright_holder;
#[cfg(any(
    any(
        feature = "copyright-holder-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#copyright_holder::*;
#[cfg(any(
    any(
        feature = "copyright-notice-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#copyright_notice;
#[cfg(any(
    any(
        feature = "copyright-notice-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#copyright_notice::*;
#[cfg(any(
    any(
        feature = "copyright-year-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#copyright_year;
#[cfg(any(
    any(
        feature = "copyright-year-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#copyright_year::*;
#[cfg(any(
    any(
        feature = "correction-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#correction;
#[cfg(any(
    any(
        feature = "correction-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#correction::*;
#[cfg(any(
    any(
        feature = "corrections-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#corrections_policy;
#[cfg(any(
    any(
        feature = "corrections-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#corrections_policy::*;
#[cfg(any(
    any(
        feature = "cost-category-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#cost_category;
#[cfg(any(
    any(
        feature = "cost-category-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#cost_category::*;
#[cfg(any(
    any(
        feature = "cost-currency-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#cost_currency;
#[cfg(any(
    any(
        feature = "cost-currency-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#cost_currency::*;
#[cfg(any(
    any(
        feature = "cost-origin-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#cost_origin;
#[cfg(any(
    any(
        feature = "cost-origin-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#cost_origin::*;
#[cfg(any(
    any(
        feature = "cost-per-unit-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#cost_per_unit;
#[cfg(any(
    any(
        feature = "cost-per-unit-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#cost_per_unit::*;
#[cfg(any(
    any(
        feature = "countries-not-supported-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#countries_not_supported;
#[cfg(any(
    any(
        feature = "countries-not-supported-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#countries_not_supported::*;
#[cfg(any(
    any(
        feature = "countries-supported-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#countries_supported;
#[cfg(any(
    any(
        feature = "countries-supported-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#countries_supported::*;
#[cfg(any(
    any(
        feature = "country-of-assembly-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#country_of_assembly;
#[cfg(any(
    any(
        feature = "country-of-assembly-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#country_of_assembly::*;
#[cfg(any(
    any(
        feature = "country-of-last-processing-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#country_of_last_processing;
#[cfg(any(
    any(
        feature = "country-of-last-processing-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#country_of_last_processing::*;
#[cfg(any(
    any(
        feature = "country-of-origin-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#country_of_origin;
#[cfg(any(
    any(
        feature = "country-of-origin-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#country_of_origin::*;
#[cfg(any(
    any(feature = "course-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#course;
#[cfg(any(
    any(feature = "course-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#course::*;
#[cfg(any(
    any(
        feature = "course-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#course_code;
#[cfg(any(
    any(
        feature = "course-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#course_code::*;
#[cfg(any(
    any(
        feature = "course-mode-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#course_mode;
#[cfg(any(
    any(
        feature = "course-mode-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#course_mode::*;
#[cfg(any(
    any(
        feature = "course-prerequisites-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#course_prerequisites;
#[cfg(any(
    any(
        feature = "course-prerequisites-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#course_prerequisites::*;
#[cfg(any(
    any(
        feature = "course-schedule-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#course_schedule;
#[cfg(any(
    any(
        feature = "course-schedule-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#course_schedule::*;
#[cfg(any(
    any(
        feature = "course-workload-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#course_workload;
#[cfg(any(
    any(
        feature = "course-workload-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#course_workload::*;
#[cfg(any(
    any(
        feature = "coverage-end-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#coverage_end_time;
#[cfg(any(
    any(
        feature = "coverage-end-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#coverage_end_time::*;
#[cfg(any(
    any(
        feature = "coverage-start-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#coverage_start_time;
#[cfg(any(
    any(
        feature = "coverage-start-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#coverage_start_time::*;
#[cfg(any(
    any(
        feature = "creative-work-status-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#creative_work_status;
#[cfg(any(
    any(
        feature = "creative-work-status-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#creative_work_status::*;
#[cfg(any(
    any(
        feature = "creator-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#creator;
#[cfg(any(
    any(
        feature = "creator-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#creator::*;
#[cfg(any(
    any(
        feature = "credential-category-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#credential_category;
#[cfg(any(
    any(
        feature = "credential-category-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#credential_category::*;
#[cfg(any(
    any(
        feature = "credit-text-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#credit_text;
#[cfg(any(
    any(
        feature = "credit-text-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#credit_text::*;
#[cfg(any(
    any(
        feature = "credited-to-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#credited_to;
#[cfg(any(
    any(
        feature = "credited-to-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#credited_to::*;
#[cfg(any(
    any(
        feature = "css-selector-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#css_selector;
#[cfg(any(
    any(
        feature = "css-selector-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#css_selector::*;
#[cfg(any(
    any(
        feature = "currencies-accepted-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#currencies_accepted;
#[cfg(any(
    any(
        feature = "currencies-accepted-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#currencies_accepted::*;
#[cfg(any(
    any(
        feature = "currency-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#currency;
#[cfg(any(
    any(
        feature = "currency-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#currency::*;
#[cfg(any(
    any(
        feature = "current-exchange-rate-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#current_exchange_rate;
#[cfg(any(
    any(
        feature = "current-exchange-rate-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#current_exchange_rate::*;
#[cfg(any(
    any(
        feature = "customer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#customer;
#[cfg(any(
    any(
        feature = "customer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#customer::*;
#[cfg(any(
    any(
        feature = "customer-remorse-return-fees-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#customer_remorse_return_fees;
#[cfg(any(
    any(
        feature = "customer-remorse-return-fees-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#customer_remorse_return_fees::*;
#[cfg(any(
    any(
        feature = "customer-remorse-return-label-source-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#customer_remorse_return_label_source;
#[cfg(any(
    any(
        feature = "customer-remorse-return-label-source-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#customer_remorse_return_label_source::*;
#[cfg(any(
    any(
        feature = "customer-remorse-return-shipping-fees-amount-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#customer_remorse_return_shipping_fees_amount;
#[cfg(any(
    any(
        feature = "customer-remorse-return-shipping-fees-amount-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#customer_remorse_return_shipping_fees_amount::*;
#[cfg(any(
    any(
        feature = "cutoff-time-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cutoff_time;
#[cfg(any(
    any(
        feature = "cutoff-time-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cutoff_time::*;
#[cfg(any(
    any(
        feature = "cvd-collection-date-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cvd_collection_date;
#[cfg(any(
    any(
        feature = "cvd-collection-date-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cvd_collection_date::*;
#[cfg(any(
    any(
        feature = "cvd-facility-county-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cvd_facility_county;
#[cfg(any(
    any(
        feature = "cvd-facility-county-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cvd_facility_county::*;
#[cfg(any(
    any(
        feature = "cvd-facility-id-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cvd_facility_id;
#[cfg(any(
    any(
        feature = "cvd-facility-id-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cvd_facility_id::*;
#[cfg(any(
    any(
        feature = "cvd-num-beds-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cvd_num_beds;
#[cfg(any(
    any(
        feature = "cvd-num-beds-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cvd_num_beds::*;
#[cfg(any(
    any(
        feature = "cvd-num-beds-occ-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cvd_num_beds_occ;
#[cfg(any(
    any(
        feature = "cvd-num-beds-occ-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cvd_num_beds_occ::*;
#[cfg(any(
    any(
        feature = "cvd-num-c-19-died-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cvd_num_c_19_died;
#[cfg(any(
    any(
        feature = "cvd-num-c-19-died-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cvd_num_c_19_died::*;
#[cfg(any(
    any(
        feature = "cvd-num-c-19-ho-pats-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cvd_num_c_19_ho_pats;
#[cfg(any(
    any(
        feature = "cvd-num-c-19-ho-pats-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cvd_num_c_19_ho_pats::*;
#[cfg(any(
    any(
        feature = "cvd-num-c-19-hosp-pats-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cvd_num_c_19_hosp_pats;
#[cfg(any(
    any(
        feature = "cvd-num-c-19-hosp-pats-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cvd_num_c_19_hosp_pats::*;
#[cfg(any(
    any(
        feature = "cvd-num-c-19-mech-vent-pats-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cvd_num_c_19_mech_vent_pats;
#[cfg(any(
    any(
        feature = "cvd-num-c-19-mech-vent-pats-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cvd_num_c_19_mech_vent_pats::*;
#[cfg(any(
    any(
        feature = "cvd-num-c-19-of-mech-vent-pats-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cvd_num_c_19_of_mech_vent_pats;
#[cfg(any(
    any(
        feature = "cvd-num-c-19-of-mech-vent-pats-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cvd_num_c_19_of_mech_vent_pats::*;
#[cfg(any(
    any(
        feature = "cvd-num-c-19-overflow-pats-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cvd_num_c_19_overflow_pats;
#[cfg(any(
    any(
        feature = "cvd-num-c-19-overflow-pats-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cvd_num_c_19_overflow_pats::*;
#[cfg(any(
    any(
        feature = "cvd-num-icu-beds-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cvd_num_icu_beds;
#[cfg(any(
    any(
        feature = "cvd-num-icu-beds-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cvd_num_icu_beds::*;
#[cfg(any(
    any(
        feature = "cvd-num-icu-beds-occ-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cvd_num_icu_beds_occ;
#[cfg(any(
    any(
        feature = "cvd-num-icu-beds-occ-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cvd_num_icu_beds_occ::*;
#[cfg(any(
    any(
        feature = "cvd-num-tot-beds-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cvd_num_tot_beds;
#[cfg(any(
    any(
        feature = "cvd-num-tot-beds-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cvd_num_tot_beds::*;
#[cfg(any(
    any(
        feature = "cvd-num-vent-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cvd_num_vent;
#[cfg(any(
    any(
        feature = "cvd-num-vent-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cvd_num_vent::*;
#[cfg(any(
    any(
        feature = "cvd-num-vent-use-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#cvd_num_vent_use;
#[cfg(any(
    any(
        feature = "cvd-num-vent-use-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#cvd_num_vent_use::*;
#[cfg(any(
    any(
        feature = "data-feed-element-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#data_feed_element;
#[cfg(any(
    any(
        feature = "data-feed-element-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#data_feed_element::*;
#[cfg(any(
    any(
        feature = "dataset-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#dataset;
#[cfg(any(
    any(
        feature = "dataset-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#dataset::*;
#[cfg(any(
    any(
        feature = "dataset-time-interval-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#dataset_time_interval;
#[cfg(any(
    any(
        feature = "dataset-time-interval-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#dataset_time_interval::*;
#[cfg(any(
    any(
        feature = "date-created-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#date_created;
#[cfg(any(
    any(
        feature = "date-created-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#date_created::*;
#[cfg(any(
    any(
        feature = "date-deleted-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#date_deleted;
#[cfg(any(
    any(
        feature = "date-deleted-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#date_deleted::*;
#[cfg(any(
    any(
        feature = "date-issued-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#date_issued;
#[cfg(any(
    any(
        feature = "date-issued-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#date_issued::*;
#[cfg(any(
    any(
        feature = "date-modified-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#date_modified;
#[cfg(any(
    any(
        feature = "date-modified-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#date_modified::*;
#[cfg(any(
    any(
        feature = "date-posted-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#date_posted;
#[cfg(any(
    any(
        feature = "date-posted-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#date_posted::*;
#[cfg(any(
    any(
        feature = "date-published-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#date_published;
#[cfg(any(
    any(
        feature = "date-published-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#date_published::*;
#[cfg(any(
    any(
        feature = "date-read-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#date_read;
#[cfg(any(
    any(
        feature = "date-read-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#date_read::*;
#[cfg(any(
    any(
        feature = "date-received-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#date_received;
#[cfg(any(
    any(
        feature = "date-received-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#date_received::*;
#[cfg(any(
    any(
        feature = "date-sent-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#date_sent;
#[cfg(any(
    any(
        feature = "date-sent-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#date_sent::*;
#[cfg(any(
    any(
        feature = "date-vehicle-first-registered-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#date_vehicle_first_registered;
#[cfg(any(
    any(
        feature = "date-vehicle-first-registered-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#date_vehicle_first_registered::*;
#[cfg(any(
    any(
        feature = "dateline-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#dateline;
#[cfg(any(
    any(
        feature = "dateline-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#dateline::*;
#[cfg(any(
    any(
        feature = "day-of-week-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#day_of_week;
#[cfg(any(
    any(
        feature = "day-of-week-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#day_of_week::*;
#[cfg(any(
    any(
        feature = "death-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#death_date;
#[cfg(any(
    any(
        feature = "death-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#death_date::*;
#[cfg(any(
    any(
        feature = "death-place-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#death_place;
#[cfg(any(
    any(
        feature = "death-place-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#death_place::*;
#[cfg(any(
    any(
        feature = "default-value-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#default_value;
#[cfg(any(
    any(
        feature = "default-value-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#default_value::*;
#[cfg(any(
    any(
        feature = "delivery-address-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#delivery_address;
#[cfg(any(
    any(
        feature = "delivery-address-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#delivery_address::*;
#[cfg(any(
    any(
        feature = "delivery-lead-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#delivery_lead_time;
#[cfg(any(
    any(
        feature = "delivery-lead-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#delivery_lead_time::*;
#[cfg(any(
    any(
        feature = "delivery-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#delivery_method;
#[cfg(any(
    any(
        feature = "delivery-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#delivery_method::*;
#[cfg(any(
    any(
        feature = "delivery-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#delivery_status;
#[cfg(any(
    any(
        feature = "delivery-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#delivery_status::*;
#[cfg(any(
    any(
        feature = "delivery-time-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#delivery_time;
#[cfg(any(
    any(
        feature = "delivery-time-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#delivery_time::*;
#[cfg(any(
    any(
        feature = "department-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#department;
#[cfg(any(
    any(
        feature = "department-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#department::*;
#[cfg(any(
    any(
        feature = "departure-airport-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#departure_airport;
#[cfg(any(
    any(
        feature = "departure-airport-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#departure_airport::*;
#[cfg(any(
    any(
        feature = "departure-boat-terminal-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#departure_boat_terminal;
#[cfg(any(
    any(
        feature = "departure-boat-terminal-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#departure_boat_terminal::*;
#[cfg(any(
    any(
        feature = "departure-bus-stop-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#departure_bus_stop;
#[cfg(any(
    any(
        feature = "departure-bus-stop-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#departure_bus_stop::*;
#[cfg(any(
    any(
        feature = "departure-gate-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#departure_gate;
#[cfg(any(
    any(
        feature = "departure-gate-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#departure_gate::*;
#[cfg(any(
    any(
        feature = "departure-platform-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#departure_platform;
#[cfg(any(
    any(
        feature = "departure-platform-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#departure_platform::*;
#[cfg(any(
    any(
        feature = "departure-station-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#departure_station;
#[cfg(any(
    any(
        feature = "departure-station-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#departure_station::*;
#[cfg(any(
    any(
        feature = "departure-terminal-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#departure_terminal;
#[cfg(any(
    any(
        feature = "departure-terminal-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#departure_terminal::*;
#[cfg(any(
    any(
        feature = "departure-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#departure_time;
#[cfg(any(
    any(
        feature = "departure-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#departure_time::*;
#[cfg(any(
    any(
        feature = "dependencies-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#dependencies;
#[cfg(any(
    any(
        feature = "dependencies-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#dependencies::*;
#[cfg(any(
    any(feature = "depth-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#depth;
#[cfg(any(
    any(feature = "depth-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#depth::*;
#[cfg(any(
    any(
        feature = "description-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#description;
#[cfg(any(
    any(
        feature = "description-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#description::*;
#[cfg(any(
    any(feature = "device-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#device;
#[cfg(any(
    any(feature = "device-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#device::*;
#[cfg(any(
    any(
        feature = "diagnosis-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#diagnosis;
#[cfg(any(
    any(
        feature = "diagnosis-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#diagnosis::*;
#[cfg(any(
    any(
        feature = "diagram-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#diagram;
#[cfg(any(
    any(
        feature = "diagram-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#diagram::*;
#[cfg(any(
    any(
        feature = "diet-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#diet;
#[cfg(any(
    any(
        feature = "diet-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#diet::*;
#[cfg(any(
    any(
        feature = "diet-features-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#diet_features;
#[cfg(any(
    any(
        feature = "diet-features-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#diet_features::*;
#[cfg(any(
    any(
        feature = "differential-diagnosis-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#differential_diagnosis;
#[cfg(any(
    any(
        feature = "differential-diagnosis-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#differential_diagnosis::*;
#[cfg(any(
    any(
        feature = "direct-apply-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#direct_apply;
#[cfg(any(
    any(
        feature = "direct-apply-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#direct_apply::*;
#[cfg(any(
    any(
        feature = "director-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#director;
#[cfg(any(
    any(
        feature = "director-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#director::*;
#[cfg(any(
    any(
        feature = "directors-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#directors;
#[cfg(any(
    any(
        feature = "directors-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#directors::*;
#[cfg(any(
    any(
        feature = "disambiguating-description-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#disambiguating_description;
#[cfg(any(
    any(
        feature = "disambiguating-description-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#disambiguating_description::*;
#[cfg(any(
    any(
        feature = "discount-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#discount;
#[cfg(any(
    any(
        feature = "discount-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#discount::*;
#[cfg(any(
    any(
        feature = "discount-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#discount_code;
#[cfg(any(
    any(
        feature = "discount-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#discount_code::*;
#[cfg(any(
    any(
        feature = "discount-currency-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#discount_currency;
#[cfg(any(
    any(
        feature = "discount-currency-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#discount_currency::*;
#[cfg(any(
    any(
        feature = "discusses-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#discusses;
#[cfg(any(
    any(
        feature = "discusses-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#discusses::*;
#[cfg(any(
    any(
        feature = "discussion-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#discussion_url;
#[cfg(any(
    any(
        feature = "discussion-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#discussion_url::*;
#[cfg(any(
    any(
        feature = "disease-prevention-info-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#disease_prevention_info;
#[cfg(any(
    any(
        feature = "disease-prevention-info-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#disease_prevention_info::*;
#[cfg(any(
    any(
        feature = "disease-spread-statistics-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#disease_spread_statistics;
#[cfg(any(
    any(
        feature = "disease-spread-statistics-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#disease_spread_statistics::*;
#[cfg(any(
    any(
        feature = "dissolution-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#dissolution_date;
#[cfg(any(
    any(
        feature = "dissolution-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#dissolution_date::*;
#[cfg(any(
    any(
        feature = "distance-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#distance;
#[cfg(any(
    any(
        feature = "distance-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#distance::*;
#[cfg(any(
    any(
        feature = "distinguishing-sign-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#distinguishing_sign;
#[cfg(any(
    any(
        feature = "distinguishing-sign-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#distinguishing_sign::*;
#[cfg(any(
    any(
        feature = "distribution-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#distribution;
#[cfg(any(
    any(
        feature = "distribution-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#distribution::*;
#[cfg(any(
    any(
        feature = "diversity-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#diversity_policy;
#[cfg(any(
    any(
        feature = "diversity-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#diversity_policy::*;
#[cfg(any(
    any(
        feature = "diversity-staffing-report-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#diversity_staffing_report;
#[cfg(any(
    any(
        feature = "diversity-staffing-report-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#diversity_staffing_report::*;
#[cfg(any(
    any(
        feature = "documentation-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#documentation;
#[cfg(any(
    any(
        feature = "documentation-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#documentation::*;
#[cfg(any(
    any(
        feature = "does-not-ship-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#does_not_ship;
#[cfg(any(
    any(
        feature = "does-not-ship-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#does_not_ship::*;
#[cfg(any(
    any(
        feature = "domain-includes-property-schema",
        feature = "meta-schema-section"
    ),
    doc
))]
mod r#domain_includes;
#[cfg(any(
    any(
        feature = "domain-includes-property-schema",
        feature = "meta-schema-section"
    ),
    doc
))]
pub use r#domain_includes::*;
#[cfg(any(
    any(
        feature = "domiciled-mortgage-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#domiciled_mortgage;
#[cfg(any(
    any(
        feature = "domiciled-mortgage-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#domiciled_mortgage::*;
#[cfg(any(
    any(
        feature = "door-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#door_time;
#[cfg(any(
    any(
        feature = "door-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#door_time::*;
#[cfg(any(
    any(
        feature = "dosage-form-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#dosage_form;
#[cfg(any(
    any(
        feature = "dosage-form-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#dosage_form::*;
#[cfg(any(
    any(
        feature = "dose-schedule-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#dose_schedule;
#[cfg(any(
    any(
        feature = "dose-schedule-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#dose_schedule::*;
#[cfg(any(
    any(
        feature = "dose-unit-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#dose_unit;
#[cfg(any(
    any(
        feature = "dose-unit-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#dose_unit::*;
#[cfg(any(
    any(
        feature = "dose-value-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#dose_value;
#[cfg(any(
    any(
        feature = "dose-value-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#dose_value::*;
#[cfg(any(
    any(
        feature = "down-payment-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#down_payment;
#[cfg(any(
    any(
        feature = "down-payment-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#down_payment::*;
#[cfg(any(
    any(
        feature = "download-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#download_url;
#[cfg(any(
    any(
        feature = "download-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#download_url::*;
#[cfg(any(
    any(
        feature = "downvote-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#downvote_count;
#[cfg(any(
    any(
        feature = "downvote-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#downvote_count::*;
#[cfg(any(
    any(
        feature = "drains-to-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#drains_to;
#[cfg(any(
    any(
        feature = "drains-to-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#drains_to::*;
#[cfg(any(
    any(
        feature = "drive-wheel-configuration-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#drive_wheel_configuration;
#[cfg(any(
    any(
        feature = "drive-wheel-configuration-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#drive_wheel_configuration::*;
#[cfg(any(
    any(
        feature = "dropoff-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#dropoff_location;
#[cfg(any(
    any(
        feature = "dropoff-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#dropoff_location::*;
#[cfg(any(
    any(
        feature = "dropoff-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#dropoff_time;
#[cfg(any(
    any(
        feature = "dropoff-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#dropoff_time::*;
#[cfg(any(
    any(
        feature = "drug-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#drug;
#[cfg(any(
    any(
        feature = "drug-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#drug::*;
#[cfg(any(
    any(
        feature = "drug-class-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#drug_class;
#[cfg(any(
    any(
        feature = "drug-class-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#drug_class::*;
#[cfg(any(
    any(
        feature = "drug-unit-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#drug_unit;
#[cfg(any(
    any(
        feature = "drug-unit-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#drug_unit::*;
#[cfg(any(
    any(feature = "duns-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#duns;
#[cfg(any(
    any(feature = "duns-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#duns::*;
#[cfg(any(
    any(
        feature = "duplicate-therapy-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#duplicate_therapy;
#[cfg(any(
    any(
        feature = "duplicate-therapy-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#duplicate_therapy::*;
#[cfg(any(
    any(
        feature = "duration-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#duration;
#[cfg(any(
    any(
        feature = "duration-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#duration::*;
#[cfg(any(
    any(
        feature = "duration-of-warranty-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#duration_of_warranty;
#[cfg(any(
    any(
        feature = "duration-of-warranty-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#duration_of_warranty::*;
#[cfg(any(
    any(
        feature = "during-media-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#during_media;
#[cfg(any(
    any(
        feature = "during-media-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#during_media::*;
#[cfg(any(
    any(
        feature = "early-prepayment-penalty-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#early_prepayment_penalty;
#[cfg(any(
    any(
        feature = "early-prepayment-penalty-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#early_prepayment_penalty::*;
#[cfg(any(
    any(
        feature = "edit-eidr-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#edit_eidr;
#[cfg(any(
    any(
        feature = "edit-eidr-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#edit_eidr::*;
#[cfg(any(
    any(feature = "editor-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#editor;
#[cfg(any(
    any(feature = "editor-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#editor::*;
#[cfg(any(
    any(
        feature = "edu-question-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#edu_question_type;
#[cfg(any(
    any(
        feature = "edu-question-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#edu_question_type::*;
#[cfg(any(
    any(
        feature = "education-requirements-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#education_requirements;
#[cfg(any(
    any(
        feature = "education-requirements-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#education_requirements::*;
#[cfg(any(
    any(
        feature = "educational-alignment-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#educational_alignment;
#[cfg(any(
    any(
        feature = "educational-alignment-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#educational_alignment::*;
#[cfg(any(
    any(
        feature = "educational-credential-awarded-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#educational_credential_awarded;
#[cfg(any(
    any(
        feature = "educational-credential-awarded-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#educational_credential_awarded::*;
#[cfg(any(
    any(
        feature = "educational-framework-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#educational_framework;
#[cfg(any(
    any(
        feature = "educational-framework-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#educational_framework::*;
#[cfg(any(
    any(
        feature = "educational-level-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#educational_level;
#[cfg(any(
    any(
        feature = "educational-level-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#educational_level::*;
#[cfg(any(
    any(
        feature = "educational-program-mode-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#educational_program_mode;
#[cfg(any(
    any(
        feature = "educational-program-mode-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#educational_program_mode::*;
#[cfg(any(
    any(
        feature = "educational-role-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#educational_role;
#[cfg(any(
    any(
        feature = "educational-role-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#educational_role::*;
#[cfg(any(
    any(
        feature = "educational-use-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#educational_use;
#[cfg(any(
    any(
        feature = "educational-use-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#educational_use::*;
#[cfg(any(
    any(
        feature = "elevation-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#elevation;
#[cfg(any(
    any(
        feature = "elevation-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#elevation::*;
#[cfg(any(
    any(
        feature = "eligibility-to-work-requirement-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#eligibility_to_work_requirement;
#[cfg(any(
    any(
        feature = "eligibility-to-work-requirement-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#eligibility_to_work_requirement::*;
#[cfg(any(
    any(
        feature = "eligible-customer-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#eligible_customer_type;
#[cfg(any(
    any(
        feature = "eligible-customer-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#eligible_customer_type::*;
#[cfg(any(
    any(
        feature = "eligible-duration-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#eligible_duration;
#[cfg(any(
    any(
        feature = "eligible-duration-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#eligible_duration::*;
#[cfg(any(
    any(
        feature = "eligible-quantity-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#eligible_quantity;
#[cfg(any(
    any(
        feature = "eligible-quantity-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#eligible_quantity::*;
#[cfg(any(
    any(
        feature = "eligible-region-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#eligible_region;
#[cfg(any(
    any(
        feature = "eligible-region-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#eligible_region::*;
#[cfg(any(
    any(
        feature = "eligible-transaction-volume-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#eligible_transaction_volume;
#[cfg(any(
    any(
        feature = "eligible-transaction-volume-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#eligible_transaction_volume::*;
#[cfg(any(
    any(feature = "email-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#email;
#[cfg(any(
    any(feature = "email-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#email::*;
#[cfg(any(
    any(
        feature = "embed-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#embed_url;
#[cfg(any(
    any(
        feature = "embed-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#embed_url::*;
#[cfg(any(
    any(
        feature = "embedded-text-caption-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#embedded_text_caption;
#[cfg(any(
    any(
        feature = "embedded-text-caption-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#embedded_text_caption::*;
#[cfg(any(
    any(
        feature = "emissions-co-2-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
mod r#emissions_co_2;
#[cfg(any(
    any(
        feature = "emissions-co-2-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
pub use r#emissions_co_2::*;
#[cfg(any(
    any(
        feature = "employee-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#employee;
#[cfg(any(
    any(
        feature = "employee-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#employee::*;
#[cfg(any(
    any(
        feature = "employees-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#employees;
#[cfg(any(
    any(
        feature = "employees-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#employees::*;
#[cfg(any(
    any(
        feature = "employer-overview-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#employer_overview;
#[cfg(any(
    any(
        feature = "employer-overview-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#employer_overview::*;
#[cfg(any(
    any(
        feature = "employment-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#employment_type;
#[cfg(any(
    any(
        feature = "employment-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#employment_type::*;
#[cfg(any(
    any(
        feature = "employment-unit-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#employment_unit;
#[cfg(any(
    any(
        feature = "employment-unit-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#employment_unit::*;
#[cfg(any(
    any(
        feature = "encodes-bio-chem-entity-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#encodes_bio_chem_entity;
#[cfg(any(
    any(
        feature = "encodes-bio-chem-entity-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#encodes_bio_chem_entity::*;
#[cfg(any(
    any(
        feature = "encodes-creative-work-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#encodes_creative_work;
#[cfg(any(
    any(
        feature = "encodes-creative-work-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#encodes_creative_work::*;
#[cfg(any(
    any(
        feature = "encoding-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#encoding;
#[cfg(any(
    any(
        feature = "encoding-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#encoding::*;
#[cfg(any(
    any(
        feature = "encoding-format-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#encoding_format;
#[cfg(any(
    any(
        feature = "encoding-format-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#encoding_format::*;
#[cfg(any(
    any(
        feature = "encoding-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#encoding_type;
#[cfg(any(
    any(
        feature = "encoding-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#encoding_type::*;
#[cfg(any(
    any(
        feature = "encodings-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#encodings;
#[cfg(any(
    any(
        feature = "encodings-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#encodings::*;
#[cfg(any(
    any(
        feature = "end-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#end_date;
#[cfg(any(
    any(
        feature = "end-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#end_date::*;
#[cfg(any(
    any(
        feature = "end-offset-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#end_offset;
#[cfg(any(
    any(
        feature = "end-offset-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#end_offset::*;
#[cfg(any(
    any(
        feature = "end-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#end_time;
#[cfg(any(
    any(
        feature = "end-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#end_time::*;
#[cfg(any(
    any(
        feature = "endorsee-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#endorsee;
#[cfg(any(
    any(
        feature = "endorsee-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#endorsee::*;
#[cfg(any(
    any(
        feature = "endorsers-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#endorsers;
#[cfg(any(
    any(
        feature = "endorsers-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#endorsers::*;
#[cfg(any(
    any(
        feature = "energy-efficiency-scale-max-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#energy_efficiency_scale_max;
#[cfg(any(
    any(
        feature = "energy-efficiency-scale-max-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#energy_efficiency_scale_max::*;
#[cfg(any(
    any(
        feature = "energy-efficiency-scale-min-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#energy_efficiency_scale_min;
#[cfg(any(
    any(
        feature = "energy-efficiency-scale-min-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#energy_efficiency_scale_min::*;
#[cfg(any(
    any(
        feature = "engine-displacement-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
mod r#engine_displacement;
#[cfg(any(
    any(
        feature = "engine-displacement-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
pub use r#engine_displacement::*;
#[cfg(any(
    any(
        feature = "engine-power-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
mod r#engine_power;
#[cfg(any(
    any(
        feature = "engine-power-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
pub use r#engine_power::*;
#[cfg(any(
    any(
        feature = "engine-type-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
mod r#engine_type;
#[cfg(any(
    any(
        feature = "engine-type-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
pub use r#engine_type::*;
#[cfg(any(
    any(
        feature = "entertainment-business-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#entertainment_business;
#[cfg(any(
    any(
        feature = "entertainment-business-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#entertainment_business::*;
#[cfg(any(
    any(
        feature = "epidemiology-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#epidemiology;
#[cfg(any(
    any(
        feature = "epidemiology-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#epidemiology::*;
#[cfg(any(
    any(
        feature = "episode-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#episode;
#[cfg(any(
    any(
        feature = "episode-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#episode::*;
#[cfg(any(
    any(
        feature = "episode-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#episode_number;
#[cfg(any(
    any(
        feature = "episode-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#episode_number::*;
#[cfg(any(
    any(
        feature = "episodes-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#episodes;
#[cfg(any(
    any(
        feature = "episodes-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#episodes::*;
#[cfg(any(
    any(feature = "equal-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#equal;
#[cfg(any(
    any(feature = "equal-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#equal::*;
#[cfg(any(
    any(feature = "error-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#error;
#[cfg(any(
    any(feature = "error-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#error::*;
#[cfg(any(
    any(
        feature = "estimated-cost-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#estimated_cost;
#[cfg(any(
    any(
        feature = "estimated-cost-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#estimated_cost::*;
#[cfg(any(
    any(
        feature = "estimated-flight-duration-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#estimated_flight_duration;
#[cfg(any(
    any(
        feature = "estimated-flight-duration-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#estimated_flight_duration::*;
#[cfg(any(
    any(
        feature = "estimated-salary-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#estimated_salary;
#[cfg(any(
    any(
        feature = "estimated-salary-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#estimated_salary::*;
#[cfg(any(
    any(
        feature = "estimates-risk-of-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#estimates_risk_of;
#[cfg(any(
    any(
        feature = "estimates-risk-of-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#estimates_risk_of::*;
#[cfg(any(
    any(
        feature = "ethics-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#ethics_policy;
#[cfg(any(
    any(
        feature = "ethics-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#ethics_policy::*;
#[cfg(any(
    any(feature = "event-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#event;
#[cfg(any(
    any(feature = "event-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#event::*;
#[cfg(any(
    any(
        feature = "event-attendance-mode-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#event_attendance_mode;
#[cfg(any(
    any(
        feature = "event-attendance-mode-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#event_attendance_mode::*;
#[cfg(any(
    any(
        feature = "event-schedule-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#event_schedule;
#[cfg(any(
    any(
        feature = "event-schedule-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#event_schedule::*;
#[cfg(any(
    any(
        feature = "event-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#event_status;
#[cfg(any(
    any(
        feature = "event-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#event_status::*;
#[cfg(any(
    any(feature = "events-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#events;
#[cfg(any(
    any(feature = "events-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#events::*;
#[cfg(any(
    any(
        feature = "evidence-level-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#evidence_level;
#[cfg(any(
    any(
        feature = "evidence-level-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#evidence_level::*;
#[cfg(any(
    any(
        feature = "evidence-origin-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#evidence_origin;
#[cfg(any(
    any(
        feature = "evidence-origin-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#evidence_origin::*;
#[cfg(any(
    any(
        feature = "example-of-work-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#example_of_work;
#[cfg(any(
    any(
        feature = "example-of-work-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#example_of_work::*;
#[cfg(any(
    any(
        feature = "except-date-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#except_date;
#[cfg(any(
    any(
        feature = "except-date-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#except_date::*;
#[cfg(any(
    any(
        feature = "exchange-rate-spread-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#exchange_rate_spread;
#[cfg(any(
    any(
        feature = "exchange-rate-spread-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#exchange_rate_spread::*;
#[cfg(any(
    any(
        feature = "executable-library-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#executable_library_name;
#[cfg(any(
    any(
        feature = "executable-library-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#executable_library_name::*;
#[cfg(any(
    any(
        feature = "exercise-course-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#exercise_course;
#[cfg(any(
    any(
        feature = "exercise-course-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#exercise_course::*;
#[cfg(any(
    any(
        feature = "exercise-plan-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#exercise_plan;
#[cfg(any(
    any(
        feature = "exercise-plan-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#exercise_plan::*;
#[cfg(any(
    any(
        feature = "exercise-related-diet-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#exercise_related_diet;
#[cfg(any(
    any(
        feature = "exercise-related-diet-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#exercise_related_diet::*;
#[cfg(any(
    any(
        feature = "exercise-type-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#exercise_type;
#[cfg(any(
    any(
        feature = "exercise-type-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#exercise_type::*;
#[cfg(any(
    any(
        feature = "exif-data-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#exif_data;
#[cfg(any(
    any(
        feature = "exif-data-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#exif_data::*;
#[cfg(any(
    any(
        feature = "expected-arrival-from-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#expected_arrival_from;
#[cfg(any(
    any(
        feature = "expected-arrival-from-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#expected_arrival_from::*;
#[cfg(any(
    any(
        feature = "expected-arrival-until-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#expected_arrival_until;
#[cfg(any(
    any(
        feature = "expected-arrival-until-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#expected_arrival_until::*;
#[cfg(any(
    any(
        feature = "expected-prognosis-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#expected_prognosis;
#[cfg(any(
    any(
        feature = "expected-prognosis-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#expected_prognosis::*;
#[cfg(any(
    any(
        feature = "expects-acceptance-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#expects_acceptance_of;
#[cfg(any(
    any(
        feature = "expects-acceptance-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#expects_acceptance_of::*;
#[cfg(any(
    any(
        feature = "experience-in-place-of-education-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#experience_in_place_of_education;
#[cfg(any(
    any(
        feature = "experience-in-place-of-education-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#experience_in_place_of_education::*;
#[cfg(any(
    any(
        feature = "experience-requirements-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#experience_requirements;
#[cfg(any(
    any(
        feature = "experience-requirements-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#experience_requirements::*;
#[cfg(any(
    any(
        feature = "expert-considerations-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#expert_considerations;
#[cfg(any(
    any(
        feature = "expert-considerations-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#expert_considerations::*;
#[cfg(any(
    any(
        feature = "expires-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#expires;
#[cfg(any(
    any(
        feature = "expires-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#expires::*;
#[cfg(any(
    any(
        feature = "expressed-in-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#expressed_in;
#[cfg(any(
    any(
        feature = "expressed-in-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#expressed_in::*;
#[cfg(any(
    any(
        feature = "family-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#family_name;
#[cfg(any(
    any(
        feature = "family-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#family_name::*;
#[cfg(any(
    any(
        feature = "fat-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#fat_content;
#[cfg(any(
    any(
        feature = "fat-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#fat_content::*;
#[cfg(any(
    any(
        feature = "fax-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#fax_number;
#[cfg(any(
    any(
        feature = "fax-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#fax_number::*;
#[cfg(any(
    any(
        feature = "feature-list-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#feature_list;
#[cfg(any(
    any(
        feature = "feature-list-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#feature_list::*;
#[cfg(any(
    any(
        feature = "fees-and-commissions-specification-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#fees_and_commissions_specification;
#[cfg(any(
    any(
        feature = "fees-and-commissions-specification-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#fees_and_commissions_specification::*;
#[cfg(any(
    any(
        feature = "fiber-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#fiber_content;
#[cfg(any(
    any(
        feature = "fiber-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#fiber_content::*;
#[cfg(any(
    any(
        feature = "file-format-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#file_format;
#[cfg(any(
    any(
        feature = "file-format-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#file_format::*;
#[cfg(any(
    any(
        feature = "file-size-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#file_size;
#[cfg(any(
    any(
        feature = "file-size-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#file_size::*;
#[cfg(any(
    any(
        feature = "financial-aid-eligible-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#financial_aid_eligible;
#[cfg(any(
    any(
        feature = "financial-aid-eligible-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#financial_aid_eligible::*;
#[cfg(any(
    any(
        feature = "first-appearance-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#first_appearance;
#[cfg(any(
    any(
        feature = "first-appearance-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#first_appearance::*;
#[cfg(any(
    any(
        feature = "first-performance-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#first_performance;
#[cfg(any(
    any(
        feature = "first-performance-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#first_performance::*;
#[cfg(any(
    any(
        feature = "flight-distance-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#flight_distance;
#[cfg(any(
    any(
        feature = "flight-distance-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#flight_distance::*;
#[cfg(any(
    any(
        feature = "flight-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#flight_number;
#[cfg(any(
    any(
        feature = "flight-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#flight_number::*;
#[cfg(any(
    any(
        feature = "floor-level-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#floor_level;
#[cfg(any(
    any(
        feature = "floor-level-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#floor_level::*;
#[cfg(any(
    any(
        feature = "floor-limit-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#floor_limit;
#[cfg(any(
    any(
        feature = "floor-limit-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#floor_limit::*;
#[cfg(any(
    any(
        feature = "floor-size-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#floor_size;
#[cfg(any(
    any(
        feature = "floor-size-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#floor_size::*;
#[cfg(any(
    any(
        feature = "followee-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#followee;
#[cfg(any(
    any(
        feature = "followee-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#followee::*;
#[cfg(any(
    any(
        feature = "follows-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#follows;
#[cfg(any(
    any(
        feature = "follows-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#follows::*;
#[cfg(any(
    any(
        feature = "followup-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#followup;
#[cfg(any(
    any(
        feature = "followup-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#followup::*;
#[cfg(any(
    any(
        feature = "food-establishment-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#food_establishment;
#[cfg(any(
    any(
        feature = "food-establishment-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#food_establishment::*;
#[cfg(any(
    any(
        feature = "food-event-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#food_event;
#[cfg(any(
    any(
        feature = "food-event-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#food_event::*;
#[cfg(any(
    any(
        feature = "food-warning-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#food_warning;
#[cfg(any(
    any(
        feature = "food-warning-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#food_warning::*;
#[cfg(any(
    any(
        feature = "founder-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#founder;
#[cfg(any(
    any(
        feature = "founder-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#founder::*;
#[cfg(any(
    any(
        feature = "founders-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#founders;
#[cfg(any(
    any(
        feature = "founders-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#founders::*;
#[cfg(any(
    any(
        feature = "founding-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#founding_date;
#[cfg(any(
    any(
        feature = "founding-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#founding_date::*;
#[cfg(any(
    any(
        feature = "founding-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#founding_location;
#[cfg(any(
    any(
        feature = "founding-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#founding_location::*;
#[cfg(any(
    any(feature = "free-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#free;
#[cfg(any(
    any(feature = "free-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#free::*;
#[cfg(any(
    any(
        feature = "free-shipping-threshold-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#free_shipping_threshold;
#[cfg(any(
    any(
        feature = "free-shipping-threshold-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#free_shipping_threshold::*;
#[cfg(any(
    any(
        feature = "frequency-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#frequency;
#[cfg(any(
    any(
        feature = "frequency-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#frequency::*;
#[cfg(any(
    any(
        feature = "from-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#from_location;
#[cfg(any(
    any(
        feature = "from-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#from_location::*;
#[cfg(any(
    any(
        feature = "fuel-capacity-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
mod r#fuel_capacity;
#[cfg(any(
    any(
        feature = "fuel-capacity-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
pub use r#fuel_capacity::*;
#[cfg(any(
    any(
        feature = "fuel-consumption-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#fuel_consumption;
#[cfg(any(
    any(
        feature = "fuel-consumption-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#fuel_consumption::*;
#[cfg(any(
    any(
        feature = "fuel-efficiency-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#fuel_efficiency;
#[cfg(any(
    any(
        feature = "fuel-efficiency-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#fuel_efficiency::*;
#[cfg(any(
    any(
        feature = "fuel-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#fuel_type;
#[cfg(any(
    any(
        feature = "fuel-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#fuel_type::*;
#[cfg(any(
    any(
        feature = "functional-class-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#functional_class;
#[cfg(any(
    any(
        feature = "functional-class-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#functional_class::*;
#[cfg(any(
    any(
        feature = "funded-item-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#funded_item;
#[cfg(any(
    any(
        feature = "funded-item-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#funded_item::*;
#[cfg(any(
    any(feature = "funder-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#funder;
#[cfg(any(
    any(feature = "funder-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#funder::*;
#[cfg(any(
    any(
        feature = "funding-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#funding;
#[cfg(any(
    any(
        feature = "funding-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#funding::*;
#[cfg(any(
    any(feature = "game-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#game;
#[cfg(any(
    any(feature = "game-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#game::*;
#[cfg(any(
    any(
        feature = "game-availability-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#game_availability_type;
#[cfg(any(
    any(
        feature = "game-availability-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#game_availability_type::*;
#[cfg(any(
    any(
        feature = "game-edition-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#game_edition;
#[cfg(any(
    any(
        feature = "game-edition-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#game_edition::*;
#[cfg(any(
    any(
        feature = "game-item-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#game_item;
#[cfg(any(
    any(
        feature = "game-item-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#game_item::*;
#[cfg(any(
    any(
        feature = "game-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#game_location;
#[cfg(any(
    any(
        feature = "game-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#game_location::*;
#[cfg(any(
    any(
        feature = "game-platform-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#game_platform;
#[cfg(any(
    any(
        feature = "game-platform-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#game_platform::*;
#[cfg(any(
    any(
        feature = "game-server-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#game_server;
#[cfg(any(
    any(
        feature = "game-server-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#game_server::*;
#[cfg(any(
    any(
        feature = "game-tip-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#game_tip;
#[cfg(any(
    any(
        feature = "game-tip-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#game_tip::*;
#[cfg(any(
    any(feature = "gender-property-schema", feature = "pending-schema-section"),
    doc
))]
mod r#gender;
#[cfg(any(
    any(feature = "gender-property-schema", feature = "pending-schema-section"),
    doc
))]
pub use r#gender::*;
#[cfg(any(
    any(feature = "genre-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#genre;
#[cfg(any(
    any(feature = "genre-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#genre::*;
#[cfg(any(
    any(feature = "geo-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#geo;
#[cfg(any(
    any(feature = "geo-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#geo::*;
#[cfg(any(
    any(
        feature = "geo-contains-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#geo_contains;
#[cfg(any(
    any(
        feature = "geo-contains-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#geo_contains::*;
#[cfg(any(
    any(
        feature = "geo-covered-by-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#geo_covered_by;
#[cfg(any(
    any(
        feature = "geo-covered-by-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#geo_covered_by::*;
#[cfg(any(
    any(
        feature = "geo-covers-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#geo_covers;
#[cfg(any(
    any(
        feature = "geo-covers-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#geo_covers::*;
#[cfg(any(
    any(
        feature = "geo-crosses-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#geo_crosses;
#[cfg(any(
    any(
        feature = "geo-crosses-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#geo_crosses::*;
#[cfg(any(
    any(
        feature = "geo-disjoint-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#geo_disjoint;
#[cfg(any(
    any(
        feature = "geo-disjoint-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#geo_disjoint::*;
#[cfg(any(
    any(
        feature = "geo-equals-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#geo_equals;
#[cfg(any(
    any(
        feature = "geo-equals-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#geo_equals::*;
#[cfg(any(
    any(
        feature = "geo-intersects-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#geo_intersects;
#[cfg(any(
    any(
        feature = "geo-intersects-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#geo_intersects::*;
#[cfg(any(
    any(
        feature = "geo-midpoint-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#geo_midpoint;
#[cfg(any(
    any(
        feature = "geo-midpoint-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#geo_midpoint::*;
#[cfg(any(
    any(
        feature = "geo-overlaps-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#geo_overlaps;
#[cfg(any(
    any(
        feature = "geo-overlaps-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#geo_overlaps::*;
#[cfg(any(
    any(
        feature = "geo-radius-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#geo_radius;
#[cfg(any(
    any(
        feature = "geo-radius-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#geo_radius::*;
#[cfg(any(
    any(
        feature = "geo-touches-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#geo_touches;
#[cfg(any(
    any(
        feature = "geo-touches-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#geo_touches::*;
#[cfg(any(
    any(
        feature = "geo-within-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#geo_within;
#[cfg(any(
    any(
        feature = "geo-within-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#geo_within::*;
#[cfg(any(
    any(
        feature = "geographic-area-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#geographic_area;
#[cfg(any(
    any(
        feature = "geographic-area-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#geographic_area::*;
#[cfg(any(
    any(
        feature = "getting-tested-info-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#getting_tested_info;
#[cfg(any(
    any(
        feature = "getting-tested-info-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#getting_tested_info::*;
#[cfg(any(
    any(
        feature = "given-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#given_name;
#[cfg(any(
    any(
        feature = "given-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#given_name::*;
#[cfg(any(
    any(
        feature = "global-location-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#global_location_number;
#[cfg(any(
    any(
        feature = "global-location-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#global_location_number::*;
#[cfg(any(
    any(
        feature = "government-benefits-info-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#government_benefits_info;
#[cfg(any(
    any(
        feature = "government-benefits-info-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#government_benefits_info::*;
#[cfg(any(
    any(
        feature = "grace-period-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#grace_period;
#[cfg(any(
    any(
        feature = "grace-period-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#grace_period::*;
#[cfg(any(
    any(
        feature = "grantee-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#grantee;
#[cfg(any(
    any(
        feature = "grantee-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#grantee::*;
#[cfg(any(
    any(
        feature = "greater-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#greater;
#[cfg(any(
    any(
        feature = "greater-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#greater::*;
#[cfg(any(
    any(
        feature = "greater-or-equal-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#greater_or_equal;
#[cfg(any(
    any(
        feature = "greater-or-equal-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#greater_or_equal::*;
#[cfg(any(
    any(feature = "gtin-property-schema", feature = "pending-schema-section"),
    doc
))]
mod r#gtin;
#[cfg(any(
    any(feature = "gtin-property-schema", feature = "pending-schema-section"),
    doc
))]
pub use r#gtin::*;
#[cfg(any(
    any(
        feature = "gtin-12-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#gtin_12;
#[cfg(any(
    any(
        feature = "gtin-12-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#gtin_12::*;
#[cfg(any(
    any(
        feature = "gtin-13-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#gtin_13;
#[cfg(any(
    any(
        feature = "gtin-13-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#gtin_13::*;
#[cfg(any(
    any(
        feature = "gtin-14-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#gtin_14;
#[cfg(any(
    any(
        feature = "gtin-14-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#gtin_14::*;
#[cfg(any(
    any(feature = "gtin-8-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#gtin_8;
#[cfg(any(
    any(feature = "gtin-8-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#gtin_8::*;
#[cfg(any(
    any(
        feature = "guideline-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#guideline;
#[cfg(any(
    any(
        feature = "guideline-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#guideline::*;
#[cfg(any(
    any(
        feature = "guideline-date-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#guideline_date;
#[cfg(any(
    any(
        feature = "guideline-date-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#guideline_date::*;
#[cfg(any(
    any(
        feature = "guideline-subject-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#guideline_subject;
#[cfg(any(
    any(
        feature = "guideline-subject-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#guideline_subject::*;
#[cfg(any(
    any(
        feature = "handling-time-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#handling_time;
#[cfg(any(
    any(
        feature = "handling-time-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#handling_time::*;
#[cfg(any(
    any(
        feature = "has-adult-consideration-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#has_adult_consideration;
#[cfg(any(
    any(
        feature = "has-adult-consideration-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#has_adult_consideration::*;
#[cfg(any(
    any(
        feature = "has-bio-chem-entity-part-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#has_bio_chem_entity_part;
#[cfg(any(
    any(
        feature = "has-bio-chem-entity-part-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#has_bio_chem_entity_part::*;
#[cfg(any(
    any(
        feature = "has-bio-polymer-sequence-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#has_bio_polymer_sequence;
#[cfg(any(
    any(
        feature = "has-bio-polymer-sequence-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#has_bio_polymer_sequence::*;
#[cfg(any(
    any(
        feature = "has-broadcast-channel-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#has_broadcast_channel;
#[cfg(any(
    any(
        feature = "has-broadcast-channel-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#has_broadcast_channel::*;
#[cfg(any(
    any(
        feature = "has-category-code-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#has_category_code;
#[cfg(any(
    any(
        feature = "has-category-code-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#has_category_code::*;
#[cfg(any(
    any(
        feature = "has-course-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#has_course;
#[cfg(any(
    any(
        feature = "has-course-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#has_course::*;
#[cfg(any(
    any(
        feature = "has-course-instance-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#has_course_instance;
#[cfg(any(
    any(
        feature = "has-course-instance-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#has_course_instance::*;
#[cfg(any(
    any(
        feature = "has-credential-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#has_credential;
#[cfg(any(
    any(
        feature = "has-credential-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#has_credential::*;
#[cfg(any(
    any(
        feature = "has-defined-term-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#has_defined_term;
#[cfg(any(
    any(
        feature = "has-defined-term-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#has_defined_term::*;
#[cfg(any(
    any(
        feature = "has-delivery-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#has_delivery_method;
#[cfg(any(
    any(
        feature = "has-delivery-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#has_delivery_method::*;
#[cfg(any(
    any(
        feature = "has-digital-document-permission-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#has_digital_document_permission;
#[cfg(any(
    any(
        feature = "has-digital-document-permission-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#has_digital_document_permission::*;
#[cfg(any(
    any(
        feature = "has-drive-through-service-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#has_drive_through_service;
#[cfg(any(
    any(
        feature = "has-drive-through-service-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#has_drive_through_service::*;
#[cfg(any(
    any(
        feature = "has-energy-consumption-details-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#has_energy_consumption_details;
#[cfg(any(
    any(
        feature = "has-energy-consumption-details-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#has_energy_consumption_details::*;
#[cfg(any(
    any(
        feature = "has-energy-efficiency-category-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#has_energy_efficiency_category;
#[cfg(any(
    any(
        feature = "has-energy-efficiency-category-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#has_energy_efficiency_category::*;
#[cfg(any(
    any(
        feature = "has-health-aspect-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#has_health_aspect;
#[cfg(any(
    any(
        feature = "has-health-aspect-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#has_health_aspect::*;
#[cfg(any(
    any(
        feature = "has-map-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#has_map;
#[cfg(any(
    any(
        feature = "has-map-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#has_map::*;
#[cfg(any(
    any(
        feature = "has-measurement-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#has_measurement;
#[cfg(any(
    any(
        feature = "has-measurement-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#has_measurement::*;
#[cfg(any(
    any(
        feature = "has-menu-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#has_menu;
#[cfg(any(
    any(
        feature = "has-menu-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#has_menu::*;
#[cfg(any(
    any(
        feature = "has-menu-item-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#has_menu_item;
#[cfg(any(
    any(
        feature = "has-menu-item-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#has_menu_item::*;
#[cfg(any(
    any(
        feature = "has-menu-section-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#has_menu_section;
#[cfg(any(
    any(
        feature = "has-menu-section-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#has_menu_section::*;
#[cfg(any(
    any(
        feature = "has-merchant-return-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#has_merchant_return_policy;
#[cfg(any(
    any(
        feature = "has-merchant-return-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#has_merchant_return_policy::*;
#[cfg(any(
    any(
        feature = "has-molecular-function-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#has_molecular_function;
#[cfg(any(
    any(
        feature = "has-molecular-function-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#has_molecular_function::*;
#[cfg(any(
    any(
        feature = "has-occupation-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#has_occupation;
#[cfg(any(
    any(
        feature = "has-occupation-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#has_occupation::*;
#[cfg(any(
    any(
        feature = "has-offer-catalog-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#has_offer_catalog;
#[cfg(any(
    any(
        feature = "has-offer-catalog-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#has_offer_catalog::*;
#[cfg(any(
    any(
        feature = "has-part-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#has_part;
#[cfg(any(
    any(
        feature = "has-part-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#has_part::*;
#[cfg(any(
    any(
        feature = "has-pos-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#has_pos;
#[cfg(any(
    any(
        feature = "has-pos-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#has_pos::*;
#[cfg(any(
    any(
        feature = "has-product-return-policy-property-schema",
        feature = "attic-schema-section"
    ),
    doc
))]
mod r#has_product_return_policy;
#[cfg(any(
    any(
        feature = "has-product-return-policy-property-schema",
        feature = "attic-schema-section"
    ),
    doc
))]
pub use r#has_product_return_policy::*;
#[cfg(any(
    any(
        feature = "has-representation-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#has_representation;
#[cfg(any(
    any(
        feature = "has-representation-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#has_representation::*;
#[cfg(any(
    any(
        feature = "has-variant-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#has_variant;
#[cfg(any(
    any(
        feature = "has-variant-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#has_variant::*;
#[cfg(any(
    any(
        feature = "headline-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#headline;
#[cfg(any(
    any(
        feature = "headline-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#headline::*;
#[cfg(any(
    any(
        feature = "health-condition-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#health_condition;
#[cfg(any(
    any(
        feature = "health-condition-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#health_condition::*;
#[cfg(any(
    any(
        feature = "health-plan-coinsurance-option-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#health_plan_coinsurance_option;
#[cfg(any(
    any(
        feature = "health-plan-coinsurance-option-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#health_plan_coinsurance_option::*;
#[cfg(any(
    any(
        feature = "health-plan-coinsurance-rate-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#health_plan_coinsurance_rate;
#[cfg(any(
    any(
        feature = "health-plan-coinsurance-rate-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#health_plan_coinsurance_rate::*;
#[cfg(any(
    any(
        feature = "health-plan-copay-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#health_plan_copay;
#[cfg(any(
    any(
        feature = "health-plan-copay-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#health_plan_copay::*;
#[cfg(any(
    any(
        feature = "health-plan-copay-option-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#health_plan_copay_option;
#[cfg(any(
    any(
        feature = "health-plan-copay-option-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#health_plan_copay_option::*;
#[cfg(any(
    any(
        feature = "health-plan-cost-sharing-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#health_plan_cost_sharing;
#[cfg(any(
    any(
        feature = "health-plan-cost-sharing-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#health_plan_cost_sharing::*;
#[cfg(any(
    any(
        feature = "health-plan-drug-option-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#health_plan_drug_option;
#[cfg(any(
    any(
        feature = "health-plan-drug-option-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#health_plan_drug_option::*;
#[cfg(any(
    any(
        feature = "health-plan-drug-tier-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#health_plan_drug_tier;
#[cfg(any(
    any(
        feature = "health-plan-drug-tier-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#health_plan_drug_tier::*;
#[cfg(any(
    any(
        feature = "health-plan-id-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#health_plan_id;
#[cfg(any(
    any(
        feature = "health-plan-id-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#health_plan_id::*;
#[cfg(any(
    any(
        feature = "health-plan-marketing-url-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#health_plan_marketing_url;
#[cfg(any(
    any(
        feature = "health-plan-marketing-url-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#health_plan_marketing_url::*;
#[cfg(any(
    any(
        feature = "health-plan-network-id-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#health_plan_network_id;
#[cfg(any(
    any(
        feature = "health-plan-network-id-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#health_plan_network_id::*;
#[cfg(any(
    any(
        feature = "health-plan-network-tier-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#health_plan_network_tier;
#[cfg(any(
    any(
        feature = "health-plan-network-tier-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#health_plan_network_tier::*;
#[cfg(any(
    any(
        feature = "health-plan-pharmacy-category-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#health_plan_pharmacy_category;
#[cfg(any(
    any(
        feature = "health-plan-pharmacy-category-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#health_plan_pharmacy_category::*;
#[cfg(any(
    any(
        feature = "healthcare-reporting-data-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#healthcare_reporting_data;
#[cfg(any(
    any(
        feature = "healthcare-reporting-data-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#healthcare_reporting_data::*;
#[cfg(any(
    any(feature = "height-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#height;
#[cfg(any(
    any(feature = "height-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#height::*;
#[cfg(any(
    any(
        feature = "high-price-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#high_price;
#[cfg(any(
    any(
        feature = "high-price-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#high_price::*;
#[cfg(any(
    any(
        feature = "hiring-organization-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#hiring_organization;
#[cfg(any(
    any(
        feature = "hiring-organization-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#hiring_organization::*;
#[cfg(any(
    any(
        feature = "holding-archive-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#holding_archive;
#[cfg(any(
    any(
        feature = "holding-archive-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#holding_archive::*;
#[cfg(any(
    any(
        feature = "home-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#home_location;
#[cfg(any(
    any(
        feature = "home-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#home_location::*;
#[cfg(any(
    any(
        feature = "home-team-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#home_team;
#[cfg(any(
    any(
        feature = "home-team-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#home_team::*;
#[cfg(any(
    any(
        feature = "honorific-prefix-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#honorific_prefix;
#[cfg(any(
    any(
        feature = "honorific-prefix-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#honorific_prefix::*;
#[cfg(any(
    any(
        feature = "honorific-suffix-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#honorific_suffix;
#[cfg(any(
    any(
        feature = "honorific-suffix-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#honorific_suffix::*;
#[cfg(any(
    any(
        feature = "hospital-affiliation-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#hospital_affiliation;
#[cfg(any(
    any(
        feature = "hospital-affiliation-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#hospital_affiliation::*;
#[cfg(any(
    any(
        feature = "hosting-organization-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#hosting_organization;
#[cfg(any(
    any(
        feature = "hosting-organization-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#hosting_organization::*;
#[cfg(any(
    any(
        feature = "hours-available-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#hours_available;
#[cfg(any(
    any(
        feature = "hours-available-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#hours_available::*;
#[cfg(any(
    any(
        feature = "how-performed-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#how_performed;
#[cfg(any(
    any(
        feature = "how-performed-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#how_performed::*;
#[cfg(any(
    any(
        feature = "http-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#http_method;
#[cfg(any(
    any(
        feature = "http-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#http_method::*;
#[cfg(any(
    any(
        feature = "iata-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#iata_code;
#[cfg(any(
    any(
        feature = "iata-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#iata_code::*;
#[cfg(any(
    any(
        feature = "icao-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#icao_code;
#[cfg(any(
    any(
        feature = "icao-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#icao_code::*;
#[cfg(any(
    any(
        feature = "identifier-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#identifier;
#[cfg(any(
    any(
        feature = "identifier-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#identifier::*;
#[cfg(any(
    any(
        feature = "identifying-exam-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#identifying_exam;
#[cfg(any(
    any(
        feature = "identifying-exam-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#identifying_exam::*;
#[cfg(any(
    any(
        feature = "identifying-test-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#identifying_test;
#[cfg(any(
    any(
        feature = "identifying-test-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#identifying_test::*;
#[cfg(any(
    any(
        feature = "illustrator-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#illustrator;
#[cfg(any(
    any(
        feature = "illustrator-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#illustrator::*;
#[cfg(any(
    any(feature = "image-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#image;
#[cfg(any(
    any(feature = "image-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#image::*;
#[cfg(any(
    any(
        feature = "imaging-technique-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#imaging_technique;
#[cfg(any(
    any(
        feature = "imaging-technique-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#imaging_technique::*;
#[cfg(any(
    any(
        feature = "in-album-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#in_album;
#[cfg(any(
    any(
        feature = "in-album-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#in_album::*;
#[cfg(any(
    any(
        feature = "in-broadcast-lineup-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#in_broadcast_lineup;
#[cfg(any(
    any(
        feature = "in-broadcast-lineup-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#in_broadcast_lineup::*;
#[cfg(any(
    any(
        feature = "in-ch-i-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#in_ch_i;
#[cfg(any(
    any(
        feature = "in-ch-i-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#in_ch_i::*;
#[cfg(any(
    any(
        feature = "in-ch-i-key-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#in_ch_i_key;
#[cfg(any(
    any(
        feature = "in-ch-i-key-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#in_ch_i_key::*;
#[cfg(any(
    any(
        feature = "in-code-set-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#in_code_set;
#[cfg(any(
    any(
        feature = "in-code-set-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#in_code_set::*;
#[cfg(any(
    any(
        feature = "in-defined-term-set-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#in_defined_term_set;
#[cfg(any(
    any(
        feature = "in-defined-term-set-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#in_defined_term_set::*;
#[cfg(any(
    any(
        feature = "in-language-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#in_language;
#[cfg(any(
    any(
        feature = "in-language-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#in_language::*;
#[cfg(any(
    any(
        feature = "in-playlist-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#in_playlist;
#[cfg(any(
    any(
        feature = "in-playlist-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#in_playlist::*;
#[cfg(any(
    any(
        feature = "in-product-group-with-id-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#in_product_group_with_id;
#[cfg(any(
    any(
        feature = "in-product-group-with-id-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#in_product_group_with_id::*;
#[cfg(any(
    any(
        feature = "in-store-returns-offered-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#in_store_returns_offered;
#[cfg(any(
    any(
        feature = "in-store-returns-offered-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#in_store_returns_offered::*;
#[cfg(any(
    any(
        feature = "in-support-of-property-schema",
        feature = "bib-schema-section"
    ),
    doc
))]
mod r#in_support_of;
#[cfg(any(
    any(
        feature = "in-support-of-property-schema",
        feature = "bib-schema-section"
    ),
    doc
))]
pub use r#in_support_of::*;
#[cfg(any(
    any(
        feature = "incentive-compensation-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#incentive_compensation;
#[cfg(any(
    any(
        feature = "incentive-compensation-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#incentive_compensation::*;
#[cfg(any(
    any(
        feature = "incentives-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#incentives;
#[cfg(any(
    any(
        feature = "incentives-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#incentives::*;
#[cfg(any(
    any(
        feature = "included-composition-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#included_composition;
#[cfg(any(
    any(
        feature = "included-composition-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#included_composition::*;
#[cfg(any(
    any(
        feature = "included-data-catalog-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#included_data_catalog;
#[cfg(any(
    any(
        feature = "included-data-catalog-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#included_data_catalog::*;
#[cfg(any(
    any(
        feature = "included-in-data-catalog-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#included_in_data_catalog;
#[cfg(any(
    any(
        feature = "included-in-data-catalog-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#included_in_data_catalog::*;
#[cfg(any(
    any(
        feature = "included-in-health-insurance-plan-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#included_in_health_insurance_plan;
#[cfg(any(
    any(
        feature = "included-in-health-insurance-plan-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#included_in_health_insurance_plan::*;
#[cfg(any(
    any(
        feature = "included-risk-factor-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#included_risk_factor;
#[cfg(any(
    any(
        feature = "included-risk-factor-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#included_risk_factor::*;
#[cfg(any(
    any(
        feature = "includes-attraction-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#includes_attraction;
#[cfg(any(
    any(
        feature = "includes-attraction-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#includes_attraction::*;
#[cfg(any(
    any(
        feature = "includes-health-plan-formulary-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#includes_health_plan_formulary;
#[cfg(any(
    any(
        feature = "includes-health-plan-formulary-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#includes_health_plan_formulary::*;
#[cfg(any(
    any(
        feature = "includes-health-plan-network-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#includes_health_plan_network;
#[cfg(any(
    any(
        feature = "includes-health-plan-network-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#includes_health_plan_network::*;
#[cfg(any(
    any(
        feature = "includes-object-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#includes_object;
#[cfg(any(
    any(
        feature = "includes-object-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#includes_object::*;
#[cfg(any(
    any(
        feature = "increases-risk-of-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#increases_risk_of;
#[cfg(any(
    any(
        feature = "increases-risk-of-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#increases_risk_of::*;
#[cfg(any(
    any(
        feature = "industry-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#industry;
#[cfg(any(
    any(
        feature = "industry-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#industry::*;
#[cfg(any(
    any(
        feature = "ineligible-region-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#ineligible_region;
#[cfg(any(
    any(
        feature = "ineligible-region-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#ineligible_region::*;
#[cfg(any(
    any(
        feature = "infectious-agent-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#infectious_agent;
#[cfg(any(
    any(
        feature = "infectious-agent-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#infectious_agent::*;
#[cfg(any(
    any(
        feature = "infectious-agent-class-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#infectious_agent_class;
#[cfg(any(
    any(
        feature = "infectious-agent-class-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#infectious_agent_class::*;
#[cfg(any(
    any(
        feature = "ingredients-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#ingredients;
#[cfg(any(
    any(
        feature = "ingredients-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#ingredients::*;
#[cfg(any(
    any(feature = "inker-property-schema", feature = "bib-schema-section"),
    doc
))]
mod r#inker;
#[cfg(any(
    any(feature = "inker-property-schema", feature = "bib-schema-section"),
    doc
))]
pub use r#inker::*;
#[cfg(any(
    any(
        feature = "insertion-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#insertion;
#[cfg(any(
    any(
        feature = "insertion-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#insertion::*;
#[cfg(any(
    any(
        feature = "install-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#install_url;
#[cfg(any(
    any(
        feature = "install-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#install_url::*;
#[cfg(any(
    any(
        feature = "instructor-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#instructor;
#[cfg(any(
    any(
        feature = "instructor-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#instructor::*;
#[cfg(any(
    any(
        feature = "instrument-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#instrument;
#[cfg(any(
    any(
        feature = "instrument-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#instrument::*;
#[cfg(any(
    any(
        feature = "intensity-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#intensity;
#[cfg(any(
    any(
        feature = "intensity-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#intensity::*;
#[cfg(any(
    any(
        feature = "interacting-drug-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#interacting_drug;
#[cfg(any(
    any(
        feature = "interacting-drug-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#interacting_drug::*;
#[cfg(any(
    any(
        feature = "interaction-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#interaction_count;
#[cfg(any(
    any(
        feature = "interaction-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#interaction_count::*;
#[cfg(any(
    any(
        feature = "interaction-service-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#interaction_service;
#[cfg(any(
    any(
        feature = "interaction-service-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#interaction_service::*;
#[cfg(any(
    any(
        feature = "interaction-statistic-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#interaction_statistic;
#[cfg(any(
    any(
        feature = "interaction-statistic-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#interaction_statistic::*;
#[cfg(any(
    any(
        feature = "interaction-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#interaction_type;
#[cfg(any(
    any(
        feature = "interaction-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#interaction_type::*;
#[cfg(any(
    any(
        feature = "interactivity-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#interactivity_type;
#[cfg(any(
    any(
        feature = "interactivity-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#interactivity_type::*;
#[cfg(any(
    any(
        feature = "interest-rate-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#interest_rate;
#[cfg(any(
    any(
        feature = "interest-rate-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#interest_rate::*;
#[cfg(any(
    any(
        feature = "interpreted-as-claim-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#interpreted_as_claim;
#[cfg(any(
    any(
        feature = "interpreted-as-claim-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#interpreted_as_claim::*;
#[cfg(any(
    any(
        feature = "inventory-level-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#inventory_level;
#[cfg(any(
    any(
        feature = "inventory-level-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#inventory_level::*;
#[cfg(any(
    any(
        feature = "inverse-of-property-schema",
        feature = "meta-schema-section"
    ),
    doc
))]
mod r#inverse_of;
#[cfg(any(
    any(
        feature = "inverse-of-property-schema",
        feature = "meta-schema-section"
    ),
    doc
))]
pub use r#inverse_of::*;
#[cfg(any(
    any(
        feature = "is-accepting-new-patients-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#is_accepting_new_patients;
#[cfg(any(
    any(
        feature = "is-accepting-new-patients-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#is_accepting_new_patients::*;
#[cfg(any(
    any(
        feature = "is-accessible-for-free-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#is_accessible_for_free;
#[cfg(any(
    any(
        feature = "is-accessible-for-free-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#is_accessible_for_free::*;
#[cfg(any(
    any(
        feature = "is-accessory-or-spare-part-for-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#is_accessory_or_spare_part_for;
#[cfg(any(
    any(
        feature = "is-accessory-or-spare-part-for-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#is_accessory_or_spare_part_for::*;
#[cfg(any(
    any(
        feature = "is-available-generically-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#is_available_generically;
#[cfg(any(
    any(
        feature = "is-available-generically-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#is_available_generically::*;
#[cfg(any(
    any(
        feature = "is-based-on-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#is_based_on;
#[cfg(any(
    any(
        feature = "is-based-on-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#is_based_on::*;
#[cfg(any(
    any(
        feature = "is-based-on-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#is_based_on_url;
#[cfg(any(
    any(
        feature = "is-based-on-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#is_based_on_url::*;
#[cfg(any(
    any(
        feature = "is-consumable-for-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#is_consumable_for;
#[cfg(any(
    any(
        feature = "is-consumable-for-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#is_consumable_for::*;
#[cfg(any(
    any(
        feature = "is-encoded-by-bio-chem-entity-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#is_encoded_by_bio_chem_entity;
#[cfg(any(
    any(
        feature = "is-encoded-by-bio-chem-entity-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#is_encoded_by_bio_chem_entity::*;
#[cfg(any(
    any(
        feature = "is-family-friendly-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#is_family_friendly;
#[cfg(any(
    any(
        feature = "is-family-friendly-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#is_family_friendly::*;
#[cfg(any(
    any(
        feature = "is-gift-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#is_gift;
#[cfg(any(
    any(
        feature = "is-gift-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#is_gift::*;
#[cfg(any(
    any(
        feature = "is-involved-in-biological-process-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#is_involved_in_biological_process;
#[cfg(any(
    any(
        feature = "is-involved-in-biological-process-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#is_involved_in_biological_process::*;
#[cfg(any(
    any(
        feature = "is-live-broadcast-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#is_live_broadcast;
#[cfg(any(
    any(
        feature = "is-live-broadcast-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#is_live_broadcast::*;
#[cfg(any(
    any(
        feature = "is-located-in-subcellular-location-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#is_located_in_subcellular_location;
#[cfg(any(
    any(
        feature = "is-located-in-subcellular-location-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#is_located_in_subcellular_location::*;
#[cfg(any(
    any(
        feature = "is-part-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#is_part_of;
#[cfg(any(
    any(
        feature = "is-part-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#is_part_of::*;
#[cfg(any(
    any(
        feature = "is-part-of-bio-chem-entity-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#is_part_of_bio_chem_entity;
#[cfg(any(
    any(
        feature = "is-part-of-bio-chem-entity-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#is_part_of_bio_chem_entity::*;
#[cfg(any(
    any(
        feature = "is-plan-for-apartment-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#is_plan_for_apartment;
#[cfg(any(
    any(
        feature = "is-plan-for-apartment-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#is_plan_for_apartment::*;
#[cfg(any(
    any(
        feature = "is-proprietary-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#is_proprietary;
#[cfg(any(
    any(
        feature = "is-proprietary-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#is_proprietary::*;
#[cfg(any(
    any(
        feature = "is-related-to-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#is_related_to;
#[cfg(any(
    any(
        feature = "is-related-to-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#is_related_to::*;
#[cfg(any(
    any(
        feature = "is-resizable-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#is_resizable;
#[cfg(any(
    any(
        feature = "is-resizable-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#is_resizable::*;
#[cfg(any(
    any(
        feature = "is-similar-to-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#is_similar_to;
#[cfg(any(
    any(
        feature = "is-similar-to-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#is_similar_to::*;
#[cfg(any(
    any(
        feature = "is-unlabelled-fallback-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#is_unlabelled_fallback;
#[cfg(any(
    any(
        feature = "is-unlabelled-fallback-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#is_unlabelled_fallback::*;
#[cfg(any(
    any(
        feature = "is-variant-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#is_variant_of;
#[cfg(any(
    any(
        feature = "is-variant-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#is_variant_of::*;
#[cfg(any(
    any(feature = "isbn-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#isbn;
#[cfg(any(
    any(feature = "isbn-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#isbn::*;
#[cfg(any(
    any(
        feature = "isic-v-4-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#isic_v_4;
#[cfg(any(
    any(
        feature = "isic-v-4-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#isic_v_4::*;
#[cfg(any(
    any(
        feature = "iso-6523-code-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#iso_6523_code;
#[cfg(any(
    any(
        feature = "iso-6523-code-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#iso_6523_code::*;
#[cfg(any(
    any(
        feature = "isrc-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#isrc_code;
#[cfg(any(
    any(
        feature = "isrc-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#isrc_code::*;
#[cfg(any(
    any(feature = "issn-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#issn;
#[cfg(any(
    any(feature = "issn-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#issn::*;
#[cfg(any(
    any(
        feature = "issue-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#issue_number;
#[cfg(any(
    any(
        feature = "issue-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#issue_number::*;
#[cfg(any(
    any(
        feature = "issued-by-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#issued_by;
#[cfg(any(
    any(
        feature = "issued-by-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#issued_by::*;
#[cfg(any(
    any(
        feature = "issued-through-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#issued_through;
#[cfg(any(
    any(
        feature = "issued-through-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#issued_through::*;
#[cfg(any(
    any(
        feature = "iswc-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#iswc_code;
#[cfg(any(
    any(
        feature = "iswc-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#iswc_code::*;
#[cfg(any(
    any(feature = "item-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#item;
#[cfg(any(
    any(feature = "item-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#item::*;
#[cfg(any(
    any(
        feature = "item-condition-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#item_condition;
#[cfg(any(
    any(
        feature = "item-condition-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#item_condition::*;
#[cfg(any(
    any(
        feature = "item-defect-return-fees-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#item_defect_return_fees;
#[cfg(any(
    any(
        feature = "item-defect-return-fees-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#item_defect_return_fees::*;
#[cfg(any(
    any(
        feature = "item-defect-return-label-source-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#item_defect_return_label_source;
#[cfg(any(
    any(
        feature = "item-defect-return-label-source-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#item_defect_return_label_source::*;
#[cfg(any(
    any(
        feature = "item-defect-return-shipping-fees-amount-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#item_defect_return_shipping_fees_amount;
#[cfg(any(
    any(
        feature = "item-defect-return-shipping-fees-amount-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#item_defect_return_shipping_fees_amount::*;
#[cfg(any(
    any(
        feature = "item-list-element-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#item_list_element;
#[cfg(any(
    any(
        feature = "item-list-element-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#item_list_element::*;
#[cfg(any(
    any(
        feature = "item-list-order-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#item_list_order;
#[cfg(any(
    any(
        feature = "item-list-order-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#item_list_order::*;
#[cfg(any(
    any(
        feature = "item-location-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#item_location;
#[cfg(any(
    any(
        feature = "item-location-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#item_location::*;
#[cfg(any(
    any(
        feature = "item-offered-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#item_offered;
#[cfg(any(
    any(
        feature = "item-offered-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#item_offered::*;
#[cfg(any(
    any(
        feature = "item-reviewed-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#item_reviewed;
#[cfg(any(
    any(
        feature = "item-reviewed-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#item_reviewed::*;
#[cfg(any(
    any(
        feature = "item-shipped-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#item_shipped;
#[cfg(any(
    any(
        feature = "item-shipped-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#item_shipped::*;
#[cfg(any(
    any(
        feature = "itinerary-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#itinerary;
#[cfg(any(
    any(
        feature = "itinerary-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#itinerary::*;
#[cfg(any(
    any(
        feature = "iupac-name-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#iupac_name;
#[cfg(any(
    any(
        feature = "iupac-name-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#iupac_name::*;
#[cfg(any(
    any(
        feature = "job-benefits-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#job_benefits;
#[cfg(any(
    any(
        feature = "job-benefits-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#job_benefits::*;
#[cfg(any(
    any(
        feature = "job-immediate-start-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#job_immediate_start;
#[cfg(any(
    any(
        feature = "job-immediate-start-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#job_immediate_start::*;
#[cfg(any(
    any(
        feature = "job-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#job_location;
#[cfg(any(
    any(
        feature = "job-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#job_location::*;
#[cfg(any(
    any(
        feature = "job-location-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#job_location_type;
#[cfg(any(
    any(
        feature = "job-location-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#job_location_type::*;
#[cfg(any(
    any(
        feature = "job-start-date-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#job_start_date;
#[cfg(any(
    any(
        feature = "job-start-date-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#job_start_date::*;
#[cfg(any(
    any(
        feature = "job-title-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#job_title;
#[cfg(any(
    any(
        feature = "job-title-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#job_title::*;
#[cfg(any(
    any(
        feature = "jurisdiction-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#jurisdiction;
#[cfg(any(
    any(
        feature = "jurisdiction-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#jurisdiction::*;
#[cfg(any(
    any(
        feature = "keywords-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#keywords;
#[cfg(any(
    any(
        feature = "keywords-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#keywords::*;
#[cfg(any(
    any(
        feature = "known-vehicle-damages-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#known_vehicle_damages;
#[cfg(any(
    any(
        feature = "known-vehicle-damages-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#known_vehicle_damages::*;
#[cfg(any(
    any(feature = "knows-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#knows;
#[cfg(any(
    any(feature = "knows-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#knows::*;
#[cfg(any(
    any(
        feature = "knows-about-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#knows_about;
#[cfg(any(
    any(
        feature = "knows-about-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#knows_about::*;
#[cfg(any(
    any(
        feature = "knows-language-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#knows_language;
#[cfg(any(
    any(
        feature = "knows-language-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#knows_language::*;
#[cfg(any(
    any(
        feature = "label-details-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#label_details;
#[cfg(any(
    any(
        feature = "label-details-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#label_details::*;
#[cfg(any(
    any(
        feature = "landlord-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#landlord;
#[cfg(any(
    any(
        feature = "landlord-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#landlord::*;
#[cfg(any(
    any(
        feature = "language-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#language;
#[cfg(any(
    any(
        feature = "language-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#language::*;
#[cfg(any(
    any(
        feature = "last-reviewed-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#last_reviewed;
#[cfg(any(
    any(
        feature = "last-reviewed-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#last_reviewed::*;
#[cfg(any(
    any(
        feature = "latitude-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#latitude;
#[cfg(any(
    any(
        feature = "latitude-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#latitude::*;
#[cfg(any(
    any(
        feature = "layout-image-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#layout_image;
#[cfg(any(
    any(
        feature = "layout-image-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#layout_image::*;
#[cfg(any(
    any(
        feature = "learning-resource-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#learning_resource_type;
#[cfg(any(
    any(
        feature = "learning-resource-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#learning_resource_type::*;
#[cfg(any(
    any(
        feature = "lease-length-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#lease_length;
#[cfg(any(
    any(
        feature = "lease-length-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#lease_length::*;
#[cfg(any(
    any(
        feature = "legal-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#legal_name;
#[cfg(any(
    any(
        feature = "legal-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#legal_name::*;
#[cfg(any(
    any(
        feature = "legal-status-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#legal_status;
#[cfg(any(
    any(
        feature = "legal-status-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#legal_status::*;
#[cfg(any(
    any(
        feature = "legislation-applies-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#legislation_applies;
#[cfg(any(
    any(
        feature = "legislation-applies-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#legislation_applies::*;
#[cfg(any(
    any(
        feature = "legislation-changes-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#legislation_changes;
#[cfg(any(
    any(
        feature = "legislation-changes-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#legislation_changes::*;
#[cfg(any(
    any(
        feature = "legislation-consolidates-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#legislation_consolidates;
#[cfg(any(
    any(
        feature = "legislation-consolidates-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#legislation_consolidates::*;
#[cfg(any(
    any(
        feature = "legislation-date-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#legislation_date;
#[cfg(any(
    any(
        feature = "legislation-date-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#legislation_date::*;
#[cfg(any(
    any(
        feature = "legislation-date-version-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#legislation_date_version;
#[cfg(any(
    any(
        feature = "legislation-date-version-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#legislation_date_version::*;
#[cfg(any(
    any(
        feature = "legislation-identifier-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#legislation_identifier;
#[cfg(any(
    any(
        feature = "legislation-identifier-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#legislation_identifier::*;
#[cfg(any(
    any(
        feature = "legislation-jurisdiction-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#legislation_jurisdiction;
#[cfg(any(
    any(
        feature = "legislation-jurisdiction-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#legislation_jurisdiction::*;
#[cfg(any(
    any(
        feature = "legislation-legal-force-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#legislation_legal_force;
#[cfg(any(
    any(
        feature = "legislation-legal-force-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#legislation_legal_force::*;
#[cfg(any(
    any(
        feature = "legislation-legal-value-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#legislation_legal_value;
#[cfg(any(
    any(
        feature = "legislation-legal-value-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#legislation_legal_value::*;
#[cfg(any(
    any(
        feature = "legislation-passed-by-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#legislation_passed_by;
#[cfg(any(
    any(
        feature = "legislation-passed-by-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#legislation_passed_by::*;
#[cfg(any(
    any(
        feature = "legislation-responsible-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#legislation_responsible;
#[cfg(any(
    any(
        feature = "legislation-responsible-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#legislation_responsible::*;
#[cfg(any(
    any(
        feature = "legislation-transposes-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#legislation_transposes;
#[cfg(any(
    any(
        feature = "legislation-transposes-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#legislation_transposes::*;
#[cfg(any(
    any(
        feature = "legislation-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#legislation_type;
#[cfg(any(
    any(
        feature = "legislation-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#legislation_type::*;
#[cfg(any(
    any(
        feature = "lei-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#lei_code;
#[cfg(any(
    any(
        feature = "lei-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#lei_code::*;
#[cfg(any(
    any(feature = "lender-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#lender;
#[cfg(any(
    any(feature = "lender-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#lender::*;
#[cfg(any(
    any(feature = "lesser-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#lesser;
#[cfg(any(
    any(feature = "lesser-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#lesser::*;
#[cfg(any(
    any(
        feature = "lesser-or-equal-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#lesser_or_equal;
#[cfg(any(
    any(
        feature = "lesser-or-equal-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#lesser_or_equal::*;
#[cfg(any(
    any(feature = "letterer-property-schema", feature = "bib-schema-section"),
    doc
))]
mod r#letterer;
#[cfg(any(
    any(feature = "letterer-property-schema", feature = "bib-schema-section"),
    doc
))]
pub use r#letterer::*;
#[cfg(any(
    any(
        feature = "license-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#license;
#[cfg(any(
    any(
        feature = "license-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#license::*;
#[cfg(any(
    any(feature = "line-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#line;
#[cfg(any(
    any(feature = "line-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#line::*;
#[cfg(any(
    any(
        feature = "link-relationship-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#link_relationship;
#[cfg(any(
    any(
        feature = "link-relationship-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#link_relationship::*;
#[cfg(any(
    any(
        feature = "live-blog-update-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#live_blog_update;
#[cfg(any(
    any(
        feature = "live-blog-update-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#live_blog_update::*;
#[cfg(any(
    any(
        feature = "loan-mortgage-mandate-amount-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#loan_mortgage_mandate_amount;
#[cfg(any(
    any(
        feature = "loan-mortgage-mandate-amount-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#loan_mortgage_mandate_amount::*;
#[cfg(any(
    any(
        feature = "loan-payment-amount-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#loan_payment_amount;
#[cfg(any(
    any(
        feature = "loan-payment-amount-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#loan_payment_amount::*;
#[cfg(any(
    any(
        feature = "loan-payment-frequency-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#loan_payment_frequency;
#[cfg(any(
    any(
        feature = "loan-payment-frequency-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#loan_payment_frequency::*;
#[cfg(any(
    any(
        feature = "loan-repayment-form-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#loan_repayment_form;
#[cfg(any(
    any(
        feature = "loan-repayment-form-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#loan_repayment_form::*;
#[cfg(any(
    any(
        feature = "loan-term-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#loan_term;
#[cfg(any(
    any(
        feature = "loan-term-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#loan_term::*;
#[cfg(any(
    any(
        feature = "loan-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#loan_type;
#[cfg(any(
    any(
        feature = "loan-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#loan_type::*;
#[cfg(any(
    any(
        feature = "location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#location;
#[cfg(any(
    any(
        feature = "location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#location::*;
#[cfg(any(
    any(
        feature = "location-created-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#location_created;
#[cfg(any(
    any(
        feature = "location-created-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#location_created::*;
#[cfg(any(
    any(
        feature = "lodging-unit-description-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#lodging_unit_description;
#[cfg(any(
    any(
        feature = "lodging-unit-description-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#lodging_unit_description::*;
#[cfg(any(
    any(
        feature = "lodging-unit-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#lodging_unit_type;
#[cfg(any(
    any(
        feature = "lodging-unit-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#lodging_unit_type::*;
#[cfg(any(
    any(feature = "logo-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#logo;
#[cfg(any(
    any(feature = "logo-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#logo::*;
#[cfg(any(
    any(
        feature = "longitude-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#longitude;
#[cfg(any(
    any(
        feature = "longitude-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#longitude::*;
#[cfg(any(
    any(feature = "loser-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#loser;
#[cfg(any(
    any(feature = "loser-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#loser::*;
#[cfg(any(
    any(
        feature = "low-price-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#low_price;
#[cfg(any(
    any(
        feature = "low-price-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#low_price::*;
#[cfg(any(
    any(
        feature = "lyricist-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#lyricist;
#[cfg(any(
    any(
        feature = "lyricist-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#lyricist::*;
#[cfg(any(
    any(feature = "lyrics-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#lyrics;
#[cfg(any(
    any(feature = "lyrics-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#lyrics::*;
#[cfg(any(
    any(
        feature = "main-content-of-page-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#main_content_of_page;
#[cfg(any(
    any(
        feature = "main-content-of-page-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#main_content_of_page::*;
#[cfg(any(
    any(
        feature = "main-entity-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#main_entity;
#[cfg(any(
    any(
        feature = "main-entity-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#main_entity::*;
#[cfg(any(
    any(
        feature = "main-entity-of-page-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#main_entity_of_page;
#[cfg(any(
    any(
        feature = "main-entity-of-page-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#main_entity_of_page::*;
#[cfg(any(
    any(
        feature = "maintainer-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#maintainer;
#[cfg(any(
    any(
        feature = "maintainer-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#maintainer::*;
#[cfg(any(
    any(
        feature = "makes-offer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#makes_offer;
#[cfg(any(
    any(
        feature = "makes-offer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#makes_offer::*;
#[cfg(any(
    any(
        feature = "manufacturer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#manufacturer;
#[cfg(any(
    any(
        feature = "manufacturer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#manufacturer::*;
#[cfg(any(
    any(feature = "map-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#map;
#[cfg(any(
    any(feature = "map-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#map::*;
#[cfg(any(
    any(
        feature = "map-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#map_type;
#[cfg(any(
    any(
        feature = "map-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#map_type::*;
#[cfg(any(
    any(feature = "maps-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#maps;
#[cfg(any(
    any(feature = "maps-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#maps::*;
#[cfg(any(
    any(
        feature = "margin-of-error-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#margin_of_error;
#[cfg(any(
    any(
        feature = "margin-of-error-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#margin_of_error::*;
#[cfg(any(
    any(
        feature = "masthead-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#masthead;
#[cfg(any(
    any(
        feature = "masthead-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#masthead::*;
#[cfg(any(
    any(
        feature = "material-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#material;
#[cfg(any(
    any(
        feature = "material-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#material::*;
#[cfg(any(
    any(
        feature = "material-extent-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#material_extent;
#[cfg(any(
    any(
        feature = "material-extent-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#material_extent::*;
#[cfg(any(
    any(
        feature = "math-expression-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#math_expression;
#[cfg(any(
    any(
        feature = "math-expression-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#math_expression::*;
#[cfg(any(
    any(
        feature = "max-price-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#max_price;
#[cfg(any(
    any(
        feature = "max-price-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#max_price::*;
#[cfg(any(
    any(
        feature = "max-value-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#max_value;
#[cfg(any(
    any(
        feature = "max-value-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#max_value::*;
#[cfg(any(
    any(
        feature = "maximum-attendee-capacity-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#maximum_attendee_capacity;
#[cfg(any(
    any(
        feature = "maximum-attendee-capacity-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#maximum_attendee_capacity::*;
#[cfg(any(
    any(
        feature = "maximum-enrollment-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#maximum_enrollment;
#[cfg(any(
    any(
        feature = "maximum-enrollment-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#maximum_enrollment::*;
#[cfg(any(
    any(
        feature = "maximum-intake-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#maximum_intake;
#[cfg(any(
    any(
        feature = "maximum-intake-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#maximum_intake::*;
#[cfg(any(
    any(
        feature = "maximum-physical-attendee-capacity-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#maximum_physical_attendee_capacity;
#[cfg(any(
    any(
        feature = "maximum-physical-attendee-capacity-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#maximum_physical_attendee_capacity::*;
#[cfg(any(
    any(
        feature = "maximum-virtual-attendee-capacity-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#maximum_virtual_attendee_capacity;
#[cfg(any(
    any(
        feature = "maximum-virtual-attendee-capacity-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#maximum_virtual_attendee_capacity::*;
#[cfg(any(
    any(
        feature = "meal-service-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#meal_service;
#[cfg(any(
    any(
        feature = "meal-service-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#meal_service::*;
#[cfg(any(
    any(
        feature = "measured-property-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#measured_property;
#[cfg(any(
    any(
        feature = "measured-property-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#measured_property::*;
#[cfg(any(
    any(
        feature = "measurement-denominator-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#measurement_denominator;
#[cfg(any(
    any(
        feature = "measurement-denominator-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#measurement_denominator::*;
#[cfg(any(
    any(
        feature = "measurement-method-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#measurement_method;
#[cfg(any(
    any(
        feature = "measurement-method-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#measurement_method::*;
#[cfg(any(
    any(
        feature = "measurement-qualifier-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#measurement_qualifier;
#[cfg(any(
    any(
        feature = "measurement-qualifier-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#measurement_qualifier::*;
#[cfg(any(
    any(
        feature = "measurement-technique-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#measurement_technique;
#[cfg(any(
    any(
        feature = "measurement-technique-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#measurement_technique::*;
#[cfg(any(
    any(
        feature = "mechanism-of-action-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#mechanism_of_action;
#[cfg(any(
    any(
        feature = "mechanism-of-action-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#mechanism_of_action::*;
#[cfg(any(
    any(
        feature = "media-authenticity-category-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#media_authenticity_category;
#[cfg(any(
    any(
        feature = "media-authenticity-category-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#media_authenticity_category::*;
#[cfg(any(
    any(
        feature = "media-item-appearance-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#media_item_appearance;
#[cfg(any(
    any(
        feature = "media-item-appearance-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#media_item_appearance::*;
#[cfg(any(
    any(feature = "median-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#median;
#[cfg(any(
    any(feature = "median-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#median::*;
#[cfg(any(
    any(
        feature = "medical-audience-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#medical_audience;
#[cfg(any(
    any(
        feature = "medical-audience-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#medical_audience::*;
#[cfg(any(
    any(
        feature = "medical-specialty-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#medical_specialty;
#[cfg(any(
    any(
        feature = "medical-specialty-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#medical_specialty::*;
#[cfg(any(
    any(
        feature = "medicine-system-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#medicine_system;
#[cfg(any(
    any(
        feature = "medicine-system-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#medicine_system::*;
#[cfg(any(
    any(
        feature = "meets-emission-standard-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
mod r#meets_emission_standard;
#[cfg(any(
    any(
        feature = "meets-emission-standard-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
pub use r#meets_emission_standard::*;
#[cfg(any(
    any(feature = "member-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#member;
#[cfg(any(
    any(feature = "member-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#member::*;
#[cfg(any(
    any(
        feature = "member-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#member_of;
#[cfg(any(
    any(
        feature = "member-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#member_of::*;
#[cfg(any(
    any(
        feature = "members-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#members;
#[cfg(any(
    any(
        feature = "members-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#members::*;
#[cfg(any(
    any(
        feature = "membership-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#membership_number;
#[cfg(any(
    any(
        feature = "membership-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#membership_number::*;
#[cfg(any(
    any(
        feature = "membership-points-earned-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#membership_points_earned;
#[cfg(any(
    any(
        feature = "membership-points-earned-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#membership_points_earned::*;
#[cfg(any(
    any(
        feature = "memory-requirements-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#memory_requirements;
#[cfg(any(
    any(
        feature = "memory-requirements-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#memory_requirements::*;
#[cfg(any(
    any(
        feature = "mentions-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#mentions;
#[cfg(any(
    any(
        feature = "mentions-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#mentions::*;
#[cfg(any(
    any(feature = "menu-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#menu;
#[cfg(any(
    any(feature = "menu-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#menu::*;
#[cfg(any(
    any(
        feature = "menu-add-on-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#menu_add_on;
#[cfg(any(
    any(
        feature = "menu-add-on-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#menu_add_on::*;
#[cfg(any(
    any(
        feature = "merchant-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#merchant;
#[cfg(any(
    any(
        feature = "merchant-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#merchant::*;
#[cfg(any(
    any(
        feature = "merchant-return-days-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#merchant_return_days;
#[cfg(any(
    any(
        feature = "merchant-return-days-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#merchant_return_days::*;
#[cfg(any(
    any(
        feature = "merchant-return-link-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#merchant_return_link;
#[cfg(any(
    any(
        feature = "merchant-return-link-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#merchant_return_link::*;
#[cfg(any(
    any(
        feature = "message-attachment-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#message_attachment;
#[cfg(any(
    any(
        feature = "message-attachment-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#message_attachment::*;
#[cfg(any(
    any(
        feature = "mileage-from-odometer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#mileage_from_odometer;
#[cfg(any(
    any(
        feature = "mileage-from-odometer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#mileage_from_odometer::*;
#[cfg(any(
    any(
        feature = "min-price-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#min_price;
#[cfg(any(
    any(
        feature = "min-price-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#min_price::*;
#[cfg(any(
    any(
        feature = "min-value-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#min_value;
#[cfg(any(
    any(
        feature = "min-value-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#min_value::*;
#[cfg(any(
    any(
        feature = "minimum-payment-due-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#minimum_payment_due;
#[cfg(any(
    any(
        feature = "minimum-payment-due-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#minimum_payment_due::*;
#[cfg(any(
    any(
        feature = "mission-coverage-priorities-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#mission_coverage_priorities_policy;
#[cfg(any(
    any(
        feature = "mission-coverage-priorities-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#mission_coverage_priorities_policy::*;
#[cfg(any(
    any(
        feature = "mobile-url-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#mobile_url;
#[cfg(any(
    any(
        feature = "mobile-url-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#mobile_url::*;
#[cfg(any(
    any(feature = "model-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#model;
#[cfg(any(
    any(feature = "model-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#model::*;
#[cfg(any(
    any(
        feature = "model-date-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
mod r#model_date;
#[cfg(any(
    any(
        feature = "model-date-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
pub use r#model_date::*;
#[cfg(any(
    any(
        feature = "modified-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#modified_time;
#[cfg(any(
    any(
        feature = "modified-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#modified_time::*;
#[cfg(any(
    any(
        feature = "molecular-formula-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#molecular_formula;
#[cfg(any(
    any(
        feature = "molecular-formula-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#molecular_formula::*;
#[cfg(any(
    any(
        feature = "molecular-weight-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#molecular_weight;
#[cfg(any(
    any(
        feature = "molecular-weight-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#molecular_weight::*;
#[cfg(any(
    any(
        feature = "monoisotopic-molecular-weight-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#monoisotopic_molecular_weight;
#[cfg(any(
    any(
        feature = "monoisotopic-molecular-weight-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#monoisotopic_molecular_weight::*;
#[cfg(any(
    any(
        feature = "monthly-minimum-repayment-amount-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#monthly_minimum_repayment_amount;
#[cfg(any(
    any(
        feature = "monthly-minimum-repayment-amount-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#monthly_minimum_repayment_amount::*;
#[cfg(any(
    any(
        feature = "months-of-experience-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#months_of_experience;
#[cfg(any(
    any(
        feature = "months-of-experience-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#months_of_experience::*;
#[cfg(any(
    any(feature = "mpn-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#mpn;
#[cfg(any(
    any(feature = "mpn-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#mpn::*;
#[cfg(any(
    any(
        feature = "multiple-values-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#multiple_values;
#[cfg(any(
    any(
        feature = "multiple-values-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#multiple_values::*;
#[cfg(any(
    any(
        feature = "muscle-action-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#muscle_action;
#[cfg(any(
    any(
        feature = "muscle-action-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#muscle_action::*;
#[cfg(any(
    any(
        feature = "music-arrangement-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#music_arrangement;
#[cfg(any(
    any(
        feature = "music-arrangement-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#music_arrangement::*;
#[cfg(any(
    any(
        feature = "music-by-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#music_by;
#[cfg(any(
    any(
        feature = "music-by-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#music_by::*;
#[cfg(any(
    any(
        feature = "music-composition-form-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#music_composition_form;
#[cfg(any(
    any(
        feature = "music-composition-form-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#music_composition_form::*;
#[cfg(any(
    any(
        feature = "music-group-member-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#music_group_member;
#[cfg(any(
    any(
        feature = "music-group-member-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#music_group_member::*;
#[cfg(any(
    any(
        feature = "music-release-format-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#music_release_format;
#[cfg(any(
    any(
        feature = "music-release-format-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#music_release_format::*;
#[cfg(any(
    any(
        feature = "musical-key-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#musical_key;
#[cfg(any(
    any(
        feature = "musical-key-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#musical_key::*;
#[cfg(any(
    any(feature = "naics-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#naics;
#[cfg(any(
    any(feature = "naics-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#naics::*;
#[cfg(any(
    any(feature = "name-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#name;
#[cfg(any(
    any(feature = "name-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#name::*;
#[cfg(any(
    any(
        feature = "named-position-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#named_position;
#[cfg(any(
    any(
        feature = "named-position-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#named_position::*;
#[cfg(any(
    any(
        feature = "nationality-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#nationality;
#[cfg(any(
    any(
        feature = "nationality-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#nationality::*;
#[cfg(any(
    any(
        feature = "natural-progression-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#natural_progression;
#[cfg(any(
    any(
        feature = "natural-progression-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#natural_progression::*;
#[cfg(any(
    any(
        feature = "negative-notes-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#negative_notes;
#[cfg(any(
    any(
        feature = "negative-notes-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#negative_notes::*;
#[cfg(any(
    any(
        feature = "nerve-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#nerve;
#[cfg(any(
    any(
        feature = "nerve-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#nerve::*;
#[cfg(any(
    any(
        feature = "nerve-motor-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#nerve_motor;
#[cfg(any(
    any(
        feature = "nerve-motor-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#nerve_motor::*;
#[cfg(any(
    any(
        feature = "net-worth-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#net_worth;
#[cfg(any(
    any(
        feature = "net-worth-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#net_worth::*;
#[cfg(any(
    any(
        feature = "news-updates-and-guidelines-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#news_updates_and_guidelines;
#[cfg(any(
    any(
        feature = "news-updates-and-guidelines-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#news_updates_and_guidelines::*;
#[cfg(any(
    any(
        feature = "next-item-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#next_item;
#[cfg(any(
    any(
        feature = "next-item-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#next_item::*;
#[cfg(any(
    any(
        feature = "no-bylines-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#no_bylines_policy;
#[cfg(any(
    any(
        feature = "no-bylines-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#no_bylines_policy::*;
#[cfg(any(
    any(
        feature = "non-equal-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#non_equal;
#[cfg(any(
    any(
        feature = "non-equal-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#non_equal::*;
#[cfg(any(
    any(
        feature = "non-proprietary-name-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#non_proprietary_name;
#[cfg(any(
    any(
        feature = "non-proprietary-name-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#non_proprietary_name::*;
#[cfg(any(
    any(
        feature = "nonprofit-status-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#nonprofit_status;
#[cfg(any(
    any(
        feature = "nonprofit-status-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#nonprofit_status::*;
#[cfg(any(
    any(
        feature = "normal-range-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#normal_range;
#[cfg(any(
    any(
        feature = "normal-range-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#normal_range::*;
#[cfg(any(
    any(feature = "nsn-property-schema", feature = "pending-schema-section"),
    doc
))]
mod r#nsn;
#[cfg(any(
    any(feature = "nsn-property-schema", feature = "pending-schema-section"),
    doc
))]
pub use r#nsn::*;
#[cfg(any(
    any(
        feature = "num-adults-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#num_adults;
#[cfg(any(
    any(
        feature = "num-adults-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#num_adults::*;
#[cfg(any(
    any(
        feature = "num-children-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#num_children;
#[cfg(any(
    any(
        feature = "num-children-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#num_children::*;
#[cfg(any(
    any(
        feature = "num-constraints-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#num_constraints;
#[cfg(any(
    any(
        feature = "num-constraints-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#num_constraints::*;
#[cfg(any(
    any(
        feature = "num-tracks-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#num_tracks;
#[cfg(any(
    any(
        feature = "num-tracks-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#num_tracks::*;
#[cfg(any(
    any(
        feature = "number-of-accommodation-units-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#number_of_accommodation_units;
#[cfg(any(
    any(
        feature = "number-of-accommodation-units-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#number_of_accommodation_units::*;
#[cfg(any(
    any(
        feature = "number-of-airbags-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#number_of_airbags;
#[cfg(any(
    any(
        feature = "number-of-airbags-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#number_of_airbags::*;
#[cfg(any(
    any(
        feature = "number-of-available-accommodation-units-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#number_of_available_accommodation_units;
#[cfg(any(
    any(
        feature = "number-of-available-accommodation-units-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#number_of_available_accommodation_units::*;
#[cfg(any(
    any(
        feature = "number-of-axles-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#number_of_axles;
#[cfg(any(
    any(
        feature = "number-of-axles-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#number_of_axles::*;
#[cfg(any(
    any(
        feature = "number-of-bathrooms-total-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#number_of_bathrooms_total;
#[cfg(any(
    any(
        feature = "number-of-bathrooms-total-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#number_of_bathrooms_total::*;
#[cfg(any(
    any(
        feature = "number-of-bedrooms-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#number_of_bedrooms;
#[cfg(any(
    any(
        feature = "number-of-bedrooms-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#number_of_bedrooms::*;
#[cfg(any(
    any(
        feature = "number-of-beds-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#number_of_beds;
#[cfg(any(
    any(
        feature = "number-of-beds-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#number_of_beds::*;
#[cfg(any(
    any(
        feature = "number-of-credits-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#number_of_credits;
#[cfg(any(
    any(
        feature = "number-of-credits-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#number_of_credits::*;
#[cfg(any(
    any(
        feature = "number-of-doors-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#number_of_doors;
#[cfg(any(
    any(
        feature = "number-of-doors-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#number_of_doors::*;
#[cfg(any(
    any(
        feature = "number-of-employees-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#number_of_employees;
#[cfg(any(
    any(
        feature = "number-of-employees-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#number_of_employees::*;
#[cfg(any(
    any(
        feature = "number-of-episodes-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#number_of_episodes;
#[cfg(any(
    any(
        feature = "number-of-episodes-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#number_of_episodes::*;
#[cfg(any(
    any(
        feature = "number-of-forward-gears-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#number_of_forward_gears;
#[cfg(any(
    any(
        feature = "number-of-forward-gears-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#number_of_forward_gears::*;
#[cfg(any(
    any(
        feature = "number-of-full-bathrooms-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#number_of_full_bathrooms;
#[cfg(any(
    any(
        feature = "number-of-full-bathrooms-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#number_of_full_bathrooms::*;
#[cfg(any(
    any(
        feature = "number-of-items-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#number_of_items;
#[cfg(any(
    any(
        feature = "number-of-items-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#number_of_items::*;
#[cfg(any(
    any(
        feature = "number-of-loan-payments-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#number_of_loan_payments;
#[cfg(any(
    any(
        feature = "number-of-loan-payments-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#number_of_loan_payments::*;
#[cfg(any(
    any(
        feature = "number-of-pages-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#number_of_pages;
#[cfg(any(
    any(
        feature = "number-of-pages-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#number_of_pages::*;
#[cfg(any(
    any(
        feature = "number-of-partial-bathrooms-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#number_of_partial_bathrooms;
#[cfg(any(
    any(
        feature = "number-of-partial-bathrooms-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#number_of_partial_bathrooms::*;
#[cfg(any(
    any(
        feature = "number-of-players-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#number_of_players;
#[cfg(any(
    any(
        feature = "number-of-players-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#number_of_players::*;
#[cfg(any(
    any(
        feature = "number-of-previous-owners-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#number_of_previous_owners;
#[cfg(any(
    any(
        feature = "number-of-previous-owners-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#number_of_previous_owners::*;
#[cfg(any(
    any(
        feature = "number-of-rooms-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#number_of_rooms;
#[cfg(any(
    any(
        feature = "number-of-rooms-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#number_of_rooms::*;
#[cfg(any(
    any(
        feature = "number-of-seasons-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#number_of_seasons;
#[cfg(any(
    any(
        feature = "number-of-seasons-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#number_of_seasons::*;
#[cfg(any(
    any(
        feature = "numbered-position-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#numbered_position;
#[cfg(any(
    any(
        feature = "numbered-position-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#numbered_position::*;
#[cfg(any(
    any(
        feature = "nutrition-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#nutrition;
#[cfg(any(
    any(
        feature = "nutrition-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#nutrition::*;
#[cfg(any(
    any(feature = "object-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#object;
#[cfg(any(
    any(feature = "object-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#object::*;
#[cfg(any(
    any(
        feature = "observation-about-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#observation_about;
#[cfg(any(
    any(
        feature = "observation-about-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#observation_about::*;
#[cfg(any(
    any(
        feature = "observation-date-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#observation_date;
#[cfg(any(
    any(
        feature = "observation-date-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#observation_date::*;
#[cfg(any(
    any(
        feature = "observation-period-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#observation_period;
#[cfg(any(
    any(
        feature = "observation-period-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#observation_period::*;
#[cfg(any(
    any(
        feature = "occupancy-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#occupancy;
#[cfg(any(
    any(
        feature = "occupancy-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#occupancy::*;
#[cfg(any(
    any(
        feature = "occupation-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#occupation_location;
#[cfg(any(
    any(
        feature = "occupation-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#occupation_location::*;
#[cfg(any(
    any(
        feature = "occupational-category-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#occupational_category;
#[cfg(any(
    any(
        feature = "occupational-category-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#occupational_category::*;
#[cfg(any(
    any(
        feature = "occupational-credential-awarded-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#occupational_credential_awarded;
#[cfg(any(
    any(
        feature = "occupational-credential-awarded-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#occupational_credential_awarded::*;
#[cfg(any(
    any(
        feature = "offer-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#offer_count;
#[cfg(any(
    any(
        feature = "offer-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#offer_count::*;
#[cfg(any(
    any(
        feature = "offered-by-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#offered_by;
#[cfg(any(
    any(
        feature = "offered-by-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#offered_by::*;
#[cfg(any(
    any(feature = "offers-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#offers;
#[cfg(any(
    any(feature = "offers-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#offers::*;
#[cfg(any(
    any(
        feature = "offers-prescription-by-mail-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#offers_prescription_by_mail;
#[cfg(any(
    any(
        feature = "offers-prescription-by-mail-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#offers_prescription_by_mail::*;
#[cfg(any(
    any(
        feature = "opening-hours-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#opening_hours;
#[cfg(any(
    any(
        feature = "opening-hours-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#opening_hours::*;
#[cfg(any(
    any(
        feature = "opening-hours-specification-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#opening_hours_specification;
#[cfg(any(
    any(
        feature = "opening-hours-specification-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#opening_hours_specification::*;
#[cfg(any(
    any(feature = "opens-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#opens;
#[cfg(any(
    any(feature = "opens-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#opens::*;
#[cfg(any(
    any(
        feature = "operating-system-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#operating_system;
#[cfg(any(
    any(
        feature = "operating-system-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#operating_system::*;
#[cfg(any(
    any(
        feature = "opponent-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#opponent;
#[cfg(any(
    any(
        feature = "opponent-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#opponent::*;
#[cfg(any(
    any(feature = "option-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#option;
#[cfg(any(
    any(feature = "option-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#option::*;
#[cfg(any(
    any(
        feature = "order-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#order_date;
#[cfg(any(
    any(
        feature = "order-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#order_date::*;
#[cfg(any(
    any(
        feature = "order-delivery-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#order_delivery;
#[cfg(any(
    any(
        feature = "order-delivery-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#order_delivery::*;
#[cfg(any(
    any(
        feature = "order-item-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#order_item_number;
#[cfg(any(
    any(
        feature = "order-item-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#order_item_number::*;
#[cfg(any(
    any(
        feature = "order-item-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#order_item_status;
#[cfg(any(
    any(
        feature = "order-item-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#order_item_status::*;
#[cfg(any(
    any(
        feature = "order-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#order_number;
#[cfg(any(
    any(
        feature = "order-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#order_number::*;
#[cfg(any(
    any(
        feature = "order-quantity-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#order_quantity;
#[cfg(any(
    any(
        feature = "order-quantity-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#order_quantity::*;
#[cfg(any(
    any(
        feature = "order-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#order_status;
#[cfg(any(
    any(
        feature = "order-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#order_status::*;
#[cfg(any(
    any(
        feature = "ordered-item-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#ordered_item;
#[cfg(any(
    any(
        feature = "ordered-item-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#ordered_item::*;
#[cfg(any(
    any(
        feature = "organizer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#organizer;
#[cfg(any(
    any(
        feature = "organizer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#organizer::*;
#[cfg(any(
    any(
        feature = "origin-address-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#origin_address;
#[cfg(any(
    any(
        feature = "origin-address-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#origin_address::*;
#[cfg(any(
    any(
        feature = "original-media-context-description-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#original_media_context_description;
#[cfg(any(
    any(
        feature = "original-media-context-description-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#original_media_context_description::*;
#[cfg(any(
    any(
        feature = "original-media-link-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#original_media_link;
#[cfg(any(
    any(
        feature = "original-media-link-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#original_media_link::*;
#[cfg(any(
    any(
        feature = "originates-from-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#originates_from;
#[cfg(any(
    any(
        feature = "originates-from-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#originates_from::*;
#[cfg(any(
    any(
        feature = "overdosage-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#overdosage;
#[cfg(any(
    any(
        feature = "overdosage-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#overdosage::*;
#[cfg(any(
    any(
        feature = "owned-from-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#owned_from;
#[cfg(any(
    any(
        feature = "owned-from-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#owned_from::*;
#[cfg(any(
    any(
        feature = "owned-through-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#owned_through;
#[cfg(any(
    any(
        feature = "owned-through-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#owned_through::*;
#[cfg(any(
    any(
        feature = "ownership-funding-info-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#ownership_funding_info;
#[cfg(any(
    any(
        feature = "ownership-funding-info-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#ownership_funding_info::*;
#[cfg(any(
    any(feature = "owns-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#owns;
#[cfg(any(
    any(feature = "owns-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#owns::*;
#[cfg(any(
    any(
        feature = "page-end-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#page_end;
#[cfg(any(
    any(
        feature = "page-end-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#page_end::*;
#[cfg(any(
    any(
        feature = "page-start-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#page_start;
#[cfg(any(
    any(
        feature = "page-start-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#page_start::*;
#[cfg(any(
    any(
        feature = "pagination-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#pagination;
#[cfg(any(
    any(
        feature = "pagination-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#pagination::*;
#[cfg(any(
    any(feature = "parent-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#parent;
#[cfg(any(
    any(feature = "parent-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#parent::*;
#[cfg(any(
    any(
        feature = "parent-item-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#parent_item;
#[cfg(any(
    any(
        feature = "parent-item-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#parent_item::*;
#[cfg(any(
    any(
        feature = "parent-organization-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#parent_organization;
#[cfg(any(
    any(
        feature = "parent-organization-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#parent_organization::*;
#[cfg(any(
    any(
        feature = "parent-service-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#parent_service;
#[cfg(any(
    any(
        feature = "parent-service-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#parent_service::*;
#[cfg(any(
    any(
        feature = "parent-taxon-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#parent_taxon;
#[cfg(any(
    any(
        feature = "parent-taxon-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#parent_taxon::*;
#[cfg(any(
    any(
        feature = "parents-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#parents;
#[cfg(any(
    any(
        feature = "parents-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#parents::*;
#[cfg(any(
    any(
        feature = "part-of-episode-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#part_of_episode;
#[cfg(any(
    any(
        feature = "part-of-episode-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#part_of_episode::*;
#[cfg(any(
    any(
        feature = "part-of-invoice-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#part_of_invoice;
#[cfg(any(
    any(
        feature = "part-of-invoice-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#part_of_invoice::*;
#[cfg(any(
    any(
        feature = "part-of-order-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#part_of_order;
#[cfg(any(
    any(
        feature = "part-of-order-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#part_of_order::*;
#[cfg(any(
    any(
        feature = "part-of-season-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#part_of_season;
#[cfg(any(
    any(
        feature = "part-of-season-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#part_of_season::*;
#[cfg(any(
    any(
        feature = "part-of-series-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#part_of_series;
#[cfg(any(
    any(
        feature = "part-of-series-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#part_of_series::*;
#[cfg(any(
    any(
        feature = "part-of-system-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#part_of_system;
#[cfg(any(
    any(
        feature = "part-of-system-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#part_of_system::*;
#[cfg(any(
    any(
        feature = "part-of-trip-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#part_of_trip;
#[cfg(any(
    any(
        feature = "part-of-trip-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#part_of_trip::*;
#[cfg(any(
    any(
        feature = "part-of-tv-series-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#part_of_tv_series;
#[cfg(any(
    any(
        feature = "part-of-tv-series-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#part_of_tv_series::*;
#[cfg(any(
    any(
        feature = "participant-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#participant;
#[cfg(any(
    any(
        feature = "participant-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#participant::*;
#[cfg(any(
    any(
        feature = "party-size-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#party_size;
#[cfg(any(
    any(
        feature = "party-size-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#party_size::*;
#[cfg(any(
    any(
        feature = "passenger-priority-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#passenger_priority_status;
#[cfg(any(
    any(
        feature = "passenger-priority-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#passenger_priority_status::*;
#[cfg(any(
    any(
        feature = "passenger-sequence-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#passenger_sequence_number;
#[cfg(any(
    any(
        feature = "passenger-sequence-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#passenger_sequence_number::*;
#[cfg(any(
    any(
        feature = "pathophysiology-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#pathophysiology;
#[cfg(any(
    any(
        feature = "pathophysiology-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#pathophysiology::*;
#[cfg(any(
    any(
        feature = "pattern-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#pattern;
#[cfg(any(
    any(
        feature = "pattern-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#pattern::*;
#[cfg(any(
    any(feature = "payload-property-schema", feature = "auto-schema-section"),
    doc
))]
mod r#payload;
#[cfg(any(
    any(feature = "payload-property-schema", feature = "auto-schema-section"),
    doc
))]
pub use r#payload::*;
#[cfg(any(
    any(
        feature = "payment-accepted-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#payment_accepted;
#[cfg(any(
    any(
        feature = "payment-accepted-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#payment_accepted::*;
#[cfg(any(
    any(
        feature = "payment-due-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#payment_due;
#[cfg(any(
    any(
        feature = "payment-due-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#payment_due::*;
#[cfg(any(
    any(
        feature = "payment-due-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#payment_due_date;
#[cfg(any(
    any(
        feature = "payment-due-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#payment_due_date::*;
#[cfg(any(
    any(
        feature = "payment-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#payment_method;
#[cfg(any(
    any(
        feature = "payment-method-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#payment_method::*;
#[cfg(any(
    any(
        feature = "payment-method-id-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#payment_method_id;
#[cfg(any(
    any(
        feature = "payment-method-id-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#payment_method_id::*;
#[cfg(any(
    any(
        feature = "payment-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#payment_status;
#[cfg(any(
    any(
        feature = "payment-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#payment_status::*;
#[cfg(any(
    any(
        feature = "payment-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#payment_url;
#[cfg(any(
    any(
        feature = "payment-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#payment_url::*;
#[cfg(any(
    any(feature = "penciler-property-schema", feature = "bib-schema-section"),
    doc
))]
mod r#penciler;
#[cfg(any(
    any(feature = "penciler-property-schema", feature = "bib-schema-section"),
    doc
))]
pub use r#penciler::*;
#[cfg(any(
    any(
        feature = "percentile-10-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#percentile_10;
#[cfg(any(
    any(
        feature = "percentile-10-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#percentile_10::*;
#[cfg(any(
    any(
        feature = "percentile-25-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#percentile_25;
#[cfg(any(
    any(
        feature = "percentile-25-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#percentile_25::*;
#[cfg(any(
    any(
        feature = "percentile-75-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#percentile_75;
#[cfg(any(
    any(
        feature = "percentile-75-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#percentile_75::*;
#[cfg(any(
    any(
        feature = "percentile-90-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#percentile_90;
#[cfg(any(
    any(
        feature = "percentile-90-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#percentile_90::*;
#[cfg(any(
    any(
        feature = "perform-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#perform_time;
#[cfg(any(
    any(
        feature = "perform-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#perform_time::*;
#[cfg(any(
    any(
        feature = "performer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#performer;
#[cfg(any(
    any(
        feature = "performer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#performer::*;
#[cfg(any(
    any(
        feature = "performer-in-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#performer_in;
#[cfg(any(
    any(
        feature = "performer-in-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#performer_in::*;
#[cfg(any(
    any(
        feature = "performers-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#performers;
#[cfg(any(
    any(
        feature = "performers-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#performers::*;
#[cfg(any(
    any(
        feature = "permission-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#permission_type;
#[cfg(any(
    any(
        feature = "permission-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#permission_type::*;
#[cfg(any(
    any(
        feature = "permissions-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#permissions;
#[cfg(any(
    any(
        feature = "permissions-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#permissions::*;
#[cfg(any(
    any(
        feature = "permit-audience-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#permit_audience;
#[cfg(any(
    any(
        feature = "permit-audience-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#permit_audience::*;
#[cfg(any(
    any(
        feature = "permitted-usage-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#permitted_usage;
#[cfg(any(
    any(
        feature = "permitted-usage-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#permitted_usage::*;
#[cfg(any(
    any(
        feature = "pets-allowed-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#pets_allowed;
#[cfg(any(
    any(
        feature = "pets-allowed-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#pets_allowed::*;
#[cfg(any(
    any(
        feature = "phonetic-text-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#phonetic_text;
#[cfg(any(
    any(
        feature = "phonetic-text-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#phonetic_text::*;
#[cfg(any(
    any(feature = "photo-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#photo;
#[cfg(any(
    any(feature = "photo-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#photo::*;
#[cfg(any(
    any(feature = "photos-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#photos;
#[cfg(any(
    any(feature = "photos-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#photos::*;
#[cfg(any(
    any(
        feature = "physical-requirement-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#physical_requirement;
#[cfg(any(
    any(
        feature = "physical-requirement-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#physical_requirement::*;
#[cfg(any(
    any(
        feature = "physiological-benefits-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#physiological_benefits;
#[cfg(any(
    any(
        feature = "physiological-benefits-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#physiological_benefits::*;
#[cfg(any(
    any(
        feature = "pickup-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#pickup_location;
#[cfg(any(
    any(
        feature = "pickup-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#pickup_location::*;
#[cfg(any(
    any(
        feature = "pickup-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#pickup_time;
#[cfg(any(
    any(
        feature = "pickup-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#pickup_time::*;
#[cfg(any(
    any(
        feature = "play-mode-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#play_mode;
#[cfg(any(
    any(
        feature = "play-mode-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#play_mode::*;
#[cfg(any(
    any(
        feature = "player-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#player_type;
#[cfg(any(
    any(
        feature = "player-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#player_type::*;
#[cfg(any(
    any(
        feature = "players-online-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#players_online;
#[cfg(any(
    any(
        feature = "players-online-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#players_online::*;
#[cfg(any(
    any(
        feature = "polygon-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#polygon;
#[cfg(any(
    any(
        feature = "polygon-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#polygon::*;
#[cfg(any(
    any(
        feature = "population-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#population_type;
#[cfg(any(
    any(
        feature = "population-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#population_type::*;
#[cfg(any(
    any(
        feature = "position-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#position;
#[cfg(any(
    any(
        feature = "position-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#position::*;
#[cfg(any(
    any(
        feature = "positive-notes-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#positive_notes;
#[cfg(any(
    any(
        feature = "positive-notes-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#positive_notes::*;
#[cfg(any(
    any(
        feature = "possible-complication-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#possible_complication;
#[cfg(any(
    any(
        feature = "possible-complication-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#possible_complication::*;
#[cfg(any(
    any(
        feature = "possible-treatment-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#possible_treatment;
#[cfg(any(
    any(
        feature = "possible-treatment-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#possible_treatment::*;
#[cfg(any(
    any(
        feature = "post-office-box-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#post_office_box_number;
#[cfg(any(
    any(
        feature = "post-office-box-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#post_office_box_number::*;
#[cfg(any(
    any(
        feature = "post-op-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#post_op;
#[cfg(any(
    any(
        feature = "post-op-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#post_op::*;
#[cfg(any(
    any(
        feature = "postal-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#postal_code;
#[cfg(any(
    any(
        feature = "postal-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#postal_code::*;
#[cfg(any(
    any(
        feature = "postal-code-begin-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#postal_code_begin;
#[cfg(any(
    any(
        feature = "postal-code-begin-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#postal_code_begin::*;
#[cfg(any(
    any(
        feature = "postal-code-end-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#postal_code_end;
#[cfg(any(
    any(
        feature = "postal-code-end-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#postal_code_end::*;
#[cfg(any(
    any(
        feature = "postal-code-prefix-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#postal_code_prefix;
#[cfg(any(
    any(
        feature = "postal-code-prefix-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#postal_code_prefix::*;
#[cfg(any(
    any(
        feature = "postal-code-range-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#postal_code_range;
#[cfg(any(
    any(
        feature = "postal-code-range-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#postal_code_range::*;
#[cfg(any(
    any(
        feature = "potential-action-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#potential_action;
#[cfg(any(
    any(
        feature = "potential-action-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#potential_action::*;
#[cfg(any(
    any(
        feature = "potential-use-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#potential_use;
#[cfg(any(
    any(
        feature = "potential-use-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#potential_use::*;
#[cfg(any(
    any(
        feature = "pre-op-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#pre_op;
#[cfg(any(
    any(
        feature = "pre-op-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#pre_op::*;
#[cfg(any(
    any(
        feature = "predecessor-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#predecessor_of;
#[cfg(any(
    any(
        feature = "predecessor-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#predecessor_of::*;
#[cfg(any(
    any(
        feature = "pregnancy-category-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#pregnancy_category;
#[cfg(any(
    any(
        feature = "pregnancy-category-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#pregnancy_category::*;
#[cfg(any(
    any(
        feature = "pregnancy-warning-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#pregnancy_warning;
#[cfg(any(
    any(
        feature = "pregnancy-warning-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#pregnancy_warning::*;
#[cfg(any(
    any(
        feature = "prep-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#prep_time;
#[cfg(any(
    any(
        feature = "prep-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#prep_time::*;
#[cfg(any(
    any(
        feature = "preparation-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#preparation;
#[cfg(any(
    any(
        feature = "preparation-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#preparation::*;
#[cfg(any(
    any(
        feature = "prescribing-info-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#prescribing_info;
#[cfg(any(
    any(
        feature = "prescribing-info-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#prescribing_info::*;
#[cfg(any(
    any(
        feature = "prescription-status-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#prescription_status;
#[cfg(any(
    any(
        feature = "prescription-status-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#prescription_status::*;
#[cfg(any(
    any(
        feature = "previous-item-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#previous_item;
#[cfg(any(
    any(
        feature = "previous-item-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#previous_item::*;
#[cfg(any(
    any(
        feature = "previous-start-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#previous_start_date;
#[cfg(any(
    any(
        feature = "previous-start-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#previous_start_date::*;
#[cfg(any(
    any(feature = "price-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#price;
#[cfg(any(
    any(feature = "price-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#price::*;
#[cfg(any(
    any(
        feature = "price-component-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#price_component;
#[cfg(any(
    any(
        feature = "price-component-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#price_component::*;
#[cfg(any(
    any(
        feature = "price-component-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#price_component_type;
#[cfg(any(
    any(
        feature = "price-component-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#price_component_type::*;
#[cfg(any(
    any(
        feature = "price-currency-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#price_currency;
#[cfg(any(
    any(
        feature = "price-currency-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#price_currency::*;
#[cfg(any(
    any(
        feature = "price-range-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#price_range;
#[cfg(any(
    any(
        feature = "price-range-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#price_range::*;
#[cfg(any(
    any(
        feature = "price-specification-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#price_specification;
#[cfg(any(
    any(
        feature = "price-specification-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#price_specification::*;
#[cfg(any(
    any(
        feature = "price-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#price_type;
#[cfg(any(
    any(
        feature = "price-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#price_type::*;
#[cfg(any(
    any(
        feature = "price-valid-until-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#price_valid_until;
#[cfg(any(
    any(
        feature = "price-valid-until-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#price_valid_until::*;
#[cfg(any(
    any(
        feature = "primary-image-of-page-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#primary_image_of_page;
#[cfg(any(
    any(
        feature = "primary-image-of-page-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#primary_image_of_page::*;
#[cfg(any(
    any(
        feature = "primary-prevention-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#primary_prevention;
#[cfg(any(
    any(
        feature = "primary-prevention-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#primary_prevention::*;
#[cfg(any(
    any(
        feature = "print-column-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#print_column;
#[cfg(any(
    any(
        feature = "print-column-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#print_column::*;
#[cfg(any(
    any(
        feature = "print-edition-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#print_edition;
#[cfg(any(
    any(
        feature = "print-edition-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#print_edition::*;
#[cfg(any(
    any(
        feature = "print-page-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#print_page;
#[cfg(any(
    any(
        feature = "print-page-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#print_page::*;
#[cfg(any(
    any(
        feature = "print-section-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#print_section;
#[cfg(any(
    any(
        feature = "print-section-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#print_section::*;
#[cfg(any(
    any(
        feature = "procedure-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#procedure;
#[cfg(any(
    any(
        feature = "procedure-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#procedure::*;
#[cfg(any(
    any(
        feature = "procedure-type-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#procedure_type;
#[cfg(any(
    any(
        feature = "procedure-type-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#procedure_type::*;
#[cfg(any(
    any(
        feature = "processing-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#processing_time;
#[cfg(any(
    any(
        feature = "processing-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#processing_time::*;
#[cfg(any(
    any(
        feature = "processor-requirements-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#processor_requirements;
#[cfg(any(
    any(
        feature = "processor-requirements-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#processor_requirements::*;
#[cfg(any(
    any(
        feature = "producer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#producer;
#[cfg(any(
    any(
        feature = "producer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#producer::*;
#[cfg(any(
    any(
        feature = "produces-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#produces;
#[cfg(any(
    any(
        feature = "produces-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#produces::*;
#[cfg(any(
    any(
        feature = "product-group-id-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#product_group_id;
#[cfg(any(
    any(
        feature = "product-group-id-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#product_group_id::*;
#[cfg(any(
    any(
        feature = "product-id-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#product_id;
#[cfg(any(
    any(
        feature = "product-id-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#product_id::*;
#[cfg(any(
    any(
        feature = "product-return-days-property-schema",
        feature = "attic-schema-section"
    ),
    doc
))]
mod r#product_return_days;
#[cfg(any(
    any(
        feature = "product-return-days-property-schema",
        feature = "attic-schema-section"
    ),
    doc
))]
pub use r#product_return_days::*;
#[cfg(any(
    any(
        feature = "product-return-link-property-schema",
        feature = "attic-schema-section"
    ),
    doc
))]
mod r#product_return_link;
#[cfg(any(
    any(
        feature = "product-return-link-property-schema",
        feature = "attic-schema-section"
    ),
    doc
))]
pub use r#product_return_link::*;
#[cfg(any(
    any(
        feature = "product-supported-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#product_supported;
#[cfg(any(
    any(
        feature = "product-supported-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#product_supported::*;
#[cfg(any(
    any(
        feature = "production-company-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#production_company;
#[cfg(any(
    any(
        feature = "production-company-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#production_company::*;
#[cfg(any(
    any(
        feature = "production-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#production_date;
#[cfg(any(
    any(
        feature = "production-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#production_date::*;
#[cfg(any(
    any(
        feature = "proficiency-level-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#proficiency_level;
#[cfg(any(
    any(
        feature = "proficiency-level-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#proficiency_level::*;
#[cfg(any(
    any(
        feature = "program-membership-used-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#program_membership_used;
#[cfg(any(
    any(
        feature = "program-membership-used-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#program_membership_used::*;
#[cfg(any(
    any(
        feature = "program-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#program_name;
#[cfg(any(
    any(
        feature = "program-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#program_name::*;
#[cfg(any(
    any(
        feature = "program-prerequisites-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#program_prerequisites;
#[cfg(any(
    any(
        feature = "program-prerequisites-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#program_prerequisites::*;
#[cfg(any(
    any(
        feature = "program-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#program_type;
#[cfg(any(
    any(
        feature = "program-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#program_type::*;
#[cfg(any(
    any(
        feature = "programming-language-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#programming_language;
#[cfg(any(
    any(
        feature = "programming-language-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#programming_language::*;
#[cfg(any(
    any(
        feature = "programming-model-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#programming_model;
#[cfg(any(
    any(
        feature = "programming-model-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#programming_model::*;
#[cfg(any(
    any(
        feature = "property-id-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#property_id;
#[cfg(any(
    any(
        feature = "property-id-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#property_id::*;
#[cfg(any(
    any(
        feature = "proprietary-name-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#proprietary_name;
#[cfg(any(
    any(
        feature = "proprietary-name-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#proprietary_name::*;
#[cfg(any(
    any(
        feature = "protein-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#protein_content;
#[cfg(any(
    any(
        feature = "protein-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#protein_content::*;
#[cfg(any(
    any(
        feature = "provider-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#provider;
#[cfg(any(
    any(
        feature = "provider-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#provider::*;
#[cfg(any(
    any(
        feature = "provider-mobility-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#provider_mobility;
#[cfg(any(
    any(
        feature = "provider-mobility-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#provider_mobility::*;
#[cfg(any(
    any(
        feature = "provides-broadcast-service-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#provides_broadcast_service;
#[cfg(any(
    any(
        feature = "provides-broadcast-service-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#provides_broadcast_service::*;
#[cfg(any(
    any(
        feature = "provides-service-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#provides_service;
#[cfg(any(
    any(
        feature = "provides-service-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#provides_service::*;
#[cfg(any(
    any(
        feature = "public-access-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#public_access;
#[cfg(any(
    any(
        feature = "public-access-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#public_access::*;
#[cfg(any(
    any(
        feature = "public-transport-closures-info-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#public_transport_closures_info;
#[cfg(any(
    any(
        feature = "public-transport-closures-info-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#public_transport_closures_info::*;
#[cfg(any(
    any(
        feature = "publication-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#publication;
#[cfg(any(
    any(
        feature = "publication-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#publication::*;
#[cfg(any(
    any(
        feature = "publication-type-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#publication_type;
#[cfg(any(
    any(
        feature = "publication-type-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#publication_type::*;
#[cfg(any(
    any(
        feature = "published-by-property-schema",
        feature = "bib-schema-section"
    ),
    doc
))]
mod r#published_by;
#[cfg(any(
    any(
        feature = "published-by-property-schema",
        feature = "bib-schema-section"
    ),
    doc
))]
pub use r#published_by::*;
#[cfg(any(
    any(
        feature = "published-on-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#published_on;
#[cfg(any(
    any(
        feature = "published-on-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#published_on::*;
#[cfg(any(
    any(
        feature = "publisher-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#publisher;
#[cfg(any(
    any(
        feature = "publisher-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#publisher::*;
#[cfg(any(
    any(
        feature = "publisher-imprint-property-schema",
        feature = "bib-schema-section"
    ),
    doc
))]
mod r#publisher_imprint;
#[cfg(any(
    any(
        feature = "publisher-imprint-property-schema",
        feature = "bib-schema-section"
    ),
    doc
))]
pub use r#publisher_imprint::*;
#[cfg(any(
    any(
        feature = "publishing-principles-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#publishing_principles;
#[cfg(any(
    any(
        feature = "publishing-principles-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#publishing_principles::*;
#[cfg(any(
    any(
        feature = "purchase-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#purchase_date;
#[cfg(any(
    any(
        feature = "purchase-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#purchase_date::*;
#[cfg(any(
    any(
        feature = "qualifications-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#qualifications;
#[cfg(any(
    any(
        feature = "qualifications-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#qualifications::*;
#[cfg(any(
    any(
        feature = "quarantine-guidelines-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#quarantine_guidelines;
#[cfg(any(
    any(
        feature = "quarantine-guidelines-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#quarantine_guidelines::*;
#[cfg(any(
    any(feature = "query-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#query;
#[cfg(any(
    any(feature = "query-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#query::*;
#[cfg(any(
    any(feature = "quest-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#quest;
#[cfg(any(
    any(feature = "quest-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#quest::*;
#[cfg(any(
    any(
        feature = "question-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#question;
#[cfg(any(
    any(
        feature = "question-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#question::*;
#[cfg(any(
    any(
        feature = "range-includes-property-schema",
        feature = "meta-schema-section"
    ),
    doc
))]
mod r#range_includes;
#[cfg(any(
    any(
        feature = "range-includes-property-schema",
        feature = "meta-schema-section"
    ),
    doc
))]
pub use r#range_includes::*;
#[cfg(any(
    any(
        feature = "rating-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#rating_count;
#[cfg(any(
    any(
        feature = "rating-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#rating_count::*;
#[cfg(any(
    any(
        feature = "rating-explanation-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#rating_explanation;
#[cfg(any(
    any(
        feature = "rating-explanation-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#rating_explanation::*;
#[cfg(any(
    any(
        feature = "rating-value-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#rating_value;
#[cfg(any(
    any(
        feature = "rating-value-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#rating_value::*;
#[cfg(any(
    any(feature = "read-by-property-schema", feature = "bib-schema-section"),
    doc
))]
mod r#read_by;
#[cfg(any(
    any(feature = "read-by-property-schema", feature = "bib-schema-section"),
    doc
))]
pub use r#read_by::*;
#[cfg(any(
    any(
        feature = "readonly-value-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#readonly_value;
#[cfg(any(
    any(
        feature = "readonly-value-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#readonly_value::*;
#[cfg(any(
    any(
        feature = "real-estate-agent-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#real_estate_agent;
#[cfg(any(
    any(
        feature = "real-estate-agent-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#real_estate_agent::*;
#[cfg(any(
    any(feature = "recipe-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#recipe;
#[cfg(any(
    any(feature = "recipe-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#recipe::*;
#[cfg(any(
    any(
        feature = "recipe-category-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#recipe_category;
#[cfg(any(
    any(
        feature = "recipe-category-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#recipe_category::*;
#[cfg(any(
    any(
        feature = "recipe-cuisine-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#recipe_cuisine;
#[cfg(any(
    any(
        feature = "recipe-cuisine-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#recipe_cuisine::*;
#[cfg(any(
    any(
        feature = "recipe-ingredient-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#recipe_ingredient;
#[cfg(any(
    any(
        feature = "recipe-ingredient-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#recipe_ingredient::*;
#[cfg(any(
    any(
        feature = "recipe-instructions-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#recipe_instructions;
#[cfg(any(
    any(
        feature = "recipe-instructions-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#recipe_instructions::*;
#[cfg(any(
    any(
        feature = "recipe-yield-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#recipe_yield;
#[cfg(any(
    any(
        feature = "recipe-yield-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#recipe_yield::*;
#[cfg(any(
    any(
        feature = "recipient-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#recipient;
#[cfg(any(
    any(
        feature = "recipient-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#recipient::*;
#[cfg(any(
    any(
        feature = "recognized-by-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#recognized_by;
#[cfg(any(
    any(
        feature = "recognized-by-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#recognized_by::*;
#[cfg(any(
    any(
        feature = "recognizing-authority-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#recognizing_authority;
#[cfg(any(
    any(
        feature = "recognizing-authority-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#recognizing_authority::*;
#[cfg(any(
    any(
        feature = "recommendation-strength-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#recommendation_strength;
#[cfg(any(
    any(
        feature = "recommendation-strength-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#recommendation_strength::*;
#[cfg(any(
    any(
        feature = "recommended-intake-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#recommended_intake;
#[cfg(any(
    any(
        feature = "recommended-intake-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#recommended_intake::*;
#[cfg(any(
    any(
        feature = "record-label-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#record_label;
#[cfg(any(
    any(
        feature = "record-label-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#record_label::*;
#[cfg(any(
    any(
        feature = "recorded-as-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#recorded_as;
#[cfg(any(
    any(
        feature = "recorded-as-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#recorded_as::*;
#[cfg(any(
    any(
        feature = "recorded-at-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#recorded_at;
#[cfg(any(
    any(
        feature = "recorded-at-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#recorded_at::*;
#[cfg(any(
    any(
        feature = "recorded-in-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#recorded_in;
#[cfg(any(
    any(
        feature = "recorded-in-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#recorded_in::*;
#[cfg(any(
    any(
        feature = "recording-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#recording_of;
#[cfg(any(
    any(
        feature = "recording-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#recording_of::*;
#[cfg(any(
    any(
        feature = "recourse-loan-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#recourse_loan;
#[cfg(any(
    any(
        feature = "recourse-loan-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#recourse_loan::*;
#[cfg(any(
    any(
        feature = "reference-quantity-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#reference_quantity;
#[cfg(any(
    any(
        feature = "reference-quantity-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#reference_quantity::*;
#[cfg(any(
    any(
        feature = "references-order-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#references_order;
#[cfg(any(
    any(
        feature = "references-order-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#references_order::*;
#[cfg(any(
    any(
        feature = "refund-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#refund_type;
#[cfg(any(
    any(
        feature = "refund-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#refund_type::*;
#[cfg(any(
    any(
        feature = "region-drained-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#region_drained;
#[cfg(any(
    any(
        feature = "region-drained-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#region_drained::*;
#[cfg(any(
    any(
        feature = "regions-allowed-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#regions_allowed;
#[cfg(any(
    any(
        feature = "regions-allowed-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#regions_allowed::*;
#[cfg(any(
    any(
        feature = "related-anatomy-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#related_anatomy;
#[cfg(any(
    any(
        feature = "related-anatomy-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#related_anatomy::*;
#[cfg(any(
    any(
        feature = "related-condition-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#related_condition;
#[cfg(any(
    any(
        feature = "related-condition-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#related_condition::*;
#[cfg(any(
    any(
        feature = "related-drug-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#related_drug;
#[cfg(any(
    any(
        feature = "related-drug-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#related_drug::*;
#[cfg(any(
    any(
        feature = "related-link-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#related_link;
#[cfg(any(
    any(
        feature = "related-link-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#related_link::*;
#[cfg(any(
    any(
        feature = "related-structure-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#related_structure;
#[cfg(any(
    any(
        feature = "related-structure-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#related_structure::*;
#[cfg(any(
    any(
        feature = "related-therapy-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#related_therapy;
#[cfg(any(
    any(
        feature = "related-therapy-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#related_therapy::*;
#[cfg(any(
    any(
        feature = "related-to-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#related_to;
#[cfg(any(
    any(
        feature = "related-to-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#related_to::*;
#[cfg(any(
    any(
        feature = "release-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#release_date;
#[cfg(any(
    any(
        feature = "release-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#release_date::*;
#[cfg(any(
    any(
        feature = "release-notes-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#release_notes;
#[cfg(any(
    any(
        feature = "release-notes-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#release_notes::*;
#[cfg(any(
    any(
        feature = "release-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#release_of;
#[cfg(any(
    any(
        feature = "release-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#release_of::*;
#[cfg(any(
    any(
        feature = "released-event-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#released_event;
#[cfg(any(
    any(
        feature = "released-event-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#released_event::*;
#[cfg(any(
    any(
        feature = "relevant-occupation-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#relevant_occupation;
#[cfg(any(
    any(
        feature = "relevant-occupation-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#relevant_occupation::*;
#[cfg(any(
    any(
        feature = "relevant-specialty-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#relevant_specialty;
#[cfg(any(
    any(
        feature = "relevant-specialty-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#relevant_specialty::*;
#[cfg(any(
    any(
        feature = "remaining-attendee-capacity-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#remaining_attendee_capacity;
#[cfg(any(
    any(
        feature = "remaining-attendee-capacity-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#remaining_attendee_capacity::*;
#[cfg(any(
    any(
        feature = "renegotiable-loan-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#renegotiable_loan;
#[cfg(any(
    any(
        feature = "renegotiable-loan-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#renegotiable_loan::*;
#[cfg(any(
    any(
        feature = "repeat-count-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#repeat_count;
#[cfg(any(
    any(
        feature = "repeat-count-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#repeat_count::*;
#[cfg(any(
    any(
        feature = "repeat-frequency-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#repeat_frequency;
#[cfg(any(
    any(
        feature = "repeat-frequency-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#repeat_frequency::*;
#[cfg(any(
    any(
        feature = "repetitions-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#repetitions;
#[cfg(any(
    any(
        feature = "repetitions-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#repetitions::*;
#[cfg(any(
    any(
        feature = "replacee-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#replacee;
#[cfg(any(
    any(
        feature = "replacee-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#replacee::*;
#[cfg(any(
    any(
        feature = "replacer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#replacer;
#[cfg(any(
    any(
        feature = "replacer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#replacer::*;
#[cfg(any(
    any(
        feature = "reply-to-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#reply_to_url;
#[cfg(any(
    any(
        feature = "reply-to-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#reply_to_url::*;
#[cfg(any(
    any(
        feature = "report-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#report_number;
#[cfg(any(
    any(
        feature = "report-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#report_number::*;
#[cfg(any(
    any(
        feature = "representative-of-page-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#representative_of_page;
#[cfg(any(
    any(
        feature = "representative-of-page-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#representative_of_page::*;
#[cfg(any(
    any(
        feature = "required-collateral-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#required_collateral;
#[cfg(any(
    any(
        feature = "required-collateral-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#required_collateral::*;
#[cfg(any(
    any(
        feature = "required-gender-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#required_gender;
#[cfg(any(
    any(
        feature = "required-gender-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#required_gender::*;
#[cfg(any(
    any(
        feature = "required-max-age-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#required_max_age;
#[cfg(any(
    any(
        feature = "required-max-age-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#required_max_age::*;
#[cfg(any(
    any(
        feature = "required-min-age-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#required_min_age;
#[cfg(any(
    any(
        feature = "required-min-age-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#required_min_age::*;
#[cfg(any(
    any(
        feature = "required-quantity-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#required_quantity;
#[cfg(any(
    any(
        feature = "required-quantity-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#required_quantity::*;
#[cfg(any(
    any(
        feature = "requirements-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#requirements;
#[cfg(any(
    any(
        feature = "requirements-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#requirements::*;
#[cfg(any(
    any(
        feature = "requires-subscription-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#requires_subscription;
#[cfg(any(
    any(
        feature = "requires-subscription-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#requires_subscription::*;
#[cfg(any(
    any(
        feature = "reservation-for-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#reservation_for;
#[cfg(any(
    any(
        feature = "reservation-for-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#reservation_for::*;
#[cfg(any(
    any(
        feature = "reservation-id-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#reservation_id;
#[cfg(any(
    any(
        feature = "reservation-id-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#reservation_id::*;
#[cfg(any(
    any(
        feature = "reservation-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#reservation_status;
#[cfg(any(
    any(
        feature = "reservation-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#reservation_status::*;
#[cfg(any(
    any(
        feature = "reserved-ticket-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#reserved_ticket;
#[cfg(any(
    any(
        feature = "reserved-ticket-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#reserved_ticket::*;
#[cfg(any(
    any(
        feature = "responsibilities-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#responsibilities;
#[cfg(any(
    any(
        feature = "responsibilities-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#responsibilities::*;
#[cfg(any(
    any(
        feature = "rest-periods-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#rest_periods;
#[cfg(any(
    any(
        feature = "rest-periods-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#rest_periods::*;
#[cfg(any(
    any(
        feature = "restocking-fee-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#restocking_fee;
#[cfg(any(
    any(
        feature = "restocking-fee-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#restocking_fee::*;
#[cfg(any(
    any(feature = "result-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#result;
#[cfg(any(
    any(feature = "result-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#result::*;
#[cfg(any(
    any(
        feature = "result-comment-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#result_comment;
#[cfg(any(
    any(
        feature = "result-comment-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#result_comment::*;
#[cfg(any(
    any(
        feature = "result-review-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#result_review;
#[cfg(any(
    any(
        feature = "result-review-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#result_review::*;
#[cfg(any(
    any(
        feature = "return-fees-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#return_fees;
#[cfg(any(
    any(
        feature = "return-fees-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#return_fees::*;
#[cfg(any(
    any(
        feature = "return-label-source-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#return_label_source;
#[cfg(any(
    any(
        feature = "return-label-source-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#return_label_source::*;
#[cfg(any(
    any(
        feature = "return-method-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#return_method;
#[cfg(any(
    any(
        feature = "return-method-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#return_method::*;
#[cfg(any(
    any(
        feature = "return-policy-category-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#return_policy_category;
#[cfg(any(
    any(
        feature = "return-policy-category-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#return_policy_category::*;
#[cfg(any(
    any(
        feature = "return-policy-country-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#return_policy_country;
#[cfg(any(
    any(
        feature = "return-policy-country-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#return_policy_country::*;
#[cfg(any(
    any(
        feature = "return-policy-seasonal-override-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#return_policy_seasonal_override;
#[cfg(any(
    any(
        feature = "return-policy-seasonal-override-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#return_policy_seasonal_override::*;
#[cfg(any(
    any(
        feature = "return-shipping-fees-amount-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#return_shipping_fees_amount;
#[cfg(any(
    any(
        feature = "return-shipping-fees-amount-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#return_shipping_fees_amount::*;
#[cfg(any(
    any(feature = "review-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#review;
#[cfg(any(
    any(feature = "review-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#review::*;
#[cfg(any(
    any(
        feature = "review-aspect-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#review_aspect;
#[cfg(any(
    any(
        feature = "review-aspect-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#review_aspect::*;
#[cfg(any(
    any(
        feature = "review-body-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#review_body;
#[cfg(any(
    any(
        feature = "review-body-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#review_body::*;
#[cfg(any(
    any(
        feature = "review-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#review_count;
#[cfg(any(
    any(
        feature = "review-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#review_count::*;
#[cfg(any(
    any(
        feature = "review-rating-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#review_rating;
#[cfg(any(
    any(
        feature = "review-rating-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#review_rating::*;
#[cfg(any(
    any(
        feature = "reviewed-by-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#reviewed_by;
#[cfg(any(
    any(
        feature = "reviewed-by-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#reviewed_by::*;
#[cfg(any(
    any(
        feature = "reviews-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#reviews;
#[cfg(any(
    any(
        feature = "reviews-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#reviews::*;
#[cfg(any(
    any(
        feature = "risk-factor-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#risk_factor;
#[cfg(any(
    any(
        feature = "risk-factor-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#risk_factor::*;
#[cfg(any(
    any(
        feature = "risks-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#risks;
#[cfg(any(
    any(
        feature = "risks-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#risks::*;
#[cfg(any(
    any(
        feature = "role-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#role_name;
#[cfg(any(
    any(
        feature = "role-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#role_name::*;
#[cfg(any(
    any(feature = "roof-load-property-schema", feature = "auto-schema-section"),
    doc
))]
mod r#roof_load;
#[cfg(any(
    any(feature = "roof-load-property-schema", feature = "auto-schema-section"),
    doc
))]
pub use r#roof_load::*;
#[cfg(any(
    any(
        feature = "rsvp-response-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#rsvp_response;
#[cfg(any(
    any(
        feature = "rsvp-response-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#rsvp_response::*;
#[cfg(any(
    any(
        feature = "runs-to-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#runs_to;
#[cfg(any(
    any(
        feature = "runs-to-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#runs_to::*;
#[cfg(any(
    any(
        feature = "runtime-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#runtime;
#[cfg(any(
    any(
        feature = "runtime-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#runtime::*;
#[cfg(any(
    any(
        feature = "runtime-platform-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#runtime_platform;
#[cfg(any(
    any(
        feature = "runtime-platform-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#runtime_platform::*;
#[cfg(any(
    any(feature = "rxcui-property-schema", feature = "pending-schema-section"),
    doc
))]
mod r#rxcui;
#[cfg(any(
    any(feature = "rxcui-property-schema", feature = "pending-schema-section"),
    doc
))]
pub use r#rxcui::*;
#[cfg(any(
    any(
        feature = "safety-consideration-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#safety_consideration;
#[cfg(any(
    any(
        feature = "safety-consideration-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#safety_consideration::*;
#[cfg(any(
    any(
        feature = "salary-currency-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#salary_currency;
#[cfg(any(
    any(
        feature = "salary-currency-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#salary_currency::*;
#[cfg(any(
    any(
        feature = "salary-upon-completion-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#salary_upon_completion;
#[cfg(any(
    any(
        feature = "salary-upon-completion-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#salary_upon_completion::*;
#[cfg(any(
    any(
        feature = "same-as-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#same_as;
#[cfg(any(
    any(
        feature = "same-as-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#same_as::*;
#[cfg(any(
    any(
        feature = "sample-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#sample_type;
#[cfg(any(
    any(
        feature = "sample-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#sample_type::*;
#[cfg(any(
    any(
        feature = "saturated-fat-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#saturated_fat_content;
#[cfg(any(
    any(
        feature = "saturated-fat-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#saturated_fat_content::*;
#[cfg(any(
    any(
        feature = "schedule-timezone-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#schedule_timezone;
#[cfg(any(
    any(
        feature = "schedule-timezone-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#schedule_timezone::*;
#[cfg(any(
    any(
        feature = "scheduled-payment-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#scheduled_payment_date;
#[cfg(any(
    any(
        feature = "scheduled-payment-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#scheduled_payment_date::*;
#[cfg(any(
    any(
        feature = "scheduled-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#scheduled_time;
#[cfg(any(
    any(
        feature = "scheduled-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#scheduled_time::*;
#[cfg(any(
    any(
        feature = "schema-version-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#schema_version;
#[cfg(any(
    any(
        feature = "schema-version-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#schema_version::*;
#[cfg(any(
    any(
        feature = "school-closures-info-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#school_closures_info;
#[cfg(any(
    any(
        feature = "school-closures-info-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#school_closures_info::*;
#[cfg(any(
    any(
        feature = "screen-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#screen_count;
#[cfg(any(
    any(
        feature = "screen-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#screen_count::*;
#[cfg(any(
    any(
        feature = "screenshot-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#screenshot;
#[cfg(any(
    any(
        feature = "screenshot-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#screenshot::*;
#[cfg(any(
    any(
        feature = "sd-date-published-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#sd_date_published;
#[cfg(any(
    any(
        feature = "sd-date-published-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#sd_date_published::*;
#[cfg(any(
    any(
        feature = "sd-license-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#sd_license;
#[cfg(any(
    any(
        feature = "sd-license-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#sd_license::*;
#[cfg(any(
    any(
        feature = "sd-publisher-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#sd_publisher;
#[cfg(any(
    any(
        feature = "sd-publisher-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#sd_publisher::*;
#[cfg(any(
    any(feature = "season-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#season;
#[cfg(any(
    any(feature = "season-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#season::*;
#[cfg(any(
    any(
        feature = "season-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#season_number;
#[cfg(any(
    any(
        feature = "season-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#season_number::*;
#[cfg(any(
    any(
        feature = "seasons-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#seasons;
#[cfg(any(
    any(
        feature = "seasons-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#seasons::*;
#[cfg(any(
    any(
        feature = "seat-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#seat_number;
#[cfg(any(
    any(
        feature = "seat-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#seat_number::*;
#[cfg(any(
    any(
        feature = "seat-row-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#seat_row;
#[cfg(any(
    any(
        feature = "seat-row-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#seat_row::*;
#[cfg(any(
    any(
        feature = "seat-section-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#seat_section;
#[cfg(any(
    any(
        feature = "seat-section-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#seat_section::*;
#[cfg(any(
    any(
        feature = "seating-capacity-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
mod r#seating_capacity;
#[cfg(any(
    any(
        feature = "seating-capacity-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
pub use r#seating_capacity::*;
#[cfg(any(
    any(
        feature = "seating-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#seating_type;
#[cfg(any(
    any(
        feature = "seating-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#seating_type::*;
#[cfg(any(
    any(
        feature = "secondary-prevention-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#secondary_prevention;
#[cfg(any(
    any(
        feature = "secondary-prevention-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#secondary_prevention::*;
#[cfg(any(
    any(
        feature = "security-clearance-requirement-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#security_clearance_requirement;
#[cfg(any(
    any(
        feature = "security-clearance-requirement-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#security_clearance_requirement::*;
#[cfg(any(
    any(
        feature = "security-screening-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#security_screening;
#[cfg(any(
    any(
        feature = "security-screening-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#security_screening::*;
#[cfg(any(
    any(feature = "seeks-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#seeks;
#[cfg(any(
    any(feature = "seeks-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#seeks::*;
#[cfg(any(
    any(feature = "seller-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#seller;
#[cfg(any(
    any(feature = "seller-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#seller::*;
#[cfg(any(
    any(feature = "sender-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#sender;
#[cfg(any(
    any(feature = "sender-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#sender::*;
#[cfg(any(
    any(
        feature = "sensory-requirement-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#sensory_requirement;
#[cfg(any(
    any(
        feature = "sensory-requirement-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#sensory_requirement::*;
#[cfg(any(
    any(
        feature = "sensory-unit-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#sensory_unit;
#[cfg(any(
    any(
        feature = "sensory-unit-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#sensory_unit::*;
#[cfg(any(
    any(
        feature = "serial-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#serial_number;
#[cfg(any(
    any(
        feature = "serial-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#serial_number::*;
#[cfg(any(
    any(
        feature = "serious-adverse-outcome-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#serious_adverse_outcome;
#[cfg(any(
    any(
        feature = "serious-adverse-outcome-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#serious_adverse_outcome::*;
#[cfg(any(
    any(
        feature = "server-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#server_status;
#[cfg(any(
    any(
        feature = "server-status-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#server_status::*;
#[cfg(any(
    any(
        feature = "serves-cuisine-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#serves_cuisine;
#[cfg(any(
    any(
        feature = "serves-cuisine-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#serves_cuisine::*;
#[cfg(any(
    any(
        feature = "service-area-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#service_area;
#[cfg(any(
    any(
        feature = "service-area-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#service_area::*;
#[cfg(any(
    any(
        feature = "service-audience-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#service_audience;
#[cfg(any(
    any(
        feature = "service-audience-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#service_audience::*;
#[cfg(any(
    any(
        feature = "service-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#service_location;
#[cfg(any(
    any(
        feature = "service-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#service_location::*;
#[cfg(any(
    any(
        feature = "service-operator-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#service_operator;
#[cfg(any(
    any(
        feature = "service-operator-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#service_operator::*;
#[cfg(any(
    any(
        feature = "service-output-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#service_output;
#[cfg(any(
    any(
        feature = "service-output-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#service_output::*;
#[cfg(any(
    any(
        feature = "service-phone-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#service_phone;
#[cfg(any(
    any(
        feature = "service-phone-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#service_phone::*;
#[cfg(any(
    any(
        feature = "service-postal-address-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#service_postal_address;
#[cfg(any(
    any(
        feature = "service-postal-address-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#service_postal_address::*;
#[cfg(any(
    any(
        feature = "service-sms-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#service_sms_number;
#[cfg(any(
    any(
        feature = "service-sms-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#service_sms_number::*;
#[cfg(any(
    any(
        feature = "service-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#service_type;
#[cfg(any(
    any(
        feature = "service-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#service_type::*;
#[cfg(any(
    any(
        feature = "service-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#service_url;
#[cfg(any(
    any(
        feature = "service-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#service_url::*;
#[cfg(any(
    any(
        feature = "serving-size-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#serving_size;
#[cfg(any(
    any(
        feature = "serving-size-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#serving_size::*;
#[cfg(any(
    any(
        feature = "sha-256-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#sha_256;
#[cfg(any(
    any(
        feature = "sha-256-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#sha_256::*;
#[cfg(any(
    any(
        feature = "shared-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#shared_content;
#[cfg(any(
    any(
        feature = "shared-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#shared_content::*;
#[cfg(any(
    any(
        feature = "shipping-destination-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#shipping_destination;
#[cfg(any(
    any(
        feature = "shipping-destination-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#shipping_destination::*;
#[cfg(any(
    any(
        feature = "shipping-details-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#shipping_details;
#[cfg(any(
    any(
        feature = "shipping-details-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#shipping_details::*;
#[cfg(any(
    any(
        feature = "shipping-label-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#shipping_label;
#[cfg(any(
    any(
        feature = "shipping-label-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#shipping_label::*;
#[cfg(any(
    any(
        feature = "shipping-origin-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#shipping_origin;
#[cfg(any(
    any(
        feature = "shipping-origin-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#shipping_origin::*;
#[cfg(any(
    any(
        feature = "shipping-rate-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#shipping_rate;
#[cfg(any(
    any(
        feature = "shipping-rate-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#shipping_rate::*;
#[cfg(any(
    any(
        feature = "shipping-settings-link-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#shipping_settings_link;
#[cfg(any(
    any(
        feature = "shipping-settings-link-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#shipping_settings_link::*;
#[cfg(any(
    any(
        feature = "sibling-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#sibling;
#[cfg(any(
    any(
        feature = "sibling-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#sibling::*;
#[cfg(any(
    any(
        feature = "siblings-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#siblings;
#[cfg(any(
    any(
        feature = "siblings-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#siblings::*;
#[cfg(any(
    any(
        feature = "sign-detected-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#sign_detected;
#[cfg(any(
    any(
        feature = "sign-detected-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#sign_detected::*;
#[cfg(any(
    any(
        feature = "sign-or-symptom-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#sign_or_symptom;
#[cfg(any(
    any(
        feature = "sign-or-symptom-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#sign_or_symptom::*;
#[cfg(any(
    any(
        feature = "significance-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#significance;
#[cfg(any(
    any(
        feature = "significance-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#significance::*;
#[cfg(any(
    any(
        feature = "significant-link-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#significant_link;
#[cfg(any(
    any(
        feature = "significant-link-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#significant_link::*;
#[cfg(any(
    any(
        feature = "significant-links-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#significant_links;
#[cfg(any(
    any(
        feature = "significant-links-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#significant_links::*;
#[cfg(any(
    any(feature = "size-property-schema", feature = "pending-schema-section"),
    doc
))]
mod r#size;
#[cfg(any(
    any(feature = "size-property-schema", feature = "pending-schema-section"),
    doc
))]
pub use r#size::*;
#[cfg(any(
    any(
        feature = "size-group-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#size_group;
#[cfg(any(
    any(
        feature = "size-group-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#size_group::*;
#[cfg(any(
    any(
        feature = "size-system-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#size_system;
#[cfg(any(
    any(
        feature = "size-system-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#size_system::*;
#[cfg(any(
    any(feature = "skills-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#skills;
#[cfg(any(
    any(feature = "skills-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#skills::*;
#[cfg(any(
    any(feature = "sku-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#sku;
#[cfg(any(
    any(feature = "sku-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#sku::*;
#[cfg(any(
    any(feature = "slogan-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#slogan;
#[cfg(any(
    any(feature = "slogan-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#slogan::*;
#[cfg(any(
    any(feature = "smiles-property-schema", feature = "pending-schema-section"),
    doc
))]
mod r#smiles;
#[cfg(any(
    any(feature = "smiles-property-schema", feature = "pending-schema-section"),
    doc
))]
pub use r#smiles::*;
#[cfg(any(
    any(
        feature = "smoking-allowed-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#smoking_allowed;
#[cfg(any(
    any(
        feature = "smoking-allowed-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#smoking_allowed::*;
#[cfg(any(
    any(
        feature = "sodium-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#sodium_content;
#[cfg(any(
    any(
        feature = "sodium-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#sodium_content::*;
#[cfg(any(
    any(
        feature = "software-add-on-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#software_add_on;
#[cfg(any(
    any(
        feature = "software-add-on-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#software_add_on::*;
#[cfg(any(
    any(
        feature = "software-help-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#software_help;
#[cfg(any(
    any(
        feature = "software-help-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#software_help::*;
#[cfg(any(
    any(
        feature = "software-requirements-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#software_requirements;
#[cfg(any(
    any(
        feature = "software-requirements-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#software_requirements::*;
#[cfg(any(
    any(
        feature = "software-version-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#software_version;
#[cfg(any(
    any(
        feature = "software-version-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#software_version::*;
#[cfg(any(
    any(
        feature = "source-organization-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#source_organization;
#[cfg(any(
    any(
        feature = "source-organization-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#source_organization::*;
#[cfg(any(
    any(
        feature = "sourced-from-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#sourced_from;
#[cfg(any(
    any(
        feature = "sourced-from-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#sourced_from::*;
#[cfg(any(
    any(
        feature = "spatial-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#spatial;
#[cfg(any(
    any(
        feature = "spatial-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#spatial::*;
#[cfg(any(
    any(
        feature = "spatial-coverage-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#spatial_coverage;
#[cfg(any(
    any(
        feature = "spatial-coverage-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#spatial_coverage::*;
#[cfg(any(
    any(
        feature = "speakable-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#speakable;
#[cfg(any(
    any(
        feature = "speakable-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#speakable::*;
#[cfg(any(
    any(
        feature = "special-commitments-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#special_commitments;
#[cfg(any(
    any(
        feature = "special-commitments-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#special_commitments::*;
#[cfg(any(
    any(
        feature = "special-opening-hours-specification-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#special_opening_hours_specification;
#[cfg(any(
    any(
        feature = "special-opening-hours-specification-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#special_opening_hours_specification::*;
#[cfg(any(
    any(
        feature = "specialty-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#specialty;
#[cfg(any(
    any(
        feature = "specialty-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#specialty::*;
#[cfg(any(
    any(
        feature = "speech-to-text-markup-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#speech_to_text_markup;
#[cfg(any(
    any(
        feature = "speech-to-text-markup-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#speech_to_text_markup::*;
#[cfg(any(
    any(feature = "speed-property-schema", feature = "auto-schema-section"),
    doc
))]
mod r#speed;
#[cfg(any(
    any(feature = "speed-property-schema", feature = "auto-schema-section"),
    doc
))]
pub use r#speed::*;
#[cfg(any(
    any(
        feature = "spoken-by-character-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#spoken_by_character;
#[cfg(any(
    any(
        feature = "spoken-by-character-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#spoken_by_character::*;
#[cfg(any(
    any(
        feature = "sponsor-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#sponsor;
#[cfg(any(
    any(
        feature = "sponsor-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#sponsor::*;
#[cfg(any(
    any(feature = "sport-property-schema", feature = "pending-schema-section"),
    doc
))]
mod r#sport;
#[cfg(any(
    any(feature = "sport-property-schema", feature = "pending-schema-section"),
    doc
))]
pub use r#sport::*;
#[cfg(any(
    any(
        feature = "sports-activity-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#sports_activity_location;
#[cfg(any(
    any(
        feature = "sports-activity-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#sports_activity_location::*;
#[cfg(any(
    any(
        feature = "sports-event-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#sports_event;
#[cfg(any(
    any(
        feature = "sports-event-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#sports_event::*;
#[cfg(any(
    any(
        feature = "sports-team-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#sports_team;
#[cfg(any(
    any(
        feature = "sports-team-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#sports_team::*;
#[cfg(any(
    any(feature = "spouse-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#spouse;
#[cfg(any(
    any(feature = "spouse-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#spouse::*;
#[cfg(any(
    any(
        feature = "stage-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#stage;
#[cfg(any(
    any(
        feature = "stage-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#stage::*;
#[cfg(any(
    any(
        feature = "stage-as-number-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#stage_as_number;
#[cfg(any(
    any(
        feature = "stage-as-number-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#stage_as_number::*;
#[cfg(any(
    any(
        feature = "star-rating-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#star_rating;
#[cfg(any(
    any(
        feature = "star-rating-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#star_rating::*;
#[cfg(any(
    any(
        feature = "start-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#start_date;
#[cfg(any(
    any(
        feature = "start-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#start_date::*;
#[cfg(any(
    any(
        feature = "start-offset-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#start_offset;
#[cfg(any(
    any(
        feature = "start-offset-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#start_offset::*;
#[cfg(any(
    any(
        feature = "start-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#start_time;
#[cfg(any(
    any(
        feature = "start-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#start_time::*;
#[cfg(any(
    any(
        feature = "stat-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#stat_type;
#[cfg(any(
    any(
        feature = "stat-type-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#stat_type::*;
#[cfg(any(
    any(
        feature = "status-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#status;
#[cfg(any(
    any(
        feature = "status-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#status::*;
#[cfg(any(
    any(
        feature = "steering-position-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#steering_position;
#[cfg(any(
    any(
        feature = "steering-position-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#steering_position::*;
#[cfg(any(
    any(feature = "step-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#step;
#[cfg(any(
    any(feature = "step-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#step::*;
#[cfg(any(
    any(
        feature = "step-value-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#step_value;
#[cfg(any(
    any(
        feature = "step-value-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#step_value::*;
#[cfg(any(
    any(feature = "steps-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#steps;
#[cfg(any(
    any(feature = "steps-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#steps::*;
#[cfg(any(
    any(
        feature = "storage-requirements-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#storage_requirements;
#[cfg(any(
    any(
        feature = "storage-requirements-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#storage_requirements::*;
#[cfg(any(
    any(
        feature = "street-address-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#street_address;
#[cfg(any(
    any(
        feature = "street-address-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#street_address::*;
#[cfg(any(
    any(
        feature = "strength-unit-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#strength_unit;
#[cfg(any(
    any(
        feature = "strength-unit-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#strength_unit::*;
#[cfg(any(
    any(
        feature = "strength-value-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#strength_value;
#[cfg(any(
    any(
        feature = "strength-value-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#strength_value::*;
#[cfg(any(
    any(
        feature = "structural-class-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#structural_class;
#[cfg(any(
    any(
        feature = "structural-class-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#structural_class::*;
#[cfg(any(
    any(
        feature = "study-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#study;
#[cfg(any(
    any(
        feature = "study-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#study::*;
#[cfg(any(
    any(
        feature = "study-design-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#study_design;
#[cfg(any(
    any(
        feature = "study-design-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#study_design::*;
#[cfg(any(
    any(
        feature = "study-location-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#study_location;
#[cfg(any(
    any(
        feature = "study-location-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#study_location::*;
#[cfg(any(
    any(
        feature = "study-subject-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#study_subject;
#[cfg(any(
    any(
        feature = "study-subject-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#study_subject::*;
#[cfg(any(
    any(
        feature = "stupid-property-property-schema",
        feature = "attic-schema-section"
    ),
    doc
))]
mod r#stupid_property;
#[cfg(any(
    any(
        feature = "stupid-property-property-schema",
        feature = "attic-schema-section"
    ),
    doc
))]
pub use r#stupid_property::*;
#[cfg(any(
    any(
        feature = "sub-event-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#sub_event;
#[cfg(any(
    any(
        feature = "sub-event-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#sub_event::*;
#[cfg(any(
    any(
        feature = "sub-events-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#sub_events;
#[cfg(any(
    any(
        feature = "sub-events-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#sub_events::*;
#[cfg(any(
    any(
        feature = "sub-organization-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#sub_organization;
#[cfg(any(
    any(
        feature = "sub-organization-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#sub_organization::*;
#[cfg(any(
    any(
        feature = "sub-reservation-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#sub_reservation;
#[cfg(any(
    any(
        feature = "sub-reservation-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#sub_reservation::*;
#[cfg(any(
    any(
        feature = "sub-stage-suffix-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#sub_stage_suffix;
#[cfg(any(
    any(
        feature = "sub-stage-suffix-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#sub_stage_suffix::*;
#[cfg(any(
    any(
        feature = "sub-structure-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#sub_structure;
#[cfg(any(
    any(
        feature = "sub-structure-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#sub_structure::*;
#[cfg(any(
    any(
        feature = "sub-test-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#sub_test;
#[cfg(any(
    any(
        feature = "sub-test-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#sub_test::*;
#[cfg(any(
    any(
        feature = "sub-trip-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#sub_trip;
#[cfg(any(
    any(
        feature = "sub-trip-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#sub_trip::*;
#[cfg(any(
    any(
        feature = "subject-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#subject_of;
#[cfg(any(
    any(
        feature = "subject-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#subject_of::*;
#[cfg(any(
    any(
        feature = "subtitle-language-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#subtitle_language;
#[cfg(any(
    any(
        feature = "subtitle-language-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#subtitle_language::*;
#[cfg(any(
    any(
        feature = "successor-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#successor_of;
#[cfg(any(
    any(
        feature = "successor-of-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#successor_of::*;
#[cfg(any(
    any(
        feature = "sugar-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#sugar_content;
#[cfg(any(
    any(
        feature = "sugar-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#sugar_content::*;
#[cfg(any(
    any(
        feature = "suggested-age-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#suggested_age;
#[cfg(any(
    any(
        feature = "suggested-age-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#suggested_age::*;
#[cfg(any(
    any(
        feature = "suggested-answer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#suggested_answer;
#[cfg(any(
    any(
        feature = "suggested-answer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#suggested_answer::*;
#[cfg(any(
    any(
        feature = "suggested-gender-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#suggested_gender;
#[cfg(any(
    any(
        feature = "suggested-gender-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#suggested_gender::*;
#[cfg(any(
    any(
        feature = "suggested-max-age-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#suggested_max_age;
#[cfg(any(
    any(
        feature = "suggested-max-age-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#suggested_max_age::*;
#[cfg(any(
    any(
        feature = "suggested-measurement-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#suggested_measurement;
#[cfg(any(
    any(
        feature = "suggested-measurement-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#suggested_measurement::*;
#[cfg(any(
    any(
        feature = "suggested-min-age-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#suggested_min_age;
#[cfg(any(
    any(
        feature = "suggested-min-age-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#suggested_min_age::*;
#[cfg(any(
    any(
        feature = "suitable-for-diet-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#suitable_for_diet;
#[cfg(any(
    any(
        feature = "suitable-for-diet-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#suitable_for_diet::*;
#[cfg(any(
    any(
        feature = "super-event-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#super_event;
#[cfg(any(
    any(
        feature = "super-event-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#super_event::*;
#[cfg(any(
    any(
        feature = "superseded-by-property-schema",
        feature = "meta-schema-section"
    ),
    doc
))]
mod r#superseded_by;
#[cfg(any(
    any(
        feature = "superseded-by-property-schema",
        feature = "meta-schema-section"
    ),
    doc
))]
pub use r#superseded_by::*;
#[cfg(any(
    any(feature = "supply-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#supply;
#[cfg(any(
    any(feature = "supply-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#supply::*;
#[cfg(any(
    any(
        feature = "supply-to-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#supply_to;
#[cfg(any(
    any(
        feature = "supply-to-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#supply_to::*;
#[cfg(any(
    any(
        feature = "supporting-data-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#supporting_data;
#[cfg(any(
    any(
        feature = "supporting-data-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#supporting_data::*;
#[cfg(any(
    any(
        feature = "surface-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#surface;
#[cfg(any(
    any(
        feature = "surface-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#surface::*;
#[cfg(any(
    any(
        feature = "syllabus-sections-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#syllabus_sections;
#[cfg(any(
    any(
        feature = "syllabus-sections-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#syllabus_sections::*;
#[cfg(any(
    any(feature = "target-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#target;
#[cfg(any(
    any(feature = "target-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#target::*;
#[cfg(any(
    any(
        feature = "target-collection-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#target_collection;
#[cfg(any(
    any(
        feature = "target-collection-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#target_collection::*;
#[cfg(any(
    any(
        feature = "target-description-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#target_description;
#[cfg(any(
    any(
        feature = "target-description-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#target_description::*;
#[cfg(any(
    any(
        feature = "target-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#target_name;
#[cfg(any(
    any(
        feature = "target-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#target_name::*;
#[cfg(any(
    any(
        feature = "target-platform-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#target_platform;
#[cfg(any(
    any(
        feature = "target-platform-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#target_platform::*;
#[cfg(any(
    any(
        feature = "target-population-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#target_population;
#[cfg(any(
    any(
        feature = "target-population-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#target_population::*;
#[cfg(any(
    any(
        feature = "target-product-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#target_product;
#[cfg(any(
    any(
        feature = "target-product-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#target_product::*;
#[cfg(any(
    any(
        feature = "target-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#target_url;
#[cfg(any(
    any(
        feature = "target-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#target_url::*;
#[cfg(any(
    any(feature = "tax-id-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#tax_id;
#[cfg(any(
    any(feature = "tax-id-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#tax_id::*;
#[cfg(any(
    any(
        feature = "taxon-rank-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#taxon_rank;
#[cfg(any(
    any(
        feature = "taxon-rank-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#taxon_rank::*;
#[cfg(any(
    any(
        feature = "taxonomic-range-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#taxonomic_range;
#[cfg(any(
    any(
        feature = "taxonomic-range-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#taxonomic_range::*;
#[cfg(any(
    any(
        feature = "teaches-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#teaches;
#[cfg(any(
    any(
        feature = "teaches-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#teaches::*;
#[cfg(any(
    any(
        feature = "telephone-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#telephone;
#[cfg(any(
    any(
        feature = "telephone-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#telephone::*;
#[cfg(any(
    any(
        feature = "temporal-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#temporal;
#[cfg(any(
    any(
        feature = "temporal-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#temporal::*;
#[cfg(any(
    any(
        feature = "temporal-coverage-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#temporal_coverage;
#[cfg(any(
    any(
        feature = "temporal-coverage-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#temporal_coverage::*;
#[cfg(any(
    any(
        feature = "term-code-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#term_code;
#[cfg(any(
    any(
        feature = "term-code-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#term_code::*;
#[cfg(any(
    any(
        feature = "term-duration-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#term_duration;
#[cfg(any(
    any(
        feature = "term-duration-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#term_duration::*;
#[cfg(any(
    any(
        feature = "terms-of-service-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#terms_of_service;
#[cfg(any(
    any(
        feature = "terms-of-service-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#terms_of_service::*;
#[cfg(any(
    any(
        feature = "terms-per-year-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#terms_per_year;
#[cfg(any(
    any(
        feature = "terms-per-year-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#terms_per_year::*;
#[cfg(any(
    any(feature = "text-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#text;
#[cfg(any(
    any(feature = "text-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#text::*;
#[cfg(any(
    any(
        feature = "text-value-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#text_value;
#[cfg(any(
    any(
        feature = "text-value-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#text_value::*;
#[cfg(any(
    any(
        feature = "thumbnail-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#thumbnail;
#[cfg(any(
    any(
        feature = "thumbnail-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#thumbnail::*;
#[cfg(any(
    any(
        feature = "thumbnail-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#thumbnail_url;
#[cfg(any(
    any(
        feature = "thumbnail-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#thumbnail_url::*;
#[cfg(any(
    any(
        feature = "ticker-symbol-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#ticker_symbol;
#[cfg(any(
    any(
        feature = "ticker-symbol-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#ticker_symbol::*;
#[cfg(any(
    any(
        feature = "ticket-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#ticket_number;
#[cfg(any(
    any(
        feature = "ticket-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#ticket_number::*;
#[cfg(any(
    any(
        feature = "ticket-token-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#ticket_token;
#[cfg(any(
    any(
        feature = "ticket-token-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#ticket_token::*;
#[cfg(any(
    any(
        feature = "ticketed-seat-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#ticketed_seat;
#[cfg(any(
    any(
        feature = "ticketed-seat-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#ticketed_seat::*;
#[cfg(any(
    any(
        feature = "time-of-day-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#time_of_day;
#[cfg(any(
    any(
        feature = "time-of-day-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#time_of_day::*;
#[cfg(any(
    any(
        feature = "time-required-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#time_required;
#[cfg(any(
    any(
        feature = "time-required-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#time_required::*;
#[cfg(any(
    any(
        feature = "time-to-complete-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#time_to_complete;
#[cfg(any(
    any(
        feature = "time-to-complete-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#time_to_complete::*;
#[cfg(any(
    any(
        feature = "tissue-sample-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#tissue_sample;
#[cfg(any(
    any(
        feature = "tissue-sample-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#tissue_sample::*;
#[cfg(any(
    any(feature = "title-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#title;
#[cfg(any(
    any(feature = "title-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#title::*;
#[cfg(any(
    any(
        feature = "title-eidr-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#title_eidr;
#[cfg(any(
    any(
        feature = "title-eidr-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#title_eidr::*;
#[cfg(any(
    any(
        feature = "to-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#to_location;
#[cfg(any(
    any(
        feature = "to-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#to_location::*;
#[cfg(any(
    any(
        feature = "to-recipient-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#to_recipient;
#[cfg(any(
    any(
        feature = "to-recipient-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#to_recipient::*;
#[cfg(any(
    any(
        feature = "toc-continuation-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#toc_continuation;
#[cfg(any(
    any(
        feature = "toc-continuation-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#toc_continuation::*;
#[cfg(any(
    any(
        feature = "toc-entry-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#toc_entry;
#[cfg(any(
    any(
        feature = "toc-entry-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#toc_entry::*;
#[cfg(any(
    any(
        feature = "tongue-weight-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
mod r#tongue_weight;
#[cfg(any(
    any(
        feature = "tongue-weight-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
pub use r#tongue_weight::*;
#[cfg(any(
    any(feature = "tool-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#tool;
#[cfg(any(
    any(feature = "tool-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#tool::*;
#[cfg(any(
    any(feature = "torque-property-schema", feature = "auto-schema-section"),
    doc
))]
mod r#torque;
#[cfg(any(
    any(feature = "torque-property-schema", feature = "auto-schema-section"),
    doc
))]
pub use r#torque::*;
#[cfg(any(
    any(
        feature = "total-historical-enrollment-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#total_historical_enrollment;
#[cfg(any(
    any(
        feature = "total-historical-enrollment-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#total_historical_enrollment::*;
#[cfg(any(
    any(
        feature = "total-job-openings-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#total_job_openings;
#[cfg(any(
    any(
        feature = "total-job-openings-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#total_job_openings::*;
#[cfg(any(
    any(
        feature = "total-payment-due-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#total_payment_due;
#[cfg(any(
    any(
        feature = "total-payment-due-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#total_payment_due::*;
#[cfg(any(
    any(
        feature = "total-price-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#total_price;
#[cfg(any(
    any(
        feature = "total-price-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#total_price::*;
#[cfg(any(
    any(
        feature = "total-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#total_time;
#[cfg(any(
    any(
        feature = "total-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#total_time::*;
#[cfg(any(
    any(
        feature = "tour-booking-page-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#tour_booking_page;
#[cfg(any(
    any(
        feature = "tour-booking-page-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#tour_booking_page::*;
#[cfg(any(
    any(
        feature = "tourist-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#tourist_type;
#[cfg(any(
    any(
        feature = "tourist-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#tourist_type::*;
#[cfg(any(
    any(feature = "track-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#track;
#[cfg(any(
    any(feature = "track-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#track::*;
#[cfg(any(
    any(
        feature = "tracking-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#tracking_number;
#[cfg(any(
    any(
        feature = "tracking-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#tracking_number::*;
#[cfg(any(
    any(
        feature = "tracking-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#tracking_url;
#[cfg(any(
    any(
        feature = "tracking-url-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#tracking_url::*;
#[cfg(any(
    any(feature = "tracks-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#tracks;
#[cfg(any(
    any(feature = "tracks-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#tracks::*;
#[cfg(any(
    any(
        feature = "trailer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#trailer;
#[cfg(any(
    any(
        feature = "trailer-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#trailer::*;
#[cfg(any(
    any(
        feature = "trailer-weight-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
mod r#trailer_weight;
#[cfg(any(
    any(
        feature = "trailer-weight-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
pub use r#trailer_weight::*;
#[cfg(any(
    any(
        feature = "train-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#train_name;
#[cfg(any(
    any(
        feature = "train-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#train_name::*;
#[cfg(any(
    any(
        feature = "train-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#train_number;
#[cfg(any(
    any(
        feature = "train-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#train_number::*;
#[cfg(any(
    any(
        feature = "training-salary-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#training_salary;
#[cfg(any(
    any(
        feature = "training-salary-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#training_salary::*;
#[cfg(any(
    any(
        feature = "trans-fat-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#trans_fat_content;
#[cfg(any(
    any(
        feature = "trans-fat-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#trans_fat_content::*;
#[cfg(any(
    any(
        feature = "transcript-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#transcript;
#[cfg(any(
    any(
        feature = "transcript-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#transcript::*;
#[cfg(any(
    any(
        feature = "transit-time-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#transit_time;
#[cfg(any(
    any(
        feature = "transit-time-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#transit_time::*;
#[cfg(any(
    any(
        feature = "transit-time-label-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#transit_time_label;
#[cfg(any(
    any(
        feature = "transit-time-label-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#transit_time_label::*;
#[cfg(any(
    any(
        feature = "translation-of-work-property-schema",
        feature = "bib-schema-section"
    ),
    doc
))]
mod r#translation_of_work;
#[cfg(any(
    any(
        feature = "translation-of-work-property-schema",
        feature = "bib-schema-section"
    ),
    doc
))]
pub use r#translation_of_work::*;
#[cfg(any(
    any(
        feature = "translator-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#translator;
#[cfg(any(
    any(
        feature = "translator-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#translator::*;
#[cfg(any(
    any(
        feature = "transmission-method-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#transmission_method;
#[cfg(any(
    any(
        feature = "transmission-method-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#transmission_method::*;
#[cfg(any(
    any(
        feature = "travel-bans-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#travel_bans;
#[cfg(any(
    any(
        feature = "travel-bans-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#travel_bans::*;
#[cfg(any(
    any(
        feature = "trial-design-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#trial_design;
#[cfg(any(
    any(
        feature = "trial-design-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#trial_design::*;
#[cfg(any(
    any(
        feature = "tributary-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#tributary;
#[cfg(any(
    any(
        feature = "tributary-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#tributary::*;
#[cfg(any(
    any(
        feature = "trip-origin-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#trip_origin;
#[cfg(any(
    any(
        feature = "trip-origin-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#trip_origin::*;
#[cfg(any(
    any(
        feature = "type-of-bed-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#type_of_bed;
#[cfg(any(
    any(
        feature = "type-of-bed-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#type_of_bed::*;
#[cfg(any(
    any(
        feature = "type-of-good-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#type_of_good;
#[cfg(any(
    any(
        feature = "type-of-good-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#type_of_good::*;
#[cfg(any(
    any(
        feature = "typical-age-range-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#typical_age_range;
#[cfg(any(
    any(
        feature = "typical-age-range-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#typical_age_range::*;
#[cfg(any(
    any(
        feature = "typical-credits-per-term-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#typical_credits_per_term;
#[cfg(any(
    any(
        feature = "typical-credits-per-term-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#typical_credits_per_term::*;
#[cfg(any(
    any(
        feature = "typical-test-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#typical_test;
#[cfg(any(
    any(
        feature = "typical-test-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#typical_test::*;
#[cfg(any(
    any(
        feature = "under-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#under_name;
#[cfg(any(
    any(
        feature = "under-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#under_name::*;
#[cfg(any(
    any(
        feature = "unit-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#unit_code;
#[cfg(any(
    any(
        feature = "unit-code-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#unit_code::*;
#[cfg(any(
    any(
        feature = "unit-text-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#unit_text;
#[cfg(any(
    any(
        feature = "unit-text-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#unit_text::*;
#[cfg(any(
    any(
        feature = "unnamed-sources-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#unnamed_sources_policy;
#[cfg(any(
    any(
        feature = "unnamed-sources-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#unnamed_sources_policy::*;
#[cfg(any(
    any(
        feature = "unsaturated-fat-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#unsaturated_fat_content;
#[cfg(any(
    any(
        feature = "unsaturated-fat-content-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#unsaturated_fat_content::*;
#[cfg(any(
    any(
        feature = "upload-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#upload_date;
#[cfg(any(
    any(
        feature = "upload-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#upload_date::*;
#[cfg(any(
    any(
        feature = "upvote-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#upvote_count;
#[cfg(any(
    any(
        feature = "upvote-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#upvote_count::*;
#[cfg(any(
    any(feature = "url-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#url;
#[cfg(any(
    any(feature = "url-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#url::*;
#[cfg(any(
    any(
        feature = "url-template-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#url_template;
#[cfg(any(
    any(
        feature = "url-template-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#url_template::*;
#[cfg(any(
    any(
        feature = "usage-info-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#usage_info;
#[cfg(any(
    any(
        feature = "usage-info-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#usage_info::*;
#[cfg(any(
    any(
        feature = "used-to-diagnose-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#used_to_diagnose;
#[cfg(any(
    any(
        feature = "used-to-diagnose-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#used_to_diagnose::*;
#[cfg(any(
    any(
        feature = "user-interaction-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#user_interaction_count;
#[cfg(any(
    any(
        feature = "user-interaction-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#user_interaction_count::*;
#[cfg(any(
    any(
        feature = "uses-device-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#uses_device;
#[cfg(any(
    any(
        feature = "uses-device-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#uses_device::*;
#[cfg(any(
    any(
        feature = "uses-health-plan-id-standard-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#uses_health_plan_id_standard;
#[cfg(any(
    any(
        feature = "uses-health-plan-id-standard-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#uses_health_plan_id_standard::*;
#[cfg(any(
    any(
        feature = "utterances-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#utterances;
#[cfg(any(
    any(
        feature = "utterances-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#utterances::*;
#[cfg(any(
    any(
        feature = "valid-for-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#valid_for;
#[cfg(any(
    any(
        feature = "valid-for-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#valid_for::*;
#[cfg(any(
    any(
        feature = "valid-from-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#valid_from;
#[cfg(any(
    any(
        feature = "valid-from-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#valid_from::*;
#[cfg(any(
    any(
        feature = "valid-in-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#valid_in;
#[cfg(any(
    any(
        feature = "valid-in-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#valid_in::*;
#[cfg(any(
    any(
        feature = "valid-through-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#valid_through;
#[cfg(any(
    any(
        feature = "valid-through-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#valid_through::*;
#[cfg(any(
    any(
        feature = "valid-until-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#valid_until;
#[cfg(any(
    any(
        feature = "valid-until-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#valid_until::*;
#[cfg(any(
    any(feature = "value-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#value;
#[cfg(any(
    any(feature = "value-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#value::*;
#[cfg(any(
    any(
        feature = "value-added-tax-included-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#value_added_tax_included;
#[cfg(any(
    any(
        feature = "value-added-tax-included-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#value_added_tax_included::*;
#[cfg(any(
    any(
        feature = "value-max-length-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#value_max_length;
#[cfg(any(
    any(
        feature = "value-max-length-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#value_max_length::*;
#[cfg(any(
    any(
        feature = "value-min-length-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#value_min_length;
#[cfg(any(
    any(
        feature = "value-min-length-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#value_min_length::*;
#[cfg(any(
    any(
        feature = "value-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#value_name;
#[cfg(any(
    any(
        feature = "value-name-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#value_name::*;
#[cfg(any(
    any(
        feature = "value-pattern-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#value_pattern;
#[cfg(any(
    any(
        feature = "value-pattern-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#value_pattern::*;
#[cfg(any(
    any(
        feature = "value-reference-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#value_reference;
#[cfg(any(
    any(
        feature = "value-reference-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#value_reference::*;
#[cfg(any(
    any(
        feature = "value-required-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#value_required;
#[cfg(any(
    any(
        feature = "value-required-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#value_required::*;
#[cfg(any(
    any(
        feature = "variable-measured-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#variable_measured;
#[cfg(any(
    any(
        feature = "variable-measured-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#variable_measured::*;
#[cfg(any(
    any(
        feature = "variables-measured-property-schema",
        feature = "attic-schema-section"
    ),
    doc
))]
mod r#variables_measured;
#[cfg(any(
    any(
        feature = "variables-measured-property-schema",
        feature = "attic-schema-section"
    ),
    doc
))]
pub use r#variables_measured::*;
#[cfg(any(
    any(
        feature = "variant-cover-property-schema",
        feature = "bib-schema-section"
    ),
    doc
))]
mod r#variant_cover;
#[cfg(any(
    any(
        feature = "variant-cover-property-schema",
        feature = "bib-schema-section"
    ),
    doc
))]
pub use r#variant_cover::*;
#[cfg(any(
    any(
        feature = "varies-by-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#varies_by;
#[cfg(any(
    any(
        feature = "varies-by-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#varies_by::*;
#[cfg(any(
    any(feature = "vat-id-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#vat_id;
#[cfg(any(
    any(feature = "vat-id-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#vat_id::*;
#[cfg(any(
    any(
        feature = "vehicle-configuration-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#vehicle_configuration;
#[cfg(any(
    any(
        feature = "vehicle-configuration-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#vehicle_configuration::*;
#[cfg(any(
    any(
        feature = "vehicle-engine-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#vehicle_engine;
#[cfg(any(
    any(
        feature = "vehicle-engine-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#vehicle_engine::*;
#[cfg(any(
    any(
        feature = "vehicle-identification-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#vehicle_identification_number;
#[cfg(any(
    any(
        feature = "vehicle-identification-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#vehicle_identification_number::*;
#[cfg(any(
    any(
        feature = "vehicle-interior-color-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#vehicle_interior_color;
#[cfg(any(
    any(
        feature = "vehicle-interior-color-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#vehicle_interior_color::*;
#[cfg(any(
    any(
        feature = "vehicle-interior-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#vehicle_interior_type;
#[cfg(any(
    any(
        feature = "vehicle-interior-type-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#vehicle_interior_type::*;
#[cfg(any(
    any(
        feature = "vehicle-model-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#vehicle_model_date;
#[cfg(any(
    any(
        feature = "vehicle-model-date-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#vehicle_model_date::*;
#[cfg(any(
    any(
        feature = "vehicle-seating-capacity-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#vehicle_seating_capacity;
#[cfg(any(
    any(
        feature = "vehicle-seating-capacity-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#vehicle_seating_capacity::*;
#[cfg(any(
    any(
        feature = "vehicle-special-usage-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
mod r#vehicle_special_usage;
#[cfg(any(
    any(
        feature = "vehicle-special-usage-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
pub use r#vehicle_special_usage::*;
#[cfg(any(
    any(
        feature = "vehicle-transmission-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#vehicle_transmission;
#[cfg(any(
    any(
        feature = "vehicle-transmission-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#vehicle_transmission::*;
#[cfg(any(
    any(feature = "vendor-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#vendor;
#[cfg(any(
    any(feature = "vendor-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#vendor::*;
#[cfg(any(
    any(
        feature = "verification-fact-checking-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#verification_fact_checking_policy;
#[cfg(any(
    any(
        feature = "verification-fact-checking-policy-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#verification_fact_checking_policy::*;
#[cfg(any(
    any(
        feature = "version-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#version;
#[cfg(any(
    any(
        feature = "version-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#version::*;
#[cfg(any(
    any(feature = "video-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#video;
#[cfg(any(
    any(feature = "video-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#video::*;
#[cfg(any(
    any(
        feature = "video-format-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#video_format;
#[cfg(any(
    any(
        feature = "video-format-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#video_format::*;
#[cfg(any(
    any(
        feature = "video-frame-size-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#video_frame_size;
#[cfg(any(
    any(
        feature = "video-frame-size-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#video_frame_size::*;
#[cfg(any(
    any(
        feature = "video-quality-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#video_quality;
#[cfg(any(
    any(
        feature = "video-quality-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#video_quality::*;
#[cfg(any(
    any(
        feature = "volume-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#volume_number;
#[cfg(any(
    any(
        feature = "volume-number-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#volume_number::*;
#[cfg(any(
    any(
        feature = "warning-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#warning;
#[cfg(any(
    any(
        feature = "warning-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#warning::*;
#[cfg(any(
    any(
        feature = "warranty-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#warranty;
#[cfg(any(
    any(
        feature = "warranty-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#warranty::*;
#[cfg(any(
    any(
        feature = "warranty-promise-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#warranty_promise;
#[cfg(any(
    any(
        feature = "warranty-promise-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#warranty_promise::*;
#[cfg(any(
    any(
        feature = "warranty-scope-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#warranty_scope;
#[cfg(any(
    any(
        feature = "warranty-scope-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#warranty_scope::*;
#[cfg(any(
    any(
        feature = "web-checkin-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#web_checkin_time;
#[cfg(any(
    any(
        feature = "web-checkin-time-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#web_checkin_time::*;
#[cfg(any(
    any(
        feature = "web-feed-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#web_feed;
#[cfg(any(
    any(
        feature = "web-feed-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#web_feed::*;
#[cfg(any(
    any(feature = "weight-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#weight;
#[cfg(any(
    any(feature = "weight-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#weight::*;
#[cfg(any(
    any(
        feature = "weight-total-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
mod r#weight_total;
#[cfg(any(
    any(
        feature = "weight-total-property-schema",
        feature = "auto-schema-section"
    ),
    doc
))]
pub use r#weight_total::*;
#[cfg(any(
    any(feature = "wheelbase-property-schema", feature = "auto-schema-section"),
    doc
))]
mod r#wheelbase;
#[cfg(any(
    any(feature = "wheelbase-property-schema", feature = "auto-schema-section"),
    doc
))]
pub use r#wheelbase::*;
#[cfg(any(
    any(feature = "width-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#width;
#[cfg(any(
    any(feature = "width-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#width::*;
#[cfg(any(
    any(feature = "winner-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#winner;
#[cfg(any(
    any(feature = "winner-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#winner::*;
#[cfg(any(
    any(
        feature = "word-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#word_count;
#[cfg(any(
    any(
        feature = "word-count-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#word_count::*;
#[cfg(any(
    any(
        feature = "work-example-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#work_example;
#[cfg(any(
    any(
        feature = "work-example-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#work_example::*;
#[cfg(any(
    any(
        feature = "work-featured-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#work_featured;
#[cfg(any(
    any(
        feature = "work-featured-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#work_featured::*;
#[cfg(any(
    any(
        feature = "work-hours-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#work_hours;
#[cfg(any(
    any(
        feature = "work-hours-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#work_hours::*;
#[cfg(any(
    any(
        feature = "work-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#work_location;
#[cfg(any(
    any(
        feature = "work-location-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#work_location::*;
#[cfg(any(
    any(
        feature = "work-performed-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#work_performed;
#[cfg(any(
    any(
        feature = "work-performed-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#work_performed::*;
#[cfg(any(
    any(
        feature = "work-presented-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#work_presented;
#[cfg(any(
    any(
        feature = "work-presented-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#work_presented::*;
#[cfg(any(
    any(
        feature = "work-translation-property-schema",
        feature = "bib-schema-section"
    ),
    doc
))]
mod r#work_translation;
#[cfg(any(
    any(
        feature = "work-translation-property-schema",
        feature = "bib-schema-section"
    ),
    doc
))]
pub use r#work_translation::*;
#[cfg(any(
    any(
        feature = "workload-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
mod r#workload;
#[cfg(any(
    any(
        feature = "workload-property-schema",
        feature = "health-lifesci-schema-section"
    ),
    doc
))]
pub use r#workload::*;
#[cfg(any(
    any(
        feature = "works-for-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#works_for;
#[cfg(any(
    any(
        feature = "works-for-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#works_for::*;
#[cfg(any(
    any(
        feature = "worst-rating-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#worst_rating;
#[cfg(any(
    any(
        feature = "worst-rating-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#worst_rating::*;
#[cfg(any(
    any(feature = "xpath-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#xpath;
#[cfg(any(
    any(feature = "xpath-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#xpath::*;
#[cfg(any(
    any(
        feature = "year-built-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
mod r#year_built;
#[cfg(any(
    any(
        feature = "year-built-property-schema",
        feature = "pending-schema-section"
    ),
    doc
))]
pub use r#year_built::*;
#[cfg(any(
    any(
        feature = "yearly-revenue-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#yearly_revenue;
#[cfg(any(
    any(
        feature = "yearly-revenue-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#yearly_revenue::*;
#[cfg(any(
    any(
        feature = "years-in-operation-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
mod r#years_in_operation;
#[cfg(any(
    any(
        feature = "years-in-operation-property-schema",
        feature = "general-schema-section"
    ),
    doc
))]
pub use r#years_in_operation::*;
#[cfg(any(
    any(feature = "yield-property-schema", feature = "general-schema-section"),
    doc
))]
mod r#yield;
#[cfg(any(
    any(feature = "yield-property-schema", feature = "general-schema-section"),
    doc
))]
pub use r#yield::*;
