/**********************************************************
 * --- Expression Part ---
 *
 * Used to store and represent an expression component
 * within a linear list prior to being parsed into an AST.
 **********************************************************/

use crate::expression::Expression;
use crate::expression::expression_parser::ExpressionParser;
use crate::expression::value_type::ValueType;

use crate::context_management::print_code_error;
use crate::context_management::position::Position;

use std::rc::Rc;

pub enum ExpressionPiece {
	Expression(Rc<Expression>),
	Prefix(usize, Position),
	Value(String, Position),
	Suffix(usize, Position),
	Infix(usize, Position),
	EncapsulatedValues(Rc<Vec<Rc<Expression>>>, Position),
	FunctionParameters(Rc<Vec<Rc<Expression>>>, Position),
	ArrayAccessParameters(Rc<Vec<Rc<Expression>>>, Position),
	TernaryCondition(Rc<Vec<Rc<Expression>>>, Position)
}

impl ExpressionPiece {
	pub fn parse_expr_parts(parser: &mut ExpressionParser) -> Rc<Expression> {
		let mut error = false;
		if parser.parts.len() == 1 {
			match Self::get_expression_from_piece(&parser.parts[0]) {
				Some(expr) => parser.parts[0] = ExpressionPiece::Expression(expr),
				None => return Rc::new(Expression::Invalid) // TODO: error
			}
		}
		while parser.parts.len() > 1 {
			let next_op_index = Self::get_next_operator(parser);
			if next_op_index.is_some() && next_op_index.unwrap() < parser.parts.len() {
				let part_index = next_op_index.unwrap();
				match &parser.parts[part_index] {
					ExpressionPiece::Prefix(index, position) => {
						let expr = Self::parse_prefix(parser, &part_index, index);
						if expr.is_some() {
							parser.parts.insert(part_index, expr.unwrap());
							for i in 0..2 { parser.parts.remove(part_index + 1); }
						} else {
							println!("Expected expression at {}:{}:{}", position.file, position.line.unwrap_or(1), position.end.unwrap_or(0));
							error = true;
							break;
						}
					},
					ExpressionPiece::Suffix(index, position) => {
						let expr = Self::parse_suffix(parser, &part_index, index);
						if expr.is_some() {
							parser.parts.insert(part_index - 1, expr.unwrap());
							for i in 0..2 { parser.parts.remove(part_index); }
						} else {
							println!("Expected expression at {}:{}:{}", position.file, position.line.unwrap_or(1), position.start);
							error = true;
							break;
						}
					},
					ExpressionPiece::Infix(index, position) => {
						let expr = Self::parse_infix(parser, &part_index, index);
						if expr.is_some() {
							parser.parts.insert(part_index - 1, expr.unwrap());
							for i in 0..3 { parser.parts.remove(part_index); }
						} else {
							print_code_error("Expected Expression", "expected expressions to surrond this operator", position, None);
							error = true;
							break;
						}
					},
					_ => {
						println!("No support for this expression atm!");
						error = true;
						break;
					}
				}
			} else {
				panic!("Could not parse expression components!");
			}
		}

		if !error {
			if parser.parts.len() > 0 {
				match parser.parts.remove(0) {
					ExpressionPiece::Expression(expr) => {
						println!("Expression: {}", expr.to_string(&parser.config_data.operators));
						return expr;
					}
					_ => ()
				}
			} else {
				println!("COULD NOT PRINT EXPR!! ");
			}
		}
		return Rc::new(Expression::Invalid);
	}

	fn parse_prefix(parser: &ExpressionParser, part_index: &usize, operator_id: &usize) -> Option<ExpressionPiece> {
		let result = Self::get_expression_from_piece(&parser.parts[part_index + 1]);
		if result.is_some() {
			return Some(ExpressionPiece::Expression(Rc::new(Expression::Prefix(result.unwrap(), *operator_id, ValueType::Unknown))));
		}
		return None;
	}

	fn parse_suffix(parser: &ExpressionParser, part_index: &usize, operator_id: &usize) -> Option<ExpressionPiece> {
		let result = Self::get_expression_from_piece(&parser.parts[part_index - 1]);
		if result.is_some() {
			return Some(ExpressionPiece::Expression(Rc::new(Expression::Suffix(result.unwrap(), *operator_id, ValueType::Unknown))));
		}
		return None;
	}

	fn parse_infix(parser: &ExpressionParser, part_index: &usize, operator_id: &usize) -> Option<ExpressionPiece> {
		let left_result = Self::get_expression_from_piece(&parser.parts[part_index - 1]);
		let right_result = Self::get_expression_from_piece(&parser.parts[part_index + 1]);
		if left_result.is_some() && right_result.is_some() {
			return Some(ExpressionPiece::Expression(Rc::new(Expression::Infix(left_result.unwrap(), right_result.unwrap(), *operator_id, ValueType::Unknown))));
		}
		return None;
	}

	fn get_expression_from_piece(piece: &ExpressionPiece) -> Option<Rc<Expression>> {
		return match piece {
			ExpressionPiece::Value(value, position) => {
				Some(Rc::new(Expression::Value(value.clone(), ValueType::Unknown)))
			},
			ExpressionPiece::Expression(expr) => {
				Some(Rc::clone(expr))
			},
			ExpressionPiece::EncapsulatedValues(expressions, position) => {
				Some(Rc::new(Expression::Expressions(Rc::clone(expressions), ValueType::Unknown)))
			},
			_ => None
		};
	}

	fn get_next_operator(parser: &mut ExpressionParser) -> Option<usize> {
		let mut next_op_index = None;
		let mut next_op_priority = -1;
		for i in 0..parser.parts.len() {
			let mut priority = -2;
			let mut reverse_priority = false;
			Self::get_piece_priority(parser, i, &mut priority, &mut reverse_priority);
			if (priority > next_op_priority) || (priority == next_op_priority && reverse_priority) {
				next_op_index = Some(i);
				next_op_priority = priority;
			}
		}
		return next_op_index;
	}

	fn get_piece_priority(parser: &mut ExpressionParser, index: usize, priority: &mut i64, reverse_priority: &mut bool) {
		let piece = &parser.parts[index];
		match piece {
			ExpressionPiece::Prefix(index, _) |
			ExpressionPiece::Suffix(index, _) |
			ExpressionPiece::Infix(index, _) => {
				let op = parser.get_operator(match piece {
					ExpressionPiece::Prefix(..) => "prefix",
					ExpressionPiece::Suffix(..) => "suffix",
					ExpressionPiece::Infix(..) => "infix",
					_ => ""
				}, *index);
				*priority = op.priority;
				*reverse_priority = op.reverse_priority;
			},
			ExpressionPiece::FunctionParameters(..) |
			ExpressionPiece::ArrayAccessParameters(..) => {
				*priority = 950;
			},
			ExpressionPiece::TernaryCondition(..) => {},
			_ => ()
		}
	}
}
