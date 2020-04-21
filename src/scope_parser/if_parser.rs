/**********************************************************
 * --- If Parser ---
 *
 * Parses an if statement.
 **********************************************************/

use crate::{
	declare_parse_whitespace,
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

use crate::scope_parser::ScopeExpression;

use std::rc::Rc;

use regex::Regex;

lazy_static! {
	pub static ref IF_REGEX: Regex = Regex::new(r"^\b(?:if|unless|else)\b").unwrap();
}

type IfParserResult = DeclarationResult<IfParser>;

pub struct IfParser {
	pub if_type: IfType,
	pub expression: Option<Rc<Expression>>,
	pub scope: Box<ScopeExpression>,
	pub line: usize,
	pub end_line: usize
}

pub enum IfType {
	If,
	Unless,
	ElseIf,
	ElseUnless,
	Else
}

impl IfType {
	pub fn is_if(&self) -> bool {
		if let IfType::If = self {
			return true;
		}
		return false;
	}

	pub fn is_unless(&self) -> bool {
		if let IfType::Unless = self {
			return true;
		}
		return false;
	}

	pub fn is_elseif(&self) -> bool {
		if let IfType::ElseIf = self {
			return true;
		}
		return false;
	}

	pub fn is_elseunless(&self) -> bool {
		if let IfType::ElseUnless = self {
			return true;
		}
		return false;
	}

	pub fn is_else(&self) -> bool {
		if let IfType::Else = self {
			return true;
		}
		return false;
	}
}

impl Declaration<IfParser> for IfParser {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of if statement";
	}
}

impl IfParser {
	pub fn new(parser: &mut Parser, file_name: String, config_data: &ConfigData, context: &mut Context) -> IfParserResult {
		let initial_line = parser.line;

		let mut if_keyword = "".to_string();
		declare_parse_ascii!(if_keyword, parser);
		if if_keyword != "if" && if_keyword != "else"  && if_keyword != "unless" {
			return IfParserResult::Err("Unexpected Keyword", "\"if\", \"else\", or \"unless\" keyword expected", parser.index - if_keyword.len(), parser.index);
		}

		let mut if_type = if if_keyword == "if" { IfType::If } else { IfType::Unless };
		let mut obtain_condition = if_keyword == "if" || if_keyword == "unless";

		declare_parse_whitespace!(parser);

		if if_keyword == "else" {
			let curr_index = parser.index;
			let curr_line = parser.line;

			let mut real_if_keyword = "".to_string();
			parse_unneccessary_ascii!(real_if_keyword, parser);
			if real_if_keyword == "if" {
				declare_parse_whitespace!(parser);
				obtain_condition = true;
				if_type = IfType::ElseIf;
			} else if real_if_keyword == "unless" {
				declare_parse_whitespace!(parser);
				obtain_condition = true;
				if_type = IfType::ElseUnless;
			} else {
				parser.reset(curr_index, curr_line);
				if_type = IfType::Else;
			}
		}

		let mut expression: Option<Rc<Expression>> = None;
		if obtain_condition {
			let mut reason = ExpressionEndReason::Unknown;
			expression = Some(parser.parse_expression(file_name.clone(), config_data, Some(context), &mut reason, Some(VariableType::boolean())));

			match reason {
				ExpressionEndReason::Unknown => return IfParserResult::Err("Unknown Error", "unknown expression parsing error", parser.index - 1, parser.index),
				ExpressionEndReason::EndOfContent =>  return IfParserResult::Err("Unexpected End of Expression", "unexpected end of expression", parser.index - 1, parser.index),
				ExpressionEndReason::NoValueError => return IfParserResult::Err("Value Expected", "expression value expected here", parser.index - 1, parser.index),
				_ => ()
			}

			declare_parse_whitespace!(parser);
		}

		let scope: Option<ScopeExpression>;
		if parser.get_curr() == '{' {
			scope = Some(ScopeExpression::new(parser, None, parser.index + 1, parser.line, &file_name, config_data, context, None));
			if parser.get_curr() == '}' {
				parser.increment();
			}
		} else {
			scope = Some(ScopeExpression::new(parser, Some(1), parser.index, parser.line, &file_name, config_data, context, None));
		}

		return IfParserResult::Ok(IfParser {
			if_type: if_type,
			expression: expression,
			scope: Box::new(scope.unwrap()),
			line: initial_line,
			end_line: parser.line
		});
	}

	pub fn is_declaration(parser: &Parser) -> bool {
		return Self::is_if_declaration(&parser.content, parser.index);
	}

	pub fn is_if_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return IF_REGEX.is_match(declare);
	}
}
