/**********************************************************
 * --- Expression ---
 *
 * Individual Tasty Fresh expressions are represented 
 * using the `Expression` struct provided in this file.
 **********************************************************/

pub mod expression_component;
pub mod expression_parser;
pub mod expression_piece;
pub mod value_type;
pub mod variable_type;
pub mod function_type;

use crate::config_management::operator_data::OperatorDataStructure;

use crate::expression::variable_type::{ Type, VariableType };
use crate::expression::value_type::{ Property, Function };

use crate::context_management::position::Position;
use crate::context_management::context::Context;

use crate::scope_parser::ScopeExpression;

use std::rc::Rc;

use regex::Regex;

/// Stores the expression and its components recursively. 
/// The `usize` represents the operators' index in the JSON data.
#[derive(Clone)]
pub enum Expression {
	Invalid,
	Value(String, VariableType, Position),
	Prefix(Rc<Expression>, usize, VariableType, Position),
	Suffix(Rc<Expression>, usize, VariableType, Position),
	Infix(Rc<Expression>, Rc<Expression>, usize, VariableType, Position),
	Ternary(Rc<Expression>, Rc<Expression>, Rc<Expression>, usize, VariableType),
	Expressions(Rc<Vec<Rc<Expression>>>, VariableType, Position),
	InitializerList(Rc<Vec<Rc<Expression>>>, VariableType, Position),
	FunctionCall(Rc<Expression>, Rc<Vec<Rc<Expression>>>, VariableType, Position),
	ConstructCall(Type, Rc<Vec<Rc<Expression>>>, VariableType, Position),
	ArrayAccess(Rc<Expression>, Rc<Vec<Rc<Expression>>>, VariableType, Position),
	Function(Rc<ScopeExpression>, Vec<String>, Vec<(VariableType, String, Option<String>)>, VariableType, usize, Position)
}

impl Expression {
	pub fn get_type(&self) -> VariableType {
		if let Expression::Invalid = self {
			return VariableType::inferred();
		} else {
			if let Expression::Function(_, _, params, return_type, _, _) = self {
				let mut is_inferred = false;
				let mut props = Vec::new();
				for p in params {
					if p.0.is_inferred() {
						is_inferred = true;
						break;
					}
					props.push(Property {
						name: p.1.clone(),
						prop_type: p.0.clone(),
						default_value: p.2.clone(),
						is_declare: false
					});
				}
				if is_inferred {
					VariableType::inferred()
				} else {
					VariableType::function(Function {
						name: "".to_string(),
						parameters: props,
						return_type: return_type.clone(),
						styles: Vec::new()
					})
				}
			} else {
				return match self {
					Expression::Value(_, v, _) => v,
					Expression::Prefix(_, _, v, _) => v,
					Expression::Suffix(_, _, v, _) => v,
					Expression::Infix(_, _, _, v, _) => v,
					Expression::Ternary(_, _, _, _, v) => v,
					Expression::Expressions(_, v, _) => v,
					Expression::InitializerList(_, v, _) => v,
					Expression::FunctionCall(_, _, v, _) => v,
					Expression::ConstructCall(_, _, v, _) => v,
					Expression::ArrayAccess(_, _, v, _) => v,
					Expression::Invalid | Expression::Function(..) => panic!("Invalid!")
				}.clone();
			}
		}
	}

	pub fn get_line_number(&self) -> Option<usize> {
		if let Expression::Ternary(e, _, _, _, _) = self {
			return e.get_line_number();
		} else {
			let pos = self.get_position();
			if pos.is_some() {
				return Some(pos.unwrap().line.unwrap_or(0));
			} else {
				return None;
			}
		}
	}

	pub fn get_position(&self) -> Option<Position> {
		if let Expression::Invalid = self {
			return None;
		} else if let Expression::Ternary(_, _, _, _, _) = self {
			return None;
		} else {
			return Some(match self {
				Expression::Value(_, _, p) => p,
				Expression::Prefix(_, _, _, p) => p,
				Expression::Suffix(_, _, _, p) => p,
				Expression::Infix(_, _, _, _, p) => p,
				Expression::Ternary(_, _, _, _, _) => panic!("Ternary!"),
				Expression::Expressions(_, _, p) => p,
				Expression::InitializerList(_, _, p) => p,
				Expression::FunctionCall(_, _, _, p) => p,
				Expression::ConstructCall(_, _, _, p) => p,
				Expression::ArrayAccess(_, _, _, p) => p,
				Expression::Function(_, _, _, _, _, p) => p,
				Expression::Invalid => panic!("Invalid!")
			}.clone());
		}
	}

	pub fn get_op_type(&self) -> Option<usize> {
		return match self {
			Expression::Value(_, _, _) => None,
			Expression::Prefix(_, id, _, _) => Some(*id),
			Expression::Suffix(_, id, _, _) => Some(*id),
			Expression::Infix(_, _, id, _, _) => Some(*id),
			Expression::Ternary(_, _, _, _, _) => None,
			Expression::Expressions(_, _, _) => None,
			Expression::InitializerList(_, _, _) => None,
			Expression::FunctionCall(_, _, _, _) => None,
			Expression::ConstructCall(_, _, _, _) => None,
			Expression::ArrayAccess(_, _, _, _) => None,
			Expression::Function(_, _, _, _, _, _) => None,
			Expression::Invalid => None
		}
	}

