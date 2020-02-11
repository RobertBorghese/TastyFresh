/**********************************************************
 * --- Variable Type ---
 *
 * The structs and enums defined here represent variables
 * within Tasty Fresh.
 **********************************************************/

use crate::value_type::{ NumberType, StringType, ClassType };

pub struct VariableType {
	var_type: Type,
	var_style: Style
}

pub enum Type {
	Null,
	Boolean,
	Number(NumberType),
	String(StringType),
	Class(ClassType)
}

pub enum Style {
	Copy,
	Ref,
	Borrow,
	Move,
	Ptr,
	AutoPtr,
	UniquePtr,
	ClassPtr
}