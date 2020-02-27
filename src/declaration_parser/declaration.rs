/**********************************************************
 * --- Attribute Declaration ---
 *
 * Represents a declaration prior to being parsed.
 **********************************************************/

use crate::context_management::print_code_error;
use crate::context_management::position::Position;

use crate::declaration_parser::parser::Parser;

/// Trait used for declaration structs.
/// Provides helper functions to easily parse certain Tasty Fresh patterns.
pub trait Declaration<T> {

	/// Implemented by declaractions to specify the message displayed for unexpected end of content.
	fn out_of_space_error_msg() -> &'static str;

	fn out_of_space(index: usize) -> DeclarationResult<T> {
		return DeclarationResult::Err("Unexpected End", Self::out_of_space_error_msg(), index - 1, index);
	}

	fn unexpected_character(index: usize) -> DeclarationResult<T> {
		return DeclarationResult::Err("Unexpected Symbol", "unexpected symbol", index, index + 1);
	}

	fn increment_parser(parser: &mut Parser) -> Option<DeclarationResult<T>> {
		parser.increment();
		if parser.out_of_space {
			return Some(Self::out_of_space(parser.index));
		}
		return None;
	}

	fn parse_whitespace(parser: &mut Parser) -> Option<DeclarationResult<T>> {
		parser.parse_whitespace();
		if parser.out_of_space {
			return Some(Self::out_of_space(parser.index));
		}
		return None;
	}

	fn parse_required_whitespace(parser: &mut Parser) -> Option<DeclarationResult<T>> {
		if !parser.parse_whitespace() {
			return Some(DeclarationResult::Err("Expected Space", "whitspace expected here", parser.index - 1, parser.index));
		}
		if parser.out_of_space {
			return Some(Self::out_of_space(parser.index));
		}
		return None;
	}

	fn parse_ascii(result: &mut String, parser: &mut Parser) -> Option<DeclarationResult<T>> {
		*result = parser.parse_ascii_char_name();
		if result.is_empty() {
			return Some(DeclarationResult::Err("Expected Identifier", "ascii identifier expected here", parser.index - 1, parser.index));
		} else if parser.out_of_space {
			return Some(Self::out_of_space(parser.index));
		}
		return None;
	}

	fn parse_required_ascii(result: &mut String, error_title: &'static str, error_msg: &'static str, parser: &mut Parser) -> Option<DeclarationResult<T>> {
		*result = parser.parse_ascii_char_name();
		if result.is_empty() {
			return Some(DeclarationResult::Err(error_title, error_msg, parser.index - 1, parser.index));
		} else if parser.out_of_space {
			return Some(Self::out_of_space(parser.index));
		}
		return None;
	}

	fn parse_required_next_char(c: char, error_msg: &'static str, next_char: &mut char, parser: &mut Parser) -> Option<DeclarationResult<T>> {
		*next_char = parser.get_curr();
		if *next_char == c {
			if parser.increment() {
				return Some(Self::out_of_space(parser.index));
			}
		} else {
			return Some(DeclarationResult::Err("Unexpected Symbol", error_msg, parser.index, parser.index + 1));
		}
		return None;
	}
}

/// Used to return either a declaraction or information regarding a syntax error.
pub enum DeclarationResult<T> {
	Ok(T),
	Err(&'static str, &'static str, usize, usize)
}

impl<T> DeclarationResult<T> {
	pub fn is_ok(&self) -> bool {
		return match self {
			DeclarationResult::Ok(_) => true,
			_ => false
		}
	}

	pub fn is_error(&self) -> bool {
		return !self.is_ok();
	}

	pub fn unwrap_and_move(self) -> T {
		match self {
			DeclarationResult::Ok(result) => return result,
			_ => panic!("Result is error.")
		}
	}

	pub fn unwrap(&self) -> &T {
		match self {
			DeclarationResult::Ok(result) => return result,
			_ => panic!("Result is error.")
		}
	}

	pub fn as_ref(&self) -> &DeclarationResult<T> {
		return self;
	}

	pub fn as_mut(&mut self) -> &mut DeclarationResult<T> {
		return self;
	}

