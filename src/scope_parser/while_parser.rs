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

use std::rc::Rc;

type WhileParserResult = DeclarationResult<WhileParser>;

pub struct WhileParser {
	pub expression: Rc<Expression>,
	pub scope: Box<ScopeExpression>,
	pub line: usize,
	pub end_line: usize
}

impl Declaration<WhileParser> for WhileParser {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of while statement";
	}
}

impl WhileParser {
	pub fn new(parser: &mut Parser, file_name: String, config_data: &ConfigData, context: &mut Context) -> WhileParserResult {
		let initial_line = parser.line;

		let mut while_keyword = "".to_string();
		declare_parse_ascii!(while_keyword, parser);
		if while_keyword != "while" {
			return WhileParserResult::Err("Unexpected Keyword", "\"while\" keyword expected", parser.index - while_keyword.len(), parser.index);
		}

		declare_parse_whitespace!(parser);

		let mut reason = ExpressionEndReason::Unknown;
		let expression = parser.parse_expression(file_name.clone(), config_data, Some(context), &mut reason, Some(VariableType::boolean()));

		match reason {
			ExpressionEndReason::Unknown => return WhileParserResult::Err("Unknown Error", "unknown expression parsing error", parser.index - 1, parser.index),
			ExpressionEndReason::EndOfContent =>  return WhileParserResult::Err("Unexpected End of Expression", "unexpected end of expression", parser.index - 1, parser.index),
			ExpressionEndReason::NoValueError => return WhileParserResult::Err("Value Expected", "expression value expected here", parser.index - 1, parser.index),
			_ => ()
		}

		declare_parse_whitespace!(parser);

		let scope: Option<ScopeExpression>;
		if parser.get_curr() == '{' {
			scope = Some(ScopeExpression::new(parser, None, parser.index + 1, parser.line, &file_name, config_data, context, None));
			if parser.get_curr() == '}' {
				parser.increment();
			}
		} else {
			scope = Some(ScopeExpression::new(parser, Some(1), parser.index, parser.line, &file_name, config_data, context, None));
		}

		return WhileParserResult::Ok(WhileParser {
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
		return declare.starts_with("while ") || declare.starts_with("while(");
	}
}
