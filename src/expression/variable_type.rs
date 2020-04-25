/**********************************************************
 * --- Variable Type ---
 *
 * The structs and enums defined here represent variables
 * within Tasty Fresh.
 **********************************************************/

use std::rc::Rc;

use crate::expression::Expression;
use crate::expression::value_type::{ NumberType, StringType, ClassType, Function };

use crate::context_management::context::Context;
use crate::context_management::typing_context::ContextType;
use crate::context_management::context_manager::ContextManager;

lazy_static! {
	pub static ref STYLE_TYPES: Vec<&'static str> = vec!("copy", "ref", "borrow", "move", "ptr", "autoptr", "uniqueptr", "classptr", "let", "ptr2", "ptr3", "ptr4", "ptr5", "ptr6", "ptr7", "ptr8", "ptr9");
	pub static ref VARIABLE_PROPS: Vec<&'static str> = vec!("const", "constexpr", "constinit", "extern", "mutable", "forever", "thread_local", "volatile", "declare");
}

#[derive(Clone, PartialEq)]
pub struct VariableType {
	pub var_type: Type,
	pub var_style: VarStyle,
	pub var_properties: Option<Vec<VarProps>>,
	pub var_optional: bool
}

impl VariableType {
	pub fn to_cpp(&self) -> String {
		let mut declare = false;
		if self.var_properties.is_some() {
			for prop in self.var_properties.as_ref().unwrap() {
				if prop.is_declare() {
					declare = true;
				}
			}
		}
		return self.var_style.to_cpp(&self.var_type, declare);
	}

	pub fn resolve(&mut self, context: &Context, ctx_manager: &mut ContextManager) {
		match &self.var_type {
			Type::Undeclared(names) => {
				if names.len() == 1 {
					let context_type = context.module.get_item(names.first().unwrap(), Some(ctx_manager), false);
					if context_type.is_some() {
						if let ContextType::Class(cls) = context_type.unwrap() {
							self.var_type = Type::Class(cls.clone());
						}
					}
				}
			},
			_ => ()
		}
	}

	pub fn from_type_style(info: (VarStyle, Type, bool)) -> VariableType {
		return VariableType {
			var_type: info.1,
			var_style: info.0,
			var_properties: None,
			var_optional: info.2
		};
	}

