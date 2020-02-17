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

mod file_system;
mod module;
mod module_component;

#[macro_use]
extern crate lazy_static;

use expression::expression_parser::ExpressionParser;

use context_management::position::Position;

use declaration_parser::parser::Parser;
use declaration_parser::module_declaration::ModuleDeclaration;

use config_management::ConfigData;

use file_system::get_all_tasty_files;

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
fn parse_arguments(args: Args) -> BTreeMap<String,Vec<String>> {
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
			let mut key: Option<String> = None;
			let mut value: Option<String> = None;
			if captures.is_none() {
				key = Some(String::from(&arg[2..]));
			} else {
				let v = captures.unwrap();
				if v.get(1).is_some() && v.get(2).is_some() {
					key = Some(v.get(1).unwrap().as_str().to_string());
					value = Some(v.get(2).unwrap().as_str().to_string());
				}
			}
			if key.is_some() {
				let key_str = key.unwrap();
				if !result.contains_key(&key_str) {
					result.insert(key_str.clone(), Vec::new());
				}
				if value.is_some() {
					result.get_mut(&key_str).unwrap().push(value.unwrap());
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
	println!("{}{}{}{}", "Unknown argument format at ".bright_red(), format!("position {}", index - 1).green(), ": ".bright_red(), arg_name.yellow());
}

/// Retrieves all source files using directories provided as arguments
///
/// # Arguments
///
/// * `arguments` - The arguments map returned by `parse_arguments`.
///
/// # Return
///
/// A map that assigns keys as the source directories that point to values of `Vec<String>` containing source file names.
fn get_source_files(arguments: &BTreeMap<String,Vec<String>>) -> Option<BTreeMap<String,Vec<String>>> {
	let mut source_files = BTreeMap::new();
	match arguments.get("src") {
		Some(src_dirs) => {
			for dir in src_dirs {
				match get_all_tasty_files(dir) {
					Some(files) => { source_files.insert(dir.clone(), files); },
					None => println!("{}{}{}", "Source directory ".bright_red(), dir.yellow(), " does not exist!".bright_red())
				}
			}
		},
		None => {
			println!("{}{}{}{}", "At least one source directory must be specified using ".bright_red(), "--src:".yellow(), "DIR".green(), ".".bright_red());
			return None;
		}
	}
	return Some(source_files);
}

/// Retrieves all output directories using information provided as arguments.
/// If the directory does not exist, an attempt is made to create it.
///
/// # Arguments
///
/// * `arguments` - The arguments map returned by `parse_arguments`.
///
/// # Return
///
/// A `Vec<String>` containing all the valid output directories.
fn get_output_dirs(arguments: &BTreeMap<String,Vec<String>>) -> Option<Vec<String>> {
	let mut output_dirs = Vec::new();
	match arguments.get("out") {
		Some(out_dirs) => {
			for dir in out_dirs {
				let path = std::path::Path::new(dir);
				if path.exists() {
					if !path.is_dir() {
						println!("{}{}", dir.yellow(), " is not a valid output directory!".bright_red());
						return None;
					} else {
						output_dirs.push(dir.clone());
					}
				} else {
					match std::fs::create_dir_all(path) {
						Ok(_) => output_dirs.push(dir.clone()),
						Err(e) => {
							println!("{}{}{}{}{}", "Could not create output directory ".bright_red(), dir.yellow(),
								" because of \"".bright_red(), e, "\".".bright_red());
							return None;
						}
					}
				}
			}
		},
		None => {
			return Some(Vec::new());
		}
	}
	return Some(output_dirs);
}

/// Transpiles the input source file into C++ and outputs it to the provided `output_dirs`.
///
/// # Arguments
///
/// * `file` - The relative or absolute path to the source file.
/// * `output_dirs` - The list of output directories to write the C++ files to.
/// * `config_data` - The configuration data for the transpiler.
///
/// # Return
///
/// If successful, `true` is returned; otherwise `false`.
fn transpile_source_file(file: &str, output_dirs: &Vec<String>, config_data: &ConfigData) -> bool {
	let content = std::fs::read_to_string(file).expect("Could not read source file.");
	let mut parser = Parser::new(content.as_str());
	let module_declaration = ModuleDeclaration::new(&parser);
	return true;
}

/// The main function of Tasty Fresh.
fn main() {
	let arguments = parse_arguments(env::args());

	let source_files = match get_source_files(&arguments) {
		Some(files) => files,
		None => return
	};

	let output_dirs = match get_output_dirs(&arguments) {
		Some(dirs) => dirs,
		None => return
	};

	let data = config_management::read_config_files();

	for files in source_files {
		for f in files.1 {
			transpile_source_file(&f, &output_dirs, &data);
		}
	}

	//let mut ender = ExpressionEnder { until_chars: Vec::new(), end_index: 0, end_char: ' ' };
	//let test = Expression::new("++!&&a(!~dfjks.jfdk[32.help],12,ew)()+sd", &operators_data, &mut ender);
	ExpressionParser::new("++&&&a++++ -  b", Position::new("test.tasty".to_string(), Some(1), 0, None), data, None);
	//"++!&&a(!~dfjks.jfdk[32.help],12,ew)()+sd"
	//" + ++ !& ^^^& (!~dfjks.  jfdk[32.help]  ,  12, ew) () + sd"
	//println!("{:?}", test.components);

	//content: &str, index: &mut usize, position: Position, line_offset: &mut usize

	//let files = list_all_files(".".to_string(), "tasty".to_string());

	//let c = "u8\"fdsfdsfd 3 \\\\ \\x1a \\n \\r\"";
	let c = "copy a: int = 32;";
	let mut parser = Parser {
		content: c,
		chars: c.chars().collect(),
		index: 0,
		line: 1,
		out_of_space: false
	};

	//println!("{}", parser.parse_string());

	let pos = Position {
		file: "tasty".to_string(),
		line: Some(1),
		start: 0,
		end: None
	};

	let result = crate::declaration_parser::variable_declaration::VariableDeclaration::new(&mut parser);

	if result.is_error() {
		result.print_error("tast2.tasty".to_string(), "copy a: int = 32;");
		return;
	}
	let rr = result.as_ref().unwrap();
	println!("---- Initialize ----");
	println!("{}", rr.name);
	println!("{}", rr.var_type.var_type.to_cpp());
	println!("{}", rr.var_type.var_style.get_name());
	println!("RESULT: {}", &c[rr.start_index..rr.end_index]);

	let attribute_content = " fdjs fdsjkldfs @Test(fds fdksleqw 21l dsfd, 999)";
	let mut parser2 = Parser::new(attribute_content);
	parser2.index = 16;
	let result2 = crate::declaration_parser::attribute_declaration::AttributeDeclaration::new(&mut parser2);
	if result2.is_error() {
		result2.print_error("atttribute.tasty".to_string(), attribute_content);
		return;
	}
	let rr2 = result2.as_ref().unwrap();
	println!("---- Attribute ----");
	println!("{}", rr2.name);
	println!("{:?}", rr2.parameters);

/*
	let mut index: usize = 0;
	let pos = Position::new("test2.tasty".to_string(), Some(1), 0, None);
	let mut line_offset: usize = 0;
	let the_code = "copy test: QVector   <  QString    , int   g > = 32 + 5;";
	let result = crate::declaration_parser::variable_declaration::VariableDeclaration::new(the_code, &mut index, pos, &mut line_offset);

	if result.is_error() {
		result.print_error("tast2.tasty".to_string());
		return;
	}
	let rr = result.as_ref().unwrap();
	println!("{}", rr.name);
	println!("{}", rr.var_type.var_type.to_cpp());
	println!("{}", rr.var_type.var_style.get_name());
	println!("RESULT: {}", &the_code[rr.start_index..rr.end_index]);*/
}
