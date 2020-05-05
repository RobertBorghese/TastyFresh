/**********************************************************
 * --- Class Declaration ---
 *
 * Represents the a class (or class-like) declaration.
 **********************************************************/

use crate::{
	declare_parse_whitespace,
	declare_parse_required_whitespace,
	declare_parse_ascii,
	declare_parse_required_ascii,
	declare_parse_required_next_char,
	declare_parse_type
};

use crate::config_management::operator_data::OperatorDataStructure;

use crate::context_management::context::Context;
use crate::context_management::context_manager::ContextManager;

use crate::expression::variable_type::Type;
use crate::expression::value_type::{ ClassType, Property, Function };

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;
use crate::declaration_parser::module_declaration::DeclarationType;
use crate::declaration_parser::attribute_declaration::AttributeDeclaration;
use crate::declaration_parser::function_declaration::{ FunctionDeclaration, FunctionDeclarationType };
use crate::declaration_parser::variable_declaration::VariableDeclaration;
use crate::declaration_parser::attributes::Attributes;

use std::collections::BTreeMap;

use regex::Regex;

lazy_static! {
	pub static ref CLASS_REGEX: Regex = Regex::new(r"^\b(?:class|enum|abstract)\b").unwrap();
	pub static ref FORWARD_REGEX: Regex = Regex::new(r"^\b(?:forward)\b").unwrap();
}

type ClassDeclarationResult = DeclarationResult<ClassDeclaration>;

#[derive(Clone)]
pub struct ClassDeclaration {
	pub name: String,
	pub class_type: ClassStyle,
	pub extensions: Option<Vec<Type>>,
	pub declarations: Vec<DeclarationType>,
	pub abstract_declarations: Option<Vec<DeclarationType>>,
	pub declaration_id: usize
}

#[derive(Clone, PartialEq)]
pub enum ClassStyle {
	Class,
	Abstract,
	Enum
}

impl ClassStyle {
	pub fn get_name(&self) -> &str {
		return match self { ClassStyle::Class => "class", ClassStyle::Abstract => "abstract", ClassStyle::Enum => "enum" };
	}

	pub fn new(index: i32) -> ClassStyle {
		return match index { 0 => ClassStyle::Class, 1 => ClassStyle::Abstract, 2 => ClassStyle::Enum, _ => panic!("Could not generate ClassType from number!") };
	}

	pub fn is_abstract(&self) -> bool {
		if let ClassStyle::Abstract = self {
			return true;
		}
		return false;
	}
}

impl Declaration<ClassDeclaration> for ClassDeclaration {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of function";
	}
}

impl ClassDeclaration {
	pub fn new(parser: &mut Parser, file_name: &str, operator_data: &OperatorDataStructure) -> ClassDeclarationResult {

		// Parse Var Style
		let mut class_keyword = "".to_string();
		declare_parse_ascii!(class_keyword, parser);
		let class_type = match class_keyword.as_str() { "class" => 0, "abstract" => 1, "enum" => 2, _ => 3 };
		if class_type == 3 {
			return ClassDeclarationResult::Err("Unexpected Keyword", "\"class\" or \"abstract\" or \"enum\" keyword expected", parser.index - class_keyword.len(), parser.index);
		}

		declare_parse_required_whitespace!(parser);

		// Parse Class Name
		let mut class_name = "".to_string();
		declare_parse_required_ascii!(class_name, "Class Name Missing", "class name missing", parser);

		declare_parse_required_whitespace!(parser);

		let mut type_extensions = Vec::new();
		if parser.get_curr() != '{' {
			let mut extend_keyword = "".to_string();
			declare_parse_ascii!(extend_keyword, parser);
			if extend_keyword == "extends" {
				if class_type != 0 {
					return ClassDeclarationResult::Err("Unexpected Keyword", "\"extends\" can only be used with \"class\"", parser.index - extend_keyword.len(), parser.index);
				}
				let mut and_text = "".to_string();
				declare_parse_required_whitespace!(parser);
				loop {
					let var_type: Type;
					declare_parse_type!(var_type, parser);
					type_extensions.push(var_type);
					declare_parse_whitespace!(parser);
					if parser.get_curr() != '{' {
						declare_parse_ascii!(and_text, parser);
						declare_parse_required_whitespace!(parser);
						if and_text != "and" {
							return ClassDeclarationResult::Err("Unexpected Keyword", "\"and\" keyword expected", parser.index - and_text.len(), parser.index);
						}
					} else {
						break;
					}
				}
			} else if extend_keyword == "becomes" {
				if class_type != 1 {
					return ClassDeclarationResult::Err("Unexpected Keyword", "\"becomes\" can only be used with \"abstract\"", parser.index - extend_keyword.len(), parser.index);
				}
				declare_parse_required_whitespace!(parser);
				let var_type: Type;
				declare_parse_type!(var_type, parser);
				type_extensions.push(var_type);
				declare_parse_whitespace!(parser);
			}
		}

		let mut next_char = ' ';
		declare_parse_required_next_char!('{', next_char, parser);

		let mut declarations = Vec::new();
		let mut abstract_declarations = if class_type == 1 { Some(Vec::new()) } else { None };
		let mut attributes = Vec::new();
		let mut forward = false;

		while !parser.out_of_space {
			parser.parse_whitespace();

			let initial_index = parser.index;

			if AttributeDeclaration::is_declaration(parser) {
				let result = AttributeDeclaration::new(parser, false);
				if result.is_error() {
					result.print_error(file_name.to_string(), &parser.content);
				} else {
					attributes.push(result.unwrap_and_move());
				}
				continue;
			}

			if class_type == 1 {
				if FORWARD_REGEX.is_match(&parser.content[parser.index..]) {
					forward = true;
					for _ in 0..7 { parser.increment(); }
					parser.parse_whitespace();
				} else {
					forward = false;
				}
			}

			if FunctionDeclaration::is_declaration(parser) {
				let result = FunctionDeclaration::new(parser, if forward {
					FunctionDeclarationType::Forward
				} else {
					FunctionDeclarationType::ClassLevel
				}, Some(operator_data));
				if result.is_error() {
					result.print_error(file_name.to_string(), &parser.content);
				} else {
					let dec_type = DeclarationType::Function(result.unwrap_and_move(), Attributes::new(if attributes.is_empty() {
						None
					} else {
						Some(std::mem::replace(&mut attributes, Vec::new()))
					}));
					if class_type == 1 {
						if forward {
							declarations.push(dec_type);
						} else {
							abstract_declarations.as_mut().unwrap().push(dec_type);
						}
					} else {
						declarations.push(dec_type);
					}	
				}
				attributes.clear();
				parser.increment();
				continue;
			}

			if VariableDeclaration::is_declaration(parser) {
				if !forward && class_type == 1 {
					return ClassDeclarationResult::Err("No Variables in Abstract", "cannot add variable fields to \"abstracts\"", parser.index, parser.index);
				}
				let result = VariableDeclaration::new(parser);
				if result.is_error() {
					result.print_error(file_name.to_string(), &parser.content);
				} else {
					let dec_type = DeclarationType::Variable(result.unwrap_and_move(), Attributes::new(if attributes.is_empty() {
						None
					} else {
						Some(std::mem::replace(&mut attributes, Vec::new()))
					}));
					if class_type == 1 {
						if forward {
							declarations.push(dec_type);
						} else {
							abstract_declarations.as_mut().unwrap().push(dec_type);
						}
					} else {
						declarations.push(dec_type);
					}
				}
				attributes.clear();
				parser.increment();
				continue;
			}

			if parser.get_curr() == '}' {
				break;
			}

			if !parser.out_of_space {
				parser.increment();
			}

			if parser.index == initial_index {
				break;
			}
		}

		return ClassDeclarationResult::Ok(ClassDeclaration {
			name: class_name,
			class_type: ClassStyle::new(class_type),
			declarations: declarations,
			abstract_declarations: abstract_declarations,
			extensions: if type_extensions.is_empty() { None } else { Some(type_extensions) },
			declaration_id: 0
		});
	}