	pub fn deconstruct_new(&self, operators: &OperatorDataStructure, context: &mut Context) -> Option<Vec<String>> {
		return match self {
			Expression::Prefix(expr, id, _, _) => {
				if *id == 9 {
					 match &**expr {
						Expression::FunctionCall(expr2, args, _, _) => {
							let mut expr_list = Vec::new();
							for e in args.iter() {
								expr_list.push(e.to_string(operators, context));
							}
							match &**expr2 {
								Expression::Value(s, _, _) => Some(vec![s.to_string(), expr_list.join(", ")]),
								Expression::Infix(_, _, _, _, _) => Some(vec![expr2.to_string(operators, context), expr_list.join(", ")]),
								_ => None
							}
						},
						_ => None
					}
				} else {
					None
				}
			},
			_ => None
		}
	}

	pub fn to_string(&self, operators: &OperatorDataStructure, context: &mut Context) -> String {
		return match self {
			Expression::Invalid => {
				"Invalid".to_string()
			},
			Expression::Value(s, _, _) => {
				if context.convert_this_to_self && s == "this" {
					"self".to_string()
				} else {
					s.to_string()
				}
			},
			Expression::Prefix(expr, id, _, _) => {
				if *id == 9 {
					expr.to_string(operators, context)
				} else {
					let operator_data = &operators["prefix"][*id];
					format!("{}{}{}",
						operator_data.name.as_ref().unwrap_or(&"".to_string()),
						if operator_data.cannot_touch { " " } else { "" },
						expr.to_string(operators, context)
					)
				}
			},
			Expression::Suffix(expr, id, _, _) => {
				format!("{}{}", expr.to_string(operators, context), operators["suffix"][*id].name.as_ref().unwrap_or(&"".to_string()))
			},
			Expression::Infix(expr_left, expr_right, id, tf_type, _) => {
				if *id == 1 {
					let insides = expr_right.to_string(operators, context);
					format!("{}<{}>", expr_left.to_string(operators, context), 
						if insides.starts_with('(') && insides.ends_with(')') {
							&insides[1..insides.len() - 1]
						} else if insides.starts_with("std::make_tuple(") && insides.ends_with(')') {
							&insides[16..insides.len() - 1]
						} else { &insides }
					)
				} else if *id == 2 {
					let expr_right_str = expr_right.to_string(operators, context);
					if expr_right.get_type().is_int() {
						format!("std::get<{}>({})", expr_right_str, expr_left.to_string(operators, context))
					} else {
						let op = expr_left.get_type().access_operator();
						format!("{}{}{}", expr_left.to_string(operators, context), op, expr_right_str)
					}
				} else if *id <= 5 && *id != 1 {
					format!("{}{}{}",
						expr_left.to_string(operators, context),
						operators["infix"][*id].name.as_ref().unwrap_or(&"".to_string()),
						expr_right.to_string(operators, context)
					)
				} else if *id >= 6 && *id <= 9 {
					let mut right = tf_type.to_cpp(); // expr_right.to_string(operators, context);
					right = match *id {
						6 => format!("({})", right),
						7 => format!("static_cast<{}>", right),
						8 => format!("reinterpret_cast<{}>", right),
						9 => format!("dynamic_cast<{}>", right),
						_ => "".to_string()
					};
					let left = expr_left.to_string(operators, context);
					if let Expression::Expressions(..) = **expr_left {
						format!("{}{}", right, left)
					} else {
						format!("{}({})", right, left)
					}
				} else if *id == 29 || *id == 30 {
					let right_str = expr_right.to_string(operators, context);
					let right_str_final = if *id == 29 && !expr_right.get_type().is_inferred() {
						expr_right.get_type().convert_between_styles(&expr_left.get_type(), &right_str).unwrap_or(right_str.to_string())
					} else {
						right_str
					};
					format!("{} {} {}", expr_left.to_string(operators, context), "=", right_str_final)
				} else {
					format!("{} {} {}", expr_left.to_string(operators, context), operators["infix"][*id].name.as_ref().unwrap_or(&"".to_string()), expr_right.to_string(operators, context))
				}
			},
			Expression::Ternary(expr_1, expr_2, expr_3, _, _) => {
				format!("{} ? {} : {}", 
					expr_1.to_string(operators, context), 
					expr_2.to_string(operators, context), 
					expr_3.to_string(operators, context))
			},
			Expression::Expressions(exprs, _, _) => {
				let mut expr_list = Vec::new();
				let mut curr_line: Option<usize> = None;
				for e in exprs.iter() {
					let prefix_lines = if curr_line.is_some() {
						let curr = curr_line.unwrap();
						let result = e.get_line_number().unwrap_or(curr) - curr;
						curr_line = Some(e.get_line_number().unwrap_or(curr));
						result
					} else {
						curr_line = Some(e.get_line_number().unwrap_or(0));
						0
					};
					let prefix = if prefix_lines == 0 {
						"".to_string()
					} else {
						let mut result = "".to_string();
						for _ in 0..prefix_lines {
							result += "\n\t";
						}
						result
					};
					expr_list.push(prefix + &e.to_string(operators, context));
				}
				if expr_list.len() == 1 {
					format!("({})", expr_list.first().unwrap())
				} else {
					format!("std::make_tuple({})", expr_list.join(", "))
				}
			},
			Expression::InitializerList(exprs, _, _) => {
				let mut expr_list = Vec::new();
				for e in exprs.iter() {
					expr_list.push(e.to_string(operators, context));
				}
				format!("{{ {} }}", expr_list.join(", "))
			},
			Expression::FunctionCall(expr, _, _, _) => {
				let expr_list = self.get_parameters(operators, context);
				format!("{}({})", expr.to_string(operators, context), expr_list.join(", "))
			},
			Expression::ConstructCall(tf_type, _, _, _) => {
				format!("{}({})", tf_type.to_cpp(false), self.get_parameters(operators, context).join(", "))
			},
			Expression::ArrayAccess(expr, exprs, _, _) => {
				let mut expr_list = Vec::new();
				for e in exprs.iter() {
					expr_list.push(e.to_string(operators, context));
				}
				format!("{}[{}]", expr.to_string(operators, context), expr_list.join(", "))
			},
			Expression::Function(scope, captures, params, return_type, end_line, pos) => {
				let mut prop_list = Vec::new();
				for p in params {
					prop_list.push({
						let var_cpp = p.0.to_cpp();
						if var_cpp == "auto" {
							p.1.clone()
						} else if p.2.is_some() {
							format!("{} {} = {}", var_cpp, p.1, p.2.as_ref().unwrap())
						} else {
							format!("{} {}", var_cpp, p.1)
						}
					});
				}

				let scope_str = scope.to_string(operators, 0, 1, context);
				let final_scope_str =  if context.align_lines {
					let re = Regex::new("(?:\n\r|\r\n|\r|\n)").unwrap();
					let mut final_line = *pos.line.as_ref().unwrap() - 1;
					for _ in re.split(&scope_str) {
						final_line += 1;
					}
					format!("{{{}{}}}", scope_str, if final_line == *end_line { " " } else { "\n" })
				} else {
					format!("{{\n\t{}\n}}", scope_str.trim())
				};

				if return_type.is_void() {
					format!("[{}]({}) {}", captures.join(", "), prop_list.join(", "), final_scope_str)
				} else {
					format!("[{}]({}) -> {} {}",
						captures.join(", "),
						prop_list.join(", "),
						return_type.to_cpp(),
						final_scope_str
					)
				}
			}
		}
	}

