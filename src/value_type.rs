/**********************************************************
 * --- Value Type ---
 *
 * The structs and enums defined here represent the types
 * inside of Tasty Fresh.
 **********************************************************/

use crate::variable_type::VariableType;

pub enum ValueType {
	Variable(String, VariableType),
	NullLiteral,
	BooleanLiteral(bool),
	CharLiteral(String),
	NumberLiteral(String, NumberType),
	StringLiteral(String, StringType)
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
	SuperLong,  // ll
	USuperLong, // ull
	Float,      // f
	Double,
	SuperDouble // l
}

pub enum StringType {
	ConstCharArray,
	MutlilineConstCharArray,
	StringClass
}

pub struct ClassType {
	name: String,
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