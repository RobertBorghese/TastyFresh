/**********************************************************
 * --- Parser ---
 *
 * A static class containing helper functions for
 * parsing text.
 **********************************************************/

use crate::expression::variable_type::{ Type, VarStyle };
use crate::expression::value_type::{ NumberType, StringType };

/// Stores information about the parser.
///
/// # Properties
///
/// * `content` - The `&str` to parse containing the content.
/// * `chars` - A copy of `content` as a `Vec<char>` to help parse more easily.
/// * `index` - The index of the `char` the parser is currently parsing.
/// * `line` - This is incremented whenever a new line character (`\n`) is encountered.
/// * `out_of_space` - This is set to `true` if the parser hits the end of `chars`.
pub struct Parser<'a> {
	pub content: &'a str,
	pub chars: Vec<char>,
	pub index: usize,
	pub line: usize,
	pub out_of_space: bool
}

impl<'a> Parser<'a> {
	pub fn new(content: &'a str) -> Parser {
		return Parser {
			content: content,
			chars: content.chars().collect(),
			index: 0,
			line: 1,
			out_of_space: false
		}
	}

	/// Increments the index of the parser.
	///
	/// # Return
	///
	/// If the end of the content is reached, `true` is returned; `false` otherwise.
	pub fn increment(&mut self) -> bool {
		self.index += 1;
		return self.check_for_end();
	}

	/// Checks if the next characters match the `&str` provided.
	///
	/// # Arguments
	///
	/// * `check` - The content to check for.
	///
	/// # Return
	///
	/// If the next characters match `check`, `true` is returned; otherwise, `false`.
	pub fn check_ahead(&self, check: &str) -> bool {
		let slice = &self.content[self.index..];
		return slice.starts_with(check)
	}

	/// Checks if the end of the content has been reached.
	/// If it has, `out_of_space` is updated to reflect this.
	///
	/// # Return
	///
	/// If the end of the content is reached, `true` is returned; `false` otherwise.
	pub fn check_for_end(&mut self) -> bool {
		if self.index >= self.chars.len() {
			self.out_of_space = true;
			return true;
		}
		return false;
	}

	/// Provides the current `char` the parser is at.
	///
	/// # Return
	///
	/// The current `char`. If the parser has reached the end of the content, `0` is returned.
	pub fn get_curr(&self) -> char {
		if !self.out_of_space {
			return self.chars[self.index];
		}
		return 0 as char;
	}

	/// Checks if the current `char` is an alphabetic character.
	///
	/// # Return
	///
	/// If the `char` is alphabetic, `true` is returned; otherwise, `false`.
	pub fn curr_is_alphabetic(&self) -> bool {
		return self.get_curr().is_ascii_alphabetic();
	}

	/// Checks if the current `char` is a number character.
	///
	/// # Return
	///
	/// If the `char` is a number, `true` is returned; otherwise, `false`.
	pub fn curr_is_numeric(&self) -> bool {
		return self.get_curr() >= 48 as char && self.get_curr() <= 57 as char;
	}

	/// Checks if the current `char` is a hex digit character.
	///
	/// # Return
	///
	/// If the `char` is a hex digit, `true` is returned; otherwise, `false`.
	pub fn curr_is_hex_numeric(&self) -> bool {
		return (self.get_curr() >= 97 as char && self.get_curr() <= 102 as char) || self.curr_is_numeric();
	}

	/// Checks if the current `char` is whitespace.
	///
	/// # Arguments
	///
	/// * `is_newline` - Set to `true` if the char is `\n`. Otherwise, it's set to `false`.
	///
	/// # Return
	///
	/// If the `char` is whitespace, returns `true`; otherwise `false`.
	pub fn is_space(&self, is_newline: &mut bool) -> bool {
		let c = self.get_curr();
		*is_newline = c == '\n';
		return *is_newline || c == ' ' || c == '\t';
	}

	/// Brings the parser to the next, non-whitespace `char`.
	///
	/// # Return
	///
	/// Returns `false` if the first character is not whitespace; otherwise, it returns `true`.
	pub fn parse_whitespace(&mut self) -> bool {
		let mut is_newline = false;
		if self.check_for_end() { return false; }
		if !self.is_space(&mut is_newline) {
			if !self.check_and_parse_comments() {
				return false;
			}
		}
		loop {
			while self.is_space(&mut is_newline) {
				if is_newline {
					self.line += 1;
				}
				if self.increment() {
					return true;
				}
			}
			if !self.check_and_parse_comments() {
				break;
			}
		}
		
		return true;
	}

