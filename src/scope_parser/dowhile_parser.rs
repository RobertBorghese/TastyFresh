/**********************************************************
 * --- Do/While Parser ---
 *
 * Parses a do/while statement.
 **********************************************************/

use crate::{
	declare_parse_whitespace,
	declare_parse_ascii,
	declare_parse_required_next_char
};

use crate::config_management::ConfigData;

use crate::expression::Expression;
use crate::expression::expression_parser::ExpressionEndReason;
use crate::expression::variable_type::VariableType;

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;

use crate::context_management::context::Context;
use crate::context_management::context_manager::ContextManager;

use crate::scope_parser::ScopeExpression;
use crate::scope_parser::while_parser::WhileType;

use std::rc::Rc;

use regex::Regex;

lazy_static! {
	pub static ref DOWHILE_REGEX: Regex = Regex::new(r"^\b(?:do)\b").unwrap();
}

type DoWhileParserResult = DeclarationResult<DoWhileParser>;

pub struct DoWhileParser {
	pub while_type: WhileType,
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
	pub fn new(parser: &mut Parser, file_name: String, config_data: &ConfigData, context: &mut Context, context_manager: &mut ContextManager) -> DoWhileParserResult {
		let initial_line = parser.line;

		let mut do_keyword = "".to_string();
		declare_parse_ascii!(do_keyword, parser);
		if do_keyword != "do" {
			return DoWhileParserResult::Err("Unexpected Keyword", "\"do\" keyword expected", parser.index - do_keyword.len(), parser.index);
		}

		declare_parse_whitespace!(parser);

		let mut next_char = ' ';
		let mut close_line = 0;
		let scope: Option<ScopeExpression>;
		if parser.get_curr() == '{' {
			scope = Some(ScopeExpression::new(parser, None, parser.index + 1, parser.line, &file_name, config_data, context, context_manager, None));
			declare_parse_whitespace!(parser);
			if parser.get_curr() == '}' {
				close_line = parser.line;
				declare_parse_required_next_char!('}', next_char, parser);
			}
		} else {
			scope = Some(ScopeExpression::new(parser, Some(1), parser.index, parser.line, &file_name, config_data, context, context_manager, None));
			close_line = parser.line;
		}

		declare_parse_whitespace!(parser);

		let while_line = parser.line - close_line;
		let mut while_type = WhileType::While;
		let mut while_keyword = "".to_string();
		declare_parse_ascii!(while_keyword, parser);
		if while_keyword != "while" && while_keyword != "until" {
			return DoWhileParserResult::Err("Unexpected Keyword", "\"while\" or \"until\" keyword expected", parser.index - while_keyword.len(), parser.index);
		}

		if while_keyword == "until" {
			while_type = WhileType::Until;
		}

		declare_parse_whitespace!(parser);

		let mut reason = ExpressionEndReason::Unknown;
		let expression = parser.parse_expression(file_name.clone(), config_data, Some(context), context_manager, &mut reason, Some(VariableType::boolean()));

		match reason {
			ExpressionEndReason::Unknown => return DoWhileParserResult::Err("Unknown Error", "unknown expression parsing error", parser.index - 1, parser.index),
			ExpressionEndReason::EndOfContent =>  return DoWhileParserResult::Err("Unexpected End of Expression", "unexpected end of expression", parser.index - 1, parser.index),
			ExpressionEndReason::NoValueError => return DoWhileParserResult::Err("Value Expected", "expression value expected here", parser.index - 1, parser.index),
			_ => ()
		}

		declare_parse_whitespace!(parser);

		declare_parse_required_next_char!(';', next_char, parser);

		return DoWhileParserResult::Ok(DoWhileParser {
			while_type: while_type,
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
		return DOWHILE_REGEX.is_match(declare);
	}
}
