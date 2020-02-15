/**********************************************************
 * --- Variable Declaration ---
 *
 * Represents a variable declaration prior to being parsed.
 **********************************************************/

use crate::{
	declare_parse_whitespace,
	declare_parse_required_whitespace,
	declare_parse_ascii,
	declare_parse_required_ascii,
	declare_parse_required_next_char,
	declare_parse_everything_until_required_next_char,
	declare_parse_type
};

use crate::expression::variable_type::VariableType;
use crate::expression::variable_type::{ Type, VarStyle };

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;

use crate::context_management::print_code_error;
use crate::context_management::position::Position;

type VariableDeclarationResult = DeclarationResult<VariableDeclaration>;

pub struct VariableDeclaration {
	pub name: String,
	pub var_type: VariableType,
	pub line: usize,
	pub start_index: usize,
	pub end_index: usize
}

impl Declaration<VariableDeclaration> for VariableDeclaration {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of variable";
	}
}

impl VariableDeclaration {
	pub fn new(content: &str, index: &mut usize, position: Position, line_offset: &mut usize) -> VariableDeclarationResult {
		let state = 0;
		let chars = content[*index..].chars().collect();
		let mut out_of_space = false;

		// Parse Var Style
		let mut style_name = "".to_string();
		declare_parse_ascii!(style_name, chars, index, out_of_space);

		// Verify Var Style
		let style = VarStyle::new(style_name.as_str());
		if style.is_unknown() {
			return VariableDeclarationResult::Err("Unknown Style", "unknown style", *index - style_name.len(), *index);
		}

		// Parse Whitespace
		declare_parse_required_whitespace!(chars, index, line_offset, out_of_space);

		// Parse Var Name
		let mut variable_name = "".to_string();
		declare_parse_required_ascii!(variable_name, "Variable Name Missing", "variable name missing", chars, index, out_of_space);

		// Parse Whitespace
		declare_parse_whitespace!(chars, index, line_offset, out_of_space);

		// Parse Var Type
		let mut next_char = chars[*index];
		let var_type: Type;
		if next_char == ':' {
			*index += 1;
			declare_parse_whitespace!(chars, index, line_offset, out_of_space);
			declare_parse_type!(var_type, chars, index, line_offset, out_of_space);
		} else if next_char == '=' {
			var_type = Type::Inferred;
		} else {
			return VariableDeclarationResult::Err("Unexpected Symbol", "unexpected symbol", *index - 1, *index);
		}

		// Parse Whitespace
		declare_parse_whitespace!(chars, index, line_offset, out_of_space);

		// Parse Assignment
		declare_parse_required_next_char!('=', next_char, chars, index);

		// Parse Expression
		let start = *index;
		declare_parse_everything_until_required_next_char!(';', chars, index, line_offset, out_of_space);
		let end = *index;
		*index += 1;

		return VariableDeclarationResult::Ok(VariableDeclaration {
			name: variable_name,
			var_type: VariableType {
				var_type: var_type,
				var_style: style
			},
			line: position.line.unwrap_or(1),
			start_index: start,
			end_index: end
		});
	}

	pub fn is_var_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		let styles = VarStyle::styles();
		for style in styles {
			if declare.starts_with(style) {
				return true;
			}
		}
		return false;
	}
}
