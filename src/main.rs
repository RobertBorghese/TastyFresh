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
mod scope_parser;

mod file_system;
mod transpiler;

#[macro_use]
extern crate lazy_static;

use expression::Expression;
use expression::expression_parser::{ ExpressionParser, ExpressionEndReason };

use context_management::position::Position;

use declaration_parser::parser::Parser;
use declaration_parser::module_declaration::{ ModuleDeclaration, DeclarationType };
use declaration_parser::attribute_declaration::AttributeDeclaration;
use declaration_parser::include_declaration::IncludeDeclaration;
use declaration_parser::import_declaration::ImportDeclaration;
use declaration_parser::function_declaration::{ FunctionDeclaration, FunctionDeclarationType };
use declaration_parser::variable_declaration::VariableDeclaration;

use scope_parser::ScopeExpression;

use config_management::ConfigData;

use file_system::get_all_tasty_files;

use transpiler::Transpiler;

use context_management::context::Context;
use context_management::typing_context::TypingContext;
use context_management::print_code_error;

use std::env;
use std::env::Args;
use std::collections::BTreeMap;

use std::path::Path;
use std::ffi::OsStr;

use std::rc::Rc;

use regex::Regex;

use colored::*;

use path_slash::PathExt;

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
			return Some(vec![".".to_string()]);
		}
	}
	if output_dirs.is_empty() {
		output_dirs.push(".".to_string());
	}
	return Some(output_dirs);
}

