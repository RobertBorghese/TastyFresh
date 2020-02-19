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

use crate::declaration_parser::assume_declaration::AssumeDeclaration;
use crate::declaration_parser::attribute_declaration::AttributeDeclaration;
use crate::declaration_parser::function_declaration::{ FunctionDeclaration, FunctionDeclarationType };
use crate::declaration_parser::import_declaration::ImportDeclaration;
use crate::declaration_parser::include_declaration::IncludeDeclaration;
use crate::declaration_parser::variable_declaration::VariableDeclaration;

pub enum DeclarationType {
	Assume(AssumeDeclaration),
	Attribute(AttributeDeclaration),
	Function(FunctionDeclaration),
	Import(ImportDeclaration),
	Include(IncludeDeclaration),
	Variable(VariableDeclaration)
}

pub struct ModuleDeclaration {
	pub declarations: Vec<DeclarationType>
}

macro_rules! parse_declaration {
	($DeclarationClass:ty, $DeclarationType:ident, $parser:expr, $file_name:expr, $declarations:expr) => {
		if <$DeclarationClass>::is_declaration($parser) {
			let mut result = <$DeclarationClass>::new($parser);
			if result.is_error() {
				result.print_error($file_name.to_string(), Some($parser.content));
			} else {
				$declarations.push(DeclarationType::$DeclarationType(result.unwrap_and_move()));
			}
		}
	}
}

impl ModuleDeclaration {
	pub fn new(parser: &mut Parser, file_name: &str) -> ModuleDeclaration {
		let mut declarations = Vec::new();

		while !parser.out_of_space {
			parser.parse_whitespace();

			let initial_index = parser.index;

			if FunctionDeclaration::is_declaration(parser) {
				let mut result = FunctionDeclaration::new(parser, FunctionDeclarationType::ModuleLevel);
				if result.is_error() {
					result.print_error(file_name.to_string(), Some(parser.content));
				} else {
					declarations.push(DeclarationType::Function(result.unwrap_and_move()));
				}
			}

			parse_declaration!(AssumeDeclaration, Assume, parser, file_name, declarations);
			parse_declaration!(AttributeDeclaration, Attribute, parser, file_name, declarations);
			parse_declaration!(ImportDeclaration, Import, parser, file_name, declarations);
			parse_declaration!(IncludeDeclaration, Include, parser, file_name, declarations);
			parse_declaration!(VariableDeclaration, Variable, parser, file_name, declarations);

			if !parser.out_of_space { parser.increment(); }

			if parser.index == initial_index {
				println!("BROKEN");
				break;
			}
		}

		println!("{} - {}", file_name, declarations.len());

		return ModuleDeclaration {
			declarations: declarations
		}
	}
}