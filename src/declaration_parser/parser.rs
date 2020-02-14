/**********************************************************
 * --- Parser ---
 *
 * A static class containing helper functions for
 * parsing text.
 **********************************************************/

use crate::expression::variable_type::Type;

pub struct Parser {
}

impl Parser {
	pub fn check_for_end(chars: &Vec<char>, index: &mut usize, out_of_space: &mut bool) -> bool {
		if *index >= chars.len() {
			*out_of_space = true;
			return true;
		}
		return false;
	}

	/// Parses all upcoming ascii characters to form a `String`.
	///
	/// # Arguments
	///
	/// * `chars` - The `Vec<char>` to parse.
	/// * `index` - The index to start and increment on. 
	/// * `out_of_space` - This is set to `true` if the parser hits the end of `chars`.
	///
	/// # Return
	///
	/// The parsed `String`.
	pub fn parse_ascii_char_name(chars: &Vec<char>, index: &mut usize, out_of_space: &mut bool) -> String {
		let mut result = "".to_string();
		if Self::check_for_end(chars, index, out_of_space) { return result; }
		while chars[*index].is_ascii_alphabetic() {
			result.push(chars[*index]);
			*index += 1;
			if Self::check_for_end(chars, index, out_of_space) { return result; }
		}
		return result;
	}

	/// Checks if the `char` is whitespace.
	///
	/// # Arguments
	///
	/// * `c` - The `char` to check.
	/// * `is_newline` - Set to `true` if the char is `\n`. Otherwise, it's set to `false`.
	///
	/// # Return
	///
	/// If the `char` is whitespace, returns `true`; otherwise `false`.
	pub fn is_space(c: char, is_newline: &mut bool) -> bool {
		*is_newline = c == '\n';
		return *is_newline || c == ' ' || c == '\t';
	}

	/// Brings the `index` value to the next, non-whitespace index of the `Vec<char>`.
	///
	/// # Arguments
	///
	/// * `chars` - The `Vec<char>` to parse.
	/// * `index` - The index to start and increment on.
	/// * `line_offset` - This is incremented whenever a new line character (`\n`) is encountered.
	/// * `out_of_space` - This is set to `true` if the parser hits the end of `chars`.
	///
	/// # Return
	///
	/// Returns `false` if the first character is not whitespace; otherwise, it returns `true`.
	pub fn parse_whitespace(chars: &Vec<char>, index: &mut usize, line_offset: &mut usize, out_of_space: &mut bool) -> bool {
		let mut is_newline = false;
		if Self::check_for_end(chars, index, out_of_space) { return false; }
		if !Self::is_space(chars[*index], &mut is_newline) {
			return false;
		}
		while Self::is_space(chars[*index], &mut is_newline) {
			if is_newline {
				*line_offset += 1;
			}
			*index += 1;
			if Self::check_for_end(chars, index, out_of_space) { return true; }
		}
		return true;
	}

	/// Brings the `index` value to the next `char` that is `c`.
	///
	/// # Arguments
	///
	/// * `c` - The char to parse until.
	/// * `chars` - The `Vec<char>` to parse.
	/// * `index` - The index to start and increment on.
	/// * `line_offset` - This is incremented whenever a new line character (`\n`) is encountered.
	/// * `out_of_space` - This is set to `true` if the parser hits the end of `chars`.
	///
	/// # Return
	///
	/// Returns `false` if the first character is the desired `char`; otherwise, it returns `true`.
	pub fn parse_until(c: char, chars: &Vec<char>, index: &mut usize, line_offset: &mut usize, out_of_space: &mut bool) -> bool {
		if Self::check_for_end(chars, index, out_of_space) { return true; }
		if chars[*index] == c {
			return false;
		}
		loop {
			*index += 1;
			if Self::check_for_end(chars, index, out_of_space) || chars[*index] == c {
				break;
			}
		}
		return true;
	}

	/// Parses the next content as Tasty Fresh syntax.
	///
	/// # Arguments
	///
	/// * `chars` - The `Vec<char>` to parse.
	/// * `index` - The index to start and increment on.
	/// * `line_offset` - This is incremented whenever a new line character (`\n`) is encountered.
	/// * `out_of_space` - This is set to `true` if the parser hits the end of `chars`.
	///
	/// # Return
	///
	/// Returns the `Type` as an `Undeclared` or `UndeclaredWParams`.
	pub fn parse_type(chars: &Vec<char>, index: &mut usize, line_offset: &mut usize, out_of_space: &mut bool, unexpected_character: &mut bool) -> Type {
		if Self::check_for_end(chars, index, out_of_space) { return Type::Inferred; }
		let name = Self::parse_ascii_char_name(chars, index, out_of_space);
		if *out_of_space { return Type::Inferred; }
		Self::parse_whitespace(chars, index, line_offset, out_of_space);
		if *out_of_space { return Type::Inferred; }
		let mut next_char = chars[*index];
		if next_char == '<' {
			*index += 1;
			if Self::check_for_end(chars, index, out_of_space) { return Type::Inferred; }
			let mut type_params = Vec::new();
			loop {
				type_params.push(Self::parse_type(chars, index, line_offset, out_of_space, unexpected_character));
				if *out_of_space || *unexpected_character { return Type::Inferred; }
				next_char = chars[*index];
				if next_char == ',' {
					*index += 1;
					if Self::check_for_end(chars, index, out_of_space) { return Type::Inferred; }
				} else if next_char == '>' {
					break;
				} else {
					*unexpected_character = true;
					return Type::Inferred;
				}
			}
			return Type::UndeclaredWParams(name, type_params);
		}
		return Type::Undeclared(name);
	}
}
