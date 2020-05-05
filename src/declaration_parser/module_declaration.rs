/**********************************************************
 * --- Module Declaration ---
 *
 * Represents the declarations contained within an
 * individual Tasty Fresh source file.
 **********************************************************/

use crate::config_management::operator_data::OperatorDataStructure;

use crate::declaration_parser::parser::Parser;

use crate::declaration_parser::module_attribute_declaration::ModuleAttributeDeclaration;
use crate::declaration_parser::assume_declaration::AssumeDeclaration;
use crate::declaration_parser::attribute_declaration::AttributeDeclaration;
use crate::declaration_parser::function_declaration::{ FunctionDeclaration, FunctionDeclarationType };
use crate::declaration_parser::import_declaration::ImportDeclaration;
use crate::declaration_parser::include_declaration::IncludeDeclaration;
use crate::declaration_parser::variable_declaration::VariableDeclaration;
use crate::declaration_parser::class_declaration::ClassDeclaration;
use crate::declaration_parser::refurbish_declaration::RefurbishDeclaration;
use crate::declaration_parser::attribute_class_declaration::AttributeClassDeclaration;
use crate::declaration_parser::inject_declaration::InjectDeclaration;
use crate::declaration_parser::attributes::Attributes;

#[derive(Clone)]
pub enum DeclarationType {
	ModuleAttribute(ModuleAttributeDeclaration),
	Assume(AssumeDeclaration, Attributes),
	Function(FunctionDeclaration, Attributes),
	Import(ImportDeclaration, Attributes),
	Include(IncludeDeclaration, Attributes),
	Variable(VariableDeclaration, Attributes),
	Class(ClassDeclaration, Attributes),
	Refurbish(RefurbishDeclaration, Attributes),
	AttributeClass(AttributeClassDeclaration, Attributes),
	Injection(InjectDeclaration, Attributes)
}

pub struct ModuleDeclaration {
	pub declarations: Vec<DeclarationType>
}

macro_rules! parse_declaration {
	($DeclarationClass:ty, $DeclarationType:ident, $parser:expr, $file_name:expr, $declarations:expr, $attributes:expr) => {
		if <$DeclarationClass>::is_declaration($parser) {
			let result = <$DeclarationClass>::new($parser);
			if result.is_error() {
				result.print_error($file_name.to_string(), &$parser.content);
			} else {
				$declarations.push(DeclarationType::$DeclarationType(result.unwrap_and_move(), Attributes::new(if $attributes.is_empty() {
					None
				} else {
					Some(std::mem::replace(&mut $attributes, Vec::new()))
				})));
			}
			$attributes.clear();
			continue;
		}
	}
}

macro_rules! parse_declaration_w_file_name {
	($DeclarationClass:ty, $DeclarationType:ident, $parser:expr, $file_name:expr, $declarations:expr, $attributes:expr) => {
		if <$DeclarationClass>::is_declaration($parser) {
			let result = <$DeclarationClass>::new($parser, $file_name);
			if result.is_error() {
				result.print_error($file_name.to_string(), &$parser.content);
			} else {
				$declarations.push(DeclarationType::$DeclarationType(result.unwrap_and_move(), Attributes::new(if $attributes.is_empty() {
					None
				} else {
					Some(std::mem::replace(&mut $attributes, Vec::new()))
				})));
			}
			$attributes.clear();
			continue;
		}
	}
}

impl ModuleDeclaration {
	pub fn new(parser: &mut Parser, file_name: &str, operator_data: &OperatorDataStructure) -> ModuleDeclaration {
		let mut declarations = Vec::new();
		let mut attributes = Vec::new();

		while !parser.out_of_space {
			parser.parse_whitespace();

			if ModuleAttributeDeclaration::is_declaration(parser) {
				let result = ModuleAttributeDeclaration::new(parser);
				if result.is_error() {
					result.print_error(file_name.to_string(), &parser.content);
				} else {
					declarations.push(DeclarationType::ModuleAttribute(result.unwrap_and_move()));
				}
			} else {
				break;
			}
		}

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
				let result = FunctionDeclaration::new(parser, FunctionDeclarationType::ModuleLevel, None);
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
				continue;
			}

			if ClassDeclaration::is_declaration(parser) {
				let result = ClassDeclaration::new(parser, file_name, operator_data);
				if result.is_error() {
					result.print_error(file_name.to_string(), &parser.content);
				} else {
					declarations.push(DeclarationType::Class(result.unwrap_and_move(), Attributes::new(if attributes.is_empty() {
						None
					} else {
						Some(std::mem::replace(&mut attributes, Vec::new()))
					})));
				}
				attributes.clear();
				continue;
			}

			if RefurbishDeclaration::is_declaration(parser) {
				let result = RefurbishDeclaration::new(parser, file_name, operator_data);
				if result.is_error() {
					result.print_error(file_name.to_string(), &parser.content);
				} else {
					declarations.push(DeclarationType::Refurbish(result.unwrap_and_move(), Attributes::new(if attributes.is_empty() {
						None
					} else {
						Some(std::mem::replace(&mut attributes, Vec::new()))
					})));
				}
				attributes.clear();
				continue;
			}

			parse_declaration!(AssumeDeclaration, Assume, parser, file_name, declarations, attributes);
			parse_declaration!(ImportDeclaration, Import, parser, file_name, declarations, attributes);
			parse_declaration!(IncludeDeclaration, Include, parser, file_name, declarations, attributes);
			parse_declaration!(VariableDeclaration, Variable, parser, file_name, declarations, attributes);
			parse_declaration!(InjectDeclaration, Injection, parser, file_name, declarations, attributes);

			parse_declaration_w_file_name!(AttributeClassDeclaration, AttributeClass, parser, file_name, declarations, attributes);

			if !parser.out_of_space { parser.increment(); }

			if parser.index == initial_index {
				break;
			}
		}

		return ModuleDeclaration {
			declarations: declarations
		}
	}
}