	pub fn is_construction_call(&self) -> bool {
		if let Expression::ConstructCall(..) = self {
			return true;
		}
		return false;
	}

	pub fn get_parameters(&self, operators: &OperatorDataStructure, context: &mut Context) -> Vec<String> {
		let mut result = Vec::new();
		match self {
			Expression::FunctionCall(_, params, _, _) => {
				for e in params.iter() {
					result.push(e.to_string(operators, context));
				}
			},
			Expression::ConstructCall(_, params, _, _) => {
				for e in params.iter() {
					result.push(e.to_string(operators, context));
				}
			},
			_ => ()
		}
		return result;
	}

	pub fn reverse_bool(&self) -> Expression {
		match self {
			Expression::Prefix(expr, operator_id, _, position) => {
				if *operator_id == 4 {
					return (**expr).clone();
				}
				return Expression::Prefix(Rc::new(self.clone()), 4, VariableType::boolean(), position.clone());
			},
			Expression::Infix(left_expr, right_expr, operator_id, _, position) => {
				if *operator_id >= 18 && *operator_id <= 23 {
					return Expression::Infix(Rc::clone(left_expr), Rc::clone(right_expr), match *operator_id {
						18 => 21,
						19 => 20,
						20 => 19,
						21 => 18,
						22 => 23,
						23 => 22,
						_ => 0
					}, VariableType::boolean(), position.clone());
				} else if *operator_id == 27 || *operator_id == 28 {
					return Expression::Infix(Rc::new(left_expr.reverse_bool()), Rc::new(right_expr.reverse_bool()), if *operator_id == 27 { 28 } else { 27 }, VariableType::boolean(), position.clone());
				}
			},
			_ => ()
		}

		// If all else fails, wrap with !
		let curr_pos = self.get_position().unwrap_or(Position::new("".to_string(), Some(0), 0, None));
		match self {
			Expression::Expressions(..) |
			Expression::Value(..) |
			Expression::FunctionCall(..) |
			Expression::ArrayAccess(..) => {
				return Expression::Prefix(Rc::new(self.clone()), 4, VariableType::boolean(), curr_pos.clone());
			},
			_ => ()
		}

		// Or even worse, wrap with !()
		let exprs = vec![Rc::new(self.clone())];
		let exprs_expr = Expression::Expressions(Rc::new(exprs), self.get_type(), curr_pos.clone());
		return Expression::Prefix(Rc::new(exprs_expr), 4, VariableType::boolean(), curr_pos.clone());
	}
}
