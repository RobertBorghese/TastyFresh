/**********************************************************
 * --- Attribute Declaration ---
 *
 * Represents an attribute declaration prior to being parsed.
 **********************************************************/

use crate::expression::variable_type::VariableType;
use crate::expression::variable_type::{ Type, VarStyle };

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;

use crate::context_management::print_code_error;
use crate::context_management::position::Position;

type AttributeDeclarationResult = DeclarationResult<AttributeDeclaration>;

pub struct AttributeDeclaration {
	pub name: String,
	pub params: Vec<String>
}

impl Declaration<AttributeDeclaration> for AttributeDeclaration {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of attribute";
	}
}

impl AttributeDeclaration {
	pub fn new(content: &str, index: &mut usize, position: Position, line_offset: &mut usize) -> AttributeDeclarationResult {
		let state = 0;
		let chars: Vec<char> = content[*index..].chars().collect();
		let mut out_of_space = false;

		if chars[*index] != '@' {
			return AttributeDeclarationResult::Err("Attribute Error", "attribute should begin with '@'", *index - 1, *index);
		}
		*index += 1;

		// Parse Var Style
		let attribute_name = Parser::parse_ascii_char_name(&chars, index, &mut out_of_space);
		if attribute_name.is_empty() {
			return AttributeDeclarationResult::Err("No Attribute Name", "attribute requires name", *index - attribute_name.len(), *index);
		} else if out_of_space {
			return Self::out_of_space(index);
		}

		// Parse Whitespace
		Parser::parse_whitespace(&chars, index, line_offset, &mut out_of_space);
		if out_of_space { return Self::out_of_space(index); }

		// Parse Name
		let variable_name = Parser::parse_ascii_char_name(&chars, index, &mut out_of_space);
		if variable_name.is_empty() {
			return AttributeDeclarationResult::Err("Variable Name Missing", "variable name missing", *index - 1, *index);
		} else if out_of_space {
			return Self::out_of_space(index);
		}

		return AttributeDeclarationResult::Err("test", "test", 0, 0);
	}

	pub fn is_attribute_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return declare.starts_with("@");
	}
}
