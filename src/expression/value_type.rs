/**********************************************************
 * --- Value Type ---
 *
 * The structs and enums defined here represent the types
 * inside of Tasty Fresh.
 **********************************************************/

use crate::expression::variable_type::VariableType;

/*
pub enum ValueType {
	Variable(String, VariableType),
	NullLiteral,
	BooleanLiteral(bool),
	CharLiteral(String),
	NumberLiteral(String, NumberType),
	StringLiteral(String, StringType)
}
*/

pub enum ValueType {
	Unknown,
	Null,
	Boolean,
	Number(NumberType),
	String,
	Class(ClassType)
}

impl ValueType {
	pub fn new(expr_str: &str) -> ValueType {
		if expr_str == "null" { return ValueType::Null; }
		if expr_str == "true" || expr_str == "false" { return ValueType::Boolean; }
		let mut all_numbers = true;
		for a in expr_str.chars() {
			if !a.is_numeric() {
				all_numbers = false;
			}
		}
		if all_numbers { return ValueType::Number(NumberType::Int); }
		return ValueType::Class(ClassType {
			name: "".to_string(),
			type_params: None,
			properties: Vec::new(),
			functions: Vec::new()
		})
	}
}

pub enum NumberType {
	Byte,
	UByte,
	Short,
	UShort,
	Int,
	UInt,       // u
	Long,       // l
	ULong,      // ul
	LongLong,   // ll
	ULongLong,  // ull
	Float,      // f
	Double,
	LongDouble  // l
}

impl NumberType {
	pub fn to_cpp(&self) -> &'static str {
		return match self {
			NumberType::Byte => "char",
			NumberType::UByte => "unsigned char",
			NumberType::Short => "short",
			NumberType::UShort => "unsigned short",
			NumberType::Int => "int",
			NumberType::UInt => "unsigned int",
			NumberType::Long => "long",
			NumberType::ULong => "unsigned long",
			NumberType::LongLong => "long long",
			NumberType::ULongLong => "unsigned long long",
			NumberType::Float => "float",
			NumberType::Double => "double",
			NumberType::LongDouble => "long double"
		}
	}
}

pub enum StringType {
	ConstCharArray,
	MutlilineConstCharArray,
	StringClass
}

impl StringType {
	pub fn to_cpp(&self) -> &'static str {
		return match self {
			StringType::ConstCharArray => "const char*",
			StringType::MutlilineConstCharArray => "const char*",
			StringType::StringClass => "std::string"
		}
	}
}

pub struct ClassType {
	pub name: String,
	pub type_params: Option<Vec<ValueType>>,
	pub properties: Vec<Property>,
	pub functions: Vec<Function>
}

pub struct Property {
	pub name: String,
	pub prop_type: VariableType,
	pub default_value: Option<String>
}

pub struct Function {
	pub name: String,
	pub parameters: Vec<Property>,
	pub return_type: VariableType
}