	pub fn default_value(&self) -> Option<&'static str> {
		let is_ptr = self.var_style.is_ptr();
		if is_ptr.is_none() {
			return None;
		} else {
			if is_ptr.unwrap() {
				return Some("nullptr");
			} else {
				return self.var_type.default_value();
			}
		}
	}

	pub fn inferred() -> VariableType {
		return VariableType {
			var_type: Type::Inferred,
			var_style: VarStyle::Copy,
			var_properties: None,
			var_optional: false
		};
	}

	pub fn void() -> VariableType {
		return VariableType {
			var_type: Type::Void,
			var_style: VarStyle::Copy,
			var_properties: None,
			var_optional: false
		};
	}

	pub fn of_inferred_style(tf_type: Type) -> VariableType {
		return VariableType {
			var_type: tf_type,
			var_style: VarStyle::Infer,
			var_properties: None,
			var_optional: false
		};
	}

	pub fn is_inferred(&self) -> bool {
		if let Type::Inferred = self.var_type {
			return true;
		}
		return false;
	}

	pub fn is_inferred_style(&self) -> bool {
		if let VarStyle::Infer = self.var_style {
			return true;
		}
		return false;
	}

	pub fn this() -> VariableType {
		return VariableType {
			var_type: Type::This,
			var_style: VarStyle::Ptr(1),
			var_properties: None,
			var_optional: false
		};
	}

	pub fn initializer_list(var_type: VariableType) -> VariableType {
		return VariableType {
			var_type: Type::InitializerList(Box::new(var_type)),
			var_style: VarStyle::Copy,
			var_properties: None,
			var_optional: false
		};
	}

	pub fn copy(var_type: Type) -> VariableType {
		return VariableType {
			var_type: var_type,
			var_style: VarStyle::Copy,
			var_properties: None,
			var_optional: false
		};
	}

	pub fn namespace() -> VariableType {
		return VariableType {
			var_type: Type::Inferred,
			var_style: VarStyle::Namespace,
			var_properties: None,
			var_optional: false
		};
	}

	pub fn is_namespace(&self) -> bool {
		if let VarStyle::Namespace = self.var_style {
			return true;
		}
		return false;
	}

	pub fn is_number(&self) -> bool {
		if let Type::Number(..) = self.var_type {
			return true;
		}
		return false;
	}

	pub fn boolean() -> VariableType {
		return VariableType {
			var_type: Type::Boolean,
			var_style: VarStyle::Copy,
			var_properties: None,
			var_optional: false
		};
	}

	pub fn class(cls: ClassType) -> VariableType {
		return VariableType {
			var_type: Type::Class(cls),
			var_style: VarStyle::Copy,
			var_properties: None,
			var_optional: false
		};
	}

	pub fn function(func: Function) -> VariableType {
		return VariableType {
			var_type: Type::Function(Box::new(func)),
			var_style: VarStyle::Copy,
			var_properties: None,
			var_optional: false
		};
	}

	pub fn quantum_function(funcs: Vec<Function>) -> VariableType {
		return VariableType {
			var_type: Type::QuantumFunction(funcs),
			var_style: VarStyle::Copy,
			var_properties: None,
			var_optional: false
		};
	}

	pub fn tuple(tuple_types: Vec<VariableType>) -> VariableType {
		return VariableType {
			var_type: Type::Tuple(tuple_types),
			var_style: VarStyle::Copy,
			var_properties: None,
			var_optional: false
		};
	}

	pub fn access_operator(&self) -> &'static str {
		return if self.is_namespace() {
			"::"
		} else if self.var_style.is_ptr().unwrap_or(false) {
			"->"
		} else {
			"."
		};
	}

	pub fn compare_types(&self, other: &VariableType) -> Option<VariableType> {
		if self == other || (self.is_inferred() && !other.is_inferred()) {
			return Some(self.clone());
		} else if !self.is_inferred() && other.is_inferred() {
			return Some(other.clone());
		}
		return None;
	}

	pub fn convert_between_styles(&self, other: &VariableType, content: &str) -> Option<String> {
		return match self.var_style {
			VarStyle::Copy |
			VarStyle::Ref |
			VarStyle::Borrow |
			VarStyle::Move => {
				match other.var_style {
					VarStyle::Copy |
					VarStyle::Ref |
					VarStyle::Borrow => Some(content.to_string()),
					VarStyle::Move => Some(format!("std::move({})", content)),
					VarStyle::Ptr(size) => Some(format!("{}{}", String::from_utf8(vec![b'&'; size]).unwrap(), content)),
					VarStyle::AutoPtr => Some(format!("std::make_shared<{}>({})", other.to_cpp(), content)),
					VarStyle::UniquePtr => Some(format!("std::make_unique<{}>({})", other.to_cpp(), content)),
					_ => None
				}
			},
			VarStyle::Ptr(self_size) => {
				let stars = String::from_utf8(vec![b'*'; self_size]).unwrap();
				match other.var_style {
					VarStyle::Copy |
					VarStyle::Ref |
					VarStyle::Borrow => Some(format!("{}{}", stars, content)),
					VarStyle::Ptr(size) => {
						if self_size < size {
							Some(format!("{}{}", String::from_utf8(vec![b'&'; size - self_size]).unwrap(), content))
						} else if self_size > size {
							Some(format!("{}{}", String::from_utf8(vec![b'*'; self_size - size]).unwrap(), content))
						} else {
							Some(content.to_string())
						}
					},
					VarStyle::AutoPtr => Some(format!("std::make_shared<{}>({}{})", other.to_cpp(), stars, content)),
					VarStyle::UniquePtr => Some(format!("std::make_unique<{}>({}{})", other.to_cpp(), stars, content)),
					_ => None
				}
			},
			VarStyle::AutoPtr => {
				match other.var_style {
					VarStyle::Copy |
					VarStyle::Ref |
					VarStyle::Borrow => Some(format!("*{}", content)),
					VarStyle::Move => Some(format!("std::move(*{})", content)),
					VarStyle::Ptr(size) => if size == 1 {
						Some(format!("{}.get()", content))
					} else {
						Some(format!("{}{}.get()", String::from_utf8(vec![b'&'; size - 1]).unwrap(), content))
					},
					VarStyle::AutoPtr => Some(content.to_string()),
					_ => None
				}
			},
			VarStyle::UniquePtr => {
				match other.var_style {
					VarStyle::Copy |
					VarStyle::Ref |
					VarStyle::Borrow => Some(format!("*{}", content)),
					VarStyle::Move => Some(format!("std::move(*{})", content)),
					VarStyle::Ptr(size) => if size == 1 {
						Some(format!("{}.get()", content))
					} else {
						Some(format!("{}{}.get()", String::from_utf8(vec![b'&'; size - 1]).unwrap(), content))
					},
					VarStyle::UniquePtr => Some(content.to_string()),
					_ => None
				}
			},
			_ => None
		}
	}

	pub fn check_accessor_content(&self, content: &str, _context: &Option<&mut Context>) -> Option<VariableType> {
		return match &self.var_type {
			Type::Class(cls_type) => {
				Some(cls_type.get_field(content))
			},
			_ => None
		}
	}

	pub fn is_quantum_function(&self) -> bool {
		if let Type::QuantumFunction(_) = self.var_type {
			return true;
		}
		return false;
	}

	pub fn is_only_static(&self) -> bool {
		let mut result = false;
		if self.var_properties.is_some() {
			for prop in self.var_properties.as_ref().unwrap() {
				if let VarProps::Static = prop {
					result = true;
				} else {
					return false;
				}
			}
		}
		return result;
	}

	pub fn types_match(&self, other: &VariableType) -> bool {
		if other.is_inferred() {
			return true;
		}
		return self.var_type == other.var_type && self.var_optional == other.var_optional;
	}

	pub fn resolve_quantum_function(&self, params: Rc<Vec<Rc<Expression>>>) -> Result<VariableType, &'static str> {
		if self.is_quantum_function() {
			if let Type::QuantumFunction(funcs) = &self.var_type {
				let mut possible_functions = funcs.clone();
				let mut index = 0;
				for p in params.iter() {
					if possible_functions.is_empty() {
						return Err("function does not exist");
					}
					let param_type = p.get_type();
					let mut new_possible_functions = Vec::new();
					for f in possible_functions {
						if index < f.parameters.len() {
							let prop_type = &f.parameters[index].prop_type;
							if param_type.types_match(prop_type) {
								new_possible_functions.push(f.clone());
							}
						}
					}
					possible_functions = new_possible_functions;
					index += 1;
				}
				if possible_functions.len() == 1 {
					return Ok(VariableType::function(possible_functions.remove(0)));
				} else {
					return Err("function that takes these parameters doesn't exist");
				}
			}
		}
		return Err("not a quantum function");
	}

	pub fn get_function_call_return(&self) -> Option<VariableType> {
		return match &self.var_type {
			Type::Function(func_type) => {
				Some(func_type.return_type.clone())
			},
			_ => None
		}
	}

	pub fn is_int(&self) -> bool {
		if let Type::Number(num_type) = &self.var_type {
			if let NumberType::Int = num_type {
				return true;
			}
		}
		return false;
	}

	pub fn is_void(&self) -> bool {
		if let Type::Void = &self.var_type {
			return true;
		}
		return false;
	}
}

