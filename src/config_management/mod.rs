/**********************************************************
 * --- Config Management ---
 *
 * The JSON files stored in the `config` directory
 * are read, parsed, and stored using the structs and
 * functions provided within this directory.
 **********************************************************/

pub mod operator_data;

use crate::config_management::operator_data::{ OperatorDataStructure, parse_operators_json };

use std::collections::BTreeMap;

use std::fs::File;
use std::io::prelude::*;

pub struct ConfigData {
	pub operators: OperatorDataStructure
}

impl ConfigData {
	pub fn new() -> ConfigData {
		return ConfigData {
			operators: BTreeMap::new()
		};
	}
}

/// Reads a text file and returns the contents as a `String`.
///
/// # Arguments
///
/// * `path` - The path of the file.
///
/// # Return
///
/// The contents of the file.
pub fn read_file(path: &str) -> std::io::Result<String> {
	let mut result = String::new();
	let mut file = File::open(path)?;
	file.read_to_string(&mut result)?;
	Ok(result)
}

/// Reads a text file and returns the contents as a `String`.
///
/// # Arguments
///
/// * `path` - The path of the file.
///
/// # Return
///
/// The contents of the file.
pub fn read_config_files() -> ConfigData {
	return ConfigData {
		operators: parse_operators_json("config/operators.json")
	};
}