	pub fn check_and_parse_comments(&mut self) -> bool {
		if self.check_ahead("//") {
			self.parse_until('\n');
			self.increment();
			self.line += 1;
			return true;
		} else if self.check_ahead("/*") {
			loop {
				self.parse_until('*');
				if self.increment() || self.get_curr() == '/' {
					break;
				}
			}
			return true;
		}
		return false;
	}

	/// Calls `parse_whitespace` and returns `out_of_space`.
	///
	/// # Return
	///
	/// The value of `out_of_space`.
	pub fn parse_whitespace_and_check_space(&mut self) -> bool {
		self.parse_whitespace();
		return self.out_of_space;
	}

	/// Parses all upcoming ascii characters to form a `String`.
	///
	/// # Return
	///
	/// The parsed `String`.
	pub fn parse_ascii_char_name(&mut self) -> String {
		let mut result = "".to_string();
		if self.check_for_end() {
			return result;
		}
		while self.curr_is_alphabetic() {
			result.push(self.chars[self.index]);
			if self.increment() {
				break;
			}
		}
		return result;
	}

	/// Brings the parser to the next `char` that is `c`.
	///
	/// # Arguments
	///
	/// * `c` - The char to parse until.
	///
	/// # Return
	///
	/// Returns `false` if the first character is the desired `char`; otherwise, it returns `true`.
	pub fn parse_until(&mut self, c: char) -> bool {
		if self.check_for_end() { return true; }
		if self.get_curr() == c {
			return false;
		}
		loop {
			if self.increment() || self.get_curr() == c {
				break;
			}
		}
		return true;
	}

	/// Brings the parser to the next `char` that is `c` at the same expression level.
	///
	/// # Arguments
	///
	/// * `c` - The char to parse until.
	///
	/// # Return
	///
	/// Returns `false` if the first character is the desired `char`; otherwise, it returns `true`.
	pub fn parse_until_at_expr(&mut self, c: char, c2: char, result: &mut char) -> bool {
		if self.check_for_end() { return true; }
		if self.get_curr() == c || self.get_curr() == c2 {
			*result = self.get_curr();
			return false;
		}
		let mut brackets = 0;
		let mut parentheses = 0;
		while !self.out_of_space {
			if self.parse_string() {
				if self.increment() {
					break;
				}
			} else if self.parse_whitespace() {
			} else {
				if self.increment() {
					break;
				}
			}
			match self.get_curr() {
				'{' => brackets += 1,
				'}' => brackets -= 1,
				'(' => parentheses += 1,
				')' => parentheses -= 1,
				_ => ()
			}
			if brackets <= 0 && parentheses <= 0 {
				if self.get_curr() == c || self.get_curr() == c2 {
					*result = self.get_curr();
					break;
				}
			}
		}
		return true;
	}

	/// Attempts to parse the upcoming content as a String.
	///
	/// # Return
	///
	/// Returns `true` if a String is parsed successfully; otherwise `false`.
	pub fn parse_string(&mut self) -> bool {
		if self.check_for_end() { return false; }
		let mut is_raw = false;
		if !self.parse_string_prefix(&mut is_raw) { return false; }
		loop {
			if self.increment() {
				return false;
			}
			match self.get_curr() {
				'"' => {
					if !is_raw {
						break;
					}
				},
				'\\' => {
					if !self.parse_escape_char() {
						return false;
					}
				},
				')' => {
					if is_raw {
						if self.increment() {
							return false;
						}
						if self.get_curr() == '"' {
							break;
						}
					}
				}
				_ => ()
			}
		}
		return true;
	}

