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
		if next_char != '%' {
			return Self::unexpected_character(parser.index);
		}
		parser.increment();

		// Parse Var Style
		let mut attribute_name = "".to_string();
		declare_parse_ascii!(attribute_name, parser);

		next_char = parser.get_curr();
		if next_char != '%' {
			return Self::unexpected_character(parser.index);
		}
		parser.increment();

		return ModuleAttributeDeclarationResult::Ok(ModuleAttributeDeclaration {
			name: attribute_name,
			line: initial_line
		});
	}

	pub fn is_declaration(parser: &mut Parser) -> bool {
		return Self::is_attribute_declaration(&parser.content, parser.index);
	}

	pub fn is_attribute_declaration(content: &str, index: usize) -> bool {
		let declare_content = &content[index..];
		if index == 0 || content[index - 1..].starts_with("\n") {
			if declare_content.starts_with("%") {
				let mut was_end_char = false;
				for c in declare_content.chars() {
					if c == '\n' {
						return was_end_char;
					}
					was_end_char = (c == '%');
				}
			}
		}
		return false;
	}
}
