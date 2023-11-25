use pretty_assertions::assert_eq;
use serde_json::{json, Value};

use super::*;
use crate::fallible::FailValue;

#[test]
fn test_date_serde() {
	let date_strings = vec![json!("2023-09-16")];
	let dates: Vec<Date> = date_strings
		.iter()
		.map(|value| serde_json::from_value(value.clone()).unwrap())
		.collect();
	let date_strings_serde: Vec<Value> = dates
		.iter()
		.map(|date| serde_json::to_value(date).unwrap())
		.collect();
	assert_eq!(date_strings, date_strings_serde);
}

#[test]
fn test_time_serde() {
	let time_strings = vec![
		json!("20:07:37Z"),
		json!("20:07:37-02:00"),
		json!("20:07:37"),
	];
	let times: Vec<Time> = time_strings
		.iter()
		.map(|value| serde_json::from_value(value.clone()).unwrap())
		.collect();
	let time_strings_serde: Vec<Value> = times
		.iter()
		.map(|time| serde_json::to_value(time).unwrap())
		.collect();
	assert_eq!(time_strings, time_strings_serde);
}

#[test]
fn test_date_time_serde() {
	let date_time_strings = vec![
		json!("2023-09-16T20:07:37Z"),
		json!("2023-09-16T20:07:37-02:00"),
		json!("2023-09-16T20:07:37"),
	];
	let date_times: Vec<DateTime> = date_time_strings
		.iter()
		.map(|value| serde_json::from_value(value.clone()).unwrap())
		.collect();
	let date_time_strings_serde: Vec<Value> = date_times
		.iter()
		.map(|date_time| serde_json::to_value(date_time).unwrap())
		.collect();
	assert_eq!(date_time_strings, date_time_strings_serde);
}

#[test]
fn test_thing_schema_serde() {
	let source_json = json!({
		"name": "name value",
		"alternateName": ["alternate name value"],
		"image": [
			"https://image.test/1",
			"https://image.test/2",
		],
		"mainEntityOfPage": "invalid_value",
	});
	let target_json = json!({
		"name": "name value",
		"alternateName": "alternate name value",
		"image": [
			"https://image.test/1",
			"https://image.test/2",
		],
		"mainEntityOfPage": "invalid_value",
	});
	let thing: Thing = serde_json::from_value(source_json).unwrap();
	assert!(match thing.name.get(0).unwrap() {
		NameProperty::Text(name) => dbg!(&name.0) == "name value",
		_ => false,
	});
	assert!(match thing.alternate_name.get(0).unwrap() {
		AlternateNameProperty::Text(alternate_name) =>
			dbg!(&alternate_name.0) == "alternate name value",
		_ => false,
	});
	assert!(match thing.image.get(0).unwrap() {
		ImageProperty::Url(image_1_url) => dbg!(image_1_url.0.as_str()) == "https://image.test/1",
		_ => false,
	});
	assert!(match thing.image.get(1).unwrap() {
		ImageProperty::Url(image_2_url) => dbg!(image_2_url.0.as_str()) == "https://image.test/2",
		_ => false,
	});
	assert!(match thing.main_entity_of_page.get(0).unwrap() {
		MainEntityOfPageProperty::SerdeFail(value) => match value {
			FailValue::String(main_entity_of_page_fail_string) =>
				dbg!(main_entity_of_page_fail_string) == "invalid_value",
			_ => false,
		},
		_ => false,
	});
	let serialized_thing = serde_json::to_value(thing).unwrap();
	assert_eq!(serialized_thing, target_json);
}

#[test]
fn test_enumeration_serde() {
	let source_json = json!("ActiveActionStatus");
	let target_json = json!("ActiveActionStatus");
	let action_status_type: ActionStatusType = serde_json::from_value(source_json).unwrap();
	let serialized_action_status_type = serde_json::to_value(action_status_type).unwrap();
	assert_eq!(serialized_action_status_type, target_json);
}

/// [`RatingCountProperty`] is a property which normally only allows [`Integer`].
/// But since every property in schema.org will realistically be used with string values...
#[test]
fn test_answer_count_serde() {
	let source_json = json!("1");
	let target_json = json!(1);
	let rating_count: RatingCountProperty = serde_json::from_value(source_json).unwrap();
	let rating_count_value = serde_json::to_value(rating_count).unwrap();
	assert_eq!(rating_count_value, target_json);
}

/// Test that unknown fields are denied so the correct property variant is deserialized.
/// <https://github.com/Toromyx/schema_org_types_rs/issues/14>
#[test]
fn test_deny_unknown_fields() {
	let source_json = json!({
		"name": "item list name",
		"description": "This is an item list because it has list items. But as recipe instructions, it will first be tried to be deserialized as creative work.",
		"itemListElement": [
			{
				"text": "list item 1 text."
			},
			{
				"text": "list item 2 text."
			}
		]
	});
	let recipe_instructions: RecipeInstructionsProperty =
		serde_json::from_value(source_json).unwrap();
	match recipe_instructions {
		RecipeInstructionsProperty::ItemList(_)
		| RecipeInstructionsProperty::HowToSection(_)
		| RecipeInstructionsProperty::HowToStep(_) => {
			// deserialized correctly
		}
		_ => {
			panic!(
				"A property variant is not correctly deserialized, probably because unknown fields are not denied."
			);
		}
	};
}