	/// Attempts to parse the String prefix up until the first `"`.
	///
	/// # Return
	///
	/// Returns `true` if a String prefix is parsed successfully; otherwise `false`.
	pub fn parse_string_prefix(&mut self, is_raw: &mut bool) -> bool {
		match self.get_curr() {
			'"' => {},
			'u' => {
				if self.increment() {
					return false;
				}
				match self.get_curr() {
					'8' => {
						if self.increment() || self.get_curr() != '"' {
							return false;
						}
					},
					'"' => {},
					_ => return false
				}
			},
			'L' | 'U' => {
				if self.increment() || self.get_curr() != '"' {
					return false;
				}
			},
			'R' => {
				*is_raw = true;
				if self.increment() || self.get_curr() != '"' {
					if self.increment() || self.get_curr() != '(' {
						return false;
					}
				}
			},
			_ => return false
		}
		return true;
	}

	pub fn parse_escape_char(&mut self) -> bool {
		if self.get_curr() != '\\' {
			return false;
		}
		if self.increment() {
			return false;
		}
		if self.curr_is_numeric() {
			return self.next_x_chars_are_numeric(2);
		}
		return match self.get_curr() {
			'\'' | '"' | '?' | '\\' |
			'a' | 'b' | 'f' | 'n' |
			'r' | 't' | 'v' | '0' => true,
			'x' => self.next_x_chars_are_numeric(1) && self.next_x_chars_are_hex(1),
			'u' => self.next_x_chars_are_hex(4),
			'U' => self.next_x_chars_are_hex(8),
			_ => false
		}
	}

	pub fn next_x_chars_are_numeric(&mut self, x: i32) -> bool {
		for i in 0..x {
			if self.increment() || !self.curr_is_numeric() {
				return false;
			}
		}
		return true;
	}

	pub fn next_x_chars_are_hex(&mut self, x: i32) -> bool {
		for i in 0..x {
			if self.increment() || !self.curr_is_hex_numeric() {
				return false;
			}
		}
		return true;
	}

	/// Parses the next content as a Tasty Fresh type.
	///
	/// # Arguments
	///
	/// * `unexpected_character` - This is set to `true` if the parser hits an unexpected `char`.
	/// * `conflicting_specifiers` - This is set to `true` if a type specifier is incompatible with the type.
	///
	/// # Return
	///
	/// Returns the `Type` as a primitive, `Inferred`, `Undeclared` or `UndeclaredWParams`.
	pub fn parse_type(&mut self, unexpected_character: &mut bool, conflicting_specifiers: &mut Option<&'static str>) -> Type {

		// Ensure Content Exists
		if self.check_for_end() { return Type::Inferred; }

		// Check const
		let mut is_const = false;
		let mut unsigned: Option<bool> = None;
		let mut long = false;
		let mut name_chain: Vec<String> = Vec::new();

		// Get Type Name and Specifiers
		'outer: loop {
			let content = self.parse_ascii_char_name();
			if self.out_of_space { return Type::Inferred; }
			match content.as_str() {
				"const" => {
					is_const = true;
				},
				"unsigned" => {
					if let None = unsigned {
						unsigned = Some(true);
					} else {
						*conflicting_specifiers = Some("\"signed\" and \"unsigned\" specifiers conflict");
						return Type::Inferred;
					}
				},
				"signed" => {
					if let None = unsigned {
						unsigned = Some(false);
					} else {
						*conflicting_specifiers = Some("\"signed\" and \"unsigned\" specifiers conflict");
						return Type::Inferred;
					}
				},
				"long" => {
					if !long {
						long = true;
					}
					self.parse_whitespace();
					if self.check_ahead("long") {
						return Type::Number({
								if unsigned.is_none() || !unsigned.unwrap() {
									NumberType::LongLong
								} else {
									NumberType::ULongLong
								}
							});
					} else if self.check_ahead("=") || self.check_ahead(",") {
						return Type::Number({
								if unsigned.is_none() || !unsigned.unwrap() {
									NumberType::Long
								} else {
									NumberType::ULong
								}
							});
					}
				},
				name => {
					match name {
						"char" => {
							if long {
								*conflicting_specifiers = Some("cannot use \"long\" specifier on \"char\"");
							}
							return Type::Number({
									if unsigned.is_none() || !unsigned.unwrap() {
										NumberType::Byte
									} else {
										NumberType::UByte
									}
								});
						},
						"short" => {
							if long {
								*conflicting_specifiers = Some("cannot use \"long\" specifier on \"short\"");
							}
							return Type::Number({
									if unsigned.is_none() || !unsigned.unwrap() {
										NumberType::Short
									} else {
										NumberType::UShort
									}
								});
						},
						"int" => {
							if long {
								*conflicting_specifiers = Some("cannot use \"long\" specifier on \"int\"");
							}
							return Type::Number({
									if unsigned.is_none() || !unsigned.unwrap() {
										NumberType::Int
									} else {
										NumberType::UInt
									}
								});
						},
						"float" => {
							if long {
								*conflicting_specifiers = Some("cannot use \"long\" specifier on \"float\"");
							}
							if unsigned.unwrap_or(false) {
								*conflicting_specifiers = Some("cannot use \"unsigned\" specifier on \"float\"");
							}
							return Type::Number(NumberType::Float);
						},
						"double" => {
							if unsigned.unwrap_or(false) {
								*conflicting_specifiers = Some("cannot use \"unsigned\" specifier on \"float\"");
							}
							return Type::Number({
								if long {
									NumberType::LongDouble
								} else {
									NumberType::Double
								}
							});
						},
						"bool" => {
							return Type::Boolean;
						},
						"void" => {
							return Type::Void;
						},
						type_name => {
							if long {
								*conflicting_specifiers = Some("cannot use \"long\" specifier on this type");
							}
							if unsigned.is_some() {
								*conflicting_specifiers = Some("cannot use sign-able specifier on this type");
							}
							name_chain.push(type_name.to_string());
							loop {
								self.parse_whitespace();
								if self.check_ahead("::") || self.check_ahead(".") {
									let content = self.parse_ascii_char_name();
									if self.out_of_space { return Type::Inferred; }
									name_chain.push(content);
								} else {
									break 'outer;
								}
							}
						}
					}
				}
			}
			self.parse_whitespace();
		}

