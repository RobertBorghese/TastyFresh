/**********************************************************
 * --- Operator Data ---
 *
 * Used to parse the `operators.json` configuration file
 * for the compiler.
 **********************************************************/

use crate::read_file;

use std::collections::BTreeMap;

use serde_json::Value;
use serde_json::map::Map;

pub type Operator = BTreeMap<&'static str,OperatorData>;
pub type OperatorDataStructure = BTreeMap<String,Vec<Operator>>;

/// An enum to help store and retrieve the distinct data types
/// required in the stucture.
pub enum OperatorData {
	SimpleOperator(String),
	ComplexOperator(Vec<Option<String>>),
	Priority(i64),
	CannotTouch(bool)
}

/// Parses the operator JSON data to a native Rust structure.
///
/// # Arguments
///
/// * `path` - The path to the `operators.json` file.
///
/// # Return
///
/// An instance of OperatorDataStructure containing all the
/// information.
pub fn parse_operators_json(path: &str) -> OperatorDataStructure {
	let json_str = read_file(path).unwrap();
	let operators_json: Map<String,Value> = serde_json::from_str(json_str.as_str()).unwrap();
	let mut operators = BTreeMap::new();
	for op_key in operators_json.keys() {
		let mut result = Vec::new();
		let sub_operators = operators_json[op_key].as_array().unwrap();
		for op_data in sub_operators {
			let op = op_data.as_object().unwrap();
			let mut operator_info = BTreeMap::new();
			if op["operator"].is_string() {
				operator_info.insert("operator", OperatorData::SimpleOperator(op["operator"].as_str().unwrap().to_string()));
			} else if op["operator"].is_array() {
				let mut r = Vec::new();
				for name_comp in op["operator"].as_array().unwrap() {
					if name_comp.is_null() {
						r.push(None);
					} else if name_comp.is_string() {
						r.push(Some(name_comp.as_str().unwrap().to_string()));
					}
				}
				operator_info.insert("operator", OperatorData::ComplexOperator(r));
			}
			if op["priority"].is_number() {
				operator_info.insert("priority", OperatorData::Priority(op["priority"].as_i64().unwrap()));
			}
			if op.contains_key("cannot_touch") && op["cannot_touch"].is_boolean() {
				operator_info.insert("cannot_touch", OperatorData::CannotTouch(op["cannot_touch"].as_bool().unwrap()));
			}
			result.push(operator_info);
		}
		operators.insert(op_key.to_string(), result);
	}
	return operators;
}
