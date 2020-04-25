/**********************************************************
 * --- Transpiler ---
 *
 * Transpiles the parsed code into C++.
 **********************************************************/

use crate::expression::Expression;
use crate::expression::expression_parser::ExpressionEndReason;
use crate::expression::variable_type::VariableType;

use crate::context_management::position::Position;
use crate::context_management::global_context::GlobalContext;
use crate::context_management::context_manager::ContextManager;

use crate::declaration_parser::parser::Parser;
use crate::declaration_parser::module_declaration::DeclarationType;
use crate::declaration_parser::variable_declaration::VariableExportType;

use crate::config_management::ConfigData;

use crate::scope_parser::ScopeExpression;

use crate::context_management::print_code_error;

use std::rc::Rc;

use regex::Regex;

lazy_static! {
	pub static ref LINE_SPLIT: Regex = Regex::new("(?:\n\r|\r\n|\r|\n)").unwrap();
}

use crate::{
	configure_declaration_with_attributes,
	get_configure_declaration_with_attributes,
	insert_output_line
};

pub struct VarFuncDeclarations {
	pub variable_declarations: Vec<String>,
	pub function_declarations: Vec<String>,
	pub variable_declarations_isolated: Vec<String>,
	pub function_declarations_isolated: Vec<String>
}

impl VarFuncDeclarations {
	pub fn new() -> VarFuncDeclarations {
		return VarFuncDeclarations {
			variable_declarations: Vec::new(),
			function_declarations: Vec::new(),
			variable_declarations_isolated: Vec::new(),
			function_declarations_isolated: Vec::new()
		}
	}

	fn push_line(d: String, lines: &mut Vec<String>, tab_count: usize, tabs: &str) {
		if tab_count > 0 {
			for line in LINE_SPLIT.split(&d) {
				if line.trim().is_empty() {
					lines.push("".to_string());
				} else {
					lines.push(format!("{}{}", tabs, line));
				}
			}
		} else {
			lines.push(d);
		}
	}

	pub fn is_empty(&self) -> bool {
		return self.variable_declarations_isolated.is_empty() &&
			self.variable_declarations.is_empty() &&
			self.function_declarations_isolated.is_empty() &&
			self.function_declarations.is_empty();
	}

	pub fn export_to_lines(self, lines: &mut Vec<String>, tab_count: usize, variable_first: bool) {
		let tabs = if tab_count > 0 {
			String::from_utf8(vec![b'\t'; tab_count]).unwrap_or("".to_string())
		} else {
			"".to_string()
		};
		if variable_first {
			Self::export_isolated(self.variable_declarations_isolated, lines, tab_count, &tabs);
			Self::export_normal(self.variable_declarations, lines, tab_count, &tabs);
			Self::export_isolated(self.function_declarations_isolated, lines, tab_count, &tabs);
			Self::export_normal(self.function_declarations, lines, tab_count, &tabs);
		} else {
			Self::export_isolated(self.function_declarations_isolated, lines, tab_count, &tabs);
			Self::export_normal(self.function_declarations, lines, tab_count, &tabs);
			Self::export_isolated(self.variable_declarations_isolated, lines, tab_count, &tabs);
			Self::export_normal(self.variable_declarations, lines, tab_count, &tabs);
		}
	}

	fn export_isolated(declares: Vec<String>, lines: &mut Vec<String>, tab_count: usize, tabs: &str) {
		if !declares.is_empty() {
			for d in declares {
				Self::push_line(d, lines, tab_count, &tabs);
				lines.push("".to_string());
			}
		}
	}

	fn export_normal(declares: Vec<String>, lines: &mut Vec<String>, tab_count: usize, tabs: &str) {
		if !declares.is_empty() {
			for d in declares {
				Self::push_line(d, lines, tab_count, &tabs);
			}
			lines.push("".to_string());
		}
	}
}

