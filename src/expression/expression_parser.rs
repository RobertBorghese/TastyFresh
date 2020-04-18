/**********************************************************
 * --- Expression Parser ---
 *
 * Individual Tasty Fresh expressions are parsed using the 
 * `ExpressionParser` provided in this file.
 **********************************************************/

use crate::expression::Expression;
use crate::expression::expression_piece::ExpressionPiece;
use crate::expression::variable_type::{ VariableType, Type };

use crate::config_management::ConfigData;
use crate::config_management::operator_data::{ Operator, OperatorDataStructure };

use crate::context_management::print_code_error;
use crate::context_management::position::Position;

use crate::declaration_parser::parser::Parser;

use crate::context_management::context::Context;

use std::convert::TryInto;

use num::*;

use std::rc::Rc;

/// Parses an expression represented as a String.
/// The properties are used throughout the parsing process implemented below.
pub struct ExpressionParser<'a> {
	pub expression: Rc<Expression>,

	pub position: ExpressionParserPosition,

	pub expr_str: String,

	pub parts: Vec<ExpressionPiece>,
	pub end_data: ExpressionEnd,
	pub expect_type: bool,

	pub config_data: &'a ConfigData
}

/// Tracks the positional information of the parser.
pub struct ExpressionParserPosition {
	pub line_offset: usize,
	pub line_start: usize,
	pub index: usize,
	pub start_position: Position
}

/// Stores important data to be retrieved after the parser ends.
pub struct ExpressionEnd {
	pub until_chars: Vec<char>,
	pub end_index: usize,
	pub reason: ExpressionEndReason
}

/// Stores important data to be retrieved after the parser ends.
#[derive(PartialEq)]
pub enum ExpressionEndReason {
	Unknown,
	ReachedChar(char),
	EndOfContent,
	EndOfExpression,
	NoValueError
}

impl ExpressionEndReason {
	pub fn to_string(&self) -> String {
		return match self {
			ExpressionEndReason::Unknown => "Unknown Reason".to_string(),
			ExpressionEndReason::ReachedChar(c) => format!("Reached Char: {}", c),
			ExpressionEndReason::EndOfContent => "End of Content".to_string(),
			ExpressionEndReason::EndOfExpression => "End of Expression".to_string(),
			ExpressionEndReason::NoValueError => "No Value Error".to_string(),
		}
	}
}

/// Represents the different states of the parser.
#[derive(Copy, Clone, PartialEq)]
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

