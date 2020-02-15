/**********************************************************
 * --- Function Declaration ---
 *
 * Represents a function declaration prior to being parsed.
 **********************************************************/

use crate::expression::variable_type::VariableType;
use crate::expression::variable_type::{ Type, VarStyle };

use crate::declaration_parser::parser::Parser;

use crate::context_management::print_code_error;
use crate::context_management::position::Position;

pub struct FunctionDeclaration {
	pub name: String,
	pub parameters: Vec<FunctionParamDeclaration>,
	pub return_type: VariableType,
	pub line: usize,
	pub start_index: usize,
	pub end_index: usize
}

pub struct FunctionParamDeclaration {
	pub name: String,
	pub var_type: VariableType
}

impl FunctionDeclaration {
	pub fn out_of_space(pos: Position, index: &mut usize) {
		let temp_pos = Position { file: pos.file, line: None, start: *index - 1, end: Some(*index) };
		print_code_error("Unexpected End", "unexpected end of variable declaration", &temp_pos);
	}

	pub fn new(content: &str, index: &mut usize, position: Position, line_offset: &mut usize) -> Option<FunctionDeclaration> {
		let state = 0;
		let chars = content[*index..].chars().collect();
		let mut out_of_space = false;

		// Parse Var Style
		loop {
			let style_name = Parser::parse_ascii_char_name(&chars, index, &mut out_of_space);
		}
	}

	pub fn is_func_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return declare.starts_with("fn");
	}
}