/**********************************************************
 * --- Inject Declaration ---
 *
 * Represents a code injection declaration.
 **********************************************************/

use crate::{
	declare_parse_ascii,
	declare_parse_whitespace,
	declare_parse_required_next_char,
	declare_parse_expr_until_next_char
};

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;

use regex::Regex;

lazy_static! {
	pub static ref INJECT_REGEX: Regex = Regex::new(r"^\b(?:inject)\b").unwrap();
}

type InjectDeclarationResult = DeclarationResult<InjectDeclaration>;

#[derive(Clone)]
pub struct InjectDeclaration {
	pub line: usize,
	pub start_index: usize,
	pub end_index: usize
}

impl Declaration<InjectDeclaration> for InjectDeclaration {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of function";
	}
}

impl InjectDeclaration {
	pub fn new(parser: &mut Parser) -> InjectDeclarationResult {
		let initial_line = parser.line;

		let mut inject_keyword = "".to_string();
		declare_parse_ascii!(inject_keyword, parser);
		if inject_keyword != "inject" {
			return InjectDeclarationResult::Err("Unexpected Keyword", "\"inject\" keyword expected", parser.index - inject_keyword.len(), parser.index);
		}

		declare_parse_whitespace!(parser);

		let mut next_char = ' ';
		declare_parse_required_next_char!('{', next_char, parser);
		let start_index = parser.index;
		declare_parse_expr_until_next_char!('}', parser);
		let end_index = parser.index;

		return InjectDeclarationResult::Ok(InjectDeclaration {
			line: initial_line,
			start_index: start_index,
			end_index: end_index
		});
	}

	pub fn is_declaration(parser: &mut Parser) -> bool {
		return Self::is_inject_declaration(&parser.content, parser.index);
	}

	pub fn is_inject_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return INJECT_REGEX.is_match(declare);
	}
}
