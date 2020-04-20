/**********************************************************
 * --- Scope Parser ---
 *
 * Functions and structures here are dedicated to parsing
 * lists of expressions contained within scopes such as 
 * functions or encapsulated code.
 **********************************************************/

pub mod expression_scope_parser;
pub mod return_parser;
pub mod if_parser;
pub mod while_parser;
pub mod loop_parser;
pub mod dowhile_parser;
pub mod for_parser;
pub mod inject_parser;

use crate::declaration_parser::parser::Parser;
use crate::declaration_parser::variable_declaration::{ VariableDeclaration, VariableExportType };

use crate::expression::Expression;
use crate::expression::expression_parser::ExpressionEndReason;
use crate::expression::variable_type::VariableType;

use crate::scope_parser::return_parser::ReturnParser;
use crate::scope_parser::if_parser::{ IfParser, IfType };
use crate::scope_parser::while_parser::WhileParser;
use crate::scope_parser::loop_parser::LoopParser;
use crate::scope_parser::dowhile_parser::DoWhileParser;
use crate::scope_parser::for_parser::ForParser;
use crate::scope_parser::inject_parser::InjectParser;

use crate::config_management::ConfigData;
use crate::config_management::operator_data::OperatorDataStructure;

use crate::context_management::context::Context;

use std::rc::Rc;

use regex::Regex;

pub enum ScopeExpression {
	Expression(Rc<Expression>),
	Scope(Vec<ScopeExpression>),
	SubScope(Box<ScopeExpression>, usize, usize),
	VariableDeclaration(VariableDeclaration, Option<Rc<Expression>>),
	Return(Rc<Expression>, usize),
	If(IfType, Option<Rc<Expression>>, Box<ScopeExpression>, usize, usize),
	While(Rc<Expression>, Box<ScopeExpression>, usize, usize),
	Loop(Box<ScopeExpression>, usize, usize),
	DoWhile(Rc<Expression>, Box<ScopeExpression>, usize, usize, usize),
	For(String, Rc<Expression>, Box<ScopeExpression>, usize, usize),
	Increment(String, Rc<Expression>, Rc<Expression>, Option<Rc<Expression>>, Box<ScopeExpression>, bool, usize, usize),
	Decrement(String, Rc<Expression>, Rc<Expression>, Option<Rc<Expression>>, Box<ScopeExpression>, bool, usize, usize),
	Injection(String, usize, usize)
}

