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
					result.print_error(file.to_string(), &parser.content);
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
					result.print_error(file.to_string(), &parser.content);
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

	pub fn to_string(&self, operators: &OperatorDataStructure, line_offset: usize, tab_offset: usize, context: &mut Context) -> String {
		return match self {
			ScopeExpression::Scope(exprs) => {
				let mut lines = Vec::new();
				for e in exprs {
					let line = (e.to_string(operators, line_offset, tab_offset, context));
					let line_number = e.get_expression().unwrap().get_line_number() - line_offset;
					while line_number >= lines.len() {
						lines.push(Vec::new());
					}
					lines[line_number].push(line);
				}
				let tabs = String::from_utf8(vec![b'\t'; tab_offset]).unwrap_or("".to_string());
				let mut content = Vec::new();
				for l in lines {
					content.push(tabs.clone() + &l.join(" "));
				}
				content.join("\n")
			},
			ScopeExpression::Expression(expr) => {
				format!("{};", expr.to_string(operators, context))
			},
			ScopeExpression::VariableDeclaration(declaration, expr) => {
				let var_type = &declaration.var_type;
				println!("DELCARE TYPE: {}", var_type.var_style.get_name());
				format!("{} {} = {};", var_type.to_cpp(), declaration.name, expr.to_string(operators, context))
			},
			ScopeExpression::Return(expr) => {
				format!("return {};", expr.to_string(operators, context))
			}
		}
	}

	pub fn get_expression(&self) -> Option<Rc<Expression>> {
		return match self {
			ScopeExpression::Expression(expr) => Some(Rc::clone(expr)),
			ScopeExpression::VariableDeclaration(_, expr) => Some(Rc::clone(expr)),
			ScopeExpression::Return(expr) => Some(Rc::clone(expr)),
			_ => None
		};
	}
}
