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

use crate::context_management::position::Position;
use crate::context_management::context::Context;

use std::rc::Rc;

/// Stores the expression and its components recursively. 
/// The `usize` represents the operators' index in the JSON data.
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
	ArrayAccess(Rc<Expression>, Rc<Vec<Rc<Expression>>>, VariableType, Position)
}

impl Expression {
	pub fn get_type(&self) -> VariableType {
		if let Expression::Invalid = self {
			return VariableType::inferred();
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
				Expression::Invalid => panic!("Invalid!")
			}.clone();
		}
	}

	pub fn get_line_number(&self) -> Option<usize> {
		if let Expression::Invalid = self {
			return None;
		} else if let Expression::Ternary(e, _, _, _, _) = self {
			return e.get_line_number();
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
				Expression::Invalid => panic!("Invalid!")
			}.line.unwrap_or(0));
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
				s.to_string()
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
			Expression::Infix(expr_left, expr_right, id, _, _) => {
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
					let op = expr_left.get_type().access_operator(&expr_right_str);
					format!("{}{}{}", expr_left.to_string(operators, context), op, expr_right_str)
				} else if *id == 6 {
					let right = expr_right.to_string(operators, context);
					let left = expr_left.to_string(operators, context);
					if let Expression::Expressions(..) = **expr_left {
						format!("({}){}", right, left)
					} else {
						format!("({})({})", right, left)
					}
				} else if *id == 26 || *id == 27 {
					let right_str = expr_right.to_string(operators, context);
					let right_str_final = if *id == 26 {
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
						for i in 0..prefix_lines {
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
			Expression::FunctionCall(expr, exprs, _, _) => {
				let mut expr_list = self.get_parameters(operators, context);
				format!("{}({})", expr.to_string(operators, context), expr_list.join(", "))
			},
			Expression::ConstructCall(tf_type, exprs, _, _) => {
				format!("{}({})", tf_type.to_cpp(), self.get_parameters(operators, context).join(", "))
			},
			Expression::ArrayAccess(expr, exprs, _, _) => {
				let mut expr_list = Vec::new();
				for e in exprs.iter() {
					expr_list.push(e.to_string(operators, context));
				}
				format!("{}[{}]", expr.to_string(operators, context), expr_list.join(", "))
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
}