	pub fn print_error(&self, file: String, file_content: &str) {
		match self {
			DeclarationResult::Err(title, message, start, end) => print_code_error(title, message, &Position::new(file, None, *start, Some(*end)), file_content),
			_ => panic!("Result is not an error.")
		}
	}
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
macro_rules! delcare_increment {
	($parser:expr) => {
		if let Some(result) = Self::increment_parser($parser) {
			return result;
		}
	}
}

/// Parses all next whitespace if any exists.
#[macro_export]
macro_rules! declare_parse_whitespace {
	($parser:expr) => {
		if let Some(result) = Self::parse_whitespace($parser) {
			return result;
		}
	}
}

/// Parses all next whitespace.
/// If the first character parsed is not whitespace, an error is returned.
#[macro_export]
macro_rules! declare_parse_required_whitespace {
	($parser:expr) => {
		if let Some(result) = Self::parse_required_whitespace($parser) {
			return result;
		}
	}
}

/// Parses all next ASCII characters to form a `String`.
#[macro_export]
macro_rules! declare_parse_ascii {
	($var_name:expr, $parser:expr) => {
		if let Some(result) = Self::parse_ascii(&mut $var_name, $parser) {
			return result;
		}
	}
}

/// Parses all next ASCII characters to form a `String`.
/// If the first character parsed is not an ASCII character, an error is returned.
#[macro_export]
macro_rules! declare_parse_required_ascii {
	($var_name:expr, $error_title:expr, $error_msg:expr, $parser:expr) => {
		if let Some(result) = Self::parse_required_ascii(&mut $var_name, $error_title, $error_msg, $parser) {
			return result;
		}
	}
}

/// Assumes the next character and parsed it.
/// If the next character is not the expected one, an error is returned.
#[macro_export]
macro_rules! declare_parse_required_next_char {
	($c:expr, $next_char:expr, $parser:expr) => {
		if let Some(result) = Self::parse_required_next_char($c, concat!("expected '", $c, "' operator"), &mut $next_char, $parser) {
			return result;
		}
	}
}

/// Parses all content until reaching the desired character.
#[macro_export]
macro_rules! declare_parse_until_char {
	($c:expr, $parser:expr) => {
		$parser.parse_until($c);
		if $parser.out_of_space { return Self::out_of_space($parser.index); }
	}
}

/// Parses all content, assumed to be an expression, until reaching the desired character.
#[macro_export]
macro_rules! declare_parse_expr_until_next_char {
	($c:expr, $parser:expr) => {
		let mut result = ' ';
		$parser.parse_until_at_expr($c, $c, &mut result);
		if $parser.out_of_space { return Self::out_of_space($parser.index); }
	}
}

/// Parses all content, assumed to be an expression, until reaching one of the two desired characters.
#[macro_export]
macro_rules! declare_parse_expr_until_either_char {
	($c:expr, $c2:expr, $result:expr, $parser:expr) => {
		$parser.parse_until_at_expr($c, $c2, &mut $result);
		if $parser.out_of_space { return Self::out_of_space($parser.index); }
	}
}

/// Parses the next content as a Tasty Fresh type.
/// No type information should be available at this point, so it will only return a primitive, `Inferred`, `Undeclared` or `UndeclaredWParams`.
#[macro_export]
macro_rules! declare_parse_type {
	($var_type:expr, $parser:expr) => {
		let mut unexpected_char = false;
		let mut specifier_error: Option<&'static str> = None;
		$var_type = $parser.parse_type(&mut unexpected_char, &mut specifier_error);
		if specifier_error.is_some() {
			return DeclarationResult::Err("Specifier Error", specifier_error.unwrap(), $parser.index - 1, $parser.index);
		} else if unexpected_char {
			return DeclarationResult::Err("Unexpected Character", "unexpected character here", $parser.index - 1, $parser.index);
		} else if  $parser.out_of_space {
			return Self::out_of_space($parser.index);
		}
	}
}//parse_type_and_style

/// Parses the next content as a Tasty Fresh style and type.
/// No type information should be available at this point, so it will only return a primitive, `Inferred`, `Undeclared` or `UndeclaredWParams`.
#[macro_export]
macro_rules! declare_parse_type_and_style {
	($var_type:expr, $var_style:expr, $parser:expr) => {
		let mut unexpected_char = false;
		let mut specifier_error: Option<&'static str> = None;
		let type_and_style = $parser.parse_type_and_style(&mut unexpected_char, &mut specifier_error);
		$var_style = type_and_style.0;
		$var_type = type_and_style.1;
		if specifier_error.is_some() {
			return DeclarationResult::Err("Specifier Error", specifier_error.unwrap(), $parser.index - 1, $parser.index);
		} else if unexpected_char {
			return DeclarationResult::Err("Unexpected Character", "unexpected character here", $parser.index - 1, $parser.index);
		} else if  $parser.out_of_space {
			return Self::out_of_space($parser.index);
		}
	}
}
