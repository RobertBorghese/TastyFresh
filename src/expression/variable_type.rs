/**********************************************************
 * --- Variable Type ---
 *
 * The structs and enums defined here represent variables
 * within Tasty Fresh.
 **********************************************************/

use crate::expression::value_type::{ NumberType, StringType, ClassType };

lazy_static! {
	pub static ref STYLE_TYPES: Vec<&'static str> = vec!("copy", "ref", "borrow", "move", "ptr", "autoptr", "uniqueptr", "classptr");
	pub static ref VARIABLE_PROPS: Vec<&'static str> = vec!("const", "constexpr", "constinit", "extern", "mutable", "static", "thread_local", "volatile");
}

pub struct VariableType {
	pub var_type: Type,
	pub var_style: VarStyle,
	pub var_properties: Vec<VarProps> 
}

pub enum Type {
	Unknown(String),
	Void,
	Boolean,
	Number(NumberType),
	String(StringType),
	Class(ClassType),
	Inferred,
	Undeclared(Vec<String>),
	UndeclaredWParams(Vec<String>, Vec<Type>)
}

impl Type {
	pub fn to_cpp(&self) -> String {
		match self {
			Type::Unknown(name) => name.clone(),
			Type::Void => "void".to_string(),
			Type::Boolean => "bool".to_string(),
			Type::Number(num_type) => num_type.to_cpp().to_string(),
			Type::String(string_type) => string_type.to_cpp().to_string(),
			Type::Class(class_type) => class_type.name.clone(),
			Type::Inferred => "auto".to_string(),
			Type::Undeclared(names) => {
				let mut result = "".to_string();
				for i in 0..names.len() {
					if i > 0 {
						result += "::";
					}
					result += &names[i];
				}
				result
			},
			Type::UndeclaredWParams(names, type_args) => {
				let mut result = "".to_string();
				for i in 0..names.len() {
					if i > 0 {
						result += "::";
					}
					result += &names[i];
				}
				result += "<";
				let mut i = 0;
				loop {
					if i < type_args.len() {
						result += type_args[i].to_cpp().as_str();
						i += 1;
						if i < type_args.len() {
							result += ", ";
						}
					} else {
						break;
					}
				}
				result += ">";
				result
			}
		}
	}
}

pub enum VarStyle {
	Unknown,
	Copy,
	Ref,
	Borrow,
	Move,
	Ptr,
	AutoPtr,
	UniquePtr,
	ClassPtr
}

impl VarStyle {
	pub fn new(name: &str) -> VarStyle {
		return match name {
			"copy" => VarStyle::Copy,
			"ref" => VarStyle::Ref,
			"borrow" => VarStyle::Borrow,
			"move" => VarStyle::Move,
			"ptr" => VarStyle::Ptr,
			"autoptr" => VarStyle::AutoPtr,
			"uniqueptr" => VarStyle::UniquePtr,
			"classptr" => VarStyle::ClassPtr,
			_ => VarStyle::Unknown
		}
	}

	pub fn styles() -> &'static Vec<&'static str> {
		return &STYLE_TYPES;
	}

	pub fn get_name(&self) -> &str {
		return match self {
			VarStyle::Unknown => "",
			VarStyle::Copy => "copy",
			VarStyle::Ref => "ref",
			VarStyle::Borrow => "borrow",
			VarStyle::Move => "move",
			VarStyle::Ptr => "ptr",
			VarStyle::AutoPtr => "autoptr",
			VarStyle::UniquePtr => "uniqueptr",
			VarStyle::ClassPtr => "classptr"
		}
	}

	pub fn is_unknown(&self) -> bool {
		return match self {
			VarStyle::Unknown => true,
			_ => false
		}
	}

	pub fn class_only(&self) -> bool {
		return match self {
			VarStyle::ClassPtr => true,
			_ => false
		}
	}

	pub fn module_only(&self) -> bool {
		return false;
	}
}

pub enum VarProps {
	Unknown,
	Const,
	Constexpr,
	Constinit,
	Extern,
	Mutable,
	Static,
	Threadlocal,
	Volatile
}

impl VarProps {
	pub fn new(name: &str) -> VarProps {
		return match name {
			"const" => VarProps::Const,
			"constexpr" => VarProps::Constexpr,
			"constinit" => VarProps::Constinit,
			"extern" => VarProps::Extern,
			"mutable" => VarProps::Mutable,
			"static" => VarProps::Static,
			"thread_local" => VarProps::Threadlocal,
			"volatile" => VarProps::Volatile,
			_ => VarProps::Unknown
		}
	}

	pub fn properties() -> &'static Vec<&'static str> {
		return &VARIABLE_PROPS;
	}

	pub fn get_name(&self) -> &str {
		return match self {
			VarProps::Unknown => "",
			VarProps::Const => "const",
			VarProps::Constexpr => "constexpr",
			VarProps::Constinit => "constinit",
			VarProps::Extern => "extern",
			VarProps::Mutable => "mutable",
			VarProps::Static => "static",
			VarProps::Threadlocal => "thread_local",
			VarProps::Volatile => "volatile"
		}
	}
}
