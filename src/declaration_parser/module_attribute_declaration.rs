/**********************************************************
 * --- Module Attribute Declaration ---
 *
 * Represents a variable declaration prior to being parsed.
 **********************************************************/

use crate::{
	declare_parse_whitespace,
	declare_parse_required_whitespace,
	declare_parse_ascii,
	declare_parse_required_ascii,
	declare_parse_required_next_char,
	declare_parse_expr_until_either_char,
	declare_parse_type
};

use crate::expression::variable_type::VariableType;
use crate::expression::variable_type::{ Type, VarStyle };

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;
use crate::declaration_parser::cpp_transpiler::CPPTranspiler;

type ModuleAttributeDeclarationResult = DeclarationResult<ModuleAttributeDeclaration>;

pub struct ModuleAttributeDeclaration {
	pub name: String,
	pub parameters: Option<Vec<(usize, usize)>>,
	pub line: usize
}

impl Declaration<ModuleAttributeDeclaration> for ModuleAttributeDeclaration {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of attribute";
	}
}

impl ModuleAttributeDeclaration {
	pub fn new(parser: &mut Parser) -> ModuleAttributeDeclarationResult {
		let initial_line = parser.line;

		let mut next_char = parser.get_curr();
		if next_char != '#' {
			return Self::unexpected_character(parser.index);
		}
		parser.increment();

		// Parse Var Style
		let mut attribute_name = "".to_string();
		declare_parse_ascii!(attribute_name, parser);

		// Parse Whitespace
		declare_parse_whitespace!(parser);

		let mut parameters = None;
		next_char = parser.get_curr();
		if next_char == '(' {
			parser.increment();
			let mut params = Vec::new();
			loop {
				let start = parser.index;
				let mut result = ' ';
				declare_parse_expr_until_either_char!(',', ')', result, parser);
				if result == ')' {
					params.push((start, parser.index));
					break;
				} else if result == ',' {
					params.push((start, parser.index));
					parser.increment();
				} else {
					return Self::out_of_space(parser.index);
				}
			}
			parameters = Some(params);
			parser.increment();
		}

		return ModuleAttributeDeclarationResult::Ok(ModuleAttributeDeclaration {
			name: attribute_name,
			parameters: parameters,
			line: initial_line
		});
	}

	pub fn is_declaration(parser: &mut Parser) -> bool {
		return Self::is_attribute_declaration(&parser.content, parser.index);
	}

	pub fn is_attribute_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return declare.starts_with("#");
	}
}
