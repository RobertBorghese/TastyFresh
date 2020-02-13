/**********************************************************
 * --- Expression Parser ---
 *
 * Individual Tasty Fresh expressions are parsed using the 
 * `ExpressionParser` provided in this file.
 **********************************************************/

use crate::value_type::ValueType;

use crate::expression::Expression;

use crate::expression_piece::ExpressionPiece;

use crate::config_management::ConfigData;
use crate::config_management::operator_data::{ Operator, OperatorDataStructure };

use crate::context_management::print_code_error;
use crate::context_management::position::Position;

use std::convert::TryInto;

use num::*;

use std::rc::Rc;

/// Parses an expression represented as a String.
/// The properties are used throughout the parsing process implemented below.
pub struct ExpressionParser {
	expression: Expression,

	position: ExpressionParserPosition,

	expr_str: String,
	expr_chars: Vec<char>,

	parts: Vec<ExpressionPiece>,
	end_data: ExpressionEnder,

	config_data: ConfigData
}

/// Tracks the positional information of the parser.
struct ExpressionParserPosition {
	line_offset: i32,
	line_start: i32,
	index: i32,
	start_position: Position
}

/// Stores important data to be retrieved after the parser ends.
struct ExpressionEnder {
	until_chars: Vec<char>,
	end_index: i32,
	end_char: char
}

/// Represents the different states of the parser.
#[derive(Copy, Clone)]
enum ParseState {
	Prefix,
	Value,
	Suffix,
	Infix,
	End
}

impl Expression {
	pub fn new() -> Expression {
		return Expression::Invalid;
	}
}

impl ExpressionParser {
	pub fn new(expr_str: &str, start_position: Position, config_data: ConfigData, end_chars: Option<Vec<char>>) -> ExpressionParser {
		let mut result = ExpressionParser {
			expr_str: expr_str.to_string(),
			expr_chars: expr_str.chars().collect(),
			position: ExpressionParserPosition {
				line_offset: 0,
				line_start: 0,
				index: 0,
				start_position: start_position
			},
			expression: Expression::Invalid,
			parts: Vec::new(),
			config_data: config_data,
			end_data: ExpressionEnder {
				until_chars: end_chars.unwrap_or(Vec::new()),
				end_index: 0,
				end_char: ' '
			}
		};
		result.parse_expr_str();
		result.parse_expr_parts();
		return result;
	}

	fn get_operator(&self, op_type: &str, index: usize) -> &Operator {
		return &self.config_data.operators[op_type][index];
	}

