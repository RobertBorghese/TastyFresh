/**********************************************************
 * --- For Parser ---
 *
 * Parses a for statement.
 **********************************************************/

use crate::{
	declare_parse_whitespace,
	declare_parse_required_whitespace,
	declare_parse_ascii,
	parse_unneccessary_ascii
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

use std::rc::Rc;

use regex::Regex;

use either::*;

lazy_static! {
	pub static ref FOR_REGEX: Regex = Regex::new(r"^\b(?:for|inc|dec)\b").unwrap();
}

type ForParserResult = DeclarationResult<ForParser>;

pub struct ForParser {
	pub for_type: ForType,
	pub var_name: String,
	pub content: Either<Rc<Expression>,(Rc<Expression>,Rc<Expression>,Option<Rc<Expression>>)>,
	pub scope: Box<ScopeExpression>,
	pub line: usize,
	pub end_line: usize
}

pub enum ForType {
	ForEach,
	Increment,
	Decrement,
	IncrementTo,
	DecrementTo,
	Invalid
}

impl ForType {
	pub fn new(id: &str) -> ForType {
		return match id {
			"for" => ForType::ForEach,
			"inc" => ForType::Increment,
			"dec" => ForType::Decrement,
			"incto" => ForType::IncrementTo,
			"decto" => ForType::DecrementTo,
			_ => ForType::Invalid };
	}

	pub fn is_for(&self) -> bool {
		if let ForType::ForEach = self {
			return true;
		}
		return false;
	}

	pub fn is_increment(&self) -> bool {
		if let ForType::Increment = self {
			return true;
		}
		return false;
	}

	pub fn is_decrement(&self) -> bool {
		if let ForType::Decrement = self {
			return true;
		}
		return false;
	}

	pub fn is_incrementto(&self) -> bool {
		if let ForType::IncrementTo = self {
			return true;
		}
		return false;
	}

	pub fn is_decrementto(&self) -> bool {
		if let ForType::DecrementTo = self {
			return true;
		}
		return false;
	}

	pub fn is_invalid(&self) -> bool {
		if let ForType::Invalid = self {
			return true;
		}
		return false;
	}
}

impl Declaration<ForParser> for ForParser {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of for statement";
	}
}

