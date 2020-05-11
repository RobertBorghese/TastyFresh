/**********************************************************
 * --- Import Declaration ---
 *
 * Represents and parses an import statement.
 **********************************************************/

use crate::{
	declare_parse_required_whitespace,
	declare_parse_ascii,
	declare_parse_until_char
};

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;
use crate::declaration_parser::cpp_transpiler::CPPTranspiler;

use regex::Regex;

lazy_static! {
	pub static ref IMPORT_REGEX: Regex = Regex::new(r"^\b(?:import|derive)\b").unwrap();
}

type ImportDeclarationResult = DeclarationResult<ImportDeclaration>;

#[derive(Clone)]
pub struct ImportDeclaration {
	pub path: String,
	pub line: usize,
	pub is_header: bool
}

impl Declaration<ImportDeclaration> for ImportDeclaration {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of import";
	}
}

impl CPPTranspiler for ImportDeclaration {
	fn to_cpp(&self) -> String {
		return "".to_string();
	}
}

impl ImportDeclaration {
	pub fn new(parser: &mut Parser) -> ImportDeclarationResult {
		let initial_line = parser.line;

		// Parse Var Style
		let mut import_keyword = "".to_string();
		declare_parse_ascii!(import_keyword, parser);
		if import_keyword != "import" && import_keyword != "derive" {
			return ImportDeclarationResult::Err("Unexpected Keyword", "\"import\" or \"derive\" keyword expected", parser.index - import_keyword.len(), parser.index);
		}

		declare_parse_required_whitespace!(parser);

		let content_start = parser.index;
		declare_parse_until_char!(';', parser);

		let import_path = parser.content[content_start..parser.index].to_string();

		return ImportDeclarationResult::Ok(ImportDeclaration {
			path: import_path,
			line: initial_line,
			is_header: import_keyword == "derive"
		});
	}

	pub fn is_declaration(parser: &mut Parser) -> bool {
		return Self::is_import_declaration(&parser.content, parser.index);
	}

	pub fn is_import_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return IMPORT_REGEX.is_match(declare);
	}
}