#[derive(Clone, PartialEq)]
pub enum Type {
	Unknown(String),
	Void,
	Boolean,
	Number(NumberType),
	String(StringType),
	Class(ClassType),
	Function(Box<Function>),
	QuantumFunction(Vec<Function>),
	InitializerList(Box<VariableType>),
	Tuple(Vec<VariableType>),
	Inferred,
	Undeclared(Vec<String>),
	UndeclaredWParams(Vec<String>, Vec<VariableType>),
	This
}

impl Type {
	pub fn to_cpp(&self, declare: bool) -> String {
		return match self {
			Type::Unknown(name) => name.clone(),
			Type::Void => "void".to_string(),
			Type::Boolean => "bool".to_string(),
			Type::Number(num_type) => num_type.to_cpp().to_string(),
			Type::String(string_type) => string_type.to_cpp().to_string(),
			Type::Class(class_type) => {
				if declare { format!("class {}", class_type.name.clone()) } else { class_type.name.clone() }
			},
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
			},
			Type::QuantumFunction(funcs) => {
				if !funcs.is_empty() {
					Type::Function(Box::new(funcs.first().unwrap().clone())).to_cpp(false)
				} else {
					"".to_string()
				}
			},
			Type::InitializerList(init_type) => {
				format!("std::initializer_list<{}>", init_type.to_cpp())
			}
			Type::Tuple(types) => {
				let mut is_inferred = false;
				for t in types {
					if t.is_inferred() {
						is_inferred = true;
						break;
					}
				}
				if is_inferred {
					"auto".to_string()
				} else {
					format!("std::tuple<{}>", types.iter().map(|t| t.to_cpp()).collect::<Vec<String>>().join(", "))
				}
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
				if declare { format!("class {}", result) } else { result }
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
			},
			Type::This => "this".to_string()
		}
	}

	pub fn is_inferred(&self) -> bool {
		if let Type::Inferred = self {
			return true;
		}
		return false;
	}

	pub fn is_undeclared(&self) -> bool {
		if let Type::Undeclared(..) = self {
			return true;
		}
		return false;
	}

	pub fn default_value(&self) -> Option<&'static str> {
		return match self {
			Type::Unknown(_) => None,
			Type::Void => None,
			Type::Boolean => Some("false"),
			Type::Number(_) => Some("0"),
			Type::String(_) => Some("\"\""),
			Type::Class(_) => None,
			Type::Function(_) => Some("nullptr"),
			Type::QuantumFunction(_) => Some("nullptr"),
			Type::InitializerList(_) => Some("{}"),
			Type::Tuple(_) => None,
			Type::Inferred => None,
			Type::Undeclared(_) => None,
			Type::UndeclaredWParams(_, _) => None,
			Type::This => None
		}
	}

	pub fn get_class_type(&self) -> Option<ClassType> {
		if let Type::Class(cls_type) = self {
			return Some(cls_type.clone());
		}
		return None;
	}
}