impl ForParser {
	pub fn new(parser: &mut Parser, file_name: String, config_data: &ConfigData, context: &mut Context, context_manager: &mut ContextManager) -> ForParserResult {
		let initial_line = parser.line;

		let mut for_keyword = "".to_string();
		declare_parse_ascii!(for_keyword, parser);
		let for_type = ForType::new(for_keyword.as_str());
		if for_type.is_invalid() {
			return ForParserResult::Err("Unexpected Keyword", "\"for\" keyword expected", parser.index - for_keyword.len(), parser.index);
		}

		declare_parse_required_whitespace!(parser);

		let mut var_name = "".to_string();
		declare_parse_ascii!(var_name, parser);
		if var_name.is_empty() {
			return ForParserResult::Err("Expected Variable Name", "variable name expected", parser.index - 1, parser.index);
		}

		declare_parse_required_whitespace!(parser);

		let content: Option<Either<Rc<Expression>,(Rc<Expression>,Rc<Expression>,Option<Rc<Expression>>)>>;
		if for_type.is_for() {

			let mut in_keyword = "".to_string();
			declare_parse_ascii!(in_keyword, parser);
			if in_keyword != "in" {
				return ForParserResult::Err("Unexpected Keyword", "\"in\" keyword expected", parser.index - in_keyword.len(), parser.index);
			}

			declare_parse_required_whitespace!(parser);

			let mut reason = ExpressionEndReason::Unknown;
			let expression = parser.parse_expression(file_name.clone(), config_data, Some(context), context_manager, &mut reason, Some(VariableType::boolean()));

			match reason {
				ExpressionEndReason::Unknown => return ForParserResult::Err("Unknown Error", "unknown expression parsing error", parser.index - 1, parser.index),
				ExpressionEndReason::EndOfContent =>  return ForParserResult::Err("Unexpected End of Expression", "unexpected end of expression", parser.index - 1, parser.index),
				ExpressionEndReason::NoValueError => return ForParserResult::Err("Value Expected", "expression value expected here", parser.index - 1, parser.index),
				_ => ()
			}

			content = Some(Left(expression));

		} else {

			// FROM
			let mut from_keyword = "".to_string();
			declare_parse_ascii!(from_keyword, parser);
			if from_keyword != "from" {
				return ForParserResult::Err("Unexpected Keyword", "\"from\" keyword expected", parser.index - from_keyword.len(), parser.index);
			}

			declare_parse_required_whitespace!(parser);

			let mut start_reason = ExpressionEndReason::Unknown;
			let start_expression = parser.parse_expression(file_name.clone(), config_data, Some(context), context_manager, &mut start_reason, Some(VariableType::boolean()));

			match start_reason {
				ExpressionEndReason::Unknown => return ForParserResult::Err("Unknown Error", "unknown expression parsing error", parser.index - 1, parser.index),
				ExpressionEndReason::EndOfContent =>  return ForParserResult::Err("Unexpected End of Expression", "unexpected end of expression", parser.index - 1, parser.index),
				ExpressionEndReason::NoValueError => return ForParserResult::Err("Value Expected", "expression value expected here", parser.index - 1, parser.index),
				_ => ()
			}

			declare_parse_whitespace!(parser);

			// TO
			let mut to_keyword = "".to_string();
			declare_parse_ascii!(to_keyword, parser);
			if to_keyword != "to" {
				return ForParserResult::Err("Unexpected Keyword", "\"to\" keyword expected", parser.index - to_keyword.len(), parser.index);
			}

			declare_parse_whitespace!(parser);

			let mut end_reason = ExpressionEndReason::Unknown;
			let end_expression = parser.parse_expression(file_name.clone(), config_data, Some(context), context_manager, &mut end_reason, Some(VariableType::boolean()));

			match end_reason {
				ExpressionEndReason::Unknown => return ForParserResult::Err("Unknown Error", "unknown expression parsing error", parser.index - 1, parser.index),
				ExpressionEndReason::EndOfContent =>  return ForParserResult::Err("Unexpected End of Expression", "unexpected end of expression", parser.index - 1, parser.index),
				ExpressionEndReason::NoValueError => return ForParserResult::Err("Value Expected", "expression value expected here", parser.index - 1, parser.index),
				_ => ()
			}

			declare_parse_whitespace!(parser);

			let curr_index = parser.index;
			let curr_line = parser.line;

			let mut by_keyword = "".to_string();
			parse_unneccessary_ascii!(by_keyword, parser);
			let mut by_expression: Option<Rc<Expression>> = None;
			if by_keyword == "by" {
				let mut by_reason = ExpressionEndReason::Unknown;
				by_expression = Some(parser.parse_expression(file_name.clone(), config_data, Some(context), context_manager, &mut by_reason, Some(VariableType::boolean())));

				match by_reason {
					ExpressionEndReason::Unknown => return ForParserResult::Err("Unknown Error", "unknown expression parsing error", parser.index - 1, parser.index),
					ExpressionEndReason::EndOfContent =>  return ForParserResult::Err("Unexpected End of Expression", "unexpected end of expression", parser.index - 1, parser.index),
					ExpressionEndReason::NoValueError => return ForParserResult::Err("Value Expected", "expression value expected here", parser.index - 1, parser.index),
					_ => ()
				}
			} else {
				parser.reset(curr_index, curr_line);
			}

			content = Some(Right((start_expression, end_expression, by_expression)));

		}

		if content.is_none() {
			return ForParserResult::Err("For Content Expected", "could not find for content", parser.index - 1, parser.index);
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

		return ForParserResult::Ok(ForParser {
			for_type: for_type,
			var_name: var_name,
			content: content.unwrap(),
			scope: Box::new(scope.unwrap()),
			line: initial_line,
			end_line: parser.line
		});
	}

	pub fn is_declaration(parser: &Parser) -> bool {
		return Self::is_for_declaration(&parser.content, parser.index);
	}

	pub fn is_for_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return FOR_REGEX.is_match(declare);
	}
}
