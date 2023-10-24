use pretty_assertions::assert_eq;
use serde_json::{json, Value};

use super::*;

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
	});
	let target_json = json!({
		"name": "name value",
		"alternateName": "alternate name value",
		"image": [
			"https://image.test/1",
			"https://image.test/2",
		],
	});
	let thing: Thing = serde_json::from_value(source_json).unwrap();
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
