/**********************************************************
 * --- Expression ---
 *
 * Individual Tasty Fresh expressions are parsed and stored 
 * using the `Expression` struct provided in this file.
 **********************************************************/

use crate::expression_components::ExpressionComponents;
use crate::operator_data::{ OperatorData, OperatorDataStructure };

use std::convert::TryInto;

use num::*;

pub struct Expression {
	pub components: Vec<ExpressionComponents>
}

pub struct ExpressionEnder {
	pub until_chars: Vec<char>,
	pub end_index: i32,
	pub end_char: char
}

#[derive(Copy, Clone)]
enum ParseState {
	Prefix,
	Value,
	Suffix,
	Infix,
	End
}

impl Expression {
	pub fn new(expr_str: &str, operator_data: &OperatorDataStructure, ender: &mut ExpressionEnder) -> Expression {
		let mut result = Expression {
			components: Vec::new()//Self::parse_expr_to_components(expr_str)
		};
		result.parse_expr_to_components(expr_str, operator_data, ender);
		return result;
	}

	fn parse_expr_to_components(&mut self, expr_str: &str, operator_data: &OperatorDataStructure, ender: &mut ExpressionEnder) {
		let chars: Vec<char> = expr_str.chars().collect();
		let mut index = 0;
		let mut state = ParseState::Prefix;
		let mut stale_index = 0;
		let mut stale_index_count = 0;
		loop {
			if !self.index_within_bounds(expr_str, &index) {
				ender.end_index = index;
				break;
			}
			if self.check_for_end_char(&chars, &mut index, ender) {
				break;
			}
			let old_state = state.clone();
			self.parse(&mut state, expr_str, &mut index, operator_data);
			if state as i32 == ParseState::End as i32 {
				break;
			}
			if old_state as i32 != state as i32 {
				if stale_index != index {
					stale_index = index;
				} else {
					stale_index_count += 1;
				}
			}
			if stale_index_count > 20 {
				panic!(format!("Unable to parse {} at position {}!", expr_str, index));
				break;
			}
		}
		
		println!("-------");
		self.print_all_components("");
		println!("-------");
	}

	fn print_all_components(&self, offset_char: &str) {
		for a in &self.components {
			match a {
				ExpressionComponents::Test(b) => println!("{}{}", offset_char, b),
				ExpressionComponents::Test2(c, d) => {
					println!("{}{}", offset_char, c);
					for e in d {
						e.print_all_components(format!(" - {}", offset_char).as_str());
						println!(" - {}{}", offset_char, ',');
					}
				},
				_ => {}
			}
		}
	}

	fn check_for_end_char(&self, chars: &Vec<char>, index: &mut i32, ender: &mut ExpressionEnder) -> bool {
		*index += self.parse_next_whitespace(index, chars);
		let curr_char = chars[index.to_usize().unwrap()];
		if ender.until_chars.contains(&curr_char) {
			ender.end_index = *index;
			ender.end_char = curr_char;
			return true;
		}
		return false;
	}

	fn parse(&mut self, state: &mut ParseState, expr_str: &str, index: &mut i32, operator_data: &OperatorDataStructure) {
		match state {
			ParseState::Prefix => {
				if !self.parse_next_prefix_operator(expr_str, index, operator_data) {
					*state = ParseState::Value;
				}
			},
			ParseState::Value => {
				if !self.parse_value(expr_str, index, operator_data) {
					panic!(format!("Could not parse value at position {}!", index));
					*state = ParseState::End;
				} else {
					*state = ParseState::Suffix;
				}
			},
			ParseState::Suffix => {
				if !self.parse_next_suffix_operator(expr_str, index, operator_data) {
					*state = ParseState::Infix;
				}
			},
			ParseState::Infix => {
				if !self.parse_next_infix_operator(expr_str, index, operator_data) {
					*state = ParseState::Prefix;
				}
			}
			ParseState::End => {}
		}
	}

	fn add_op(&mut self, op: String) {
		self.components.push(ExpressionComponents::Test(op));
	}

	fn add_prefix_op(&mut self, op: String) {
		self.add_op(op);
	}

	fn add_value(&mut self, op: String) {
		self.add_op(op);
	}

	fn add_suffix_op(&mut self, op: String) {
		self.add_op(op);
	}

	fn add_infix_op(&mut self, op: String) {
		self.add_op(op);
	}

	fn add_expressions(&mut self, op: String, expr: Vec<Expression>) {
		self.components.push(ExpressionComponents::Test2(op, expr));
	}

	fn index_within_bounds(&self, expr_str: &str, index: &i32) -> bool {
		if *index >= expr_str.len().try_into().unwrap() {
			return false;
		}
		return true;
	}

	fn parse_next_whitespace(&self, index: &i32, chars: &Vec<char>) -> i32 {
		let mut space_offset = 0;
		while chars[(*index + space_offset).to_usize().unwrap()] == ' ' {
			space_offset += 1;
		}
		return space_offset;
	}

