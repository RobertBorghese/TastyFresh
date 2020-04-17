/**********************************************************
 * --- Loop Parser ---
 *
 * Parses a loop statement.
 **********************************************************/

use crate::{
	declare_parse_whitespace,
	declare_parse_required_whitespace,
	declare_parse_ascii,
	declare_parse_until_char
};

use crate::config_management::ConfigData;

use crate::expression::Expression;
use crate::expression::expression_parser::ExpressionEndReason;
use crate::expression::variable_type::VariableType;

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;
use crate::declaration_parser::cpp_transpiler::CPPTranspiler;

use crate::context_management::context::Context;

use crate::scope_parser::ScopeExpression;

use std::rc::Rc;

type LoopParserResult = DeclarationResult<LoopParser>;

pub struct LoopParser {
	pub scope: Box<ScopeExpression>,
	pub line: usize,
	pub end_line: usize
}

impl Declaration<LoopParser> for LoopParser {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of loop statement";
	}
}

impl LoopParser {
	pub fn new(parser: &mut Parser, file_name: String, config_data: &ConfigData, context: &mut Context) -> LoopParserResult {
		let initial_line = parser.line;

		let mut loop_keyword = "".to_string();
		declare_parse_ascii!(loop_keyword, parser);
		if loop_keyword != "loop" {
			return LoopParserResult::Err("Unexpected Keyword", "\"loop\" keyword expected", parser.index - loop_keyword.len(), parser.index);
		}

		declare_parse_whitespace!(parser);

		let mut scope: Option<ScopeExpression> = None;
		if parser.get_curr() == '{' {
			scope = Some(ScopeExpression::new(parser, None, parser.index + 1, parser.line, &file_name, config_data, context, None));
			if parser.get_curr() == '}' {
				parser.increment();
			}
		} else {
			scope = Some(ScopeExpression::new(parser, Some(1), parser.index, parser.line, &file_name, config_data, context, None));
		}

		return LoopParserResult::Ok(LoopParser {
			scope: Box::new(scope.unwrap()),
			line: initial_line,
			end_line: parser.line
		});
	}

	pub fn is_declaration(parser: &Parser) -> bool {
		return Self::is_loop_declaration(&parser.content, parser.index);
	}

	pub fn is_loop_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return declare.starts_with("loop");
	}
}