impl<'a> ExpressionParser<'a> {
	pub fn new(parser: &mut Parser, start_position: Position, config_data: &'a ConfigData, context: &mut Option<&mut Context>, end_chars: Option<Vec<char>>, final_desired_type: Option<VariableType>) -> ExpressionParser<'a> {
		let mut result = ExpressionParser {
			expr_str: parser.content.to_string(),
			position: ExpressionParserPosition {
				line_offset: 0,
				line_start: 0,
				index: start_position.start,
				start_position: start_position
			},
			expression: Rc::new(Expression::Invalid),
			parts: Vec::new(),
			config_data: config_data,
			end_data: ExpressionEnd {
				until_chars: end_chars.unwrap_or(Vec::new()),
				end_index: 0,
				reason: ExpressionEndReason::Unknown
			},
			expect_type: false
		};
		result.parse_expr_str(parser, context);
		result.expression = ExpressionPiece::parse_expr_parts(&mut result, context, &parser.content, final_desired_type);
		return result;
	}

	pub fn get_operator(&self, op_type: &str, index: usize) -> &Operator {
		return &self.config_data.operators[op_type][index];
	}

	fn parse_expr_str(&mut self, parser: &mut Parser, context: &mut Option<&mut Context>) {
		let mut state = ParseState::Prefix;
		loop {
			if !self.index_within_bounds(parser) {
				break;
			}
			if self.check_for_end_char(parser) {
				break;
			}
			self.parse(&mut state, parser, context);
			if state == ParseState::End {
				break;
			}
		}
	}

	fn check_for_end_char(&mut self, parser: &mut Parser) -> bool {
		parser.index += self.parse_next_whitespace(parser);
		let curr_char = parser.get_curr();
		if self.end_data.until_chars.contains(&curr_char) {
			self.end_data.end_index = parser.index;
			self.set_end_reason(ExpressionEndReason::ReachedChar(curr_char));
			return true;
		}
		return false;
	}

	fn set_end_reason(&mut self, reason: ExpressionEndReason) -> bool {
		if self.end_data.reason == ExpressionEndReason::Unknown {
			self.end_data.reason = reason;
			return true;
		}
		return false;
	}

	fn parse(&mut self, state: &mut ParseState, parser: &mut Parser, context: &mut Option<&mut Context>) {
		match state {
			ParseState::Prefix => {
				if !self.parse_next_prefix_operator(parser) {
					*state = ParseState::Value;
				}
			},
			ParseState::Value => {
				if self.expect_type {
					self.expect_type = false;
					let start_index = parser.index;
					let mut include_style = false;
					if parser.get_curr() == '(' {
						include_style = true;
						parser.increment();
					}

					let mut unexpected_char = false;
					let mut specifier_error: Option<&'static str> = None;
					let tf_type = if include_style {
						VariableType::from_type_style(parser.parse_type_and_style(&mut unexpected_char, &mut specifier_error))
					} else {
						VariableType::of_inferred_style(parser.parse_type(&mut unexpected_char, &mut specifier_error))
					};
					self.add_type(tf_type, start_index, parser.index);

					if include_style {
						parser.parse_whitespace();
						if parser.get_curr() == ')' {
							parser.increment();
						}
					}

					*state = ParseState::Suffix;
				} else if !self.parse_value(parser, context) {
					self.set_end_reason(ExpressionEndReason::NoValueError);
					*state = ParseState::End;
				} else {
					*state = ParseState::Suffix;
				}
			},
			ParseState::Suffix => {
				if !self.parse_next_suffix_operator(parser, context) {
					*state = ParseState::Infix;
				}
			},
			ParseState::Infix => {
				if self.parse_next_infix_operator(parser, context) {
					*state = ParseState::Prefix;
				} else {
					self.set_end_reason(ExpressionEndReason::EndOfExpression);
					*state = ParseState::End;
				}
			}
			ParseState::End => {}
		}
	}

	fn generate_pos(&self, start: usize, end: Option<usize>) -> Position {
		let pos_start = {
			if self.position.line_offset == 0 {
				start + self.position.start_position.start
			} else {
				start - self.position.line_start
			}
		};
		let pos_end = {
			if end.is_none() {
				None
			} else if self.position.line_offset == 0 {
				Some(end.unwrap() + self.position.start_position.start)
			} else {
				Some(end.unwrap() - self.position.line_start)
			}
		};
		return Position::new(
			self.position.start_position.file.clone(),
			Some(self.position.start_position.line.unwrap_or(1) + self.position.line_offset),
			pos_start,
			pos_end
		);
	}

	fn add_prefix_op(&mut self, op: usize, start: usize, end: usize) {
		//println!("Added prefix: {}", op);
		if op == 8 || op == 9 {
			self.expect_type = true;
		}
		self.parts.push(ExpressionPiece::Prefix(op, self.generate_pos(start, Some(end))));
	}

	fn add_value(&mut self, val: String, start: usize, end: usize) {
		//println!("Added value: {}", val);
		self.parts.push(ExpressionPiece::Value(val, self.generate_pos(start, Some(end))));
	}

	fn add_suffix_op(&mut self, op: usize, start: usize, end: usize) {
		//println!("Added suffix: {}", op);
		self.parts.push(ExpressionPiece::Suffix(op, self.generate_pos(start, Some(end))));
	}

	fn add_infix_op(&mut self, op: usize, start: usize, end: usize) {
		//println!("Added infix: {}", op);
		if op >= 6 && op <= 9 {
			self.expect_type = true;
		}
		self.parts.push(ExpressionPiece::Infix(op, self.generate_pos(start, Some(end))));
	}

	fn add_ternary_op(&mut self, expr: Rc<Expression>, op: usize, start: usize, end: usize) {
		//println!("Added ternary: {}", op);
		self.parts.push(ExpressionPiece::Ternary(op, expr, self.generate_pos(start, Some(end))));
	}

	fn add_encapsulated_values(&mut self, expressions: Vec<Rc<Expression>>, start: usize, end: usize) {
		self.parts.push(ExpressionPiece::EncapsulatedValues(Rc::new(expressions), self.generate_pos(start, Some(end))));
	}

	fn add_initilizer_list(&mut self, expressions: Vec<Rc<Expression>>, start: usize, end: usize) {
		self.parts.push(ExpressionPiece::InitializerList(Rc::new(expressions), self.generate_pos(start, Some(end))));
	}

	fn add_function_params(&mut self, expressions: Vec<Rc<Expression>>, start: usize, end: usize) {
		self.parts.push(ExpressionPiece::FunctionParameters(Rc::new(expressions), self.generate_pos(start, Some(end))));
	}

	fn add_array_access_params(&mut self, expressions: Vec<Rc<Expression>>, start: usize, end: usize) {
		self.parts.push(ExpressionPiece::ArrayAccessParameters(Rc::new(expressions), self.generate_pos(start, Some(end))));
	}

	fn add_type(&mut self, tf_type: VariableType, start: usize, end: usize) {
		self.parts.push(ExpressionPiece::Type(tf_type, self.generate_pos(start, Some(end))));
	}

	fn str_len(&self) -> usize {
		return self.expr_str.len();
	}

	fn index_within_bounds(&mut self, parser: &mut Parser) -> bool {
		if parser.index >= self.str_len() {
			self.end_data.end_index = parser.index;
			self.set_end_reason(ExpressionEndReason::EndOfContent);
			return false;
		}
		return true;
	}

	fn parse_next_whitespace(&mut self, parser: &mut Parser) -> usize {
		let mut space_offset = 0;
		if parser.index >= parser.chars.len() {
			return 0;
		}
		let old_index = parser.index;
		let old_line = parser.line;
		parser.parse_whitespace();

		space_offset = parser.index - old_index;
		self.position.line_offset += parser.line - old_line;

		parser.line = old_line;
		parser.index = old_index;
		return space_offset;
	}

	fn parse_value(&mut self, parser: &mut Parser, context: &mut Option<&mut Context>) -> bool {
		if !self.index_within_bounds(parser) { return false; }
		let space_offset = self.parse_next_whitespace(parser);
		let value_start = parser.index + space_offset;
		parser.index = value_start;
		let mut offset = 0;
		let first_char = parser.chars[value_start];
		if first_char == '(' {
			parser.index += 1;
			return self.parse_internal_expressions(')', true, parser, context);
		} else if first_char == '{' {
			parser.index += 1;
			return self.parse_internal_expressions('}', true, parser, context);
		} else {
			let mut finish = false;
			let mut number_offset = 0;
			if parser.check_for_number(&mut number_offset) {
				offset += number_offset - 1;
				finish = true;
			} else if parser.check_for_string() {
				let old_index = parser.index;
				parser.parse_string();
				offset = parser.index - old_index + 1;
				parser.index = old_index;
				finish = true;
			} else {
				loop {
					if value_start + offset >= self.str_len() {
						finish = true;
					} else {
						let cc = parser.chars[value_start + offset];
						if !cc.is_alphanumeric() && cc != '_' {
							finish = true;
						}
					}
					if finish {
						break;
					}
					offset += 1;
				}
			}
			if finish {
				let substr = self.gen_substr(value_start, value_start + offset).to_string();
				if !substr.is_empty() {
					self.add_value(substr, value_start, value_start + offset);
					parser.index += offset;
					return true;
				}
			}
		}
		return false;
	}

	fn gen_substr(&self, left: usize, right: usize) -> String {
		return self.expr_str.as_str()[(left..right)].to_string();
	}

	fn parse_next_operator(&mut self, op_type: &str, parser: &mut Parser) -> bool {
		if !self.index_within_bounds(parser) { return false; }
		let space_offset = self.parse_next_whitespace(parser);
		let operator_start = parser.index + space_offset;
		let mut offset = 0;
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
			parser.index += offset + space_offset;
			return true;
		}
		return false;
	}

