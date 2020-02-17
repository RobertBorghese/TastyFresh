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
	declare_parse_expr_until_next_char,
	declare_parse_type
};

use crate::expression::variable_type::VariableType;
use crate::expression::variable_type::{ Type, VarStyle };

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;

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
	pub fn new(parser: &mut Parser) -> VariableDeclarationResult {
		let initial_line = parser.line;

		// Parse Var Style
		let mut style_name = "".to_string();
		declare_parse_ascii!(style_name, parser);

		// Verify Var Style
		let style = VarStyle::new(style_name.as_str());
		if style.is_unknown() {
			return VariableDeclarationResult::Err("Unknown Style", "unknown style", parser.index - style_name.len(), parser.index);
		}

		// Parse Whitespace
		declare_parse_required_whitespace!(parser);

		// Parse Var Name
		let mut variable_name = "".to_string();
		declare_parse_required_ascii!(variable_name, "Variable Name Missing", "variable name missing", parser);

		// Parse Whitespace
		declare_parse_whitespace!(parser);

		// Parse Var Type
		let mut next_char = parser.get_curr();
		let var_type: Type;
		if next_char == ':' {
			parser.increment();
			declare_parse_whitespace!(parser);
			declare_parse_type!(var_type, parser);
		} else if next_char == '=' {
			var_type = Type::Inferred;
			parser.increment();
		} else {
			return VariableDeclarationResult::Err("Unexpected Symbol", "unexpected symbol", parser.index - 1, parser.index);
		}

		// Parse Whitespace
		declare_parse_whitespace!(parser);

		// Parse Assignment
		declare_parse_required_next_char!('=', next_char, parser);

		// Parse Expression
		let start = parser.index;
		declare_parse_expr_until_next_char!(';', parser);
		let end = parser.index;
		parser.increment();

		return VariableDeclarationResult::Ok(VariableDeclaration {
			name: variable_name,
			var_type: VariableType {
				var_type: var_type,
				var_style: style
			},
			line: initial_line,
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
