/**********************************************************
 * --- Value Type ---
 *
 * The structs and enums defined here represent the types
 * inside of Tasty Fresh.
 **********************************************************/

use std::collections::BTreeMap;

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
/*
#[derive(Clone)]
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
		});
	}
}*/

#[derive(Clone, PartialEq)]
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
	LongDouble, // l
	UnknownNumber
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
			NumberType::LongDouble => "long double",
			NumberType::UnknownNumber => "int (unknown)"
		}
	}

	pub fn from_value_text(value: &str) -> NumberType {
		let mut offset = 0;
		return Self::parse_value_for_type(value, false, &mut offset);
	}

	pub fn parse_value_for_type(value: &str, infinite: bool, offset: &mut usize) -> NumberType {
		let mut unsigned = false;
		let mut long = 0;
		let mut float = false;
		let mut dot = false;

		let mut suffix = false;

		let mut bits = false;
		let mut hex = false;

		let mut complete_number = true;
		let mut real_number = true;

		let mut index = -1;
		let mut rindex = value.len() + 1;
		for c in value.chars() {
			*offset += 1;
			index += 1;
			rindex -= 1;

			if index == 1 {
				if c == 'b' {
					bits = true;
					continue;
				} else if c == 'x' {
					hex = true;
					continue;
				}
			}
			if c == '.' {
				if !dot && !suffix {
					dot = true;
					continue;
				} else {
					if dot { complete_number = true; }
					else { complete_number = false; }
					real_number = false;
					break;
				}
			}
			if dot && c == 'f' {
				float = true;
				suffix = true;
				if !infinite && rindex > 1 {
					real_number = false;
					break;
				}
				continue;
			}
			if c == 'l' {
				long += 1;
				suffix = true;
				if long > 2 || (dot && long > 1) {
					real_number = false;
					break;
				}
				continue;
			}
			if c == 'u' {
				if unsigned || dot {
					real_number = false;
					break;
				}
				unsigned = true;
				suffix = true;
				continue;
			}
			if suffix
				|| (bits && hex)
				|| (bits && !hex && c != '1' && c != '0')
				|| (!bits && hex && !c.is_ascii_hexdigit())
				|| (!bits && !hex && !c.is_ascii_digit())
			{
					real_number = false;
					break;
			}
		}

		if !real_number {
			return NumberType::UnknownNumber;
		}

		return {
			if long == 2 {
				if unsigned {
					NumberType::ULongLong
				} else {
					NumberType::LongLong
				}
			} else if long == 1 {
				if dot {
					NumberType::LongDouble
				} else if unsigned {
					NumberType::ULong
				} else {
					NumberType::Long
				}
			} else {
				if dot {
					if float {
						NumberType::Float
					} else {
						NumberType::Double
					}
				} else if unsigned {
					NumberType::UInt
				} else {
					NumberType::Int
				}
			}
		};
	}
}

#[derive(Clone, PartialEq)]
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

#[derive(Clone, PartialEq)]
pub struct ClassType {
	pub name: String,
	pub type_params: Option<Vec<VariableType>>,
	pub properties: Vec<Property>,
	pub functions: Vec<Function>,
	pub operators: BTreeMap<usize,Vec<Function>>
}

impl ClassType {
	pub fn get_field(&self, name: &str) -> VariableType {
		for p in &self.properties {
			if p.name == name {
				return p.prop_type.clone();
			}
		}
		let mut possible_functions = Vec::new();
		for f in &self.functions {
			if f.name == name {
				possible_functions.push(f.clone());
			}
		}
		if possible_functions.len() == 1 {
			return VariableType::function(possible_functions.remove(0));
		} else if possible_functions.len() > 1 {
			return VariableType::quantum_function(possible_functions);
		}
		return VariableType::inferred();
	}
}

#[derive(Clone, PartialEq)]
pub struct Property {
	pub name: String,
	pub prop_type: VariableType,
	pub default_value: Option<String>
}

impl Property {
	pub fn to_cpp(&self) -> String {
		return match &self.default_value {
			Some(value) => format!("{} {} = {}", self.prop_type.to_cpp(), self.name, value),
			None => format!("{} {}", self.prop_type.to_cpp(), self.name)
		}
	}
}

#[derive(Clone, PartialEq)]
pub struct Function {
	pub name: String,
	pub parameters: Vec<Property>,
	pub return_type: VariableType
}

impl Function {
	pub fn to_cpp(&self) -> String {
		format!("{} {}({})", self.return_type.to_cpp(), self.name, self.parameters.iter().map(|param| param.to_cpp()).collect::<Vec<String>>().join(", "))
	}
}
