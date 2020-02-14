/**********************************************************
 * --- Variable Type ---
 *
 * The structs and enums defined here represent variables
 * within Tasty Fresh.
 **********************************************************/

use crate::expression::value_type::{ NumberType, StringType, ClassType };

pub struct VariableType {
	pub var_type: Type,
	pub var_style: Style
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

pub enum Style {
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

impl Style {
	pub fn new(name: &str) -> Style {
		return match name {
			"copy" => Style::Copy,
			"ref" => Style::Ref,
			"borrow" => Style::Borrow,
			"move" => Style::Move,
			"ptr" => Style::Ptr,
			"autoptr" => Style::AutoPtr,
			"uniqueptr" => Style::UniquePtr,
			"classptr" => Style::ClassPtr,
			_ => Style::Unknown
		}
	}

	pub fn styles() -> Vec<&'static str> {
		return vec!("copy", "ref", "borrow", "move", "ptr", "autoptr", "uniqueptr", "classptr");
	}

	pub fn get_name(&self) -> &str {
		return match self {
			Style::Unknown => "",
			Style::Copy => "copy",
			Style::Ref => "ref",
			Style::Borrow => "borrow",
			Style::Move => "move",
			Style::Ptr => "ptr",
			Style::AutoPtr => "autoptr",
			Style::UniquePtr => "uniqueptr",
			Style::ClassPtr => "classptr"
		}
	}

	pub fn is_unknown(&self) -> bool {
		return match self {
			Style::Unknown => true,
			_ => false
		}
	}
}