impl ScopeExpression {
	pub fn new(parser: &mut Parser, limit: Option<usize>, start_index: usize, line: usize, file: &str, config_data: &ConfigData, context: &mut Context, expected_return_type: Option<VariableType>) -> ScopeExpression {
		parser.reset(start_index, line);

		let mut scope_exprs = Vec::new();

		loop {
			if limit.is_some() {
				if scope_exprs.len() >= *limit.as_ref().unwrap() {
					break;
				}
			}
			parser.parse_whitespace();
			if ReturnParser::is_declaration(parser) {
				let result = ReturnParser::new(parser, file.to_string(), config_data, context, expected_return_type.clone());
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
			} else if IfParser::is_declaration(parser) {
				let result = IfParser::new(parser, file.to_string(), config_data, context);
				if result.is_error() {
					result.print_error(file.to_string(), &parser.content);
					break;
				} else {
					parser.parse_whitespace();
					let if_declare = result.unwrap_and_move();
					scope_exprs.push(ScopeExpression::If(if_declare.if_type, if_declare.expression, if_declare.scope, if_declare.line, if_declare.end_line));
				}
			} else if WhileParser::is_declaration(parser) {
				let result = WhileParser::new(parser, file.to_string(), config_data, context);
				if result.is_error() {
					result.print_error(file.to_string(), &parser.content);
					break;
				} else {
					parser.parse_whitespace();
					let while_declare = result.unwrap_and_move();
					scope_exprs.push(ScopeExpression::While(while_declare.expression, while_declare.scope, while_declare.line, while_declare.end_line));
				}
			} else if LoopParser::is_declaration(parser) {
				let result = LoopParser::new(parser, file.to_string(), config_data, context);
				if result.is_error() {
					result.print_error(file.to_string(), &parser.content);
					break;
				} else {
					parser.parse_whitespace();
					let loop_declare = result.unwrap_and_move();
					scope_exprs.push(ScopeExpression::Loop(loop_declare.scope, loop_declare.line, loop_declare.end_line));
				}
			} else if DoWhileParser::is_declaration(parser) {
				let result = DoWhileParser::new(parser, file.to_string(), config_data, context);
				if result.is_error() {
					result.print_error(file.to_string(), &parser.content);
					break;
				} else {
					parser.parse_whitespace();
					let do_while_declare = result.unwrap_and_move();
					scope_exprs.push(ScopeExpression::DoWhile(do_while_declare.expression, do_while_declare.scope, do_while_declare.line, do_while_declare.end_line, do_while_declare.while_offset));
				}
			} else if InjectParser::is_declaration(parser) {
				let result = InjectParser::new(parser);
				if result.is_error() {
					result.print_error(file.to_string(), &parser.content);
					break;
				} else {
					parser.parse_whitespace();
					let inject_declare = result.unwrap_and_move();
					scope_exprs.push(ScopeExpression::Injection(parser.content[inject_declare.start_index..inject_declare.end_index].to_string(), inject_declare.line, inject_declare.end_line));
				}
			} else if ForParser::is_declaration(parser) {
				let result = ForParser::new(parser, file.to_string(), config_data, context);
				if result.is_error() {
					result.print_error(file.to_string(), &parser.content);
					break;
				} else {
					parser.parse_whitespace();
					let for_declare = result.unwrap_and_move();
					if for_declare.for_type.is_for() {
						scope_exprs.push(ScopeExpression::For(
							for_declare.var_name,
							for_declare.content.left().unwrap(),
							for_declare.scope,
							for_declare.line,
							for_declare.end_line
						));
					} else if for_declare.for_type.is_increment() || for_declare.for_type.is_incrementto() {
						let exprs = for_declare.content.right().unwrap();
						scope_exprs.push(ScopeExpression::Increment(
							for_declare.var_name,
							exprs.0,
							exprs.1,
							exprs.2,
							for_declare.scope,
							for_declare.for_type.is_incrementto(),
							for_declare.line,
							for_declare.end_line
						));
					} else if for_declare.for_type.is_decrement() || for_declare.for_type.is_decrementto() {
						let exprs = for_declare.content.right().unwrap();
						scope_exprs.push(ScopeExpression::Decrement(
							for_declare.var_name,
							exprs.0,
							exprs.1,
							exprs.2,
							for_declare.scope,
							for_declare.for_type.is_decrementto(),
							for_declare.line,
							for_declare.end_line
						));
					}
				}
			} else if VariableDeclaration::is_declaration(parser) {
				let result = VariableDeclaration::new(parser);
				if result.is_error() {
					result.print_error(file.to_string(), &parser.content);
					break;
				} else {
					let mut var_declare = result.unwrap_and_move();
					if var_declare.value.is_some() {
						parser.reset(var_declare.value.as_ref().unwrap().0, var_declare.line);
						let mut reason = ExpressionEndReason::Unknown;
						let expr = parser.parse_expression(file.to_string(), config_data, Some(context), &mut reason, Some(var_declare.var_type.clone()));
						if reason == ExpressionEndReason::EndOfExpression {
							parser.parse_whitespace();
							if parser.get_curr() == ';' {
								parser.increment();
								if var_declare.var_type.is_inferred() {
									var_declare.var_type.var_type = expr.get_type().var_type;
								}
								if var_declare.var_type.var_style.is_inferred() {
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
			} else if parser.get_curr() == '{' {
				//line: usize, file: &str, config_data: &ConfigData, context
				let initial_line = parser.line;
				parser.increment();
				scope_exprs.push(ScopeExpression::SubScope(
					Box::new(ScopeExpression::new(parser, limit, parser.index, parser.line, file, config_data, context, None)),
					initial_line,
					parser.line
				));
				parser.increment();
				parser.parse_whitespace();
			} else {
				if parser.get_curr() == '}' {
					break;
				}
				let mut reason = ExpressionEndReason::Unknown;
				let expr = parser.parse_expression(file.to_string(), config_data, Some(context), &mut reason, None);
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
					let line = e.to_string(operators, line_offset, tab_offset, context);
					let real_line_number = e.get_line().unwrap_or(line_offset) - line_offset;
					let line_number = if context.align_lines {
						real_line_number
					} else {
						if e.is_extend() {
							last_line_offset
						} else if real_line_number - real_last_line_offset > 1 {
							last_line_offset + 2
						} else {
							last_line_offset + 1
						}
					};
					let re = Regex::new("(?:\n\r|\r\n|\r|\n)").unwrap();
					let mut curr_line = line_number;
					for scope_line in re.split(&line) {
						while curr_line >= lines.len() {
							lines.push(Vec::new());
						}
						lines[curr_line].push(scope_line.to_string());
						curr_line += 1;
					}
					last_line_offset = curr_line - 1;
					real_last_line_offset = e.get_end_line().unwrap_or(real_line_number + line_offset) - line_offset;
				}
				let tabs = String::from_utf8(vec![b'\t'; tab_offset]).unwrap_or("".to_string());
				let mut content = Vec::new();
				let mut first = true;
				for l in &lines {
					let sub_lines = l.join(" ").split("\n").map(|line| if line.is_empty() { "".to_string() } else { if first { " ".to_string() + line } else { tabs.clone() + line } }).collect::<Vec<String>>().join("\n");
					content.push(sub_lines);
					first = false;
				}
				content.join("\n")
			},
			ScopeExpression::Expression(expr) => {
				format!("{};", expr.to_string(operators, context))
			},
			ScopeExpression::Injection(content, _, _) => {
				let mut result = "".to_string();
				let re = Regex::new("(?:\n\r|\r\n|\r|\n)").unwrap();
				let mut initial_tab_offset: Option<usize> = None;
				for line in re.split(&content) {
					if line.trim().is_empty() {
						if initial_tab_offset.is_none() { result += "\n"; }
						continue;
					}
					let chars = line.chars().collect::<Vec<char>>();
					let mut front_tab_index = 0;
					while front_tab_index < chars.len() && chars[front_tab_index] == '\t' {
						front_tab_index += 1;
					}
					if initial_tab_offset.is_some() {
						let init = *initial_tab_offset.as_ref().unwrap();
						if init < front_tab_index {
							front_tab_index = init;
						}
					} else {
						initial_tab_offset = Some(front_tab_index);
					}
					result += format!("{}{}", &line[front_tab_index..], "\n").as_str();
				}
				if context.align_lines {
					format!("{}", result)
				} else {
					format!("{}", result.trim())
				}
			},
			ScopeExpression::VariableDeclaration(declaration, expr) => {
				declaration.to_cpp(expr, operators, context, VariableExportType::Scoped, false)
			},
			ScopeExpression::Return(expr, _) => {
				format!("return {};", expr.to_string(operators, context))
			},
			ScopeExpression::SubScope(scope, line, end_line) => {
				let scope_str = scope.to_string(operators, *line, tab_offset, context);
				format!("{}", self.format_scope_contents(&scope_str, context, line, end_line))
			},
			ScopeExpression::If(if_type, expr, scope, line, end_line) => {
				let expr_str = if expr.is_some() { expr.as_ref().unwrap().to_string(operators, context) } else { "".to_string() };
				let scope_str = scope.to_string(operators, *line, tab_offset, context);
				format!("{} {}", if if_type.is_else() {
						"else".to_string()
					} else {
						format!("{}if({})", if if_type.is_elseif() {
								"else "
							} else {
								""
							}, if context.align_lines {
								&expr_str
							} else {
								expr_str.trim()
							}
						)
					}, self.format_scope_contents(&scope_str, context, line, end_line))
			},
			ScopeExpression::While(expr, scope, line, end_line) => {
				let expr_str = expr.to_string(operators, context);
				let scope_str = scope.to_string(operators, *line, tab_offset, context);
				format!("while({}) {}", if context.align_lines {
					&expr_str
				} else {
					expr_str.trim()
				}, self.format_scope_contents(&scope_str, context, line, end_line))
			},
			ScopeExpression::Loop(scope, line, end_line) => {
				let scope_str = scope.to_string(operators, *line, tab_offset, context);
				format!("while(true) {}", self.format_scope_contents(&scope_str, context, line, end_line))
			},
			ScopeExpression::DoWhile(expr, scope, line, end_line, while_offset) => {
				let expr_str = expr.to_string(operators, context);
				let scope_str = scope.to_string(operators, *line, tab_offset, context);
				format!("do {}{}while({});",
					self.format_scope_contents(&scope_str, context, line, end_line),
					if context.align_lines {
						let tabs = String::from_utf8(vec![b'\t'; tab_offset]).unwrap_or("".to_string());
						let mut result = "".to_string();
						for _ in 0..*while_offset {
							result += format!("{}\n", tabs).as_str();
						}
						result
					} else { " ".to_string() },
					if context.align_lines {
						&expr_str
					} else {
						expr_str.trim()
					}
				)
			},
			ScopeExpression::For(name, expr, scope, line, end_line) => {
				let expr_str = expr.to_string(operators, context);
				let scope_str = scope.to_string(operators, *line, tab_offset, context);
				format!("for(auto& {} : {}) {}", name, if context.align_lines {
					&expr_str
				} else {
					expr_str.trim()
				}, self.format_scope_contents(&scope_str, context, line, end_line))
			},
			ScopeExpression::Increment(name, start_expr, end_expr, by_expr, scope, is_to, line, end_line) => {
				let start_str = start_expr.to_string(operators, context);
				let end_str = end_expr.to_string(operators, context);
				let by_str = if by_expr.is_none() { None } else { Some(by_expr.as_ref().unwrap().to_string(operators, context)) };
				let scope_str = scope.to_string(operators, *line, tab_offset, context);
				format!("for({} {} = {}; i {} {}; {}) {}", start_expr.get_type().to_cpp(false), name, if context.align_lines {
					&start_str
				} else {
					start_str.trim()
				},
				if *is_to { "<=" } else { "<" },
				if context.align_lines {
					&end_str
				} else {
					end_str.trim()
				},
				if by_str.is_none() {
					"i++".to_string()
				} else {
					format!("i += {}", if context.align_lines {
						&by_str.as_ref().unwrap()
					} else {
						by_str.as_ref().unwrap().trim()
					})
				}, self.format_scope_contents(&scope_str, context, line, end_line))
			},
			ScopeExpression::Decrement(name, start_expr, end_expr, by_expr, scope, is_to, line, end_line) => {
				let start_str = start_expr.to_string(operators, context);
				let end_str = end_expr.to_string(operators, context);
				let by_str = if by_expr.is_none() { None } else { Some(by_expr.as_ref().unwrap().to_string(operators, context)) };
				let scope_str = scope.to_string(operators, *line, tab_offset, context);
				format!("for({} {} = {}; i {} {}; {}) {}", start_expr.get_type().to_cpp(false), name, if context.align_lines {
					&start_str
				} else {
					start_str.trim()
				},
				if *is_to { ">=" } else { ">" },
				if context.align_lines {
					&end_str
				} else {
					end_str.trim()
				},
				if by_str.is_none() {
					"i--".to_string()
				} else {
					format!("i -= {}", if context.align_lines {
						&by_str.as_ref().unwrap()
					} else {
						by_str.as_ref().unwrap().trim()
					})
				}, self.format_scope_contents(&scope_str, context, line, end_line))
			}
		}
	}

	pub fn format_scope_contents(&self, scope_str: &str, context: &mut Context, line: &usize, end_line: &usize) -> String {
		if context.align_lines {
			let re = Regex::new("(?:\n\r|\r\n|\r|\n)").unwrap();
			let mut final_line = *line - 1;
			for _ in re.split(&scope_str) {
				final_line += 1;
			}
			return format!("{{{}{}}}", scope_str, if final_line == *end_line { " " } else { "\n" });
		}
		return format!("{{\n\t{}\n}}", scope_str.trim());
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
			ScopeExpression::SubScope(_, line, _) => Some(*line),
			ScopeExpression::VariableDeclaration(declare, _) => Some(declare.line),
			ScopeExpression::Return(_, line) => Some(*line),
			ScopeExpression::If(_, _, _, line, _) => Some(*line),
			ScopeExpression::While(_, _, line, _) => Some(*line),
			ScopeExpression::Loop(_, line, _) => Some(*line),
			ScopeExpression::DoWhile(_, _, line, _, _) => Some(*line),
			ScopeExpression::For(_, _, _, line, _) => Some(*line),
			ScopeExpression::Increment(_, _, _, _, _, _, line, _) => Some(*line),
			ScopeExpression::Decrement(_, _, _, _, _, _, line, _) => Some(*line),
			ScopeExpression::Injection(_, line, _) => Some(*line),
			_ => None
		};
	}

	pub fn get_end_line(&self) -> Option<usize> {
		return match self {
			ScopeExpression::Expression(expr) => expr.get_line_number(),
			ScopeExpression::SubScope(_, _, end_line) => Some(*end_line),
			ScopeExpression::If(_, _, _, _, end_line) => Some(*end_line),
			ScopeExpression::While(_, _, _, end_line) => Some(*end_line),
			ScopeExpression::Loop(_, _, end_line) => Some(*end_line),
			ScopeExpression::DoWhile(_, _, _, _, end_line) => Some(*end_line),
			ScopeExpression::For(_, _, _, _, end_line) => Some(*end_line),
			ScopeExpression::Increment(_, _, _, _, _, _, _, end_line) => Some(*end_line),
			ScopeExpression::Decrement(_, _, _, _, _, _, _, end_line) => Some(*end_line),
			ScopeExpression::Injection(_, _, end_line) => Some(*end_line),
			_ => None
		};
	}

	pub fn is_extend(&self) -> bool {
		return match self {
			ScopeExpression::If(if_type, _, _, _, _) => if_type.is_elseif() || if_type.is_else(),
			_ => false
		};
	}

	pub fn is_inject(&self) -> bool {
		return match self {
			ScopeExpression::Injection(..) => true,
			_ => false
		};
	}
}
