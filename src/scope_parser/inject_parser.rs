/**********************************************************
 * --- Inject Parser ---
 *
 * Parses an inject statement.
 **********************************************************/

use crate::{
	declare_parse_whitespace,
	declare_parse_ascii,
	declare_parse_required_next_char,
	declare_parse_expr_until_next_char
};

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;

type InjectParserResult = DeclarationResult<InjectParser>;

pub struct InjectParser {
	pub start_index: usize,
	pub end_index: usize,
	pub line: usize,
	pub end_line: usize
}

impl Declaration<InjectParser> for InjectParser {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of inject statement";
	}
}

impl InjectParser {
	pub fn new(parser: &mut Parser) -> InjectParserResult {
		let initial_line = parser.line;

		let mut inject_keyword = "".to_string();
		declare_parse_ascii!(inject_keyword, parser);
		if inject_keyword != "inject" {
			return InjectParserResult::Err("Unexpected Keyword", "\"inject\" keyword expected", parser.index - inject_keyword.len(), parser.index);
		}

		declare_parse_whitespace!(parser);

		let mut next_char = ' ';
		declare_parse_required_next_char!('{', next_char, parser);
		let start_index = parser.index;
		declare_parse_expr_until_next_char!('}', parser);
		let end_index = parser.index;

		parser.increment();

		return InjectParserResult::Ok(InjectParser {
			start_index: start_index,
			end_index: end_index,
			line: initial_line,
			end_line: parser.line
		});
	}

	pub fn is_declaration(parser: &Parser) -> bool {
		return Self::is_inject_declaration(&parser.content, parser.index);
	}

	pub fn is_inject_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return declare.starts_with("inject");
	}
}
