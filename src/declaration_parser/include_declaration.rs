/**********************************************************
 * --- Include Declaration ---
 *
 * Represents and parses an include statement.
 **********************************************************/

use crate::{
	declare_parse_required_whitespace,
	declare_parse_ascii,
	declare_parse_until_char
};

use crate::expression::variable_type::VariableType;
use crate::expression::variable_type::{ Type, VarStyle };

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;
use crate::declaration_parser::cpp_transpiler::CPPTranspiler;

type IncludeDeclarationResult = DeclarationResult<IncludeDeclaration>;

pub struct IncludeDeclaration {
	pub path: String,
	pub inc_type: IncludeType,
	pub line: usize
}

#[derive(Clone, Copy)]
pub enum IncludeType {
	Local,
	Header
}

impl Declaration<IncludeDeclaration> for IncludeDeclaration {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of include";
	}
}

impl CPPTranspiler for IncludeDeclaration {
	fn to_cpp(&self) -> String {
		return match self.inc_type {
			IncludeType::Local => format!("#include \"{}\"", self.path),
			IncludeType::Header => format!("#include <{}>", self.path)
		}
	}
}

impl IncludeDeclaration {
	pub fn new(parser: &mut Parser) -> IncludeDeclarationResult {
		let initial_line = parser.line;

		// Parse Var Style
		let mut include_keyword = "".to_string();
		declare_parse_ascii!(include_keyword, parser);
		if include_keyword != "include" {
			return IncludeDeclarationResult::Err("Unexpected Keyword", "\"include\" keyword expected", parser.index - include_keyword.len(), parser.index);
		}

		// Parse Whitespace
		declare_parse_required_whitespace!(parser);

		// Parse Var Style
		let mut explicit_type = false;
		let mut inc_type: IncludeType;
		let mut type_keyword = "".to_string();
		declare_parse_ascii!(type_keyword, parser);
		if type_keyword == "local" {
			inc_type = IncludeType::Local;
			explicit_type = true;
		} else if type_keyword == "system" {
			inc_type = IncludeType::Header;
			explicit_type = true;
		} else {
			inc_type = IncludeType::Local;
		}

		if explicit_type {
			declare_parse_required_whitespace!(parser);
		}

		let content_start = parser.index;
		declare_parse_until_char!(';', parser);

		let mut include_path = parser.content[content_start..parser.index].to_string();

		if !explicit_type {
			include_path = type_keyword + &include_path;
		}

		return IncludeDeclarationResult::Ok(IncludeDeclaration {
			path: include_path,
			inc_type: inc_type,
			line: initial_line
		});
	}

	pub fn is_declaration(parser: &mut Parser) -> bool {
		return Self::is_include_declaration(parser.content, parser.index);
	}

	pub fn is_include_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return declare.starts_with("include ");
	}
}
