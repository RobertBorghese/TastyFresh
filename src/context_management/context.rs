/**********************************************************
 * --- Context ---
 *
 * Keeps track of the header files that need to be added
 * depending on the usage of certain classes and functions.
 **********************************************************/

use crate::context_management::typing_context::TypingContext;
use crate::context_management::header_context::HeaderContext;

use crate::expression::variable_type::{ VariableType, Type, VarStyle };
use crate::expression::value_type::NumberType;

pub struct Context {
	pub typing: TypingContext,
	pub module: TypingContext,
	pub headers: HeaderContext,
	pub align_lines: bool
}

impl Context {
	pub fn new() -> Context {
		return Context {
			typing: TypingContext::new(false),
			module: TypingContext::new(true),
			headers: HeaderContext::new(),
			align_lines: false
		}
	}

	pub fn add_header(&mut self, path: &str, is_system: bool) {
		self.headers.add_header(path, is_system);
	}

	pub fn register_type(&mut self, var_type: &VariableType) {
		match &var_type.var_type {
			Type::Function(_) => self.add_header("functional", true),
			Type::Tuple(_) => self.add_header("tuple", true),
			Type::Number(num_type) => {
				match num_type {
					NumberType::Size | NumberType::WChar => self.add_header("stddef.h", true),
					_ => ()
				}
			}
			_ => ()
		}
		match &var_type.var_style {
			VarStyle::AutoPtr => self.add_header("memory", true),
			VarStyle::UniquePtr => self.add_header("memory", true),
			_ => ()
		}
	}

	pub fn register_module_attribute(&mut self, attribute: &str) {
		if attribute == "TastyAlign" {
			self.align_lines = true;
		}
	}
}
