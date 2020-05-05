/**********************************************************
 * --- Value Type ---
 *
 * The structs and enums defined here represent the types
 * inside of Tasty Fresh.
 **********************************************************/

use std::collections::BTreeMap;

use crate::declaration_parser::class_declaration::ClassStyle;

use crate::expression::variable_type::{ VariableType, Type };
use crate::expression::function_type::FunStyle;

use crate::declaration_parser::function_declaration::FunctionType;

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
	Size,
	WChar,
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
			NumberType::Size => "size_t",
			NumberType::WChar => "wchar_t",
			NumberType::UnknownNumber => "int (unknown)"
		}
	}

	pub fn from_value_text(value: &mut String) -> NumberType {
		let mut offset = 0;
		let mut edit = "".to_string();
		let mut changed = false;
		let result = Self::parse_value_for_type(value, false, &mut offset, &mut changed, Some(&mut edit));
		if changed {
			(*value) = edit;
		}
		return result;
	}

	pub fn parse_value_for_type(value: &str, infinite: bool, offset: &mut usize, changed_val: &mut bool, value_mod: Option<&mut String>) -> NumberType {
		let mut unsigned = false;
		let mut long = 0;
		let mut float = false;
		let mut double = false;
		let mut dot = false;

		let mut suffix = false;

		let mut bits = false;
		let mut hex = false;

		let mut real_number = true;

		let mut index = -1;
		let mut rindex = value.len() + 1;
		let mut expect_num = false;
		for c in value.chars() {
			*offset += 1;
			index += 1;
			rindex -= 1;

			if expect_num {
				if !c.is_numeric() {
					*offset -= 1;
					break;
				} else {
					expect_num = false;
				}
			}

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
					expect_num = true;
					dot = true;
					continue;
				} else {
					real_number = false;
					break;
				}
			}
			if c == 'f' {
				if suffix {
					real_number = false;
					break;
				}
				float = true;
				suffix = true;
				if !infinite && rindex > 1 {
					real_number = false;
					break;
				}
				continue;
			}
			if c == 'd' {
				if suffix {
					real_number = false;
					break;
				}
				double = true;
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

		if float && !dot {
			if value_mod.is_some() {
				(*(value_mod.unwrap())) = format!("{}.0f", &value[0..value.len() - 1]);
				(*changed_val) = true;
			}
		} else if double {
			if value_mod.is_some() {
				let true_value_mod = value_mod.unwrap();
				(*true_value_mod) = if !dot { format!("{}.0", &value[0..value.len() - 1]) } else { value[0..value.len() - 1].to_string() };
				(*changed_val) = true;
			}
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
				if float {
					NumberType::Float
				} else if dot {
					NumberType::Double
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
	pub style: ClassStyle,
	pub extensions: Option<Vec<Type>>,
	pub type_params: Option<Vec<VariableType>>,
	pub properties: Vec<Property>,
	pub functions: Vec<Function>,
	pub operators: BTreeMap<usize,Vec<Function>>,
	pub required_includes: Vec<(String,bool)>
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
	pub default_value: Option<String>,
	pub is_declare: bool
}

impl Property {
	pub fn to_cpp(&self, is_header: bool) -> String {
		let declare_text = if self.is_declare && is_header { format!("class ") } else { "".to_string() };
		if self.default_value.is_some() && is_header {
			format!("{}{} {} = {}", declare_text, self.prop_type.to_cpp(), self.name, self.default_value.as_ref().unwrap())
		} else {
			format!("{}{} {}", declare_text, self.prop_type.to_cpp(), self.name)
		}
	}
}

#[derive(Clone, PartialEq)]
pub struct Function {
	pub name: String,
	pub parameters: Vec<Property>,
	pub return_type: VariableType,
	pub styles: Vec<FunStyle>
}

impl Function {
	pub fn to_cpp(&self, use_styles: bool, header: bool, class_name: Option<&str>, func_type: &FunctionType) -> String {
		let mut style_content = Vec::new();
		let mut post_style_content = Vec::new();
		if (func_type.is_normal() || func_type.is_destructor()) && use_styles {
			for s in &self.styles {
				if (class_name.is_some() && s.class_exportable()) ||
					(class_name.is_none() && s.module_exportable()) {
					if !func_type.is_destructor() || s.is_virtual() {
						if s.is_extern() {
							style_content.clear();
							style_content.push("extern".to_string());
							break;
						}
						if s.is_override() {
							post_style_content.push(s.get_name().to_string());
						} else {
							style_content.push(s.get_name().to_string());
						}
					}
				}
			}
		}
		format!("{}{}{}{}({}){}",
			if style_content.is_empty() { "".to_string() } else { format!("{} ", style_content.join(" ")) },
			if func_type.is_normal_or_operator() { format!("{} ", self.return_type.to_cpp()) } else { "".to_string() },
			if header || class_name.is_none() { "".to_string() } else { format!("{}::", class_name.unwrap()) },
			if func_type.is_constructor() {
				class_name.unwrap().to_string()
			} else if func_type.is_destructor() {
				format!("~{}", class_name.unwrap())
			} else if func_type.is_operator() {
				format!("operator{}", self.name)
			} else {
				self.name.clone()
			},
			self.parameters.iter().map(|param| param.to_cpp(header)).collect::<Vec<String>>().join(", "),
			if post_style_content.is_empty() { "".to_string() } else { format!(" {}", post_style_content.join(" ")) }
		)
	}
}