#[derive(Clone, PartialEq)]
pub enum VarStyle {
	Unknown,
	Namespace,
	Copy,
	Ref,
	Borrow,
	Move,
	Ptr(usize),
	AutoPtr,
	UniquePtr,
	ClassPtr,
	Infer
}

impl VarStyle {
	pub fn new(name: &str) -> VarStyle {
		if name.len() > 3 && name.starts_with("ptr") {
			let num_str = name[3..].parse::<usize>();
			let mut num = num_str.unwrap_or(1);
			if num < 1 { num = 1 }
			if num > 9 { num = 9 }
			return VarStyle::Ptr(num);
		}
		return match name {
			"copy" => VarStyle::Copy,
			"ref" => VarStyle::Ref,
			"borrow" => VarStyle::Borrow,
			"move" => VarStyle::Move,
			"ptr" => VarStyle::Ptr(1),
			"autoptr" => VarStyle::AutoPtr,
			"uniqueptr" => VarStyle::UniquePtr,
			"classptr" => VarStyle::ClassPtr,
			"let" => VarStyle::Infer,
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
			VarStyle::Ptr(amount) => {
				match amount { 1 => "ptr", 2 => "ptr2", 3 => "ptr3", 4 => "ptr4", 5 => "ptr5", 6 => "ptr6", 7 => "ptr7", 8 => "ptr8", 9 => "ptr9", _ => "ptr" }
			},
			VarStyle::AutoPtr => "autoptr",
			VarStyle::UniquePtr => "uniqueptr",
			VarStyle::ClassPtr => "classptr",
			VarStyle::Infer => "let",
			VarStyle::Namespace => "namespace",
			VarStyle::Unknown => "unknown"
		}
	}

