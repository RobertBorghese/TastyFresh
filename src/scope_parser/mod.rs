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

use crate::context_management::context::Context;

use std::rc::Rc;

pub enum ScopeExpression {
	Expression(Rc<Expression>),
	Scope(Vec<ScopeExpression>),
	VariableDeclaration(VariableDeclaration, Option<Rc<Expression>>),
	Return(Rc<Expression>, usize)
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
						let return_declare = result.unwrap_and_move();
						scope_exprs.push(ScopeExpression::Return(return_declare.expression, return_declare.line));
					}
				}
			} else if VariableDeclaration::is_declaration(parser) {
				let mut result = VariableDeclaration::new(parser);
				if result.is_error() {
					result.print_error(file.to_string(), &parser.content);
					break;
				} else {
					let mut var_declare = result.unwrap_and_move();
					if var_declare.value.is_some() {
						parser.reset(var_declare.value.as_ref().unwrap().0, var_declare.line);
						let mut reason = ExpressionEndReason::Unknown;
						let expr = parser.parse_expression(file.to_string(), config_data, Some(context), &mut reason);
						if reason == ExpressionEndReason::EndOfExpression {
							parser.parse_whitespace();
							if parser.get_curr() == ';' {
								parser.increment();
								if var_declare.var_type.is_inferred() {
									var_declare.var_type.var_type = expr.get_type().var_type;
								}
								println!("{}", var_declare.var_type.var_style.get_name());
								if var_declare.var_type.var_style.is_inferred() {
									println!("test");
									var_declare.var_type.var_style = var_declare.var_type.var_style.attempt_inference(&expr.get_type());
								}
								context.register_type(&var_declare.var_type);
								context.typing.add_variable(var_declare.name.clone(), var_declare.var_type.clone());
								scope_exprs.push(ScopeExpression::VariableDeclaration(var_declare, Some(expr)));
							}
						}
					} else {
						context.register_type(&var_declare.var_type);
						context.typing.add_variable(var_declare.name.clone(), var_declare.var_type.clone());
						scope_exprs.push(ScopeExpression::VariableDeclaration(var_declare, None));
						if parser.get_curr() == ';' {
							parser.increment();
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
				let mut last_line_offset = 0;
				let mut real_last_line_offset = 0;
				for e in exprs {
					let line = (e.to_string(operators, line_offset, tab_offset, context));
					let real_line_number = e.get_line().unwrap_or(line_offset) - line_offset;
					let line_number = if context.align_lines {
						real_line_number
					} else {
						if real_line_number - real_last_line_offset > 1 {
							last_line_offset + 2
						} else {
							last_line_offset + 1
						}
					};
					while line_number >= lines.len() {
						lines.push(Vec::new());
					}
					lines[line_number].push(line);
					last_line_offset = line_number;
					real_last_line_offset = real_line_number;
				}
				let tabs = String::from_utf8(vec![b'\t'; tab_offset]).unwrap_or("".to_string());
				let mut content = Vec::new();
				for l in &lines {
					let sub_lines = l.join(" ").split("\n").map(|line| if line.is_empty() { "".to_string() } else { tabs.clone() + line }).collect::<Vec<String>>().join("\n");
					content.push(sub_lines);
				}
				content.join("\n")
			},
			ScopeExpression::Expression(expr) => {
				format!("{};", expr.to_string(operators, context))
			},
			ScopeExpression::VariableDeclaration(declaration, expr) => {
				declaration.to_cpp(expr, operators, context)
			},
			ScopeExpression::Return(expr, _) => {
				format!("return {};", expr.to_string(operators, context))
			}
		}
	}

	pub fn get_expression(&self) -> Option<Rc<Expression>> {
		return match self {
			ScopeExpression::Expression(expr) => Some(Rc::clone(expr)),
			ScopeExpression::VariableDeclaration(_, expr) => {
				if expr.is_some() {
					Some(Rc::clone(expr.as_ref().unwrap()))
				} else {
					None
				}
			},
			ScopeExpression::Return(expr, _) => Some(Rc::clone(expr)),
			_ => None
		};
	}

	pub fn get_line(&self) -> Option<usize> {
		return match self {
			ScopeExpression::Expression(expr) => expr.get_line_number(),
			ScopeExpression::VariableDeclaration(declare, _) => Some(declare.line),
			ScopeExpression::Return(_, line) => Some(*line),
			_ => None
		};
	}
}