	pub fn is_declaration(parser: &mut Parser) -> bool {
		return Self::is_class_declaration(&parser.content, parser.index);
	}

	pub fn is_class_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return CLASS_REGEX.is_match(declare);
	}

	pub fn to_cpp(&self, attributes: &Attributes, content: &str) -> String {
		return format!("{}{}{}{}{}{}{{",
			self.class_type.get_name(),
			if attributes.has_attribute("DeclarePreName") {
				format!(" {} ", attributes.get_attribute_parameters("DeclarePreName", content).join(" "))
			} else {
				" ".to_string()
			},
			self.name,
			if attributes.has_attribute("DeclarePostName") {
				format!(" {}{}", attributes.get_attribute_parameters("DeclarePostName", content).join(" "),
					if self.extensions.is_none() { "" } else { " " })
			} else {
				"".to_string()
			},
			if self.extensions.is_none() {
				"".to_string()
			} else {
				format!(": {}", self.extensions.as_ref().unwrap().iter().map(|cls| format!("public {}", cls.to_cpp(false))).collect::<Vec<String>>().join(", "))
			},
			if attributes.has_attribute("DeclarePreBracket") {
				format!(" {} ", attributes.get_attribute_parameters("DeclarePreBracket", content).join(" "))
			} else {
				" ".to_string()
			}
		);
	}

	pub fn to_class(&self, context: &mut Context, manager: &mut ContextManager, content: &str, attributes: &Attributes) -> ClassType {
		let mut properties = Vec::new();
		let mut functions = Vec::new();
		let mut operators: BTreeMap<usize,Vec<Function>> = BTreeMap::new();
		for declaration in &self.declarations {
			match declaration {
				DeclarationType::Function(d, _) => {
					if d.function_type.is_operator() {
						let op_type = d.function_type.get_operator_type();
						let base_id = match op_type.as_str() { "suffix" => 100, "prefix" => 200, "infix" => 300, _ => panic!("Invalid operator type") };
						let op_id = d.function_type.get_operator_id() + base_id;
						if operators.contains_key(&op_id) {
							operators.get_mut(&op_id).unwrap().push(d.to_function(content));
						} else {
							let op_funcs = vec![d.to_function(content)];
							operators.insert(op_id, op_funcs);
						}
					} else {
						functions.push(d.to_function(content));
					}
					for p in &d.parameters {
						context.register_type(&p.0);
					}
					context.register_type(&d.return_type);
				},
				DeclarationType::Variable(d, _) => {
					let mut prop = d.var_type.clone();
					prop.resolve(context, manager);
					properties.push(Property {
						name: d.name.clone(),
						prop_type: prop,
						default_value: None,
						is_declare: false
					});
					context.register_type(&d.var_type);
				},
				_ => ()
			}
		}
		return ClassType {
			name: self.name.clone(),
			style: self.class_type.clone(),
			extensions: self.extensions.clone(),
			type_params: None,
			properties: properties,
			functions: functions,
			operators: operators,
			required_includes: attributes.get_required_includes()
		};
	}
}