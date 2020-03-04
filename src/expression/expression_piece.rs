/**********************************************************
 * --- Expression Part ---
 *
 * Used to store and represent an expression component
 * within a linear list prior to being parsed into an AST.
 **********************************************************/

use crate::declaration_parser::parser::Parser;

use crate::expression::Expression;
use crate::expression::expression_parser::ExpressionParser;
use crate::expression::value_type::{ NumberType, StringType };
use crate::expression::variable_type::{ VariableType, Type, VarStyle };

use crate::context_management::print_code_error;
use crate::context_management::position::Position;
use crate::context_management::context::Context;
use crate::context_management::typing_context::ContextType;

use std::rc::Rc;

pub enum ExpressionPiece {
	Expression(Rc<Expression>),
	Prefix(usize, Position),
	Value(String, Position),
	Suffix(usize, Position),
	Infix(usize, Position),
	Ternary(usize, Rc<Expression>, Position),
	EncapsulatedValues(Rc<Vec<Rc<Expression>>>, Position),
	InitializerList(Rc<Vec<Rc<Expression>>>, Position),
	FunctionParameters(Rc<Vec<Rc<Expression>>>, Position),
	ArrayAccessParameters(Rc<Vec<Rc<Expression>>>, Position)
}

impl ExpressionPiece {
	pub fn get_encapsulated_type(&self) -> Option<VariableType> {
		return match self {
			ExpressionPiece::EncapsulatedValues(exprs, _) => {
				if exprs.len() > 0 {
					let mut result = Vec::new();
					for e in exprs.iter() {
						result.push(((**e)).get_type());
					}
					Some(VariableType::tuple(result))
				} else {
					None
				}
			},
			ExpressionPiece::InitializerList(exprs, _) => {
				if exprs.len() > 0 {
					let mut curr: Option<VariableType> = None;
					for e in exprs.iter() {
						if curr.is_none() {
							curr = Some(e.get_type().clone());
						} else {
							if *curr.as_ref().unwrap() != e.get_type() {
								curr = None;
								break;
							}
						}
					}
					if curr.is_none() {
						None
					} else {
						Some(VariableType::initializer_list(curr.unwrap().clone()))
					}
				} else {
					None
				}
			}
			ExpressionPiece::Expression(expr) => {
				Some(expr.get_type())
			},
			_ => None
		};
	}

