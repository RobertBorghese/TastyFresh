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

use crate::expression::value_type::ValueType;

use crate::config_management::operator_data::OperatorDataStructure;

use std::rc::Rc;

/// Stores the expression and its components recursively. 
/// The `i32` represents the operators' index in the JSON data.
pub enum Expression {
	Invalid,
	Value(String, ValueType),
	Prefix(Rc<Expression>, usize, ValueType),
	Suffix(Rc<Expression>, usize, ValueType),
	Infix(Rc<Expression>, Rc<Expression>, usize, ValueType),
	Ternary(Rc<Expression>, Rc<Expression>, Rc<Expression>, usize, ValueType),
	Expressions(Rc<Vec<Rc<Expression>>>, ValueType),
	FunctionCall(Rc<Vec<Rc<Expression>>>, ValueType),
	ArrayAccess(Rc<Vec<Rc<Expression>>>, ValueType)
}

impl Expression {
	pub fn to_string(&self, operators: &OperatorDataStructure) -> String {
		match self {
			Expression::Invalid => {
				return "Invalid".to_string();
			}
			Expression::Value(s, _) => {
				return s.to_string();
			}
			Expression::Prefix(expr, id, _) => {
				return String::from(format!("{}({})", operators["prefix"][*id].name.as_ref().unwrap_or(&"".to_string()), expr.to_string(operators)));
			}
			Expression::Suffix(expr, id, _) => {
				return String::from(format!("({}){}", expr.to_string(operators), operators["suffix"][*id].name.as_ref().unwrap_or(&"".to_string())));
			}
			Expression::Infix(expr_left, expr_right, id, _) => {
				return String::from(format!("({}) {} ({})", expr_left.to_string(operators), operators["infix"][*id].name.as_ref().unwrap_or(&"".to_string()), expr_right.to_string(operators)));
			}
			Expression::Ternary(..) => {
				return String::from("[ternary]");
			}
			Expression::Expressions(..) => {
				return String::from("[expressions]");
			}
			Expression::FunctionCall(..) => {
				return String::from("[functioncall]");
			}
			Expression::ArrayAccess(..) => {
				return String::from("[arrayaccess]");
			}
		}
	}
}
