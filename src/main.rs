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

#![allow(dead_code)]

mod config_management;
mod context_management;
mod declaration_parser;
mod expression;
mod scope_parser;

mod file_system;
mod transpiler;

#[macro_use]
extern crate lazy_static;

use context_management::global_context::GlobalContext;
use context_management::context_manager::ContextManager;
use context_management::static_extension::StaticExtension;

use declaration_parser::parser::Parser;
use declaration_parser::module_declaration::{ ModuleDeclaration, DeclarationType };
use declaration_parser::attributes::Attributes;

use expression::variable_type::{ VariableType, Type };

use config_management::ConfigData;

use file_system::get_all_tasty_files;

use transpiler::Transpiler;

use context_management::context::Context;

use std::env;
use std::env::Args;
use std::collections::BTreeMap;

use std::path::Path;
use std::ffi::OsStr;

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
/// * `module_contexts` - A reference to store the file declarations within.
///
/// # Return
///
/// The `ModuleDeclaration` for the file is returned.
fn parse_source_file(file: &str, source_location: &str, config_data: &ConfigData, module_contexts: &mut ContextManager, parser: &mut Parser, global_context: &mut GlobalContext) -> ModuleDeclaration {
	let content = std::fs::read_to_string(file).expect("Could not read source file.");
	if !file.ends_with(".tasty") { panic!("File is not a .tasty. You should be ashamed."); }
	*parser = Parser::new(content);
	let mut curr_index = 0;
	let mut context = Context::new();
	let mut module_declaration = ModuleDeclaration::new(parser, file, &config_data.operators);
	let mut attribute_class_indexes = Vec::new();
	for declaration in &mut module_declaration.declarations {
		match declaration {
			DeclarationType::Function(d, _) => {
				d.declaration_id = context.module.add_function(d.name.clone(), d.to_function(&parser.content), Some(module_contexts));
				for p in &d.parameters {
					context.register_type(&p.0);
				}
				context.register_type(&d.return_type);
			},
			DeclarationType::Variable(d, _) => {
				d.declaration_id = context.module.add_variable(d.name.clone(), d.var_type.clone(), Some(module_contexts));
				context.register_type(&d.var_type);
			},
			DeclarationType::Class(d, attributes) => {
				let class_data = d.to_class(&mut context, module_contexts, &parser.content, &attributes);

				for inc in &class_data.required_includes {
					context.add_header(&inc.0, inc.1);
				}

				if d.extensions.is_some() {
					for e in d.extensions.as_ref().unwrap() {
						context.register_type_only(e);
					}
				}

				if d.abstract_declarations.is_some() {
					for extend in d.abstract_declarations.as_ref().unwrap() {
						if let DeclarationType::Function(d2, _) = extend {
							context.static_extends.insert(d2.name.clone(),
								StaticExtension::new(
									format!("{}_{}", d.name, d2.name),
									d2.to_function(&parser.content),
									VariableType::copy(Type::Undeclared(vec![d.name.clone()]))
								)
							);
						}
					}
				}

				d.declaration_id = context.module.add_class(d.name.clone(), class_data, Some(module_contexts));
			},
			DeclarationType::Refurbish(d, attributes) => {
				for inc in attributes.get_required_includes() {
					context.add_header(&inc.0, inc.1);
				}

				context.register_type_only(&d.refurbish_type);

				for extend in &d.declarations {
					if let DeclarationType::Function(d2, _) = extend {
						context.static_extends.insert(d2.name.clone(),
							StaticExtension::new(
								format!("{}_{}", d.make_name(), d2.name),
								d2.to_function(&parser.content),
								VariableType::copy(d.refurbish_type.clone())
							)
						);
					}
				}
			},
			DeclarationType::AttributeClass(_, _) => {
				attribute_class_indexes.push(curr_index);
			},
			_ => {
			}
		}
		curr_index += 1;
	}
	module_contexts.add_context((&file[source_location.len() + 1..file.len() - 6]).to_string(), context);

	let mut attribute_classes_processed = 0;
	for attribute_index in attribute_class_indexes {
		let attribute_class_declare = module_declaration.declarations.remove(attribute_index - attribute_classes_processed);
		if let DeclarationType::AttributeClass(d, _) = attribute_class_declare {
			global_context.add_attribute_class(d);
		}
		attribute_classes_processed += 1;
	}

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
fn transpile_source_file(file: &str, source_location: &str, output_dirs: &Vec<String>, config_data: &ConfigData, module_contexts: &mut ContextManager, module_declaration: &mut ModuleDeclaration, parser: &mut Parser, global_context: &mut GlobalContext) -> bool {
	let access_file_path = &file[source_location.len() + 1..file.len() - 6];
	{
		/*let context = module_contexts.get_context(access_file_path);
		let typing = &mut context.typing;
		typing.add(access_file_path.to_string());//&context.module);
		*/
	}

	let mut transpile_context = Transpiler::new(file, access_file_path, config_data, module_contexts, parser);
	transpile_context.parse_declarations(&mut module_declaration.declarations, global_context, None, None);

	if !transpile_context.output_lines.is_empty() {
		if transpile_context.header_include_line.is_none() {
			if !transpile_context.output_lines[0].is_empty() {
				transpile_context.output_lines.insert(0, "".to_string());
			}
			if transpile_context.output_lines.len() > 1 && !transpile_context.output_lines[1].is_empty() {
				transpile_context.output_lines.insert(0, "".to_string());
			}
			transpile_context.header_include_line = Some(0);
		}
	}

	let declarations_are_empty = transpile_context.class_declarations.is_empty() && transpile_context.declarations.is_empty();
	let mut header_lines: Vec<String> = Vec::new();
	{
		let file_path = Path::new(file);
		let marco_name = file_path.file_stem().unwrap().to_str().unwrap().to_uppercase() + "_TASTYFILE";
		if config_data.pragma_guard {
			header_lines.push("#pragma once".to_string());
		} else {
			header_lines.push("#ifndef ".to_string() + &marco_name);
			header_lines.push("#define ".to_string() + &marco_name);
		}
		header_lines.push("".to_string());
		let context_headers = &transpile_context.module_contexts.get_context(access_file_path).headers;
		if !context_headers.is_empty() || !transpile_context.header_system_includes.is_empty() {
			for head in &context_headers.headers {
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
			if !cls.1.is_empty() || !cls.2.is_empty() {
				header_lines.push("public:".to_string());
				if !cls.1.is_empty() {
					cls.1.export_to_lines(&mut header_lines, 1, false);
				}
				if !cls.2.is_empty() {
					cls.2.export_to_lines(&mut header_lines, 1, false);
					header_lines.pop();
				}
			}
			if !cls.3.is_empty() {
				header_lines.push("private:".to_string());
				cls.3.export_to_lines(&mut header_lines, 1, false);
				header_lines.pop();
			}
			header_lines.push("};".to_string());
			header_lines.push("".to_string());
		}
		if !config_data.pragma_guard {
			header_lines.push("#endif".to_string());
		}
	}

	for dir in output_dirs {
		let path = Path::new(dir).join(file);
		let path_str = path.to_slash();
		if path_str.is_some() {
			let path_str_unwrap = path_str.unwrap();
			let path_base = path_str_unwrap[..(path_str_unwrap.len() - path.extension().and_then(OsStr::to_str).unwrap_or("").len())].to_string();
			let header_path = path_base.clone() + (if config_data.hpp_headers { "hpp" } else { "h" });
			if transpile_context.header_include_line.is_some() {
				insert_output_line(&mut transpile_context.output_lines, format!("#include \"{}\"",
				if header_path.starts_with(format!("./{}/", source_location).as_str()) {
					&header_path[source_location.len() + 3..]
				} else if header_path.starts_with(format!("{}/", source_location).as_str()) {
					&header_path[source_location.len() + 1..]
				} else {
					&header_path
				}).as_str(), transpile_context.header_include_line.unwrap(), true);
			}
			let full_source_path = path_base + "cpp";
			let full_header_path = header_path;

			let full_source_path_obj = Path::new(&full_source_path);
			let full_header_path_obj = Path::new(&full_header_path);
			if transpile_context.output_lines.is_empty() &&
				declarations_are_empty &&
				!full_source_path_obj.exists() &&
				!full_header_path_obj.exists() {
				return true;
			}

			let full_source_path_obj_parent = full_source_path_obj.parent();
			if full_source_path_obj_parent.is_some() && !full_source_path_obj_parent.as_ref().unwrap().exists() {
				let result = std::fs::create_dir_all(full_source_path_obj_parent.unwrap());
				if !result.is_ok() {
					println!("Could not create directories for writing source files: {}\n{}", full_source_path, result.err().unwrap());
				}
			}

			let full_header_path_obj_parent = full_header_path_obj.parent();
			if full_header_path_obj_parent.is_some() && !full_header_path_obj_parent.as_ref().unwrap().exists() {
				let result = std::fs::create_dir_all(full_header_path_obj_parent.unwrap());
				if !result.is_ok() {
					println!("Could not create directories for writing header files: {}\n{}", full_header_path, result.err().unwrap());
				}
			}

			let source_write = std::fs::write(&full_source_path, transpile_context.output_lines.join("\n"));
			let header_write = std::fs::write(&full_header_path, header_lines.join("\n"));
			if !source_write.is_ok() {
				println!("Could not write to file: {}\n{}", full_source_path, source_write.err().unwrap());
			}
			if !header_write.is_ok() {
				println!("Could not write to file: {}\n{}", full_header_path, header_write.err().unwrap());
			}
		} else {
			println!("\nCOULD NOT WRITE TO FILE: {}", format!("{}{}", dir, file));
		}
	}
	return true;
}

fn get_configure_declaration_with_attributes(isolated: &mut bool, declaration: &str, attributes: &Attributes, content: &str, semicolon: bool) -> String {
	let prepend = attributes.get_attribute_parameters("DeclarePrepend", content);
	let append = attributes.get_attribute_parameters("DeclareAppend", content);
	*isolated = attributes.has_attribute("Isolated");
	let result = format!("{}{}{}{}", 
		if prepend.is_empty() { "".to_string() } else { format!("{}\n", prepend.join("\n")) }, 
		declaration,
		if semicolon { ";" } else { "" },
		if append.is_empty() { "".to_string() } else { format!("\n{}", append.join("\n")) }
	);
	return result;
}

fn configure_declaration_with_attributes(delcarations: &mut Vec<String>, declarations_isolated: &mut Vec<String>, declaration: &str, attributes: &Attributes, content: &str, semicolon: bool) {
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

	let mut data = config_management::read_config_files();

	data.pragma_guard = arguments.contains_key("pragma-guard");
	data.hpp_headers = !arguments.contains_key("h-headers");

	let mut file_contexts = ContextManager::new();//BTreeMap::new();
	let mut file_declarations = BTreeMap::new();
	let mut file_parsers = BTreeMap::new();

	let mut global_context = GlobalContext::new();

	for files in &source_files {
		for f in files.1 {
			let mut parser: Parser = Parser::new("".to_string());
			file_declarations.insert(f.clone(), parse_source_file(&f, &files.0, &data, &mut file_contexts, &mut parser, &mut global_context));
			file_parsers.insert(f, parser);
		}
	}

	for files in &source_files {
		for f in files.1 {
			transpile_source_file(&f, &files.0, &output_dirs, &data, &mut file_contexts, file_declarations.get_mut(f).unwrap(), file_parsers.get_mut(f).unwrap(), &mut global_context);
		}
	}
}
