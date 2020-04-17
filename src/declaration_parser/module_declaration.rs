/**********************************************************
 * --- Module Declaration ---
 *
 * Represents the declarations contained within an
 * individual Tasty Fresh source file.
 **********************************************************/

use crate::expression::variable_type::VariableType;
use crate::expression::variable_type::{ Type, VarStyle };

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;

use crate::declaration_parser::module_attribute_declaration::ModuleAttributeDeclaration;
use crate::declaration_parser::assume_declaration::AssumeDeclaration;
use crate::declaration_parser::attribute_declaration::AttributeDeclaration;
use crate::declaration_parser::function_declaration::{ FunctionDeclaration, FunctionDeclarationType };
use crate::declaration_parser::import_declaration::ImportDeclaration;
use crate::declaration_parser::include_declaration::IncludeDeclaration;
use crate::declaration_parser::variable_declaration::VariableDeclaration;
use crate::declaration_parser::class_declaration::ClassDeclaration;
use crate::declaration_parser::attribute_class_declaration::AttributeClassDeclaration;

pub enum DeclarationType {
	ModuleAttribute(ModuleAttributeDeclaration),
	Assume(AssumeDeclaration, Option<Vec<AttributeDeclaration>>),
	Function(FunctionDeclaration, Option<Vec<AttributeDeclaration>>),
	Import(ImportDeclaration, Option<Vec<AttributeDeclaration>>),
	Include(IncludeDeclaration, Option<Vec<AttributeDeclaration>>),
	Variable(VariableDeclaration, Option<Vec<AttributeDeclaration>>),
	Class(ClassDeclaration, Option<Vec<AttributeDeclaration>>),
	AttributeClass(AttributeClassDeclaration, Option<Vec<AttributeDeclaration>>)
}

pub struct ModuleDeclaration {
	pub declarations: Vec<DeclarationType>
}

macro_rules! parse_declaration {
	($DeclarationClass:ty, $DeclarationType:ident, $parser:expr, $file_name:expr, $declarations:expr, $attributes:expr) => {
		if <$DeclarationClass>::is_declaration($parser) {
			let mut result = <$DeclarationClass>::new($parser);
			if result.is_error() {
				result.print_error($file_name.to_string(), &$parser.content);
			} else {
				$declarations.push(DeclarationType::$DeclarationType(result.unwrap_and_move(), if $attributes.is_empty() {
					None
				} else {
					Some(std::mem::replace(&mut $attributes, Vec::new()))
				}));
			}
			$attributes.clear();
			continue;
		}
	}
}

macro_rules! parse_declaration_w_file_name {
	($DeclarationClass:ty, $DeclarationType:ident, $parser:expr, $file_name:expr, $declarations:expr, $attributes:expr) => {
		if <$DeclarationClass>::is_declaration($parser) {
			let mut result = <$DeclarationClass>::new($parser, $file_name);
			if result.is_error() {
				result.print_error($file_name.to_string(), &$parser.content);
			} else {
				$declarations.push(DeclarationType::$DeclarationType(result.unwrap_and_move(), if $attributes.is_empty() {
					None
				} else {
					Some(std::mem::replace(&mut $attributes, Vec::new()))
				}));
			}
			$attributes.clear();
			continue;
		}
	}
}

impl ModuleDeclaration {
	pub fn new(parser: &mut Parser, file_name: &str) -> ModuleDeclaration {
		let mut declarations = Vec::new();
		let mut attributes = Vec::new();

		while !parser.out_of_space {
			parser.parse_whitespace();

			if ModuleAttributeDeclaration::is_declaration(parser) {
				let mut result = ModuleAttributeDeclaration::new(parser);
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
				let mut result = AttributeDeclaration::new(parser, false);
				if result.is_error() {
					result.print_error(file_name.to_string(), &parser.content);
				} else {
					attributes.push(result.unwrap_and_move());
				}
				continue;
			}

			if FunctionDeclaration::is_declaration(parser) {
				let mut result = FunctionDeclaration::new(parser, FunctionDeclarationType::ModuleLevel);
				if result.is_error() {
					result.print_error(file_name.to_string(), &parser.content);
				} else {
					declarations.push(DeclarationType::Function(result.unwrap_and_move(), if attributes.is_empty() {
						None
					} else {
						Some(std::mem::replace(&mut attributes, Vec::new()))
					}));
				}
				attributes.clear();
				continue;
			}

			parse_declaration_w_file_name!(ClassDeclaration, Class, parser, file_name, declarations, attributes);

			parse_declaration!(AssumeDeclaration, Assume, parser, file_name, declarations, attributes);
			parse_declaration!(ImportDeclaration, Import, parser, file_name, declarations, attributes);
			parse_declaration!(IncludeDeclaration, Include, parser, file_name, declarations, attributes);
			parse_declaration!(VariableDeclaration, Variable, parser, file_name, declarations, attributes);

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