		// Parse Whitespace
		if self.parse_whitespace_and_check_space() { return Type::Inferred; }

		// Check for Type Parameters
		let mut next_char = self.get_curr();
		if next_char == '<' {

			// Skip `<`
			if self.increment() { return Type::Inferred; }

			// Store Type Parameters
			let mut type_params = Vec::new();
			loop {

				// Parse Whitespace
				if self.parse_whitespace_and_check_space() { return Type::Inferred; }

				// Parse Type Parameter
				type_params.push(self.parse_type(unexpected_character, conflicting_specifiers));

				// Check for Errors
				if self.out_of_space || *unexpected_character || conflicting_specifiers.is_some() { return Type::Inferred; }

				// Parse Whitespace
				if self.parse_whitespace_and_check_space() { return Type::Inferred; }

				// Check for End of Parameters or Next Parameter
				next_char = self.get_curr();
				if next_char == ',' {
					if self.increment() { return Type::Inferred; }
				} else if next_char == '>' {
					self.increment();
					break;
				} else {
					*unexpected_character = true;
					return Type::Inferred;
				}
			}

			// Return Type with Parameters
			return Type::UndeclaredWParams(name_chain, type_params);
		}

		// Return Type Without Parameters
		return Type::Undeclared(name_chain);
	}

	/// Parses the next content as a Tasty Fresh style and type.
	///
	/// # Arguments
	///
	/// * `unexpected_character` - This is set to `true` if the parser hits an unexpected `char`.
	/// * `conflicting_specifiers` - This is set to `true` if a type specifier is incompatible with the type.
	///
	/// # Return
	///
	/// Returns the `VarStyle` as `Copy` by default and the `Type` as a primitive, `Inferred`, `Undeclared` or `UndeclaredWParams`.
	pub fn parse_type_and_style(&mut self, unexpected_character: &mut bool, conflicting_specifiers: &mut Option<&'static str>) -> (VarStyle, Type) {
		let old_index = self.index;
		let content = self.parse_ascii_char_name();
		if self.out_of_space { return (VarStyle::Copy, Type::Inferred); }

		let mut style = VarStyle::Copy;
		if VarStyle::styles().contains(&content.as_str()) {
			style = VarStyle::new(&content);
		} else {
			self.index = old_index;
		}

		self.parse_whitespace();

		let var_type = self.parse_type(unexpected_character, conflicting_specifiers);

		return (style, var_type);
	}
}