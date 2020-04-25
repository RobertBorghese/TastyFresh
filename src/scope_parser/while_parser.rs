/**********************************************************
 * --- While Parser ---
 *
 * Parses a while statement.
 **********************************************************/

use crate::{
	declare_parse_whitespace,
	declare_parse_ascii
};

use crate::config_management::ConfigData;

use crate::expression::Expression;
use crate::expression::expression_parser::ExpressionEndReason;
use crate::expression::variable_type::VariableType;

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;

use crate::context_management::context::Context;

use crate::scope_parser::ScopeExpression;
use crate::context_management::context_manager::ContextManager;

use std::rc::Rc;

use regex::Regex;

lazy_static! {
	pub static ref WHILE_REGEX: Regex = Regex::new(r"^\b(?:while|until)\b").unwrap();
}

type WhileParserResult = DeclarationResult<WhileParser>;

pub struct WhileParser {
	pub while_type: WhileType,
	pub expression: Rc<Expression>,
	pub scope: Box<ScopeExpression>,
	pub line: usize,
	pub end_line: usize
}

pub enum WhileType {
	While,
	Until
}

impl WhileType {
	pub fn is_while(&self) -> bool {
		if let WhileType::While = self {
			return true;
		}
		return false;
	}

	pub fn is_until(&self) -> bool {
		if let WhileType::Until = self {
			return true;
		}
		return false;
	}
}

impl Declaration<WhileParser> for WhileParser {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of while statement";
	}
}

impl WhileParser {
	pub fn new(parser: &mut Parser, file_name: String, config_data: &ConfigData, context: &mut Context, context_manager: &mut ContextManager) -> WhileParserResult {
		let initial_line = parser.line;

		let mut while_type = WhileType::While;
		let mut while_keyword = "".to_string();
		declare_parse_ascii!(while_keyword, parser);
		if while_keyword != "while" && while_keyword != "until" {
			return WhileParserResult::Err("Unexpected Keyword", "\"while\" or \"until\" keyword expected", parser.index - while_keyword.len(), parser.index);
		}

		if while_keyword == "until" {
			while_type = WhileType::Until;
		}

		declare_parse_whitespace!(parser);

		let mut reason = ExpressionEndReason::Unknown;
		let expression = parser.parse_expression(file_name.clone(), config_data, Some(context), context_manager, &mut reason, Some(VariableType::boolean()));

		match reason {
			ExpressionEndReason::Unknown => return WhileParserResult::Err("Unknown Error", "unknown expression parsing error", parser.index - 1, parser.index),
			ExpressionEndReason::EndOfContent =>  return WhileParserResult::Err("Unexpected End of Expression", "unexpected end of expression", parser.index - 1, parser.index),
			ExpressionEndReason::NoValueError => return WhileParserResult::Err("Value Expected", "expression value expected here", parser.index - 1, parser.index),
			_ => ()
		}

		declare_parse_whitespace!(parser);

		let scope: Option<ScopeExpression>;
		if parser.get_curr() == '{' {
			scope = Some(ScopeExpression::new(parser, None, parser.index + 1, parser.line, &file_name, config_data, context, context_manager, None));
			if parser.get_curr() == '}' {
				parser.increment();
			}
		} else {
			scope = Some(ScopeExpression::new(parser, Some(1), parser.index, parser.line, &file_name, config_data, context, context_manager, None));
		}

		return WhileParserResult::Ok(WhileParser {
			while_type: while_type,
			expression: expression,
			scope: Box::new(scope.unwrap()),
			line: initial_line,
			end_line: parser.line
		});
	}

	pub fn is_declaration(parser: &Parser) -> bool {
		return Self::is_while_declaration(&parser.content, parser.index);
	}

	pub fn is_while_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return WHILE_REGEX.is_match(declare);
	}
}