	pub fn parse_expr_parts(parser: &mut ExpressionParser, context: &mut Option<&mut Context>, file_content: &str) -> Rc<Expression> {
		let mut error = false;
		if parser.parts.len() == 1 {
			match Self::get_expression_from_piece(&parser.parts[0], context) {
				Some(expr) => parser.parts[0] = ExpressionPiece::Expression(expr),
				None => return Rc::new(Expression::Invalid) // TODO: error
			}
		}
		while parser.parts.len() > 1 {
			let next_op_index = Self::get_next_operator(parser);
			if next_op_index.is_some() && next_op_index.unwrap() < parser.parts.len() {
				let part_index = next_op_index.unwrap();
				match parser.parts.remove(part_index) {
					ExpressionPiece::Prefix(index, position) => {
						let expr_and_pos = Self::parse_prefix(parser, &part_index, index, context, position);
						if expr_and_pos.0.is_some() {
							parser.parts.insert(part_index, expr_and_pos.0.unwrap());
							for i in 0..1 { parser.parts.remove(part_index + 1); }
						} else {
							let pos = expr_and_pos.1.unwrap();
							print_code_error("Expected Expression", "expected expression after this operator", &pos, file_content);
							error = true;
							break;
						}
					},
					ExpressionPiece::Suffix(index, position) => {
						let expr_and_pos = Self::parse_suffix(parser, &part_index, index, context, position);
						if expr_and_pos.0.is_some() {
							parser.parts.insert(part_index - 1, expr_and_pos.0.unwrap());
							for i in 0..1 { parser.parts.remove(part_index); }
						} else {
							let pos = expr_and_pos.1.unwrap();
							print_code_error("Expected Expression", "expected expression before this operator", &pos, file_content);
							error = true;
							break;
						}
					},
					ExpressionPiece::Infix(index, position) => {
						let expr_and_pos = Self::parse_infix(parser, &part_index, index, context, position);
						if expr_and_pos.0.is_some() {
							parser.parts.insert(part_index - 1, expr_and_pos.0.unwrap());
							for i in 0..2 { parser.parts.remove(part_index); }
						} else {
							let pos = expr_and_pos.1.unwrap();
							print_code_error("Expected Expression", "expected expressions to surrond this operator", &pos, file_content);
							error = true;
							break;
						}
					},
					ExpressionPiece::Ternary(index, expr, position) => {
						let expr_and_pos = Self::parse_ternary(parser, &part_index, expr, index, context, position);
						if expr_and_pos.0.is_some() {
							parser.parts.insert(part_index - 1, expr_and_pos.0.unwrap());
							for i in 0..2 { parser.parts.remove(part_index); }
						} else {
							let pos = expr_and_pos.1.unwrap();
							let err_type = expr_and_pos.2.unwrap();
							if err_type == 1 {
								print_code_error("Must Share Type", "ternary expressions must share same type", &pos, file_content);
							} else if err_type == 2 {
								print_code_error("Expected Expression", "expected expressions to surrond this operator", &pos, file_content);
							}
							error = true;
							break;
						}
					},
					ExpressionPiece::FunctionParameters(exprs, position) => {
						let expr_and_pos = Self::parse_function_call(parser, &part_index, exprs, context, position, file_content);
						if expr_and_pos.0.is_some() {
							parser.parts.insert(part_index - 1, expr_and_pos.0.unwrap());
							for i in 0..1 { parser.parts.remove(part_index); }
						} else {
							let pos = expr_and_pos.1.unwrap();
							print_code_error("Expected Expression", "expected expression before function call", &pos, file_content);
							error = true;
							break;
						}
					},
					ExpressionPiece::ArrayAccessParameters(exprs, position) => {
						let expr_and_pos = Self::parse_array_access(parser, &part_index, exprs, context, position);
						if expr_and_pos.0.is_some() {
							parser.parts.insert(part_index - 1, expr_and_pos.0.unwrap());
							for i in 0..1 { parser.parts.remove(part_index); }
						} else {
							let pos = expr_and_pos.1.unwrap();
							print_code_error("Expected Expression", "expected expression before array access", &pos, file_content);
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
						if context.is_some() {
							let c = context.as_mut().unwrap();
							//println!("Expression: {}", expr.to_string(&parser.config_data.operators, c));
						}
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

	fn parse_prefix(parser: &ExpressionParser, part_index: &usize, operator_id: usize, context: &Option<&mut Context>, position: Position) -> (Option<ExpressionPiece>,Option<Position>) {
		let result = Self::get_expression_from_piece(&parser.parts[*part_index], context);
		if result.is_some() {
			return (Some(ExpressionPiece::Expression(Rc::new(Expression::Prefix(result.unwrap(), operator_id, VariableType::inferred(), position)))), None);
		}
		return (None, Some(position));
	}

	fn parse_suffix(parser: &ExpressionParser, part_index: &usize, operator_id: usize, context: &Option<&mut Context>, position: Position) -> (Option<ExpressionPiece>,Option<Position>) {
		let result = Self::get_expression_from_piece(&parser.parts[part_index - 1], context);
		if result.is_some() {
			return (Some(ExpressionPiece::Expression(Rc::new(Expression::Suffix(result.unwrap(), operator_id, VariableType::inferred(), position)))), None);
		}
		return (None, Some(position));
	}

	fn parse_infix(parser: &ExpressionParser, part_index: &usize, operator_id: usize, context: &Option<&mut Context>, position: Position) -> (Option<ExpressionPiece>,Option<Position>) {
		let left_result = Self::get_expression_from_piece(&parser.parts[part_index - 1], context);
		let mut final_type = VariableType::inferred();
		if left_result.is_some() && operator_id <= 4 {
			let left_type = left_result.as_ref().unwrap().get_type();
			let access_name = Self::get_access_from_piece(&left_type, &parser.parts[*part_index], context);
			if access_name.is_some() {
				let temp_type = left_type.check_accessor_content(&access_name.unwrap(), context);
				if temp_type.is_some() {
					final_type = temp_type.unwrap();
				}
			}
		}
		let right_result = Self::get_expression_from_piece(&parser.parts[*part_index], context);
		if left_result.is_some() && right_result.is_some() {
			return (Some(ExpressionPiece::Expression(Rc::new(Expression::Infix(left_result.unwrap(), right_result.unwrap(), operator_id, final_type, position)))), None);
		}
		return (None, Some(position));
	}

	fn parse_ternary(parser: &ExpressionParser, part_index: &usize, expr: Rc<Expression>, operator_id: usize, context: &Option<&mut Context>, position: Position) -> (Option<ExpressionPiece>,Option<Position>,Option<usize>) {
		let left_result = Self::get_expression_from_piece(&parser.parts[part_index - 1], context);
		let right_result = Self::get_expression_from_piece(&parser.parts[*part_index], context);
		if left_result.is_some() && right_result.is_some() {
			let left_type = expr.get_type();
			let right_type = right_result.as_ref().unwrap().get_type();
			let result_type = left_type.compare_types(&right_type);
			if result_type.is_some() {
				return (Some(ExpressionPiece::Expression(Rc::new(Expression::Ternary(left_result.unwrap(), expr, right_result.unwrap(), operator_id, result_type.unwrap())))), None, None);
			} else {
				return (None, Some(position), Some(1));
			}
		}
		return (None, Some(position), Some(2));
	}

	fn parse_function_call(parser: &ExpressionParser, part_index: &usize, exprs: Rc<Vec<Rc<Expression>>>, context: &Option<&mut Context>, position: Position, file_content: &str) -> (Option<ExpressionPiece>,Option<Position>) {
		let result = Self::get_expression_from_piece(&parser.parts[part_index - 1], context);
		if result.is_some() {
			let left_expr = result.unwrap();
			let mut left_type = left_expr.get_type();
			if left_type.is_quantum_function() {
				let left_type_resolved = left_type.resolve_quantum_function(Rc::clone(&exprs));
				if left_type_resolved.is_ok() {
					left_type = left_type_resolved.unwrap();
				} else {
					print_code_error("Function Error", left_type_resolved.err().unwrap(), &position, file_content);
					left_type = VariableType::inferred();
				}
			}
			let final_type = left_type.get_function_call_return().unwrap_or(VariableType::inferred());
			return (Some(ExpressionPiece::Expression(Rc::new(Expression::FunctionCall(left_expr, Rc::clone(&exprs), final_type, position)))), None);
		}
		return (None, Some(position));
	}

	fn parse_array_access(parser: &ExpressionParser, part_index: &usize, exprs: Rc<Vec<Rc<Expression>>>, context: &Option<&mut Context>, position: Position) -> (Option<ExpressionPiece>,Option<Position>) {
		let result = Self::get_expression_from_piece(&parser.parts[part_index - 1], context);
		if result.is_some() {
			return (Some(ExpressionPiece::Expression(Rc::new(Expression::ArrayAccess(result.unwrap(), exprs, VariableType::inferred(), position)))), None);
		}
		return (None, Some(position));
	}

	fn get_expression_from_piece(piece: &ExpressionPiece, context: &Option<&mut Context>) -> Option<Rc<Expression>> {
		return match piece {
			ExpressionPiece::Value(value, position) => {
				Some(Rc::new(Expression::Value(value.clone(), Self::infer_type_from_value_string(&value, context), position.clone())))
			},
			ExpressionPiece::Expression(expr) => {
				Some(Rc::clone(expr))
			},
			ExpressionPiece::EncapsulatedValues(expressions, position) => {
				Some(Rc::new(Expression::Expressions(Rc::clone(expressions), piece.get_encapsulated_type().unwrap_or(VariableType::inferred()), position.clone())))
			},
			ExpressionPiece::InitializerList(expressions, position) => {
				Some(Rc::new(Expression::InitializerList(Rc::clone(expressions), piece.get_encapsulated_type().unwrap_or(VariableType::inferred()), position.clone())))
			},
			_ => None
		};
	}

	fn get_access_from_piece(access_type: &VariableType, piece: &ExpressionPiece, context: &Option<&mut Context>) -> Option<String> {
		return match piece {
			ExpressionPiece::Value(value, position) => Some(value.clone()),
			_ => None
		};
	}

	fn infer_type_from_value_string(value: &str, context: &Option<&mut Context>) -> VariableType {
		if value.is_empty() {
			return VariableType::inferred();
		}
		let first = value.chars().nth(0).unwrap();
		if first.is_ascii_digit() {
			return VariableType::copy(Self::infer_number_type(value));
		} else if Self::check_if_string(value) {
			return VariableType::copy(Type::String(StringType::ConstCharArray));
		} else if value == "true" || value == "false" {
			return VariableType::boolean();
		} else if context.is_some() {
			let c = context.as_ref().unwrap();
			let ct = c.typing.get_item(value);
			if ct.is_some() {
				return match ct.unwrap() {
					ContextType::Variable(variable_type) => variable_type,
					ContextType::Function(function) => VariableType::function(function),
					ContextType::QuantumFunction(functions) => VariableType::quantum_function(functions),
					ContextType::Class(class_type) => VariableType::class(class_type),
					ContextType::Namespace(content) => VariableType::namespace(),
					_ => VariableType::inferred()
				}
			}
		}
		return VariableType::inferred();
	}

	fn infer_number_type(value: &str) -> Type {
		return Type::Number(NumberType::from_value_text(value));
	}

	fn check_if_string(value: &str) -> bool {
		let mut parser = Parser::new(value.to_string());
		return parser.check_for_string();
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
			ExpressionPiece::Infix(index, _) |
			ExpressionPiece::Ternary(index, _, _) => {
				let op = parser.get_operator(match piece {
					ExpressionPiece::Prefix(..) => "prefix",
					ExpressionPiece::Suffix(..) => "suffix",
					ExpressionPiece::Infix(..) => "infix",
					ExpressionPiece::Ternary(..) => "ternary",
					_ => ""
				}, *index);
				*priority = op.priority;
				*reverse_priority = op.reverse_priority;
			},
			ExpressionPiece::FunctionParameters(..) |
			ExpressionPiece::ArrayAccessParameters(..) => {
				*priority = 950;
			},
			_ => ()
		}
	}
}
