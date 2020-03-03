/**********************************************************
 * --- Context ---
 *
 * Keeps track of the header files that need to be added
 * depending on the usage of certain classes and functions.
 **********************************************************/

use crate::context_management::typing_context::TypingContext;
use crate::context_management::header_context::HeaderContext;

use crate::expression::variable_type::{ VariableType, Type };

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

	pub fn add_header(&mut self, path: String, is_system: bool) {
		self.headers.add_header(path, is_system);
	}

	pub fn register_type(&mut self, var_type: &VariableType) {
		match &var_type.var_type {
			Type::Function(func) => self.add_header("functional".to_string(), true),
			Type::Tuple(types) => self.add_header("tuple".to_string(), true),
			_ => ()
		}
	}

	pub fn register_module_attribute(&mut self, attribute: &str) {
		if attribute == "TastyAlign" {
			self.align_lines = true;
		}
	}
}