	fn parse_expr_parts(&mut self) {
		while self.parts.len() > 1 {
			let mut i = 0;
			let mut next_op_index = None;
			let mut next_op_priority = -1;
			for part in &self.parts {
				let mut priority = -2;
				let mut reverse_priority = false;
				match part {
					ExpressionPiece::Prefix(index, _) |
					ExpressionPiece::Suffix(index, _) |
					ExpressionPiece::Infix(index, _) => {
						let op = self.get_operator(match part {
							ExpressionPiece::Prefix(..) => "prefix",
							ExpressionPiece::Suffix(..) => "suffix",
							ExpressionPiece::Infix(..) => "infix",
							_ => ""
						}, *index);
						priority = op.priority;
						reverse_priority = op.reverse_priority;
					},
					ExpressionPiece::FunctionParameters(..) |
					ExpressionPiece::ArrayAccessParameters(..) => {
						next_op_priority = 950;
					},
					ExpressionPiece::TernaryCondition(..) => {},
					_ => ()
				}
				if (priority > next_op_priority) || (priority == next_op_priority && reverse_priority) {
					next_op_index = Some(i);
					next_op_priority = priority;
				}
				i += 1;
			}

			if next_op_index.is_some() && next_op_index.unwrap() < self.parts.len() {
				let op_index = next_op_index.unwrap();
				match &self.parts[op_index] {
					ExpressionPiece::Prefix(index, position) => {
						let result = match &self.parts[op_index + 1] {
							ExpressionPiece::Value(value, position) => {
								Expression::Prefix(Rc::new(Expression::Value(value.clone(), ValueType::Unknown)), *index, ValueType::Unknown)
							},
							ExpressionPiece::Expression(expr) => {
								Expression::Prefix(Rc::clone(expr), *index, ValueType::Unknown)
							},
							ExpressionPiece::EncapsulatedValues(expressions, position) => {
								Expression::Prefix(Rc::new(Expression::Expressions(Rc::clone(expressions), ValueType::Unknown)), *index, ValueType::Unknown)
							},
							_ => Expression::Invalid
						};
						self.parts.insert(op_index, ExpressionPiece::Expression(Rc::new(result)));
						self.parts.remove(op_index + 1);
						self.parts.remove(op_index + 1);
					},
					ExpressionPiece::Suffix(index, position) => {
						let result = match &self.parts[op_index - 1] {
							ExpressionPiece::Value(value, position) => {
								Expression::Suffix(Rc::new(Expression::Value(value.clone(), ValueType::Unknown)), *index, ValueType::Unknown)
							},
							ExpressionPiece::Expression(expr) => {
								Expression::Suffix(Rc::clone(expr), *index, ValueType::Unknown)
							},
							ExpressionPiece::EncapsulatedValues(expressions, position) => {
								Expression::Suffix(Rc::new(Expression::Expressions(Rc::clone(expressions), ValueType::Unknown)), *index, ValueType::Unknown)
							},
							_ => Expression::Invalid
						};
						self.parts.insert(op_index - 1, ExpressionPiece::Expression(Rc::new(result)));
						self.parts.remove(op_index);
						self.parts.remove(op_index);
					},
					ExpressionPiece::Infix(index, position) => {
						let left_result = match &self.parts[op_index - 1] {
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
						let right_result = match &self.parts[op_index + 1] {
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
						if left_result.is_some() && right_result.is_some() {
							self.parts.insert(op_index - 1, ExpressionPiece::Expression(Rc::new(Expression::Infix(left_result.unwrap(), right_result.unwrap(), *index, ValueType::Unknown))));
							for i in 0..3 { self.parts.remove(op_index); }
						}
						
					},
					_ => {
						println!("No support for this expression atm!");
						break;
					}
				}
			} else {
				panic!("Could not parse expression components!");
			}
		}

		match &self.parts[0] {
			ExpressionPiece::Expression(expr) => {
				println!("Expression: {}", expr.to_string(&self.config_data.operators));
			}
			_ => ()
		}
	}

	fn parse_expr_str(&mut self) {
		let parse_end_id = ParseState::End as i32;
		let mut state = ParseState::Prefix;
		let mut stale_index = 0;
		let mut stale_index_count = 0;
		loop {
			if !self.index_within_bounds() {
				self.end_data.end_index = self.position.index;
				break;
			}
			if self.check_for_end_char() {
				break;
			}
			let old_state_id = state.clone() as i32;
			self.parse(&mut state);
			let state_id = state as i32;
			if state_id == parse_end_id {
				break;
			}
			if old_state_id != state_id {
				if stale_index != self.position.index {
					stale_index = self.position.index;
				} else {
					stale_index_count += 1;
				}
			}
			if stale_index_count > 20 {
				panic!(format!("Unable to parse {} at position {}!", self.expr_str, self.position.index));
			}
		}
	}

	fn check_for_end_char(&mut self) -> bool {
		self.position.index += self.parse_next_whitespace();
		let curr_char = self.expr_chars[self.position.index.to_usize().unwrap()];
		if self.end_data.until_chars.contains(&curr_char) {
			self.end_data.end_index = self.position.index;
			self.end_data.end_char = curr_char;
			return true;
		}
		return false;
	}

	fn parse(&mut self, state: &mut ParseState) {
		match state {
			ParseState::Prefix => {
				if !self.parse_next_prefix_operator() {
					*state = ParseState::Value;
				}
			},
			ParseState::Value => {
				if !self.parse_value() {
					panic!(format!("Could not parse value at position {}!", self.position.index));
				} else {
					*state = ParseState::Suffix;
				}
			},
			ParseState::Suffix => {
				if !self.parse_next_suffix_operator() {
					*state = ParseState::Infix;
				}
			},
			ParseState::Infix => {
				if !self.parse_next_infix_operator() {
					*state = ParseState::Prefix;
				}
			}
			ParseState::End => {}
		}
	}

	fn generate_pos(&self, start: i32, end: i32) -> Position {
		let pos_start = {
			if self.position.line_offset == 0 {
				start.to_usize().unwrap() + self.position.start_position.start
			} else {
				(start - self.position.line_start).to_usize().unwrap()
			}
		};
		let pos_end = {
			if end == -1 {
				None
			} else if self.position.line_offset == 0 {
				Some(end.to_usize().unwrap() + self.position.start_position.start)
			} else {
				Some((end - self.position.line_start).to_usize().unwrap())
			}
		};
		return Position::new(
			self.position.start_position.file.clone(),
			self.position.start_position.line + self.position.line_offset.to_usize().unwrap(),
			pos_start.to_usize().unwrap(),
			pos_end
		);
	}

	fn add_prefix_op(&mut self, op: usize, start: i32, end: i32) {
		self.parts.push(ExpressionPiece::Prefix(op, self.generate_pos(start, end)));
	}

	fn add_value(&mut self, val: String, start: i32, end: i32) {
		self.parts.push(ExpressionPiece::Value(val, self.generate_pos(start, end)));
	}

	fn add_suffix_op(&mut self, op: usize, start: i32, end: i32) {
		self.parts.push(ExpressionPiece::Suffix(op, self.generate_pos(start, end)));
	}

	fn add_infix_op(&mut self, op: usize, start: i32, end: i32) {
		self.parts.push(ExpressionPiece::Infix(op, self.generate_pos(start, end)));
	}

	fn add_encapsulated_values(&mut self, expressions: Vec<Expression>, start: i32, end: i32) {
		self.parts.push(ExpressionPiece::EncapsulatedValues(Rc::new(expressions), self.generate_pos(start, end)));
	}

	fn add_function_params(&mut self, expressions: Vec<Expression>, start: i32, end: i32) {
		self.parts.push(ExpressionPiece::FunctionParameters(Rc::new(expressions), self.generate_pos(start, end)));
	}

	fn add_array_access_params(&mut self, expressions: Vec<Expression>, start: i32, end: i32) {
		self.parts.push(ExpressionPiece::ArrayAccessParameters(Rc::new(expressions), self.generate_pos(start, end)));
	}

	fn add_ternary_internals(&mut self, expressions: Vec<Expression>, start: i32, end: i32) {
		self.parts.push(ExpressionPiece::TernaryCondition(Rc::new(expressions), self.generate_pos(start, end)));
	}
/*
	fn add_expressions(&mut self, op: String, expr: Vec<Expression>) {
		//self.expression.components.push(ExpressionComponents::Test2(op, expr));
	}
*/

	fn str_len(&self) -> i32 {
		return self.expr_str.len().try_into().unwrap();
	}

	fn index_within_bounds(&self) -> bool {
		if self.position.index >= self.str_len() {
			return false;
		}
		return true;
	}

	fn parse_next_whitespace(&mut self) -> i32 {
		let mut space_offset = 0;
		let mut next_char = self.expr_chars[(self.position.index + space_offset).to_usize().unwrap()];
		while next_char == ' ' || next_char == '\t' || next_char == '\n' {
			space_offset += 1;
			if next_char == '\n' {
				self.position.line_offset += 1;
			}
			next_char = self.expr_chars[(self.position.index + space_offset).to_usize().unwrap()];
		}
		return space_offset;
	}

	fn parse_value(&mut self) -> bool {
		if !self.index_within_bounds() { return false; }
		let space_offset = self.parse_next_whitespace();
		let value_start = self.position.index + space_offset;
		let mut offset = 0;
		let first_char = self.expr_chars[(value_start).to_usize().unwrap()];
		if first_char == '(' {
			self.position.index += space_offset + 1;
			return self.parse_internal_expressions(')', true);
		} else {
			loop {
				let mut finish = false;
				if value_start + offset >= self.str_len() {
					finish = true;
				} else {
					let cc = self.expr_chars[(value_start + offset).to_usize().unwrap()];
					if !cc.is_alphanumeric() {
						finish = true;
					}
				}
				if finish {
					let substr = self.gen_substr(value_start, value_start + offset).to_string();
					if substr.is_empty() {
						break;
					}
					self.add_value(substr, value_start, value_start + offset);
					self.position.index += offset;
					return true;
				}
				offset += 1;
			}
		}
		return false;
	}

	fn gen_substr(&self, left: i32, right: i32) -> String {
		return self.expr_str.as_str()[(left.to_usize().unwrap()..right.to_usize().unwrap())].to_string();
	}

	fn parse_next_operator(&mut self, op_type: &str) -> bool {
		if !self.index_within_bounds() { return false; }
		let space_offset = self.parse_next_whitespace();
		let operator_start = self.position.index + space_offset;
		let mut offset = 0;
		//let mut possible_operators: Vec<String>;
		let mut possible_operators: Vec<usize>;
		loop {
			let substr = self.gen_substr(operator_start, operator_start + offset);
			possible_operators = self.check_operator(substr.as_str(), op_type, false);
			if possible_operators.is_empty() {
				if offset > 0 {
					let prev_substr = self.gen_substr(operator_start, operator_start + offset - 1);
					let exact_operators = self.check_operator(prev_substr.as_str(), op_type, true);
					if exact_operators.len() == 1 {
						possible_operators = exact_operators;
						offset -= 1;
					}
				}
				break; 
			} else if possible_operators.len() == 1 {
				let op = possible_operators.first().unwrap();
				if substr == *self.get_operator(op_type, *op).name.as_ref().unwrap() {
					break;
				}
			}
			offset += 1;
			if operator_start + offset > self.str_len() {
				return false;
			}
		}
		if possible_operators.len() > 0 {
			let op = possible_operators.remove(0);
			match op_type {
				"prefix" => self.add_prefix_op(op, operator_start, operator_start + offset),
				"suffix" => self.add_suffix_op(op, operator_start, operator_start + offset),
				"infix" => self.add_infix_op(op, operator_start, operator_start + offset),
				_ => {}
			}
			self.position.index += offset + space_offset;
			return true;
		}
		return false;
	}

	fn parse_next_prefix_operator(&mut self) -> bool {
		return self.parse_next_operator("prefix");
	}

	fn parse_next_infix_operator(&mut self) -> bool {
		return self.parse_next_operator("infix");
	}

	fn is_group_char(&self, c: char) -> bool {
		return c == '(' || c == '[';
	}

	fn get_end_char(&self, c: char) -> char {
		return match c {
			'(' => ')',
			'[' => ']',
			_ => ' '
		}
	}

	fn parse_next_suffix_operator(&mut self) -> bool {
		let space_offset = self.parse_next_whitespace();
		let start_char = self.expr_chars[(self.position.index + space_offset).to_usize().unwrap()];
		if self.is_group_char(start_char) {
			self.position.index += space_offset + 1;
			return self.parse_suffix_internal_expressions(start_char);
		}
		return self.parse_next_operator("suffix");
	}

	fn parse_suffix_internal_expressions(&mut self, start_char: char) -> bool {
		let end_char = self.get_end_char(start_char);
		let full_operator = format!("{}{}", start_char, end_char);
		let space_offset = self.parse_next_whitespace();
		let real_second_char = self.expr_chars[(self.position.index + space_offset).to_usize().unwrap()];
		if real_second_char == end_char {
			let empty = Vec::new();
			match end_char {
				')' => self.add_function_params(empty, self.position.index, self.position.index + space_offset),
				']' => self.add_array_access_params(empty, self.position.index, self.position.index + space_offset),
				_ => {}
			}
			self.position.index += space_offset + 1;
		} else {
			return self.parse_internal_expressions(end_char, false);
		}
		return false;
	}

	fn parse_internal_expressions(&mut self, end_char: char, is_value: bool) -> bool {
		let mut expressions = Vec::new();
		let start_pos = self.position.index;
		loop {
			let chars = vec!(end_char, ',');
			let config_data = std::mem::replace(&mut self.config_data, ConfigData::new());
			let expr = &self.expr_str.as_str()[self.position.index.to_usize().unwrap()..];
			let parser = ExpressionParser::new(expr, self.generate_pos(self.position.index, -1), config_data, Some(chars));
			self.config_data = parser.config_data;
			expressions.push(parser.expression);
			self.position.index += parser.end_data.end_index + 1;
			if parser.end_data.end_char == end_char {
				break;
			}
		}
		match end_char {
			')' => {
				if is_value {
					self.add_encapsulated_values(expressions, start_pos, self.position.index);
				} else {
					self.add_function_params(expressions, start_pos, self.position.index);
				}
			},
			']' => self.add_array_access_params(expressions, start_pos, self.position.index),
			_ => {}
		}
		return true;
	}

	fn check_operator(&self, op_str: &str, op_type: &str, exact: bool) -> Vec<usize> {
		let mut result = Vec::new();
		let prefixes = &self.config_data.operators[op_type];
		for i in 0..prefixes.len() {
			let op = &prefixes[i];
			if op.name.is_some() {
				let v = op.name.as_ref().unwrap();
				if (exact && v == op_str) || (!exact && v.starts_with(op_str)) {
					result.push(i);
				}
			}
		}
		return result;
	}
}