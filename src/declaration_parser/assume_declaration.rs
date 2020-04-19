/**********************************************************
 * --- Assume Declaration ---
 *
 * Represents and parses an assume statement.
 **********************************************************/

use crate::{
	declare_parse_required_whitespace,
	declare_parse_ascii,
	declare_parse_until_char
};

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;
use crate::declaration_parser::cpp_transpiler::CPPTranspiler;

type AssumeDeclarationResult = DeclarationResult<AssumeDeclaration>;

pub struct AssumeDeclaration {
	pub path: String,
	pub line: usize
}

impl Declaration<AssumeDeclaration> for AssumeDeclaration {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of assume";
	}
}

impl CPPTranspiler for AssumeDeclaration {
	fn to_cpp(&self) -> String {
		return "".to_string();
	}
}

impl AssumeDeclaration {
	pub fn new(parser: &mut Parser) -> AssumeDeclarationResult {
		let initial_line = parser.line;

		// Parse Var Style
		let mut assume_keyword = "".to_string();
		declare_parse_ascii!(assume_keyword, parser);
		if assume_keyword != "assume" {
			return AssumeDeclarationResult::Err("Unexpected Keyword", "\"assume\" keyword expected", parser.index - assume_keyword.len(), parser.index);
		}

		declare_parse_required_whitespace!(parser);

		let content_start = parser.index;
		declare_parse_until_char!(';', parser);

		let assume_path = parser.content[content_start..parser.index].to_string();

		return AssumeDeclarationResult::Ok(AssumeDeclaration {
			path: assume_path,
			line: initial_line
		});
	}

	pub fn is_declaration(parser: &mut Parser) -> bool {
		return Self::is_assume_declaration(&parser.content, parser.index);
	}

	pub fn is_assume_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return declare.starts_with("assume ");
	}
}
