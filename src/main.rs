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

mod config_management;
mod context_management;
mod declaration_parser;
mod expression;

mod module;
mod module_component;

#[macro_use]
extern crate lazy_static;

use expression::expression_parser::ExpressionParser;

use context_management::position::Position;

use std::env;
use std::env::Args;
use std::collections::BTreeMap;

use regex::Regex;

use colored::*;

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

	//let operators_data = operator_data::parse_operators_json("config/operators.json");
	let data = config_management::read_config_files();

	//let mut ender = ExpressionEnder { until_chars: Vec::new(), end_index: 0, end_char: ' ' };
	//let test = Expression::new("++!&&a(!~dfjks.jfdk[32.help],12,ew)()+sd", &operators_data, &mut ender);
	ExpressionParser::new("++&&&a++++ -  b", Position::new("test.tasty".to_string(), Some(1), 0, None), data, None);
	//"++!&&a(!~dfjks.jfdk[32.help],12,ew)()+sd"
	//" + ++ !& ^^^& (!~dfjks.  jfdk[32.help]  ,  12, ew) () + sd"
	//println!("{:?}", test.components);

	//content: &str, index: &mut usize, position: Position, line_offset: &mut usize

	/*
	let mut index: usize = 0;
	let pos = Position::new("test2.tasty".to_string(), Some(1), 0, None);
	let mut line_offset: usize = 0;
	let the_code = "copy test: QVector<QString> = 32 + 5;";
	let result = crate::declaration_parser::variable_declaration::VariableDeclaration::new(the_code, &mut index, pos, &mut line_offset);

	if result.is_none() { return; }
	println!("RESULT: {}", &the_code[result.as_ref().unwrap().start_index..result.as_ref().unwrap().end_index]);
	*/
}
