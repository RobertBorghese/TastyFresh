/**********************************************************
 * --- Attribute Declaration ---
 *
 * Represents a declaration prior to being parsed.
 **********************************************************/

use crate::declaration_parser::parser::Parser;

/// Trait used for declaration structs.
/// Provides helper functions to easily parse certain Tasty Fresh patterns.
pub trait Declaration<T> {

	/// Implemented by declaractions to specify the message displayed for unexpected end of content.
	fn out_of_space_error_msg() -> &'static str;

	fn out_of_space(index: &mut usize) -> DeclarationResult<T> {
		return DeclarationResult::Err("Unexpected End", Self::out_of_space_error_msg(), *index - 1, *index);
	}

	fn parse_whitespace(chars: &Vec<char>, index: &mut usize, line_offset: &mut usize, out_of_space: &mut bool) -> Option<DeclarationResult<T>> {
		Parser::parse_whitespace(chars, index, line_offset, out_of_space);
		if *out_of_space {
			return Some(Self::out_of_space(index));
		}
		return None;
	}

	fn parse_required_whitespace(chars: &Vec<char>, index: &mut usize, line_offset: &mut usize, out_of_space: &mut bool) -> Option<DeclarationResult<T>> {
		if !Parser::parse_whitespace(chars, index, line_offset, out_of_space) {
			return Some(DeclarationResult::Err("Expected Space", "whitspace expected here", *index - 1, *index));
		}
		if *out_of_space {
			return Some(Self::out_of_space(index));
		}
		return None;
	}

	fn parse_ascii(result: &mut String, chars: &Vec<char>, index: &mut usize, out_of_space: &mut bool) -> Option<DeclarationResult<T>> {
		*result = Parser::parse_ascii_char_name(chars, index, out_of_space);
		if result.is_empty() {
			return Some(DeclarationResult::Err("Expected Identifier", "ascii identifier expected here", *index - 1, *index));
		} else if *out_of_space {
			return Some(Self::out_of_space(index));
		}
		return None;
	}

	fn parse_required_ascii(result: &mut String, error_title: &'static str, error_msg: &'static str, chars: &Vec<char>, index: &mut usize, out_of_space: &mut bool) -> Option<DeclarationResult<T>> {
		*result = Parser::parse_ascii_char_name(chars, index, out_of_space);
		if result.is_empty() {
			return Some(DeclarationResult::Err(error_title, error_msg, *index - 1, *index));
		} else if *out_of_space {
			return Some(Self::out_of_space(index));
		}
		return None;
	}

	fn parse_required_next_char(c: char, error_msg: &'static str, next_char: &mut char, chars: &Vec<char>, index: &mut usize) -> Option<DeclarationResult<T>> {
		*next_char = chars[*index];
		if *next_char == c {
			*index += 1;
			if *index >= chars.len() { return Some(Self::out_of_space(index)); }
		} else {
			return Some(DeclarationResult::Err("Unexpected Symbol", error_msg, *index - 1, *index));
		}
		return None;
	}
}

/// Used to return either a declaraction or information regarding a syntax error.
pub enum DeclarationResult<T> {
	Ok(T),
	Err(&'static str, &'static str, usize, usize)
}

/// The following macros help simplify common patterns that must be parsed in declared Tasty Fresh code.
/// The following parameters are expected frequently:
///
/// * `$chars` - A `Vec<char>` of the `&str` currently being parsed.
/// * `$index` - A `&mut usize` of the current index the parser is at.
/// * `$line_offset` - A `&mut usize` that tracks the line offset of the parsing.
/// * `$out_of_space` - A `mut bool` that is set to `true` if the parser unexpectedly reaches the end.
/// * `$var_name` - A `mut String` that stores the retrieved characters.

/// Parses all next whitespace if any exists.
#[macro_export]
macro_rules! declare_parse_whitespace {
	($chars:expr, $index:expr, $line_offset:expr, $out_of_space:expr) => {
		if let Some(result) = Self::parse_whitespace(&$chars, $index, $line_offset, &mut $out_of_space) {
			return result;
		}
	}
}

/// Parses all next whitespace.
/// If the first character parsed is not whitespace, an error is returned.
#[macro_export]
macro_rules! declare_parse_required_whitespace {
	($chars:expr, $index:expr, $line_offset:expr, $out_of_space:expr) => {
		if let Some(result) = Self::parse_required_whitespace(&$chars, $index, $line_offset, &mut $out_of_space) {
			return result;
		}
	}
}

/// Parses all next ASCII characters to form a `String`.
#[macro_export]
macro_rules! declare_parse_ascii {
	($var_name:expr, $chars:expr, $index:expr, $out_of_space:expr) => {
		if let Some(result) = Self::parse_ascii(&mut $var_name, &$chars, $index, &mut $out_of_space) {
			return result;
		}
	}
}

/// Parses all next ASCII characters to form a `String`.
/// If the first character parsed is not an ASCII character, an error is returned.
#[macro_export]
macro_rules! declare_parse_required_ascii {
	($var_name:expr, $error_title:expr, $error_msg:expr, $chars:expr, $index:expr, $out_of_space:expr) => {
		if let Some(result) = Self::parse_required_ascii(&mut $var_name, $error_title, $error_msg, &$chars, $index, &mut $out_of_space) {
			return result;
		}
	}
}

/// Assumes the next character and parsed it.
/// If the next character is not the expected one, an error is returned.
#[macro_export]
macro_rules! declare_parse_required_next_char {
	($c:expr, $next_char:expr, $chars:expr, $index:expr) => {
		if let Some(result) = Self::parse_required_next_char($c, concat!("expected '", $c, "' operator"), &mut $next_char, &$chars, $index) {
			return result;
		}
	}
}

/// Parses all content until reaching the desired character.
#[macro_export]
macro_rules! declare_parse_everything_until_required_next_char {
	($c:expr, $chars:expr, $index:expr, $line_offset:expr, $out_of_space:expr) => {
		Parser::parse_until($c, &$chars, $index, $line_offset, &mut $out_of_space);
		if $out_of_space { return Self::out_of_space($index); }
	}
}

/// Parses the next content as a Tasty Fresh type.
/// No type information should be available at this point, so it will only return either an `Undeclared` or `UndeclaredWParams`.
#[macro_export]
macro_rules! declare_parse_type {
	($var_type:expr, $chars:expr, $index:expr, $line_offset:expr, $out_of_space:expr) => {
		let mut unexpected_char = false;
		$var_type = Parser::parse_type(&$chars, $index, $line_offset, &mut $out_of_space, &mut unexpected_char);
		if unexpected_char {
			return DeclarationResult::Err("Unexpected Character", "unexpected character here", *$index - 1, *$index);
		} else if $out_of_space {
			return Self::out_of_space($index);
		}
	}
}
