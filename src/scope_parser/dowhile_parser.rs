/**********************************************************
 * --- Do/While Parser ---
 *
 * Parses a do/while statement.
 **********************************************************/

use crate::{
	declare_parse_whitespace,
	declare_parse_required_whitespace,
	declare_parse_ascii,
	declare_parse_until_char,
	declare_parse_required_next_char
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

type DoWhileParserResult = DeclarationResult<DoWhileParser>;

pub struct DoWhileParser {
	pub expression: Rc<Expression>,
	pub scope: Box<ScopeExpression>,
	pub line: usize,
	pub end_line: usize,
	pub while_offset: usize
}

impl Declaration<DoWhileParser> for DoWhileParser {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of do_while statement";
	}
}

impl DoWhileParser {
	pub fn new(parser: &mut Parser, file_name: String, config_data: &ConfigData, context: &mut Context) -> DoWhileParserResult {
		let initial_line = parser.line;

		let mut do_keyword = "".to_string();
		declare_parse_ascii!(do_keyword, parser);
		if do_keyword != "do" {
			return DoWhileParserResult::Err("Unexpected Keyword", "\"do\" keyword expected", parser.index - do_keyword.len(), parser.index);
		}

		declare_parse_whitespace!(parser);

		let mut close_line = 0;
		let mut scope: Option<ScopeExpression> = None;
		if parser.get_curr() == '{' {
			scope = Some(ScopeExpression::new(parser, None, parser.index + 1, parser.line, &file_name, config_data, context, None));
			if parser.get_curr() == '}' {
				close_line = parser.line;
				parser.increment();
			}
		} else {
			scope = Some(ScopeExpression::new(parser, Some(1), parser.index, parser.line, &file_name, config_data, context, None));
			close_line = parser.line;
		}

		declare_parse_whitespace!(parser);

		let while_line = parser.line - close_line;
		let mut while_keyword = "".to_string();
		declare_parse_ascii!(while_keyword, parser);
		if while_keyword != "while" {
			return DoWhileParserResult::Err("Unexpected Keyword", "\"while\" keyword expected", parser.index - while_keyword.len(), parser.index);
		}

		declare_parse_whitespace!(parser);

		let mut reason = ExpressionEndReason::Unknown;
		let expression = parser.parse_expression(file_name.clone(), config_data, Some(context), &mut reason, Some(VariableType::boolean()));

		match reason {
			ExpressionEndReason::Unknown => return DoWhileParserResult::Err("Unknown Error", "unknown expression parsing error", parser.index - 1, parser.index),
			ExpressionEndReason::EndOfContent =>  return DoWhileParserResult::Err("Unexpected End of Expression", "unexpected end of expression", parser.index - 1, parser.index),
			ExpressionEndReason::NoValueError => return DoWhileParserResult::Err("Value Expected", "expression value expected here", parser.index - 1, parser.index),
			_ => ()
		}

		declare_parse_whitespace!(parser);

		let mut next_char = ' ';
		declare_parse_required_next_char!(';', next_char, parser);

		return DoWhileParserResult::Ok(DoWhileParser {
			expression: expression,
			scope: Box::new(scope.unwrap()),
			line: initial_line,
			end_line: parser.line,
			while_offset: while_line
		});
	}

	pub fn is_declaration(parser: &Parser) -> bool {
		return Self::is_do_while_declaration(&parser.content, parser.index);
	}

	pub fn is_do_while_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return declare.starts_with("do");
	}
}
