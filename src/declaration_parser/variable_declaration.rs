/**********************************************************
 * --- Variable Declaration ---
 *
 * Represents a variable declaration prior to being parsed.
 **********************************************************/

use crate::expression::variable_type::VariableType;
use crate::expression::variable_type::{ Type, Style };

use crate::declaration_parser::parser::Parser;

use crate::context_management::print_code_error;
use crate::context_management::position::Position;

pub struct VariableDeclaration {
	pub name: String,
	pub var_type: VariableType,
	pub line: usize,
	pub start_index: usize,
	pub end_index: usize
}

impl VariableDeclaration {
	pub fn out_of_space(pos: Position, index: &mut usize) {
		let temp_pos = Position { file: pos.file, line: None, start: *index - 1, end: Some(*index) };
		print_code_error("Unexpected End", "unexpected end of variable declaration", &temp_pos);
	}

	pub fn new(content: &str, index: &mut usize, position: Position, line_offset: &mut usize) -> Option<VariableDeclaration> {
		let state = 0;
		let chars = content[*index..].chars().collect();
		let mut out_of_space = false;

		// Parse Var Style
		let style_name = Parser::parse_ascii_char_name(&chars, index, &mut out_of_space);
		let style = Style::new(style_name.as_str());
		if style.is_unknown() {
			let temp_pos = Position { file: position.file, line: None, start: *index - style_name.len(), end: Some(*index) };
			print_code_error("Unknown Style", "unknown style", &temp_pos);
			return None;
		} else if out_of_space { Self::out_of_space(position, index); return None; }

		// Parse Whitespace
		if !Parser::parse_whitespace(&chars, index, line_offset, &mut out_of_space) {
			let temp_pos = Position { file: position.file, line: None, start: *index - 1, end: Some(*index) };
			print_code_error("Expected space", "whitspace expected here", &temp_pos);
			return None;
		} else if out_of_space { Self::out_of_space(position, index); return None; }

		// Parse Name
		let variable_name = Parser::parse_ascii_char_name(&chars, index, &mut out_of_space);
		if variable_name.is_empty() {
			let temp_pos = Position { file: position.file, line: None, start: *index - 1, end: Some(*index) };
			print_code_error("Variable Name Missing", "variable name missing", &temp_pos);
			return None;
		} else if out_of_space { Self::out_of_space(position, index); return None; }

		// Parse Whitespace
		Parser::parse_whitespace(&chars, index, line_offset, &mut out_of_space);
		if out_of_space { Self::out_of_space(position, index); return None; }

		// Parse Var Type
		let mut next_char = chars[*index];
		let var_type: Type;
		if next_char == ':' {
			*index += 1;
			Parser::parse_whitespace(&chars, index, line_offset, &mut out_of_space);
			if out_of_space { Self::out_of_space(position, index); return None; }
			let mut unexpected_char = false;
			var_type = Parser::parse_type(&chars, index, line_offset, &mut out_of_space, &mut unexpected_char);
			*index += 1;
			if unexpected_char {
				let temp_pos = Position { file: position.file, line: None, start: *index - 1, end: Some(*index) };
				print_code_error("Unexpected Character", "unexpected character here", &temp_pos);
				return None;
			}
			else if out_of_space { Self::out_of_space(position, index); return None; }
		} else if next_char == '=' {
			var_type = Type::Inferred;
		} else {
			let temp_pos = Position { file: position.file, line: None, start: *index - 1, end: Some(*index) };
			print_code_error("Unexpected Symbol", "unexpected symbol", &temp_pos);
			return None;
		}

		// Parse Whitespace
		Parser::parse_whitespace(&chars, index, line_offset, &mut out_of_space);
		if out_of_space { Self::out_of_space(position, index); return None; }

		// Parse Assignment
		next_char = chars[*index];
		if next_char == '=' {
			*index += 1;
			if *index >= chars.len() { Self::out_of_space(position, index); return None; }
		} else {
			let temp_pos = Position { file: position.file, line: None, start: *index - 1, end: Some(*index) };
			print_code_error("Unexpected Symbol", "expected '=' operator", &temp_pos);
			return None;
		}

		// Parse Expression
		let start = *index;
		Parser::parse_until(';', &chars, index, line_offset, &mut out_of_space);
		if out_of_space { Self::out_of_space(position, index); return None; }
		let end = *index;
		return Some(VariableDeclaration {
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
		let styles = Style::styles();
		for style in styles {
			if declare.starts_with(style) {
				return true;
			}
		}
		return false;
	}
}