pub struct Transpiler<'a> {
	pub output_lines: Vec<String>,

	pub declarations: VarFuncDeclarations,
	pub class_declarations: Vec<(String,VarFuncDeclarations,VarFuncDeclarations,VarFuncDeclarations)>,
	
	pub handling_module_attributes: bool,
	pub header_include_line: Option<usize>,
	pub end_line: usize,

	pub header_system_includes: Vec<String>,
	pub header_local_includes: Vec<String>,

	pub file: &'a str,
	pub access_file_path: &'a str,
	pub config_data: &'a ConfigData,
	pub module_contexts: &'a mut ContextManager,
	pub parser: &'a mut Parser
}

impl<'a> Transpiler<'a> {
	pub fn new(file: &'a str, access_file_path: &'a str, config_data: &'a ConfigData, module_contexts: &'a mut ContextManager, parser: &'a mut Parser) -> Transpiler<'a> {
		return Transpiler {
			output_lines: Vec::new(),

			declarations: VarFuncDeclarations::new(),
			class_declarations: Vec::new(),
			
			handling_module_attributes: true,
			header_include_line: None,
			end_line: 0,

			header_system_includes: Vec::new(),
			header_local_includes: Vec::new(),

			file: file,
			access_file_path: access_file_path,
			config_data: config_data,
			module_contexts: module_contexts,
			parser: parser
		}
	}

	pub fn parse_declarations(&mut self, declarations: &mut Vec<DeclarationType>, global_context: &GlobalContext, mut class_declarations: Option<(&str, &mut VarFuncDeclarations, &mut VarFuncDeclarations, &mut VarFuncDeclarations)>) {
		let is_class_declare = !class_declarations.is_none();

		//let mut declarations_clone = declarations.clone();
		for declaration in declarations.iter_mut() {
			match declaration {
				DeclarationType::Variable(var_data, attributes) => {
					attributes.flatten_attributes(global_context, self.parser.content.as_str());

					let mut context = self.module_contexts.take_context(self.access_file_path);
					let mut reason = ExpressionEndReason::Unknown;
					let mut expr: Option<Rc<Expression>> = None;
					if var_data.value.is_some() {
						self.parser.reset(var_data.value.unwrap().0, var_data.line);
						expr = Some(self.parser.parse_expression(self.file.to_string(), self.config_data, Some(&mut context), self.module_contexts, &mut reason, Some(var_data.var_type.clone())));
						if var_data.var_type.is_inferred() {
							var_data.var_type.var_type = expr.as_ref().unwrap().get_type().var_type;
						}
					}
					let var_type = &var_data.var_type;
					let line = if context.align_lines { var_data.line } else {
						if self.end_line > var_data.line || var_data.line - self.end_line < 2 {
							self.output_lines.len()
						} else {
							self.output_lines.len() + 1
						}
					};
					if is_class_declare {
						context.typing.add_variable(var_data.name.clone(), var_data.var_type.clone(), None);
					} else {
						context.module.add_variable(var_data.name.clone(), var_data.var_type.clone(), Some(self.module_contexts));
					}
					if !is_class_declare || var_data.is_only_static() {
						insert_output_line(&mut self.output_lines,
							&var_data.to_cpp(&expr,
								&self.config_data.operators,
								&mut context,
								if is_class_declare && var_data.is_only_static() {
									VariableExportType::ClassSource(class_declarations.as_ref().unwrap().0)
								} else {
									VariableExportType::ModuleSource
								}
							),
							line,
							false,
						);
					}
					self.end_line = var_data.line;
					let add_to_header = !attributes.has_attribute("NoHeader");
					if add_to_header {
						if !is_class_declare {
							let var_declaraction = format!("extern {} {}", var_type.to_cpp(), var_data.name);
							configure_declaration_with_attributes(
								&mut self.declarations.variable_declarations,
								&mut self.declarations.variable_declarations_isolated,
								&var_declaraction,
								&attributes,
								&self.parser.content,
								true
							);
						} else {
							let var_declaraction = if is_class_declare && var_data.is_only_static() {
								format!("static {} {} ", var_type.to_cpp(), var_data.name)
							} else {
								var_data.to_cpp(&expr, &self.config_data.operators, &mut context, if is_class_declare {
									VariableExportType::ClassHeader
								} else {
									VariableExportType::ModuleHeader
								})
							};
							let temp = &mut class_declarations.as_mut().unwrap().2;
							configure_declaration_with_attributes(
								&mut temp.variable_declarations,
								&mut temp.variable_declarations_isolated,
								&var_declaraction[0..var_declaraction.len() - 1],
								&attributes,
								&self.parser.content,
								true
							);
						};
					}

					self.module_contexts.add_context(self.access_file_path.to_string(), context);
				},
				_ => ()
			}
		}

		for declaration in declarations {
			// Module Attributes
			if let DeclarationType::ModuleAttribute(module_attribute) = declaration {
				let context = self.module_contexts.get_context(self.access_file_path);
				context.register_module_attribute(&module_attribute.name);
				if context.align_lines {
					self.header_include_line = Some(module_attribute.line);
				}
				continue;
			} else if self.handling_module_attributes {
				self.handling_module_attributes = false;
			}

			// All Others
			match declaration {
				DeclarationType::Class(class_declare, attributes) => {
					attributes.flatten_attributes(global_context, self.parser.content.as_str());

					let mut construct_declares = VarFuncDeclarations::new();
					let mut public_declares = VarFuncDeclarations::new();
					let mut private_declares = VarFuncDeclarations::new();

					{
						let context = self.module_contexts.get_context(self.access_file_path);
						context.typing.push_context();
						context.typing.add_variable("this".to_string(), VariableType::this(), None);
					}
					self.parse_declarations(&mut class_declare.declarations, global_context, Some((&class_declare.name, &mut construct_declares, &mut public_declares, &mut private_declares)));
					{
						let context = self.module_contexts.get_context(self.access_file_path);
						context.typing.pop_context();
					}

					if class_declare.declaration_id != 0 {
						let mut context = self.module_contexts.take_context(self.access_file_path);
						let class_data = class_declare.to_class(&mut context, self.module_contexts, &self.parser.content);
						self.module_contexts.add_context(self.access_file_path.to_string(), context);
						self.module_contexts.update_class(class_declare.declaration_id, class_data);
					}

					let mut isolated = false;
					let mut class_content = get_configure_declaration_with_attributes(
						&mut isolated,
						&class_declare.to_cpp(attributes, self.parser.content.as_str()),
						&attributes,
						&self.parser.content,
						false
					);

					if isolated {
						class_content += "\n";
					}

					self.class_declarations.push((class_content, construct_declares, public_declares, private_declares));
				},
				DeclarationType::Injection(injection, _attributes) => {
					let context = self.module_contexts.get_context(self.access_file_path);
					let mut line = if context.align_lines { injection.line } else { self.output_lines.len() + 1 };
					let injection = if context.align_lines {
						&self.parser.content[injection.start_index..injection.end_index]
					} else {
						&self.parser.content[injection.start_index..injection.end_index].trim()
					};
					for inject_line in LINE_SPLIT.split(injection) {
						if !context.align_lines && inject_line.trim().is_empty() { continue; }
						insert_output_line(&mut self.output_lines, inject_line, line, false);
						line += 1;
					}
				},
				DeclarationType::Assume(_assume, _attributes) => {
				},
				DeclarationType::Import(import, _attributes) => {
					if self.module_contexts.module_exists(&import.path) {
						let context = self.module_contexts.get_context(self.access_file_path);
						let typing = &mut context.module;
						typing.add(import.path.clone());
						let line = if context.align_lines { import.line } else { self.output_lines.len() };
						insert_output_line(&mut self.output_lines, if self.config_data.hpp_headers { 
							format!("#include \"{}.hpp\"", import.path)
						} else {
							format!("#include \"{}.h\"", import.path)
						}.as_str(), line, false);
					} else {
						let pos = Position::new(self.file.to_string(), Some(import.line + 1), 7, Some(7 + import.path.len()));
						print_code_error("Import Not Found", "could not find Tasty Fresh source file", &pos, &self.parser.content)
					}
				},
				DeclarationType::Include(include, _attributes) => {
					if include.location.is_header() {
						if include.inc_type.is_local() {
							self.header_local_includes.push(include.path.clone());
						} else {
							self.header_system_includes.push(include.path.clone());
						}
					} else {
						let context = self.module_contexts.get_context(self.access_file_path);
						let line = if context.align_lines { include.line } else { self.output_lines.len() };
						insert_output_line(&mut self.output_lines, format!("#include {}", if include.inc_type.is_local() {
							format!("\"{}\"", include.path)
						} else {
							format!("<{}>", include.path)
						}).as_str(), line, false);
					}
					
				},
				DeclarationType::Function(func_data, attributes) => {
					println!("FUNC IS TRANSPILED");
					attributes.flatten_attributes(global_context, self.parser.content.as_str());

					let mut context = self.module_contexts.take_context(self.access_file_path);
					let mut func_content: Option<String> = None;
					let mut line = if context.align_lines { func_data.line } else { self.output_lines.len() + 1 };
					let add_to_header = !attributes.has_attribute("NoHeader");
					self.end_line = line;
					if !func_data.header_only() {
						if func_data.start_index.is_some() && func_data.end_index.is_some() {
							context.typing.push_context();
							for param in &func_data.parameters {
								context.typing.add_variable(param.1.clone(), param.0.clone(), None);
							}
							let scope = ScopeExpression::new(self.parser, None, func_data.start_index.unwrap(), func_data.line, self.file, self.config_data, &mut context, self.module_contexts, Some(func_data.return_type.clone()));
							func_content = Some(scope.to_string(&self.config_data.operators, func_data.line, 1, &mut context));
							context.typing.pop_context();
						}
						let func_declaration = func_data.to_function(&self.parser.content).to_cpp(false, false,
							if is_class_declare { Some(class_declarations.as_ref().unwrap().0) } else { None },
							&func_data.function_type
						);
						insert_output_line(&mut self.output_lines, &func_declaration, line, false);
						if func_content.is_some() {
							let original_line = line;
							insert_output_line(&mut self.output_lines, "{", line, false);
							for func_line in LINE_SPLIT.split(&func_content.unwrap()) {
								insert_output_line(&mut self.output_lines, func_line, line, false);
								line += 1;
							}
							insert_output_line(&mut self.output_lines, "}", if original_line == line - 1 { original_line } else { line }, false);
						} else {
							insert_output_line(&mut self.output_lines, ";", line, false);
						}
						self.end_line = func_data.line + (line - self.end_line);
					}
					if add_to_header {
						let header_func_declare = func_data.to_function(&self.parser.content).to_cpp(true,
							true,
							if is_class_declare { Some(class_declarations.as_ref().unwrap().0) } else { None },
							&func_data.function_type
						);
						if !is_class_declare {
							configure_declaration_with_attributes(
								&mut self.declarations.function_declarations,
								&mut self.declarations.function_declarations_isolated,
								&header_func_declare,
								&attributes,
								&self.parser.content,
								true
							);
						} else {
							if func_data.function_type.is_constructor_or_destructor() {
								let temp = &mut class_declarations.as_mut().unwrap().1;
								configure_declaration_with_attributes(
									&mut temp.function_declarations,
									&mut temp.function_declarations_isolated,
									&header_func_declare,
									&attributes,
									&self.parser.content,
									true
								);
							} else {
								let temp = &mut class_declarations.as_mut().unwrap().2;
								configure_declaration_with_attributes(
									&mut temp.function_declarations,
									&mut temp.function_declarations_isolated,
									&header_func_declare,
									&attributes,
									&self.parser.content,
									true
								);
							}
						}

						self.module_contexts.add_context(self.access_file_path.to_string(), context);
					}
				},
				_ => {
				}
			}
		}
	}
}