	fn parse_next_prefix_operator(&mut self, parser: &mut Parser) -> bool {
		return self.parse_next_operator("prefix", parser);
	}

	fn parse_next_infix_operator(&mut self, parser: &mut Parser, context: &mut Option<&mut Context>) -> bool {
		if self.parse_next_operator("infix", parser) {
			return true;
		}
		return self.parse_next_ternary(parser, context);
	}

	fn parse_next_ternary(&mut self, parser: &mut Parser, context: &mut Option<&mut Context>) -> bool {
		parser.parse_whitespace();
		let original_index = parser.index;
		if parser.get_curr() == '?' {
			let end_char = ':';
			parser.index += 1;
			let start_pos = parser.index;
			let chars = vec!(end_char);
			if self.index_within_bounds(parser) {
				let expr_parser = ExpressionParser::new(parser, self.generate_pos(parser.index, None), self.config_data, context, Some(chars), None);
				let expr = expr_parser.expression;
				parser.index += 1;
				if let ExpressionEndReason::ReachedChar(c) = expr_parser.end_data.reason {
					if end_char == c {
						self.add_ternary_op(Rc::clone(&expr), 0, original_index, parser.index);
						return true;
					}
				}
			}
		}
		parser.index = original_index;
		return false;
	}

	fn is_group_char(&self, c: char) -> bool {
		return c == '(' || c == '[';
	}

