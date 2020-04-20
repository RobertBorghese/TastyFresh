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

use crate::expression::variable_type::Type;

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;

use crate::declaration_parser::module_declaration::DeclarationType;
use crate::declaration_parser::attribute_declaration::AttributeDeclaration;
use crate::declaration_parser::function_declaration::{ FunctionDeclaration, FunctionDeclarationType };
use crate::declaration_parser::variable_declaration::VariableDeclaration;
use crate::declaration_parser::attributes::Attributes;

type ClassDeclarationResult = DeclarationResult<ClassDeclaration>;

pub struct ClassDeclaration {
	pub name: String,
	pub class_type: ClassType,
	pub extensions: Option<Vec<Type>>,
	pub declarations: Vec<DeclarationType>
}

pub enum ClassType {
	Class,
	Abstract,
	Enum
}

impl ClassType {
	pub fn get_name(&self) -> &str {
		return match self { ClassType::Class => "class", ClassType::Abstract => "abstract", ClassType::Enum => "enum" };
	}

	pub fn new(index: i32) -> ClassType {
		return match index { 0 => ClassType::Class, 1 => ClassType::Abstract, 2 => ClassType::Enum, _ => panic!("Could not generate ClassType from number!") };
	}
}

impl Declaration<ClassDeclaration> for ClassDeclaration {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of function";
	}
}

impl ClassDeclaration {
	pub fn new(parser: &mut Parser, file_name: &str) -> ClassDeclarationResult {

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
		let mut attributes = Vec::new();

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

			if FunctionDeclaration::is_declaration(parser) {
				let result = FunctionDeclaration::new(parser, FunctionDeclarationType::ClassLevel);
				if result.is_error() {
					result.print_error(file_name.to_string(), &parser.content);
				} else {
					declarations.push(DeclarationType::Function(result.unwrap_and_move(), Attributes::new(if attributes.is_empty() {
						None
					} else {
						Some(std::mem::replace(&mut attributes, Vec::new()))
					})));
				}
				attributes.clear();
				parser.increment();
				continue;
			}

			if VariableDeclaration::is_declaration(parser) {
				let result = VariableDeclaration::new(parser);
				if result.is_error() {
					result.print_error(file_name.to_string(), &parser.content);
				} else {
					declarations.push(DeclarationType::Variable(result.unwrap_and_move(), Attributes::new(if attributes.is_empty() {
						None
					} else {
						Some(std::mem::replace(&mut attributes, Vec::new()))
					})));
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
			class_type: ClassType::new(class_type),
			declarations: declarations,
			extensions: if type_extensions.is_empty() { None } else { Some(type_extensions) }
		});
	}

	pub fn is_declaration(parser: &mut Parser) -> bool {
		return Self::is_class_declaration(&parser.content, parser.index);
	}

	pub fn is_class_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return declare.starts_with("class ") || declare.starts_with("abstract ") || declare.starts_with("enum ");
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
}