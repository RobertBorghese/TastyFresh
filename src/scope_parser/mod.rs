/**********************************************************
 * --- Scope Parser ---
 *
 * Functions and structures here are dedicated to parsing
 * lists of expressions contained within scopes such as 
 * functions or encapsulated code.
 **********************************************************/

pub mod expression_scope_parser;
pub mod return_parser;

use crate::declaration_parser::parser::Parser;
use crate::declaration_parser::variable_declaration::VariableDeclaration;

use crate::expression::Expression;
use crate::expression::expression_parser::ExpressionEndReason;
use crate::expression::variable_type::Type;

use crate::scope_parser::return_parser::ReturnParser;

use crate::config_management::ConfigData;
use crate::config_management::operator_data::OperatorDataStructure;

use crate::context_management::typing_context::Context;

use std::rc::Rc;

pub enum ScopeExpression {
	Expression(Rc<Expression>),
	Scope(Vec<ScopeExpression>),
	VariableDeclaration(VariableDeclaration, Rc<Expression>),
	Return(Rc<Expression>)
}

impl ScopeExpression {
	pub fn new(parser: &mut Parser, start_index: usize, line: usize, file: &str, config_data: &ConfigData, context: &mut Context) -> ScopeExpression {
		parser.reset(start_index, line);

		let mut scope_exprs = Vec::new();

		loop {
			parser.parse_whitespace();
			if ReturnParser::is_declaration(parser) {
				let mut result = ReturnParser::new(parser, file.to_string(), config_data, context);
				if result.is_error() {
					result.print_error(file.to_string(), Some(&parser.content));
					break;
				} else {
					parser.parse_whitespace();
					if parser.get_curr() == ';' {
						parser.increment();
						scope_exprs.push(ScopeExpression::Return(result.unwrap_and_move().expression));
					}
				}
			} else if VariableDeclaration::is_declaration(parser) {
				let mut result = VariableDeclaration::new(parser);
				if result.is_error() {
					result.print_error(file.to_string(), Some(&parser.content));
					break;
				} else {
					let mut var_declare = result.unwrap_and_move();
					parser.reset(var_declare.start_index, var_declare.line);
					let mut reason = ExpressionEndReason::Unknown;
					let expr = parser.parse_expression(file.to_string(), config_data, Some(context), &mut reason);
					if reason == ExpressionEndReason::EndOfExpression {
						parser.parse_whitespace();
						if parser.get_curr() == ';' {
							parser.increment();
							if let Type::Inferred = var_declare.var_type.var_type {
								var_declare.var_type.var_type = expr.get_type().var_type;
							}
							context.add_variable(var_declare.name.clone(), var_declare.var_type.clone());
							scope_exprs.push(ScopeExpression::VariableDeclaration(var_declare, expr));
						}
					}
				}
			} else {
				let mut reason = ExpressionEndReason::Unknown;
				let expr = parser.parse_expression(file.to_string(), config_data, Some(context), &mut reason);
				if reason != ExpressionEndReason::EndOfExpression {
					break;
				} else {
					parser.parse_whitespace();
					if parser.get_curr() == ';' {
						parser.increment();
						scope_exprs.push(ScopeExpression::Expression(expr));
					}
				}
			}
		}

		return ScopeExpression::Scope(scope_exprs);
	}

	pub fn to_string(&self, operators: &OperatorDataStructure) -> String {
		return match self {
			ScopeExpression::Scope(exprs) => {
				let mut result = "".to_string();
				for e in exprs {
					result += &(e.to_string(operators) + "\n");
				}
				result
			},
			ScopeExpression::Expression(expr) => {
				format!("{};", expr.to_string(operators))
			},
			ScopeExpression::VariableDeclaration(declaration, expr) => {
				let var_type = &declaration.var_type;
				format!("{} {} = {};", var_type.var_style.to_cpp(&var_type.var_type), declaration.name, expr.to_string(operators))
			},
			ScopeExpression::Return(expr) => {
				format!("return {};", expr.to_string(operators))
			}
		}
	}
}/*pub enum ScopeExpression {
	Expression(Rc<Expression>),
	Scope(Vec<ScopeExpression>),
	VariableDeclaration(VariableDeclaration, Rc<Expression>),
	Return(Rc<Expression>)
}*/
