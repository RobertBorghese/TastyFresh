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

use crate::expression::variable_type::VariableType;

use crate::context_management::position::Position;
use crate::context_management::typing_context::Context;

use std::rc::Rc;

/// Stores the expression and its components recursively. 
/// The `i32` represents the operators' index in the JSON data.
pub enum Expression {
	Invalid,
	Value(String, VariableType, Position),
	Prefix(Rc<Expression>, usize, VariableType, Position),
	Suffix(Rc<Expression>, usize, VariableType, Position),
	Infix(Rc<Expression>, Rc<Expression>, usize, VariableType, Position),
	Ternary(Rc<Expression>, Rc<Expression>, Rc<Expression>, usize, VariableType),
	Expressions(Rc<Vec<Rc<Expression>>>, VariableType, Position),
	InitializerList(Rc<Vec<Rc<Expression>>>, VariableType, Position),
	FunctionCall(Rc<Vec<Rc<Expression>>>, VariableType, Position),
	ArrayAccess(Rc<Vec<Rc<Expression>>>, VariableType, Position)
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
				Expression::FunctionCall(_, v, _) => v,
				Expression::ArrayAccess(_, v, _) => v,
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
				Expression::FunctionCall(_, _, p) => p,
				Expression::ArrayAccess(_, _, p) => p,
				Expression::Invalid => panic!("Invalid!")
			}.line.unwrap_or(0));
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
				String::from(format!("{}{}", operators["prefix"][*id].name.as_ref().unwrap_or(&"".to_string()), expr.to_string(operators, context)))
			},
			Expression::Suffix(expr, id, _, _) => {
				String::from(format!("{}{}", expr.to_string(operators, context), operators["suffix"][*id].name.as_ref().unwrap_or(&"".to_string())))
			},
			Expression::Infix(expr_left, expr_right, id, _, _) => {
				if *id <= 1 {
					let unknown_op = "".to_string();
					let op = if *id == 1 && expr_left.get_type().is_namespace() { "::" } else { operators["infix"][*id].name.as_ref().unwrap_or(&unknown_op) };
					String::from(format!("{}{}{}", expr_left.to_string(operators, context), op, expr_right.to_string(operators, context)))
				} else {
					String::from(format!("{} {} {}", expr_left.to_string(operators, context), operators["infix"][*id].name.as_ref().unwrap_or(&"".to_string()), expr_right.to_string(operators, context)))
				}
			},
			Expression::Ternary(..) => {
				String::from("[ternary]")
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
				format!("std::make_tuple({})", expr_list.join(", "))
			},
			Expression::InitializerList(exprs, _, _) => {
				let mut expr_list = Vec::new();
				for e in exprs.iter() {
					expr_list.push(e.to_string(operators, context));
				}
				format!("{{ {} }}", expr_list.join(", "))
			},
			Expression::FunctionCall(..) => {
				String::from("[functioncall]")
			},
			Expression::ArrayAccess(..) => {
				String::from("[arrayaccess]")
			}
		}
	}
}
