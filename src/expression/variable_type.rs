/**********************************************************
 * --- Variable Type ---
 *
 * The structs and enums defined here represent variables
 * within Tasty Fresh.
 **********************************************************/

use crate::expression::value_type::{ NumberType, StringType, ClassType, Function };

lazy_static! {
	pub static ref STYLE_TYPES: Vec<&'static str> = vec!("copy", "ref", "borrow", "move", "ptr", "autoptr", "uniqueptr", "classptr");
	pub static ref VARIABLE_PROPS: Vec<&'static str> = vec!("const", "constexpr", "constinit", "extern", "mutable", "static", "thread_local", "volatile");
}

#[derive(Clone)]
pub struct VariableType {
	pub var_type: Type,
	pub var_style: VarStyle,
	pub var_properties: Option<Vec<VarProps>> 
}

impl VariableType {
	pub fn to_cpp(&self) -> String {
		return self.var_style.to_cpp(&self.var_type);
	}

	pub fn from_type_style(info: (VarStyle, Type)) -> VariableType {
		return VariableType {
			var_type: info.1,
			var_style: info.0,
			var_properties: None
		};
	}

	pub fn inferred() -> VariableType {
		return VariableType {
			var_type: Type::Inferred,
			var_style: VarStyle::Copy,
			var_properties: None
		};
	}

	pub fn copy(var_type: Type) -> VariableType {
		return VariableType {
			var_type: var_type,
			var_style: VarStyle::Copy,
			var_properties: None
		};
	}

	pub fn namespace() -> VariableType {
		return VariableType {
			var_type: Type::Inferred,
			var_style: VarStyle::Namespace,
			var_properties: None
		};
	}

	pub fn is_namespace(&self) -> bool {
		if let VarStyle::Namespace = self.var_style {
			return true;
		}
		return false;
	}

	pub fn boolean() -> VariableType {
		return VariableType {
			var_type: Type::Boolean,
			var_style: VarStyle::Copy,
			var_properties: None
		};
	}

	pub fn class(cls: ClassType) -> VariableType {
		return VariableType {
			var_type: Type::Class(cls),
			var_style: VarStyle::Copy,
			var_properties: None
		};
	}

	pub fn function(func: Function) -> VariableType {
		return VariableType {
			var_type: Type::Function(Box::new(func)),
			var_style: VarStyle::Copy,
			var_properties: None
		};
	}
}

#[derive(Clone)]
pub enum Type {
	Unknown(String),
	Void,
	Boolean,
	Number(NumberType),
	String(StringType),
	Class(ClassType),
	Function(Box<Function>),
	Inferred,
	Undeclared(Vec<String>),
	UndeclaredWParams(Vec<String>, Vec<VariableType>)
}

/*#[derive(Clone)]
pub struct Property {
	pub name: String,
	pub prop_type: VariableType,
	pub default_value: Option<String>
}

#[derive(Clone)]
pub struct Function {
	pub name: String,
	pub parameters: Vec<Property>,
	*/

impl Type {
	pub fn to_cpp(&self) -> String {
		match self {
			Type::Unknown(name) => name.clone(),
			Type::Void => "void".to_string(),
			Type::Boolean => "bool".to_string(),
			Type::Number(num_type) => num_type.to_cpp().to_string(),
			Type::String(string_type) => string_type.to_cpp().to_string(),
			Type::Class(class_type) => class_type.name.clone(),
			Type::Function(func) => {
				let params = &func.parameters;
				let mut params_output = "".to_string();
				for i in 0..params.len() {
					params_output += &params[i].prop_type.to_cpp();
					if i < params.len() - 1 {
						params_output += ", ";
					}
				}
				format!("std::function<{}({})>", func.return_type.to_cpp(), params_output)
			}
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

#[derive(Clone)]
pub enum VarStyle {
	Unknown,
	Namespace,
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
			VarStyle::Copy => "copy",
			VarStyle::Ref => "ref",
			VarStyle::Borrow => "borrow",
			VarStyle::Move => "move",
			VarStyle::Ptr => "ptr",
			VarStyle::AutoPtr => "autoptr",
			VarStyle::UniquePtr => "uniqueptr",
			VarStyle::ClassPtr => "classptr",
			VarStyle::Namespace => "namespace",
			VarStyle::Unknown => "unknown"
		}
	}

	pub fn to_cpp(&self, var_type: &Type) -> String {
		return match self {
			VarStyle::Copy => var_type.to_cpp(),
			VarStyle::Ref => format!("{}&", var_type.to_cpp()),
			VarStyle::Borrow => {
				if let Type::String(str_type) = var_type {
					if let StringType::ConstCharArray = str_type {
						format!("{}&", var_type.to_cpp())
					} else {
						format!("const {}&", var_type.to_cpp())
					}
				} else {
					format!("const {}&", var_type.to_cpp())
				}
			},
			VarStyle::Move => format!("{}&&", var_type.to_cpp()),
			VarStyle::Ptr => format!("{}*", var_type.to_cpp()),
			VarStyle::AutoPtr => format!("std::shared_ptr<{}>", var_type.to_cpp()),
			VarStyle::UniquePtr => format!("std::unique_ptr<{}>", var_type.to_cpp()),
			VarStyle::ClassPtr => format!("{}*", var_type.to_cpp()),
			_ => var_type.to_cpp()
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

	pub fn is_namespace(&self) -> bool {
		return match self {
			VarStyle::Namespace => true,
			_ => false
		}
	}
}

#[derive(Clone)]
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