	fn parse_value(&mut self, expr_str: &str, index: &mut i32, operator_data: &OperatorDataStructure) -> bool {
		if !self.index_within_bounds(expr_str, index) { return false; }
		let chars: Vec<char> = expr_str.chars().collect();
		let mut space_offset = self.parse_next_whitespace(index, &chars);
		let mut value_start = *index + space_offset;
		let mut offset = 0;
		let first_char = chars[(value_start).to_usize().unwrap()];
		if first_char == '(' {
			*index += space_offset + 1;
			return self.parse_suffix_internal_expressions(expr_str, index, &chars, first_char, operator_data);
		} else {
			loop {
				let mut finish = false;
				if value_start + offset >= expr_str.len().try_into().unwrap() {
					finish = true;
				} else {
					let cc = chars[(value_start + offset).to_usize().unwrap()];
					if !cc.is_alphanumeric() {
						finish = true;
					}
				}
				if finish {
					let substr = &expr_str[(value_start).to_usize().unwrap()..(value_start + offset).to_usize().unwrap()];
					if substr.is_empty() {
						break;
					}
					self.add_value(substr.to_string());
					*index += offset;
					return true;
				}
				offset += 1;
			}
		}
		return false;
	}

	fn gen_substr(&self, string: &str, left: i32, right: i32) -> String {
		return string[(left.to_usize().unwrap()..right.to_usize().unwrap())].to_string();
	}

	fn parse_next_operator(&mut self, expr_str: &str, index: &mut i32, op_type: &str, operator_data: &OperatorDataStructure) -> bool {
		if !self.index_within_bounds(expr_str, index) { return false; }
		let chars: Vec<char> = expr_str.chars().collect();
		let mut space_offset = self.parse_next_whitespace(index, &chars);
		let mut prefix_start = *index + space_offset;
		let mut offset = 0;
		let mut possible_operators: Vec<String>;
		loop {
			let substr = self.gen_substr(expr_str, prefix_start, prefix_start + offset);
			possible_operators = self.check_prefix_operator(substr.as_str(), op_type, false, operator_data);
			if possible_operators.is_empty() {
				if offset > 0 {
					let prev_substr = self.gen_substr(expr_str, prefix_start, prefix_start + offset - 1);
					let exact_operators = self.check_prefix_operator(prev_substr.as_str(), op_type, true, operator_data);
					if exact_operators.len() == 1 {
						possible_operators = exact_operators;
						offset -= 1;
					}
				}
				break; 
			} else if possible_operators.len() == 1 {
				if substr == *possible_operators.first().unwrap() {
					break;
				}
			}
			offset += 1;
			if prefix_start + offset >= expr_str.len().try_into().unwrap() {
				return false;
			}
		}
		if possible_operators.len() > 0 {
			let op = possible_operators.remove(0);
			match op_type {
				"prefix" => self.add_prefix_op(op),
				"suffix" => self.add_suffix_op(op),
				"infix" => self.add_infix_op(op),
				_ => {}
			}
			*index += offset + space_offset;
			return true;
		}
		return false;
	}

	fn parse_next_prefix_operator(&mut self, expr_str: &str, index: &mut i32, operator_data: &OperatorDataStructure) -> bool {
		return self.parse_next_operator(expr_str, index, "prefix", operator_data);
	}

	fn parse_next_infix_operator(&mut self, expr_str: &str, index: &mut i32, operator_data: &OperatorDataStructure) -> bool {
		return self.parse_next_operator(expr_str, index, "infix", operator_data);
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

	fn parse_next_suffix_operator(&mut self, expr_str: &str, index: &mut i32, operator_data: &OperatorDataStructure) -> bool {
		let chars: Vec<char> = expr_str.chars().collect();
		let mut space_offset = self.parse_next_whitespace(index, &chars);
		let start_char = chars[(*index + space_offset).to_usize().unwrap()];
		if self.is_group_char(start_char) {
			*index += space_offset + 1;
			return self.parse_suffix_internal_expressions(expr_str, index, &chars, start_char, operator_data);
		}
		return self.parse_next_operator(expr_str, index, "suffix", operator_data);
	}

	fn parse_suffix_internal_expressions(&mut self, expr_str: &str, index: &mut i32, chars: &Vec<char>, start_char: char, operator_data: &OperatorDataStructure) -> bool {
		let end_char = self.get_end_char(start_char);
		let full_operator = format!("{}{}", start_char, end_char);
		let space_offset = self.parse_next_whitespace(index, &chars);
		let real_second_char = chars[(*index + space_offset).to_usize().unwrap()];
		if real_second_char == end_char {
			self.add_suffix_op(full_operator);
			*index += space_offset + 1;
		} else {
			return self.parse_internal_expressions(expr_str, index, end_char, full_operator.as_str(), operator_data);
		}
		return false;
	}

	fn parse_internal_expressions(&mut self, expr_str: &str, index: &mut i32, end_char: char, full_operator: &str, operator_data: &OperatorDataStructure) -> bool {
		let mut expressions = Vec::new();
		loop {
			let chars = vec!(end_char, ',');
			let mut ender = ExpressionEnder { until_chars: chars, end_index: 0, end_char: ' ' };
			let result = Expression::new(&expr_str[index.to_usize().unwrap()..], operator_data, &mut ender);
			expressions.push(result);
			*index += ender.end_index + 1;
			if ender.end_char == end_char {
				break;
			}
		}
		self.add_expressions(full_operator.to_string(), expressions);
		return true;
	}

	fn check_prefix_operator(&self, op_str: &str, op_type: &str, exact: bool, operator_data: &OperatorDataStructure) -> Vec<String> {
		let mut result = Vec::new();
		let prefixes = &operator_data[op_type];
		for op in prefixes {
			match &op["operator"] {
				OperatorData::SimpleOperator(v) => {
					if (exact && v == op_str) || (!exact && v.starts_with(op_str)) {
						result.push(v.clone());
					}
				},
				OperatorData::ComplexOperator(v) => {
				},
				_ => {}
			}
		}
		return result;
	}
}