	pub fn is_inferred(&self) -> bool {
		if let VarStyle::Infer = &self {
			return true;
		}
		return false;
	}

	pub fn attempt_inference(self, other: &VariableType) -> VarStyle {
		if self.is_inferred() {
			return other.var_style.clone();
		}
		return self;
	}

	pub fn to_cpp(&self, var_type: &Type, declare: bool) -> String {
		if var_type.is_inferred() {
			return "auto".to_string();
		}
		return match self {
			VarStyle::Copy => var_type.to_cpp(declare),
			VarStyle::Ref => format!("{}&", var_type.to_cpp(declare)),
			VarStyle::Borrow => {
				if let Type::String(str_type) = var_type {
					if let StringType::ConstCharArray = str_type {
						format!("{}&", var_type.to_cpp(declare))
					} else {
						format!("const {}&", var_type.to_cpp(declare))
					}
				} else {
					format!("const {}&", var_type.to_cpp(declare))
				}
			},
			VarStyle::Move => format!("{}&&", var_type.to_cpp(declare)),
			VarStyle::Ptr(amount) => {
				let stars = if *amount < 1 { 1 } else if *amount > 9 { 9 } else { *amount };
				format!("{}{}", var_type.to_cpp(declare), String::from_utf8(vec![b'*'; stars]).unwrap_or("*".to_string()))
			},
			VarStyle::AutoPtr => format!("std::shared_ptr<{}>", var_type.to_cpp(declare)),
			VarStyle::UniquePtr => format!("std::unique_ptr<{}>", var_type.to_cpp(declare)),
			VarStyle::ClassPtr => format!("{}*", var_type.to_cpp(declare)),
			_ => var_type.to_cpp(declare)
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

	pub fn is_ptr(&self) -> Option<bool> {
		return match self {
			VarStyle::Copy => Some(false),
			VarStyle::Ref => Some(false),
			VarStyle::Borrow => Some(false),
			VarStyle::Move => Some(false),
			VarStyle::Ptr(_) => Some(true),
			VarStyle::AutoPtr => Some(true),
			VarStyle::UniquePtr => Some(true),
			VarStyle::ClassPtr => Some(true),
			VarStyle::Infer => Some(false),
			VarStyle::Namespace => None,
			VarStyle::Unknown => None
		}
	}
}

#[derive(Clone, PartialEq)]
pub enum VarProps {
	Unknown,
	Const,
	Constexpr,
	Constinit,
	Extern,
	Mutable,
	Static,
	Threadlocal,
	Volatile,
	Declare
}

impl VarProps {
	pub fn new(name: &str) -> VarProps {
		return match name {
			"const" => VarProps::Const,
			"constexpr" => VarProps::Constexpr,
			"constinit" => VarProps::Constinit,
			"extern" => VarProps::Extern,
			"mutable" => VarProps::Mutable,
			"forever" => VarProps::Static,
			"thread_local" => VarProps::Threadlocal,
			"volatile" => VarProps::Volatile,
			"declare" => VarProps::Declare,
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
			VarProps::Volatile => "volatile",
			VarProps::Declare => ""
		}
	}

	pub fn is_declare(&self) -> bool {
		if let VarProps::Declare = self {
			return true;
		}
		return false;
	}
}