/// Parses the input source file into its declaration data.
///
/// # Arguments
///
/// * `file` - The relative or absolute path to the source file.
/// * `output_dirs` - The list of output directories to write the C++ files to.
/// * `config_data` - The configuration data for the transpiler.
/// * `module_contexts` - A reference to store the file declarations within.
///
/// # Return
///
/// The `ModuleDeclaration` for the file is returned.
fn parse_source_file(file: &str, output_dirs: &Vec<String>, config_data: &ConfigData, module_contexts: &mut BTreeMap<String,Context>, parser: &mut Parser) -> ModuleDeclaration {
	let content = std::fs::read_to_string(file).expect("Could not read source file.");
	*parser = Parser::new(content);
	let mut context = Context::new();
	let module_declaration = ModuleDeclaration::new(parser, file);
	for declaration in &module_declaration.declarations {
		match declaration {
			DeclarationType::Function(d, attributes) => {
				context.module.add_function(d.name.clone(), d.to_function(&parser.content));
				for p in &d.parameters {
					context.register_type(&p.0);
				}
				context.register_type(&d.return_type);
			},
			DeclarationType::Variable(d, attributes) => {
				context.module.add_variable(d.name.clone(), d.var_type.clone());
				context.register_type(&d.var_type);
			},
			_ => {
			}
		}
	}
	module_contexts.insert(if file.ends_with(".tasty") {
		(&file[0..file.len() - 6]).to_string()
	} else {
		file.to_string()
	}, context);
	return module_declaration;
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
fn transpile_source_file(file: &str, output_dirs: &Vec<String>, config_data: &ConfigData, module_contexts: &mut BTreeMap<String,Context>, module_declaration: &mut ModuleDeclaration, parser: &mut Parser) -> bool {
	let access_file_path = if file.ends_with(".tasty") {
		&file[0..file.len() - 6]
	} else {
		file
	};
	{
		let mut context = module_contexts.get_mut(access_file_path).unwrap();
		let mut typing = &mut context.typing;
		typing.add(&context.module);
	}

	/*
	let mut output_lines = Vec::new();

	let mut variable_declarations = Vec::<String>::new();
	let mut function_declarations = Vec::<String>::new();
	let mut variable_declarations_isolated = Vec::<String>::new();
	let mut function_declarations_isolated = Vec::<String>::new();
	
	let mut handling_module_attributes = true;
	let mut header_include_line: Option<usize> = None;
	let mut end_line = 0;

	let mut header_system_includes = Vec::new();
	let mut header_local_includes = Vec::new();
	*/

	let mut transpile_context = Transpiler::new(file, access_file_path, config_data, module_contexts, parser);
	transpile_context.parse_declarations(&mut module_declaration.declarations, None);

	if transpile_context.header_include_line.is_none() {
		if !transpile_context.output_lines[0].is_empty() {
			transpile_context.output_lines.insert(0, "".to_string());
		}
		if !transpile_context.output_lines[1].is_empty() {
			transpile_context.output_lines.insert(0, "".to_string());
		}
		transpile_context.header_include_line = Some(0);
	}

	let mut header_lines = Vec::new();
	{
		let file_path = Path::new(file);
		let marco_name = file_path.file_stem().unwrap().to_str().unwrap().to_uppercase() + "_TASTYFILE";
		header_lines.push("#ifndef ".to_string() + &marco_name);
		header_lines.push("#define ".to_string() + &marco_name);
		header_lines.push("".to_string());
		let context = transpile_context.module_contexts.get_mut(access_file_path).unwrap();
		if !context.headers.is_empty() || !transpile_context.header_system_includes.is_empty() {
			for head in &context.headers.headers {
				header_lines.push(format!("#include <{}>", head.path));
			}
			for head_path in &transpile_context.header_system_includes {
				header_lines.push(format!("#include <{}>", head_path));
			}
			header_lines.push("".to_string());
		}
		if !transpile_context.header_local_includes.is_empty() {
			for head_path in &transpile_context.header_local_includes {
				header_lines.push(format!("#include \"{}\"", head_path));
			}
			header_lines.push("".to_string());
		}
		transpile_context.declarations.export_to_lines(&mut header_lines, 0, true);
		for cls in transpile_context.class_declarations {
			header_lines.push(cls.0);
			if !cls.1.is_empty() {
				header_lines.push("public:".to_string());
				cls.1.export_to_lines(&mut header_lines, 1, false);
				header_lines.pop();
			}
			if !cls.2.is_empty() {
				header_lines.push("private:".to_string());
				cls.2.export_to_lines(&mut header_lines, 1, false);
				header_lines.pop();
			}
			header_lines.push("};".to_string());
			header_lines.push("".to_string());
		}
		header_lines.push("#endif".to_string());
	}

	for dir in output_dirs {
		let path = Path::new(dir).join(file);
		let path_str = path.to_slash();
		if path_str.is_some() {
			let path_str_unwrap = path_str.unwrap();
			let path_base = path_str_unwrap[..(path_str_unwrap.len() - path.extension().and_then(OsStr::to_str).unwrap_or("").len())].to_string();
			let header_path = path_base.clone() + "hpp";
			if transpile_context.header_include_line.is_some() {
				insert_output_line(&mut transpile_context.output_lines, format!("#include \"{}\"", if header_path.starts_with("./") {
					&header_path[2..]
				} else { &header_path }).as_str(), transpile_context.header_include_line.unwrap(), true);
			}
			std::fs::write(path_base + "cpp", transpile_context.output_lines.join("\n"));
			std::fs::write(header_path, header_lines.join("\n"));
		} else {
			println!("\nCOULD NOT WRITE TO FILE: {}", format!("{}{}", dir, file));
		}
	}
	return true;
}

fn get_configure_declaration_with_attributes(isolated: &mut bool, declaration: &str, attributes: &Option<Vec<AttributeDeclaration>>, content: &str, semicolon: bool) -> String {
	let mut prepend = "".to_string();
	let mut append = "".to_string();
	if attributes.is_some() {
		for a in attributes.as_ref().unwrap() {
			if a.name == "DeclarePrepend" {
				if a.parameters.is_some() {
					for param in a.parameters.as_ref().unwrap() {
						prepend += &content[param.0..param.1];
					}
				}
			} else if a.name == "DeclareAppend" {
				if a.parameters.is_some() {
					for param in a.parameters.as_ref().unwrap() {
						append += &content[param.0..param.1];
					}
				}
			} else if a.name == "Isolated" {
				*isolated = true;
			}
		}
	}
	let mut result = format!("{}{}{}{}", 
		if prepend.is_empty() { "".to_string() } else { format!("{}\n", prepend) }, 
		declaration,
		if semicolon { ";" } else { "" },
		if append.is_empty() { "".to_string() } else { format!("\n{}", append) }
	);
	return result;
}

fn configure_declaration_with_attributes(delcarations: &mut Vec<String>, declarations_isolated: &mut Vec<String>, declaration: &str, attributes: &Option<Vec<AttributeDeclaration>>, content: &str, semicolon: bool) {
	let mut isolated = false;
	let result = get_configure_declaration_with_attributes(&mut isolated, declaration, attributes, content, semicolon);
	if isolated {
		declarations_isolated.push(result);
	} else {
		delcarations.push(result);
	}
}

fn insert_output_line(output_lines: &mut Vec<String>, line: &str, line_number: usize, clear: bool) {
	while line_number >= output_lines.len() {
		output_lines.push("".to_string());
	}
	if line.is_empty() {
		return;
	}
	if !output_lines[line_number].is_empty() {
		output_lines[line_number] += " ";
	}
	if !clear {
		output_lines[line_number] += line;
	} else {
		output_lines[line_number] = line.to_string();
	}
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

	let mut file_contexts = BTreeMap::new();
	let mut file_declarations = BTreeMap::new();
	let mut file_parsers = BTreeMap::new();

	for files in &source_files {
		for f in files.1 {
			let mut parser: Parser = Parser::new("".to_string());
			file_declarations.insert(f.clone(), parse_source_file(&f, &output_dirs, &data, &mut file_contexts, &mut parser));
			file_parsers.insert(f, parser);
		}
	}

	for files in &source_files {
		for f in files.1 {
			transpile_source_file(&f, &output_dirs, &data, &mut file_contexts, file_declarations.get_mut(f).unwrap(), file_parsers.get_mut(f).unwrap());
		}
	}

	//let mut ender = ExpressionEnder { until_chars: Vec::new(), end_index: 0, end_char: ' ' };
	//let test = Expression::new("++!&&a(!~dfjks.jfdk[32.help],12,ew)()+sd", &operators_data, &mut ender);

	/*
	let temp_expr_content = "(++&&&a++++ - a);";
	let mut parser = Parser::new(temp_expr_content);
	let bla = ExpressionParser::new(&mut parser, Position::new("test.tasty".to_string(), Some(1), 0, None), &data, None);
	println!("{}", match bla.end_data.reason {
		ExpressionEndReason::EndOfExpression => "enx od epxression",
		_ => "other"
	});
	*/

	//"++!&&a(!~dfjks.jfdk[32.help],12,ew)()+sd"
	//" + ++ !& ^^^& (!~dfjks.  jfdk[32.help]  ,  12, ew) () + sd"
	//println!("{:?}", test.components);

	return;
	//content: &str, index: &mut usize, position: Position, line_offset: &mut usize

	//let files = list_all_files(".".to_string(), "tasty".to_string());

	//let c = "u8\"fdsfdsfd 3 \\\\ \\x1a \\n \\r\"";

	// VARIABLE INIT
	let c = "const static copy a: int = 32;";
	let mut parser = Parser::new(c.to_string());
	let result = crate::declaration_parser::variable_declaration::VariableDeclaration::new(&mut parser);
	if result.is_error() {
		result.print_error("tast2.tasty".to_string(), c);
		return;
	}
	let rr = result.as_ref().unwrap();
	println!("---- Initialize ----");
	for a in &rr.var_type.var_properties.unwrap_or(Vec::new()) { println!("{}", a.get_name()); }
	println!("{}", rr.var_type.var_style.get_name());
	println!("{}", rr.var_type.var_type.to_cpp());
	println!("{}", rr.name);
	let val = rr.value.unwrap();
	println!("RESULT: {}", &c[val.0..val.1]);

	// ATTRIBUTE
	let attribute_content = " fdjs fdsjkldfs @Test(fds fdksleqw 21l dsfd, 999)";
	let mut parser2 = Parser::new(attribute_content.to_string());
	parser2.index = 16;
	let result2 = AttributeDeclaration::new(&mut parser2);
	if result2.is_error() {
		result2.print_error("atttribute.tasty".to_string(), attribute_content);
		return;
	}
	let rr2 = result2.as_ref().unwrap();
	println!("---- Attribute ----");
	println!("{}", rr2.name);
	println!("{:?}", rr2.parameters);

	// INCLUDE
	let include_content = "include local hjkj/sdfdsf\\qrewre.h;";
	let mut parser3 = Parser::new(include_content.to_string());
	parser3.index = 0;
	let result3 = IncludeDeclaration::new(&mut parser3);
	if result3.is_error() {
		result3.print_error("include.tasty".to_string(), include_content);
		return;
	}
	let rr3 = result3.as_ref().unwrap();
	println!("---- Include ----");
	println!("{}", rr3.path);
	println!("{}", (rr3.inc_type as i32));

	// IMPORT
	let import_content = "import test/bla;";
	let mut parser4 = Parser::new(import_content.to_string());
	parser4.index = 0;
	let result4 = ImportDeclaration::new(&mut parser4);
	if result4.is_error() {
		result4.print_error("include.tasty".to_string(), import_content);
		return;
	}
	let rr4 = result4.as_ref().unwrap();
	println!("---- Import ----");
	println!("{}", rr4.path);

	// FUNCTION
	let func_content = "static inline fn test(copy a: vector<unsigned char>, ptr b: Bla) -> unsigned int { return 0; }";
	let mut parser5 = Parser::new(func_content.to_string());
	parser5.index = 0;
	let result5 = FunctionDeclaration::new(&mut parser5, FunctionDeclarationType::ModuleLevel);
	if result5.is_error() {
		result5.print_error("include.tasty".to_string(), func_content);
		return;
	}
	let rr5 = result5.as_ref().unwrap();
	println!("---- Function ----");
	println!("{}", rr5.name);
	println!(",");
	println!("{}", rr5.return_type.var_style.get_name());
	println!("{}", rr5.return_type.var_type.to_cpp());
	println!(",");
	for prop in &rr5.props {
		println!("{}", prop.get_name());
	}
	for param in &rr5.parameters {
		println!(",");
		println!("{}", param.0.var_style.get_name());
		println!("{}", param.0.var_type.to_cpp());
		println!("{}", param.1);
	}
	if rr5.start_index.is_some() { println!("RESULT: {}", &func_content[rr5.start_index.unwrap()..rr5.end_index.unwrap()]); }
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