	fn get_end_char(&self, c: char) -> char {
		return match c {
			'(' => ')',
			'[' => ']',
			'{' => '}',
			_ => ' '
		}
	}

	fn parse_next_suffix_operator(&mut self, parser: &mut Parser, context: &mut Option<&mut Context>) -> bool {
		let space_offset = self.parse_next_whitespace(parser);
		let start_char = parser.chars[parser.index + space_offset];
		if self.is_group_char(start_char) {
			parser.index += space_offset + 1;
			return self.parse_suffix_internal_expressions(start_char, parser, context);
		}
		return self.parse_next_operator("suffix", parser);
	}

	fn parse_suffix_internal_expressions(&mut self, start_char: char, parser: &mut Parser, context: &mut Option<&mut Context>) -> bool {
		let end_char = self.get_end_char(start_char);
		let space_offset = self.parse_next_whitespace(parser);
		let real_second_char = parser.chars[parser.index + space_offset];
		if real_second_char == end_char {
			let empty = Vec::new();
			match end_char {
				')' => self.add_function_params(empty, parser.index, parser.index + space_offset),
				']' => self.add_array_access_params(empty, parser.index, parser.index + space_offset),
				_ => {}
			}
			parser.index += space_offset + 1;
			return true;
		} else {
			return self.parse_internal_expressions(end_char, false, parser, context);
		}
		return false;
	}

	fn parse_internal_expressions(&mut self, end_char: char, is_value: bool, parser: &mut Parser, context: &mut Option<&mut Context>) -> bool {
		let mut expressions = Vec::new();
		let start_pos = parser.index;
		let mut final_line_offset = 0;
		loop {
			let chars = vec!(end_char, ',');
			if self.index_within_bounds(parser) {
				let expr_parser = ExpressionParser::new(parser, self.generate_pos(parser.index, None), self.config_data, context, Some(chars), None);
				expressions.push(expr_parser.expression);
				parser.index += 1;
				final_line_offset = parser.line + expr_parser.position.line_offset;
				if let ExpressionEndReason::ReachedChar(c) = expr_parser.end_data.reason {
					if end_char == c {
						break;
					}
				}
			} else {
				break;
			}
		}
		match end_char {
			')' => {
				if is_value {
					if !expressions.is_empty() {
						parser.line += final_line_offset - expressions.last().as_ref().unwrap().get_line_number().unwrap_or(final_line_offset);
					}
					self.add_encapsulated_values(expressions, start_pos, parser.index);
				} else {
					self.add_function_params(expressions, start_pos, parser.index);
				}
			},
			']' => self.add_array_access_params(expressions, start_pos, parser.index),
			'}' => self.add_initilizer_list(expressions, start_pos, parser.index),
			_ => {}
		}
		return true;
	}

	fn check_operator(&self, op_str: &str, op_type: &str, exact: bool) -> Vec<usize> {
		let mut result = Vec::new();
		let operators = &self.config_data.operators[op_type];
		for i in 0..operators.len() {
			let op = &operators[i];
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