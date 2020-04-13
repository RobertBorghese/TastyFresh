/**********************************************************
 * --- If Parser ---
 *
 * Parses an if statement.
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

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;
use crate::declaration_parser::cpp_transpiler::CPPTranspiler;

use crate::context_management::context::Context;

use crate::scope_parser::ScopeExpression;

use std::rc::Rc;

type IfParserResult = DeclarationResult<IfParser>;

pub struct IfParser {
	pub expression: Rc<Expression>,
	pub scope: Box<ScopeExpression>,
	pub line: usize,
	pub end_line: usize
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
		if if_keyword != "if" {
			return IfParserResult::Err("Unexpected Keyword", "\"if\" keyword expected", parser.index - if_keyword.len(), parser.index);
		}

		declare_parse_whitespace!(parser);

		let mut reason = ExpressionEndReason::Unknown;
		let expression = parser.parse_expression(file_name.clone(), config_data, Some(context), &mut reason);

		match reason {
			ExpressionEndReason::Unknown => return IfParserResult::Err("Unknown Error", "unknown expression parsing error", parser.index - 1, parser.index),
			ExpressionEndReason::ReachedChar(c) => (),
			ExpressionEndReason::EndOfContent =>  return IfParserResult::Err("Unexpected End of Expression", "unexpected end of expression", parser.index - 1, parser.index),
			ExpressionEndReason::EndOfExpression => {
				//println!("REACHED FIRST END OF EXPR IF {}", expression.to_string(&config_data.operators, context));
			},
			ExpressionEndReason::NoValueError => return IfParserResult::Err("Value Expected", "expression value expected here", parser.index - 1, parser.index)
		}

		let mut scope: Option<ScopeExpression> = None;
		if parser.get_curr() == '{' {
			scope = Some(ScopeExpression::new(parser, None, parser.index + 1, parser.line, &file_name, config_data, context));
			if parser.get_curr() == '}' {
				//println!("SUCCESS!");
			}
		} else {
			scope = Some(ScopeExpression::new(parser, Some(1), parser.index, parser.line, &file_name, config_data, context));
		}

		return IfParserResult::Ok(IfParser {
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
		return declare.starts_with("if ") || declare.starts_with("if(");
	}
}
