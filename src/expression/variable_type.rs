/**********************************************************
 * --- Variable Type ---
 *
 * The structs and enums defined here represent variables
 * within Tasty Fresh.
 **********************************************************/

use crate::expression::value_type::{ NumberType, StringType, ClassType };

lazy_static! {
	pub static ref STYLE_TYPES: Vec<&'static str> = vec!("copy", "ref", "borrow", "move", "ptr", "autoptr", "uniqueptr", "classptr");
}

pub struct VariableType {
	pub var_type: Type,
	pub var_style: VarStyle
}

pub enum Type {
	Unknown(String),
	Null,
	Boolean,
	Number(NumberType),
	String(StringType),
	Class(ClassType),
	Inferred,
	Undeclared(String),
	UndeclaredWParams(String, Vec<Type>)
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
