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

use std::rc::Rc;

/// Stores the expression and its components recursively. 
/// The `i32` represents the operators' index in the JSON data.
pub enum Expression {
	Invalid,
	Value(String, VariableType),
	Prefix(Rc<Expression>, usize, VariableType),
	Suffix(Rc<Expression>, usize, VariableType),
	Infix(Rc<Expression>, Rc<Expression>, usize, VariableType),
	Ternary(Rc<Expression>, Rc<Expression>, Rc<Expression>, usize, VariableType),
	Expressions(Rc<Vec<Rc<Expression>>>, VariableType),
	FunctionCall(Rc<Vec<Rc<Expression>>>, VariableType),
	ArrayAccess(Rc<Vec<Rc<Expression>>>, VariableType)
}

impl Expression {
	pub fn get_type(&self) -> VariableType {
		if let Expression::Invalid = self {
			return VariableType::inferred();
		} else {
			return match self {
				Expression::Value(_, v) => v,
				Expression::Prefix(_, _, v) => v,
				Expression::Suffix(_, _, v) => v,
				Expression::Infix(_, _, _, v) => v,
				Expression::Ternary(_, _, _, _, v) => v,
				Expression::Expressions(_, v) => v,
				Expression::FunctionCall(_, v) => v,
				Expression::ArrayAccess(_, v) => v,
				Expression::Invalid => panic!("Invalid!")
			}.clone();
		}
	}

	pub fn to_string(&self, operators: &OperatorDataStructure) -> String {
		return match self {
			Expression::Invalid => {
				"Invalid".to_string()
			},
			Expression::Value(s, _) => {
				s.to_string()
			},
			Expression::Prefix(expr, id, _) => {
				String::from(format!("{}{}", operators["prefix"][*id].name.as_ref().unwrap_or(&"".to_string()), expr.to_string(operators)))
			},
			Expression::Suffix(expr, id, _) => {
				String::from(format!("{}{}", expr.to_string(operators), operators["suffix"][*id].name.as_ref().unwrap_or(&"".to_string())))
			},
			Expression::Infix(expr_left, expr_right, id, _) => {
				if *id == 1 {
					String::from(format!("{}::{}", expr_left.to_string(operators), expr_right.to_string(operators)))
				} else {
					String::from(format!("{} {} {}", expr_left.to_string(operators), operators["infix"][*id].name.as_ref().unwrap_or(&"".to_string()), expr_right.to_string(operators)))
				}
			},
			Expression::Ternary(..) => {
				String::from("[ternary]")
			},
			Expression::Expressions(..) => {
				String::from("[expressions]")
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
