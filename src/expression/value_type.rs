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

pub enum StringType {
	ConstCharArray,
	MutlilineConstCharArray,
	StringClass
}

pub struct ClassType {
	name: String,
	type_params: Option<Vec<ValueType>>,
	properties: Vec<Property>,
	functions: Vec<Function>
}

pub struct Property {
	name: String,
	prop_type: VariableType,
	default_value: Option<String>
}

pub struct Function {
	name: String,
	parameters: Vec<Property>,
	return_type: VariableType
}