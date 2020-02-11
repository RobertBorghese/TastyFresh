/**********************************************************
 * --- Name ---
 * Tasty Fresh Programming Language
 *
 * --- Developer ---
 * Robert Borghese
 *
 * --- License ---
 * LGPLv3
 * https://www.gnu.org/licenses/lgpl-3.0.en.html
 *
 * --- Compiler Options ---
 *
 * [ src ]
 *   Determines the location of the source directory
 *   Tasty Fresh source files will be retrieved from.
 *
 *   [ examples ]
 *      --src:src
 *      --src:"My Sources"
 *
 * ----------
 *
 * [ out ]
 *   Determines the location of the output files
 *   Tasty Fresh source files will be transpiled to.
 *   By default, it will use the source directory.
 *
 *   [ examples ]
 *      --out:out
 *      --out:"My Output"
 *
 **********************************************************/

mod expression;
mod expression_components;
mod value_type;
mod variable_type;

mod operator_data;

use expression::{ Expression, ExpressionEnder };

use std::env;
use std::env::Args;
use std::collections::BTreeMap;

use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

use colored::*;

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

/// Parses arguments with `--KEY` or `--KEY:VALUE` format.
///
/// # Arguments
///
/// * `args` - The instance of `std::env::Args` to parse.
///
/// # Return
///
/// An instance of BTreeMap containing the key/value pairs
/// passed to the compiler.
fn parse_arguments(args: Args) -> BTreeMap<String,String> {
	let arg_regexp = Regex::new(r"^--(\w[\w\d]*):(.*)$").unwrap();
	let mut result = BTreeMap::new();
	let mut index = 0;
	for arg in args {
		index += 1;
		if index == 1 {
			continue;
		}
		if arg.len() >= 3 && arg.starts_with("--") {
			let captures = arg_regexp.captures(arg.as_str());
			if captures.is_none() {
				result.insert(String::from(&arg[2..]), "".to_string());
			} else {
				let v = captures.unwrap();
				if v.get(1).is_some() && v.get(2).is_some() {
					result.insert(v.get(1).unwrap().as_str().to_string(), v.get(2).unwrap().as_str().to_string());
				}
			}
		} else {
			print_unknown_argument(arg.as_str(), index);
		}
	}
	return result;
}

/// Prints a warning upon an encounter with an unknown compiler argument.
///
/// # Arguments
///
/// * `arg_name` - The name of the unknown argument.
/// * `index` - The index of the argument in the list.
fn print_unknown_argument(arg_name: &str, index: i32) {
	println!("{}: {}", format!("Unknown argument format at index {}", index).yellow(), arg_name.bright_blue().bold());
}

/// The main function of Tasty Fresh.
fn main() {
	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);

	let arguments = parse_arguments(env::args());
	println!("{:?}", arguments);

	let operators_data = operator_data::parse_operators_json("config/operators.json");

	let mut ender = ExpressionEnder { until_chars: Vec::new(), end_index: 0, end_char: ' ' };
	let test = Expression::new("++!&&a(!~dfjks.jfdk[32.help],12,ew)()+sd", &operators_data, &mut ender);
	//" + ++ !& ^^^& (!~dfjks.  jfdk[32.help]  ,  12, ew) () + sd"
	//println!("{:?}", test.components);
}
